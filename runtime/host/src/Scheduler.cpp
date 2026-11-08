/* Scheduler.cpp — SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/Scheduler.hpp>

#include <algorithm>

namespace tplc {

void Scheduler::addProgram(Program& prog) {
    const TplcProgramDescriptor& d = prog.desc();
    for (uint32_t i = 0; i < d.task_count; i++) {
        const TplcTaskDesc& td = d.tasks[i];
        if (!(td.flags & TPLC_TASKF_ENABLED_AT_START)) continue;
        if (td.kind != TPLC_TASK_CYCLIC && td.kind != TPLC_TASK_FREEWHEELING)
            continue;  /* v1 implements CYCLIC (+FREEWHEELING best-effort) */
        Entry e;
        e.prog = &prog; e.task_idx = i; e.desc = &td;
        e.next_due = 0;  /* first tick() stamps it */
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
       thread, cooperative) */
    std::vector<Entry*> due;
    for (auto& e : tasks_) {
        if (e.first_run) e.next_due = now_ns;      /* due immediately */
        if (e.suspended && e.step_budget == 0) continue;
        /* a step budget overrides the release clock: the op means
           "run this task once more", regardless of next_due        */
        bool is_due = (e.suspended && e.step_budget > 0) ||
                      e.desc->kind == TPLC_TASK_FREEWHEELING ||
                      now_ns >= e.next_due;
        if (is_due) due.push_back(&e);
    }
    std::sort(due.begin(), due.end(),
              [](const Entry* a, const Entry* b){
                  return a->desc->priority < b->desc->priority;
              });

    for (Entry* e : due) {
        Program& p = *e->prog;
        int64_t late = e->first_run ? 0
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

bool Scheduler::suspended(Program& prog, uint32_t task_idx) const {
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
