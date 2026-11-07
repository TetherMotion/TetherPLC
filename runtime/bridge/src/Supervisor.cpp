#include <tplc/bridge/Supervisor.hpp>
#include <tplc/bridge/ImageServer.hpp>
#include <tplc/tplc_image.h>
#include <csignal>
#include <cstdio>
#include <sys/wait.h>
#include <unistd.h>
#include <vector>

namespace tplc {

pid_t Supervisor::spawn(const std::vector<std::string>& args) {
    args_ = args;
    pid_t pid = fork();
    if (pid == 0) {
        std::vector<char*> argv;
        for (auto& a : args_) argv.push_back(a.data());
        argv.push_back(nullptr);
        execvp(argv[0], argv.data());
        _exit(127);
    }
    if (pid < 0) return -1;
    pid_ = pid;
    hb_stale_since_ = 0;
    return pid;
}

int Supervisor::tick(ImageServer* img, uint64_t now_ns) {
    if (pid_ > 0) {
        int st;
        pid_t r = waitpid(pid_, &st, WNOHANG);
        if (r == pid_) {
            last_exit_ = WIFEXITED(st) ? WEXITSTATUS(st)
                       : WIFSIGNALED(st) ? 128 + WTERMSIG(st) : 1;
            fprintf(stderr, "supervisor: child %d exit %d\n",
                    (int)pid_, last_exit_);
            pid_ = -1;
        } else if (img && cfg_.hb_timeout_ms && img->plcStatus()) {
            /* hb staleness: a RUNNING plc must keep ticking */
            uint64_t hb = img->plcHbNs();
            if (hb && now_ns > hb) {
                uint64_t stale_ms = (now_ns - hb) / 1000000ull;
                if (stale_ms > cfg_.hb_timeout_ms) {
                    fprintf(stderr,
                            "supervisor: plc hb stale %llums — kill\n",
                            (unsigned long long)stale_ms);
                    kill(pid_, SIGKILL);
                }
            }
        }
    }
    if (pid_ > 0 || gave_up_) return pid_ > 0 ? 0 : 2;

    bool fault = last_exit_ != 0;
    bool go = cfg_.policy == RESPAWN_ALWAYS ||
              (cfg_.policy == RESPAWN_ON_FAULT && fault);
    if (!go) { gave_up_ = true; return 3; }
    while (!restarts_.empty() &&
           now_ns - restarts_.front() > 60ull * 1000 * 1000 * 1000)
        restarts_.pop_front();
    if (restarts_.size() >= cfg_.restart_max) {
        fprintf(stderr, "supervisor: restart rate cap hit\n");
        gave_up_ = true;
        return 2;
    }
    restarts_.push_back(now_ns);
    usleep((useconds_t)(cfg_.restart_delay_ms * 1000));
    spawn(args_);
    return 1;
}

void Supervisor::stop() {
    if (pid_ > 0) {
        kill(pid_, SIGTERM);
        int st;
        for (int i = 0; i < 100 && waitpid(pid_, &st, WNOHANG) == 0; i++)
            usleep(20000);
        if (waitpid(pid_, &st, WNOHANG) == 0) {
            kill(pid_, SIGKILL);
            waitpid(pid_, &st, 0);
        }
        pid_ = -1;
    }
    gave_up_ = true;
}

} /* namespace tplc */
