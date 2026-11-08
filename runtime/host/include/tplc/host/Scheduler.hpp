/* Scheduler.hpp — single-thread cyclic scheduler (plan doc 04 §4
 * slice needed for P0: multi-rate CYCLIC tasks on one thread,
 * priority-ordered within a tick). SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_SCHEDULER_HPP
#define TPLC_HOST_SCHEDULER_HPP

#include <tplc/host/Program.hpp>

#include <cstdint>
#include <functional>
#include <vector>

namespace tplc {

struct TickHooks {
    /* wired by the caller to the image latch/commit (may be empty —
       pure-ABI runs without an image) */
    Program::IoHook latch;
    Program::IoHook commit;
    /* fault notification: called once per faulting task run */
    std::function<void(Program&, uint32_t task_idx, int32_t rc)> fault;
};

class Scheduler {
public:
    /* Register all enabled-at-start tasks of a program. Call again
       after adding a program; unregisterAll on unload. */
    void addProgram(Program& prog);
    void removeProgram(Program& prog);
    void clear() { tasks_.clear(); }

    /* Run every task whose deadline <= now_ns once, in priority order
       (lower value first). Returns tasks run. */
    size_t tick(uint64_t now_ns, const TickHooks& hooks);

    /* Next scheduled instant across all tasks (for sleep-until). */
    uint64_t nextDueNs() const;
    size_t   taskCount() const { return tasks_.size(); }

    /* Debug-plane task control (doc 12 §5): suspend deschedules at the
       next boundary; step grants n due-runs while suspended. All apply
       only when the caller is on the scan thread between ticks. */
    bool suspend(Program& prog, uint32_t task_idx);
    bool resume(Program& prog, uint32_t task_idx);
    bool step(Program& prog, uint32_t task_idx, uint32_t n);
    bool suspended(const Program& prog, uint32_t task_idx) const;

    /* Stats for the service plane */
    struct TaskStats {
        uint64_t runs = 0, late = 0, faults = 0, overruns = 0;
        int64_t  max_late_ns = 0;
        uint64_t exec_min_ns = UINT64_MAX, exec_max_ns = 0,
                 exec_total_ns = 0;
        uint64_t execAvgNs() const {
            return runs ? exec_total_ns / runs : 0;
        }
    };
    const TaskStats* stats(Program& prog, uint32_t task_idx) const;

private:
    struct Entry {
        Program* prog;
        uint32_t task_idx;
        const TplcTaskDesc* desc;
        uint64_t next_due = 0;
        uint64_t last_run = 0;
        bool     first_run = true;
        bool     suspended = false;
        uint32_t step_budget = 0;
        TaskStats stats;
    };
    std::vector<Entry> tasks_;
    /* scratch due-set, reused across ticks — a per-scan heap alloc in
       the scan loop is exactly what "no allocation after init" forbids */
    std::vector<Entry*> due_scratch_;
};

} /* namespace tplc */
#endif /* TPLC_HOST_SCHEDULER_HPP */
