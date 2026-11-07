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

    /* Stats for the service plane */
    struct TaskStats {
        uint64_t runs = 0, late = 0, faults = 0;
        int64_t  max_late_ns = 0;
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
        TaskStats stats;
    };
    std::vector<Entry> tasks_;
};

} /* namespace tplc */
#endif /* TPLC_HOST_SCHEDULER_HPP */
