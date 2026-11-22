/* Scheduler.cpp — SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/Scheduler.hpp>

#include <algorithm>
#include <cstdio>
#include <cstring>

namespace tplc {

void Scheduler::addProgram(Program& prog) {
    const TplcProgramDescriptor& d = prog.desc();
    uint32_t nev = 0;
    const TplcTaskEvent* evs =
        d.abi_ext & TPLC_EXT_EVENT_TASKS
            ? prog.module().taskEvents(&nev)
            : nullptr;
    for (uint32_t i = 0; i < d.task_count; i++) {
        const TplcTaskDesc& td = d.tasks[i];
        if (!(td.flags & TPLC_TASKF_ENABLED_AT_START)) continue;
        if (td.kind != TPLC_TASK_CYCLIC &&
            td.kind != TPLC_TASK_FREEWHEELING &&
            td.kind != TPLC_TASK_EVENT)
            continue;  /* SFC/reserved kinds not implemented */
        Entry e;
        e.prog = &prog; e.task_idx = i; e.desc = &td;
        e.next_due = 0;  /* first tick() stamps it */
        if (td.kind == TPLC_TASK_EVENT && evs) {
            for (uint32_t k = 0; k < nev; k++)
                if (strncmp((const char*)evs[k].task,
                            (const char*)td.name,
                            TPLC_TASK_NAME_MAX) == 0) {
                    e.ev = &evs[k];
                    break;
                }
        }
        tasks_.push_back(e);
    }
}

void Scheduler::removeProgram(Program& prog) {
    tasks_.erase(std::remove_if(tasks_.begin(), tasks_.end(),
                 [&](const Entry& e){ return e.prog == &prog; }),
                 tasks_.end());
}

size_t Scheduler::tick(uint64_t now_ns, const TickHooks& hooks) {
    /* collect due, dispatch in priority order (doc 04 §4: single
       thread, cooperative) — scratch vector keeps its capacity
       across ticks so steady-state scans never hit the heap       */
    due_scratch_.clear();
    std::vector<Entry*>& due = due_scratch_;
    for (auto& e : tasks_) {
        if (e.first_run && e.desc->kind != TPLC_TASK_EVENT)
            e.next_due = now_ns;      /* due immediately */
        if (e.suspended && e.step_budget == 0) continue;
        /* a step budget overrides the release clock: the op means
           "run this task once more", regardless of next_due        */
        bool is_due = (e.suspended && e.step_budget > 0);
        if (!is_due && !e.suspended) {
            if (e.desc->kind == TPLC_TASK_EVENT)
                is_due = eventDue(e);
            else
                is_due = e.desc->kind == TPLC_TASK_FREEWHEELING ||
                         now_ns >= e.next_due;
        }
        if (is_due) due.push_back(&e);
    }
    std::sort(due.begin(), due.end(),
              [](const Entry* a, const Entry* b){
                  return a->desc->priority < b->desc->priority;
              });

    for (Entry* e : due) {
        Program& p = *e->prog;
        bool is_event = e->desc->kind == TPLC_TASK_EVENT;
        int64_t late = (e->first_run || is_event) ? 0
                       : (int64_t)now_ns - (int64_t)e->next_due;
        int32_t rc = p.executeTask(e->task_idx, now_ns,
                                   hooks.latch, hooks.commit);
        uint64_t exec = p.lastExecNs();
        e->stats.exec_total_ns += exec;
        if (exec < e->stats.exec_min_ns) e->stats.exec_min_ns = exec;
        if (exec > e->stats.exec_max_ns) e->stats.exec_max_ns = exec;
        e->stats.runs++;
        if (e->first_run) e->first_run = false;
        /* stepped runs are not releases: a stale next_due must not
           pollute the late stats                                   */
        if (!e->suspended && late > 0) {
            e->stats.late++;
            if (late > e->stats.max_late_ns) e->stats.max_late_ns = late;
        }
        if (rc != 0) {
            e->stats.faults++;
            if (hooks.fault) hooks.fault(p, e->task_idx, rc);
        }
        e->last_run = now_ns;
        if (is_event) e->ev_pending = false;  /* manual fire consumed */
        if (e->step_budget > 0) e->step_budget--;
        /* stepped runs leave next_due untouched — they are not real
           releases; resume() re-arms the schedule                  */
        if (!e->suspended && e->desc->kind == TPLC_TASK_CYCLIC) {
            e->next_due += e->desc->period_ns;     /* phase-stable */
            /* overrun (doc 04 §3.2): missed releases are skipped,
               never made up; re-align to the next future phase
               boundary so a single lag does not leave the task
               permanently due (running at scan rate)              */
            if (now_ns >= e->next_due) {
                e->stats.overruns++;
                uint64_t lag = now_ns - e->next_due;
                e->next_due +=
                    (lag / e->desc->period_ns + 1) *
                    e->desc->period_ns;
            }
        } else if (!e->suspended) {
            e->next_due = now_ns;
        }
    }
    return due.size();
}

bool Scheduler::suspend(Program& prog, uint32_t task_idx) {
    for (auto& e : tasks_)
        if (e.prog == &prog && e.task_idx == task_idx) {
            e.suspended = true;
            return true;
        }
    return false;
}

bool Scheduler::resume(Program& prog, uint32_t task_idx) {
    for (auto& e : tasks_)
        if (e.prog == &prog && e.task_idx == task_idx) {
            e.suspended = false;
            e.step_budget = 0;
            /* re-arm schedule (doc 12 §5): next release is one full
               period out — no catch-up of releases missed while
               suspended */
            e.next_due = 0;
            e.first_run = true;
            return true;
        }
    return false;
}

bool Scheduler::step(Program& prog, uint32_t task_idx, uint32_t n) {
    for (auto& e : tasks_)
        if (e.prog == &prog && e.task_idx == task_idx) {
            if (!e.suspended) return false;   /* step only while held */
            e.step_budget += n;
            return true;
        }
    return false;
}

/* ---- event tasks ------------------------------------------------ */

bool Scheduler::eventDue(Entry& e) {
    if (e.ev_pending) return true;   /* manual raise (raiseEvent)   */
    const TplcTaskEvent* ev = e.ev;
    if (!ev) return false;
    if (ev->trig == TPLC_TRIG_MANUAL) return false;
    char sig[TPLC_TASK_EVENT_SIG_MAX + 1];
    memcpy(sig, ev->signal, TPLC_TASK_EVENT_SIG_MAX);
    sig[TPLC_TASK_EVENT_SIG_MAX] = 0;
    bool cur = false;
    if (!readSignal(*e.prog, sig, cur)) cur = false;
    if (ev->trig == TPLC_TRIG_LEVEL) {
        e.ev_prev = cur;
        return cur;
    }
    /* RISE: fire once per false→true transition */
    bool due = cur && !e.ev_prev;
    e.ev_prev = cur;
    return due;
}

/* Signal resolution: "%IX<b>.<bit>"/"%QX<b>.<bit>" reads an image
   bit directly; anything else is a sym-table name ("PROG.var",
   "var", "GVL.var") resolved through the program's SymService —
   AT-bound vars read the live image region, the rest the state
   blob. Value truthiness: BOOL/bit → the bit; everything else →
   any nonzero byte (covers int/real bit patterns).               */
bool Scheduler::readSignal(Program& p, const char* sig,
                           bool& out) const {
    out = false;
    if (!sig || !*sig) return false;
    if (sig[0] == '%') {
        char region = 0;
        unsigned byte = 0, bit = 0;
        if (std::sscanf(sig, "%%%cX%u.%u", &region, &byte, &bit) != 3 ||
            bit > 7)
            return false;
        const uint8_t* img =
            (region == 'I' || region == 'i') ? p.imageIn()
            : (region == 'Q' || region == 'q') ? p.imageOut()
            : nullptr;
        if (!img) return false;
        out = (img[byte] >> bit) & 1u;
        return true;
    }
    const TplcVarDesc* v = p.services().symbols().findVar(sig);
    if (!v) return false;
    const uint8_t* base;
    uint32_t off, size;
    bool bit_sel = false;
    uint32_t bit_idx = 0;
    if (v->is_at) {
        base = v->region == TPLC_REGION_IMAGE_IN ? p.imageIn()
             : v->region == TPLC_REGION_IMAGE_OUT ? p.imageOut()
             : p.markers();
        off = v->image_offset;
        if (v->type_code == TPLC_IOT_BOOL) {
            bit_sel = true;
            bit_idx = v->unit;
        }
        size = v->size;
    } else {
        base = p.blob();
        off = v->offset;
        size = v->size;
    }
    if (bit_sel) {
        out = bit_idx < 8 && ((base[off] >> bit_idx) & 1u) != 0;
        return true;
    }
    uint32_t n = size < 8 ? size : 8;
    for (uint32_t i = 0; i < n; i++)
        if (base[off + i]) { out = true; break; }
    return true;
}

bool Scheduler::raiseEvent(Program& prog, uint32_t task_idx) {
    for (auto& e : tasks_)
        if (e.prog == &prog && e.task_idx == task_idx) {
            if (e.desc->kind != TPLC_TASK_EVENT) return false;
            e.ev_pending = true;
            return true;
        }
    return false;
}

bool Scheduler::suspended(const Program& prog, uint32_t task_idx) const {
    for (const auto& e : tasks_)
        if (e.prog == &prog && e.task_idx == task_idx) return e.suspended;
    return false;
}

uint64_t Scheduler::nextDueNs() const {
    uint64_t best = UINT64_MAX;
    for (const auto& e : tasks_)
        if (!(e.suspended && e.step_budget == 0) &&
            e.desc->kind == TPLC_TASK_CYCLIC && e.next_due < best)
            best = e.next_due;
    return best;
}

const Scheduler::TaskStats*
Scheduler::stats(Program& prog, uint32_t task_idx) const {
    for (const auto& e : tasks_)
        if (e.prog == &prog && e.task_idx == task_idx) return &e.stats;
    return nullptr;
}

} /* namespace tplc */
