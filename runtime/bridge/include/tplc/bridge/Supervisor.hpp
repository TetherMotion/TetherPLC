/* Supervisor — spawn/respawn a tether-plc child per doc 04 §7:
 * liveness via waitpid (+ optional plc_hb staleness through the
 * image), respawn policy, restart delay, 3-per-60s rate cap.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#pragma once

#include <sys/types.h>
#include <cstdint>
#include <deque>
#include <string>
#include <vector>

namespace tplc {

enum RespawnPolicy : uint8_t { RESPAWN_NEVER = 0, RESPAWN_ON_FAULT = 1,
                               RESPAWN_ALWAYS = 2 };

struct SupervisorConfig {
    RespawnPolicy policy = RESPAWN_ON_FAULT;
    uint64_t restart_delay_ms = 1000;
    uint32_t restart_max = 3;             /* per 60 s window */
    uint64_t hb_timeout_ms = 3000;        /* 0 = hb check off */
};

class ImageServer;

class Supervisor {
public:
    explicit Supervisor(SupervisorConfig cfg) : cfg_(cfg) {}

    /* spawn tether-plc with args (no shell). Returns pid or <0. */
    pid_t spawn(const std::vector<std::string>& args);

    /* liveness tick: reap exited child, check hb staleness, decide
       respawn. Returns: 0 running, 1 respawned, 2 gave up (rate cap
       or never), 3 clean exit per policy. `kill` on hb timeout.    */
    int tick(ImageServer* img, uint64_t now_ns);

    pid_t pid() const { return pid_; }
    bool  running() const { return pid_ > 0; }
    int   lastExit() const { return last_exit_; }
    void  stop();                          /* SIGTERM + wait          */

private:
    SupervisorConfig cfg_;
    std::vector<std::string> args_;
    pid_t pid_ = -1;
    int   last_exit_ = 0;
    bool  gave_up_ = false;
    uint64_t hb_stale_since_ = 0;
    std::deque<uint64_t> restarts_;
};

} /* namespace tplc */
