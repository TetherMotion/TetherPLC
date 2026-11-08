/* Sandbox.cpp — process containment for tether-plc (doc 15 §3).
 *
 * Layers implemented here:
 *   rlimits — NOFILE 256, FSIZE 64 MiB (retain bound), MEMLOCK 0,
 *             CORE 0
 *             (NPROC counts all tasks of the real UID system-wide,
 *             so it only works under the dedicated deploy user —
 *             set in the unit file, not here)
 *   prctl   — PR_SET_NO_NEW_PRIVS: no privilege gain via exec/setuid
 *   seccomp — default KILL_PROCESS; whitelist of scan-cycle, ctl
 *             (epoll/UDS), file-io, retain, scheduler and loader
 *             syscalls; socket() allowed only for AF_UNIX (no
 *             network by construction — the process speaks UDS only)
 * User/namespace/FS-read-only layers are deploy-level (systemd unit /
 * container profile) — see docs/plc/plan/15-safety-security.md §3.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/Sandbox.hpp>

#include <cstdio>
#include <sys/prctl.h>
#include <sys/resource.h>
#include <sys/socket.h>

#ifdef TPLC_HAVE_SECCOMP
#include <seccomp.h>
#endif

namespace tplc {
namespace {

void setLimit(int res, rlim_t soft, rlim_t hard) {
    struct rlimit rl{soft, hard};
    if (setrlimit(res, &rl) != 0)
        perror("sandbox: setrlimit");
}

} /* namespace */

Status installSandbox() {
    setLimit(RLIMIT_NOFILE,  256, 256);
    setLimit(RLIMIT_FSIZE,   64ull << 20, 64ull << 20);
    setLimit(RLIMIT_MEMLOCK, 0, 0);
    setLimit(RLIMIT_CORE,    0, 0);
    prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);

#ifdef TPLC_HAVE_SECCOMP
    scmp_filter_ctx ctx = seccomp_init(SCMP_ACT_KILL_PROCESS);
    if (!ctx)
        return Status::Err(TPLC_E_HOST_DLOPEN, "seccomp_init failed");

    static const char* allow[] = {
        /* io / files (retain store, package extract, ctl) */
        "read","write","pread64","pwrite64","readv","writev","close",
        "lseek","fsync","fdatasync","ftruncate","fstat","newfstatat",
        "statx","getdents64","open","openat","faccessat","faccessat2",
        "access","unlink","unlinkat","mkdir","mkdirat","rename",
        "renameat","fcntl","dup","dup2","dup3","readlink","readlinkat",
        "umask","chmod","fchmod","fchmodat","fchmodat2","fchownat",
        "chdir","fchdir",
        "truncate","flock","sendfile","splice","tee","vmsplice",
        "getcwd","link","linkat","symlinkat","mknod","mknodat",
        /* memory / dynamic loader */
        "mmap","mprotect","munmap","brk","madvise","mremap","mincore",
        /* time / scheduling */
        "clock_gettime","clock_getres","clock_nanosleep","nanosleep",
        "gettimeofday","times","sched_yield","sched_getaffinity",
        "sched_setaffinity","sched_getscheduler","sched_setscheduler",
        "sched_getparam","sched_setparam","sched_get_priority_max",
        "sched_get_priority_min","sched_rr_get_interval",
        /* signals */
        "rt_sigaction","rt_sigprocmask","rt_sigpending",
        "rt_sigqueueinfo","rt_sigreturn","rt_sigsuspend",
        "rt_sigtimedwait","sigaltstack","kill","tgkill","tkill",
        "rt_tgsigqueueinfo",
        /* sync / epoll / eventfd / timerfd */
        "futex","set_robust_list","get_robust_list",
        "epoll_create","epoll_create1","epoll_ctl","epoll_wait",
        "epoll_pwait","epoll_pwait2","eventfd","eventfd2",
        "timerfd_create","timerfd_settime","timerfd_gettime",
        "signalfd","signalfd4","pselect6","ppoll","poll","select",
        /* process identity / misc */
        "getpid","gettid","getppid","getuid","geteuid","getgid",
        "getegid","getgroups","getrusage","getrlimit","prlimit64",
        "prctl","arch_prctl","set_tid_address","uname","getrandom",
        "rseq","sysinfo","restart_syscall","personality",
        /* threads + child procs — filter is inherited, children stay
           confined; process count bounded by RLIMIT_NPROC */
        "clone","clone3","fork","vfork","wait4","waitid","gettid",
        "exit","exit_group",
        /* UDS plumbing (socket itself is arg-gated below) */
        "bind","listen","accept","accept4","connect","socketpair",
        "shutdown","sendto","recvfrom","sendmsg","recvmsg",
        "getsockname","getpeername","setsockopt","getsockopt",
        /* aio/io_uring-adjacent none; fallocate for retain files */
        "fallocate","posix_fadvise","fadvise64","fadvise64_64",
    };
    int added = 0;
    for (const char* n : allow) {
        int nr = seccomp_syscall_resolve_name(n);
        if (nr < 0) continue;                     /* unknown here — skip */
        if (seccomp_rule_add(ctx, SCMP_ACT_ALLOW, nr, 0) == 0) added++;
    }
    /* socket(): UDS only — AF_INET/AF_NETLINK/etc. kill the process */
    int snr = seccomp_syscall_resolve_name("socket");
    if (snr >= 0)
        seccomp_rule_add(ctx, SCMP_ACT_ALLOW, snr, 1,
                      SCMP_A0(SCMP_CMP_EQ, AF_UNIX));
    int rc = seccomp_load(ctx);
    int rules = added;
    seccomp_release(ctx);
    if (rc != 0)
        return Status::Err(TPLC_E_HOST_DLOPEN, "seccomp_load failed");
    fprintf(stderr, "tether-plc: sandbox on (seccomp %d rules, "
                    "socket=AF_UNIX only)\n", rules);
    return Status();
#else
    fprintf(stderr, "tether-plc: sandbox limited (no libseccomp — "
                    "rlimits only)\n");
    return Status();
#endif
}

} /* namespace tplc */
