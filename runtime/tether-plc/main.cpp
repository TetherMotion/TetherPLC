/* tether-plc — the PLC runtime process (P0-07 slice of plan doc 04).
 * Attaches to the SHM process image (plc role), loads a program
 * module, schedules its tasks, publishes heartbeats/status, and
 * serves the control channel. SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */

#include <tplc/tplc_abi.h>
#include <tplc/tplc_image.h>
#include <tplc/tplc_ctl.h>
#include <tplc/host/Module.hpp>
#include <tplc/pkg/Package.hpp>
#include <tplc/host/Program.hpp>
#include <tplc/host/Scheduler.hpp>
#include <tplc/host/CtlServer.hpp>
#include <tplc/host/RetainStore.hpp>

#include <cerrno>
#include <csignal>
#include <cstdio>
#include <cstdint>
#include <cstring>
#include <ctime>
#include <string>
#include <unistd.h>

static volatile sig_atomic_t g_stop = 0;
static void on_sig(int) { g_stop = 1; }

static uint64_t mono_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

static void usage(const char* a0) {
    fprintf(stderr,
        "usage: %s --image <shm_path> --module <so>\n"
        "          [--ctl <uds_path>|none] [--wait-ms N] [--poll-ns N]\n"
        "          [--retain DIR] [--retain-flush-ms N] [--sim]\n",
        a0);
}

enum ProgState : uint8_t { PS_STOPPED = 0, PS_RUNNING = 1, PS_FAULT = 2 };

int main(int argc, char** argv) {
    std::string image_path, module_path, package_path, ctl_path;
    std::string retain_dir;
    uint64_t retain_flush_ms = 1000;
    int wait_ms = 10000;
    uint64_t poll_ns = 1000 * 1000;          /* 1 ms max idle         */
    uint32_t ctx_flags = 0;

    for (int i = 1; i < argc; i++) {
        std::string a = argv[i];
        auto next = [&]() -> std::string {
            return i + 1 < argc ? argv[++i] : "";
        };
        if (a == "--image") image_path = next();
        else if (a == "--module") module_path = next();
        else if (a == "--package") package_path = next();
        else if (a == "--ctl") ctl_path = next();
        else if (a == "--wait-ms") wait_ms = atoi(next().c_str());
        else if (a == "--poll-ns") poll_ns = strtoull(next().c_str(), 0, 0);
        else if (a == "--retain") retain_dir = next();
        else if (a == "--retain-flush-ms")
            retain_flush_ms = strtoull(next().c_str(), 0, 0);
        else if (a == "--sim") ctx_flags |= TPLC_CTXF_SIM;
        else if (a == "--help" || a == "-h") { usage(argv[0]); return 0; }
        else { usage(argv[0]); return 2; }
    }
    if (image_path.empty() || (module_path.empty() && package_path.empty())) {
        usage(argv[0]);
        return 2;
    }
    if (ctl_path.empty()) ctl_path = image_path + ".ctl";
    const bool ctl_enabled = (ctl_path != "none");

    /* ---------------- attach image (plc role) ---------------- */
    TplcImage img;
    int rc = tplc_image_attach(&img, image_path.c_str(), wait_ms);
    if (rc != 0) {
        fprintf(stderr, "tether-plc: attach %s failed (%d)\n",
                image_path.c_str(), rc);
        return 1;
    }
    tplc_image_set_plc_status(&img, TPLC_IMG_PLC_OPEN);

    /* ---------------- package → extracted module -------------- */
    std::string extracted_so;
    if (!package_path.empty()) {
        auto pres = tplc::Package::open(package_path);
        if (!pres.ok()) {
            fprintf(stderr, "tether-plc: package %s unreadable (%d) %s\n",
                    package_path.c_str(), pres.st.code, pres.st.msg.c_str());
            tplc_image_close(&img);
            return 1;
        }
        tplc::Package pkg = std::move(pres.value);
        auto vst = pkg.validateForLoad(false);
        if (!vst.ok()) {
            fprintf(stderr, "tether-plc: package not loadable (%d) %s\n",
                    vst.code, vst.msg.c_str());
            tplc_image_close(&img);
            return 1;
        }
        tplc::PkgSection obj{};
        /* v1: first target_obj (multi-target selection comes with
           manifest.targets[] parsing) */
        if (!pkg.section(TPLC_SEC_TARGET_OBJ, obj)) {
            fprintf(stderr, "tether-plc: no target_obj section\n");
            tplc_image_close(&img);
            return 1;
        }
        char tpath[] = "/tmp/tplc_pkg_XXXXXX";
        int fd = mkstemp(tpath);
        if (fd < 0 || write(fd, obj.data, obj.size) != (ssize_t)obj.size) {
            fprintf(stderr, "tether-plc: extract target_obj failed\n");
            if (fd >= 0) close(fd);
            tplc_image_close(&img);
            return 1;
        }
        close(fd);
        extracted_so = tpath;
        module_path = extracted_so;
        fprintf(stderr, "tether-plc: package %s → %s\n",
                package_path.c_str(), extracted_so.c_str());
    }

    /* ---------------- load module ----------------------------- */
    auto mres = tplc::Module::open(module_path, /*host_caps*/ 0);
    if (!mres.ok()) {
        fprintf(stderr, "tether-plc: module load failed (%d) %s\n",
                mres.st.code, mres.st.msg.c_str());
        tplc_image_set_plc_status(&img, TPLC_IMG_PLC_FAULT);
        tplc_image_set_fault(&img, (uint64_t)-mres.st.code);
        tplc_image_close(&img);
        return 1;
    }
    tplc::Module mod = std::move(mres.value);

    /* image must be big enough for the program's declared regions
       (doc 03 §3.1) */
    const TplcProgramDescriptor& d = mod.descriptor();
    if (d.image_in_bytes > tplc_image_in_size(&img) ||
        d.image_out_bytes > tplc_image_out_size(&img)) {
        fprintf(stderr, "tether-plc: image too small (%d)\n",
                TPLC_E_IMG_TOO_SMALL);
        tplc_image_set_fault(&img, (uint64_t)-TPLC_E_IMG_TOO_SMALL);
        tplc_image_close(&img);
        return 1;
    }

    tplc::ProgramConfig pcfg;
    pcfg.ctx_flags = ctx_flags;
    auto pres = tplc::Program::create(mod, pcfg);
    if (!pres.ok()) {
        fprintf(stderr, "tether-plc: program create failed %s\n",
                pres.st.msg.c_str());
        tplc_image_close(&img);
        return 1;
    }
    tplc::Program prog = std::move(pres.value);

    /* retain store (doc 04 §5): mirror retain_region to
       <dir>/<uuid>.bin; a uuid+layout-hash match restores bytes and
       marks ctx_flags.retain_restored so init skips retain inits */
    tplc::RetainStore retain;
    if (!retain_dir.empty()) {
        uint64_t lh;
        memcpy(&lh, d.layout_hash, 8);
        tplc::Status st = retain.open(retain_dir, d.program_uuid, lh,
                         prog.retainSize());
        if (!st.ok()) {
            fprintf(stderr, "tether-plc: retain open failed %s\n",
                    st.msg.c_str());
            tplc_image_close(&img);
            return 1;
        }
        if (retain.load(prog.retainRegion())) {
            prog.setRetainRestored();
            fprintf(stderr, "tether-plc: retain restored (%s)\n",
                    retain.path().c_str());
        }
    }
    tplc::Status st = prog.start();
    if (!st.ok()) {
        fprintf(stderr, "tether-plc: init failed (%d) %s\n",
                st.code, st.msg.c_str());
        tplc_image_set_fault(&img, (uint64_t)-st.code);
        tplc_image_set_plc_status(&img, TPLC_IMG_PLC_FAULT);
        tplc_image_close(&img);
        return 1;
    }

    /* ---------------- scheduler + hooks ------------------------ */
    tplc::Scheduler sched;
    sched.addProgram(prog);
    ProgState prog_state = PS_RUNNING;
    int32_t last_fault = 0;

    tplc::TickHooks hooks;
    hooks.latch = [&img](tplc::Program& p, uint32_t) -> int32_t {
        return tplc_image_latch_inputs(&img, p.imageIn(), 0,
                                       p.desc().image_in_bytes);
    };
    hooks.commit = [&img](tplc::Program& p, uint32_t) -> int32_t {
        return tplc_image_commit_outputs(&img, p.imageOut(), 0,
                                         p.desc().image_out_bytes);
    };
    hooks.fault = [&](tplc::Program&, uint32_t, int32_t frc) {
        /* doc 02 §4.6: program fault → stop scheduling, safe-state
           is the master's job once it sees plc_status=fault        */
        prog_state = PS_FAULT;
        last_fault = frc;
        tplc_image_set_fault(&img, (uint64_t)-frc);
        tplc_image_set_plc_status(&img, TPLC_IMG_PLC_FAULT);
        fprintf(stderr, "tether-plc: program fault rc=%d\n", frc);
    };

    /* ---------------- control channel -------------------------- */
    tplc::CtlServer ctl;
    if (ctl_enabled) {
        tplc::Status cst = ctl.open(ctl_path, 0660);
        if (!cst.ok())
            fprintf(stderr, "tether-plc: ctl %s failed: %s\n",
                    ctl_path.c_str(), cst.msg.c_str());
        /* else: keep running headless — ctl is optional per spec   */
    }

    uint64_t started_ns = mono_ns();

    ctl.on(TPLC_OP_PING, [&](int fd, uint16_t t,
                           const uint8_t*, uint32_t, uint64_t req) {
        tplc::CtlServer::W w;
        w.u64(mono_ns());
        ctl.reply(fd, t, w.data(), w.size(), req);
    });

    auto write_prog_status = [&](tplc::CtlServer::W& w) {
        TplcCtlProgStatus ps{};
        memcpy(ps.program_uuid, d.program_uuid, 16);
        memcpy(ps.name, d.name, TPLC_PROG_NAME_MAX);
        ps.state = prog_state;
        ps.task_count = d.task_count;
        ps.cycles = prog.ctx().cycle_no;
        ps.last_fault_code = last_fault;
        w.bytes(&ps, sizeof ps);
        for (uint32_t i = 0; i < d.task_count; i++) {
            const TplcTaskDesc& td = d.tasks[i];
            TplcCtlTaskStat ts{};
            memcpy(ts.name, td.name, TPLC_TASK_NAME_MAX);
            ts.kind = td.kind;
            ts.state = prog_state == PS_RUNNING ? 1 : 0;
            ts.period_ns = td.period_ns;
            if (const auto* s = sched.stats(prog, i)) {
                ts.runs = s->runs; ts.late = s->late;
                ts.faults = s->faults;
                ts.last_late_ns = (int32_t)s->max_late_ns;
            }
            w.bytes(&ts, sizeof ts);
        }
    };

    ctl.on(TPLC_OP_PROG_STATUS, [&](int fd, uint16_t t,
                                    const uint8_t*, uint32_t,
                                    uint64_t req) {
        tplc::CtlServer::W w;
        write_prog_status(w);
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_TASK_STATS, [&](int fd, uint16_t t,
                                   const uint8_t*, uint32_t,
                                   uint64_t req) {
        tplc::CtlServer::W w;
        write_prog_status(w);   /* same payload for now */
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_PROG_LIST, [&](int fd, uint16_t t,
                                  const uint8_t*, uint32_t,
                                  uint64_t req) {
        tplc::CtlServer::W w;
        w.u32(1);                       /* program count */
        w.bytes(d.program_uuid, 16);
        w.strn((const char*)d.name, TPLC_PROG_NAME_MAX);
        w.u8(prog_state); w.u8(0); w.u16(0);
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_PROG_STOP, [&](int fd, uint16_t t,
                                  const uint8_t*, uint32_t,
                                  uint64_t req) {
        /* STOP halts scheduling only — state is kept; re-init is the
           (P2) reset op's job (doc 04 lifecycle) */
        TplcCtlProgStateChange sc{(uint8_t)prog_state, PS_STOPPED, 0};
        if (prog_state == PS_RUNNING) prog_state = PS_STOPPED;
        ctl.reply(fd, t, &sc, sizeof sc, req);
    });
    ctl.on(TPLC_OP_PROG_START, [&](int fd, uint16_t t,
                                   const uint8_t*, uint32_t,
                                   uint64_t req) {
        uint8_t prev = prog_state;
        if (prog_state == PS_STOPPED) prog_state = PS_RUNNING;
        TplcCtlProgStateChange sc{prev, (uint8_t)prog_state, 0};
        ctl.reply(fd, t, &sc, sizeof sc, req);
    });
    ctl.on(TPLC_OP_IMAGE_STATS, [&](int fd, uint16_t t,
                                    const uint8_t*, uint32_t,
                                    uint64_t req) {
        TplcCtlImageStats is{};
        is.epoch       = tplc_image_epoch(&img);
        is.in_seq      = tplc_img_load_u64(&img.hdr->in_seq);
        is.out_seq     = tplc_img_load_u64(&img.hdr->out_seq);
        is.in_writes   = tplc_image_in_writes(&img);
        is.out_writes  = tplc_image_out_writes(&img);
        is.in_retries  = tplc_image_in_retries(&img);
        is.out_retries = tplc_image_out_retries(&img);
        is.in_size     = tplc_image_in_size(&img);
        is.out_size    = tplc_image_out_size(&img);
        ctl.reply(fd, t, &is, sizeof is, req);
    });
    ctl.on(TPLC_OP_RETAIN_STATUS, [&](int fd, uint16_t t,
                                      const uint8_t*, uint32_t,
                                      uint64_t req) {
        tplc::CtlServer::W w;
        w.u8(retain.attached() ? 1 : 0);
        w.u8(retain.dirty() ? 1 : 0);
        w.u16(0);
        w.u64(retain.len());
        w.u64(retain.generation());
        w.u64(retain.flushes());
        w.u64(retain.restores());
        w.u64(retain.losses());
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_RETAIN_FLUSH, [&](int fd, uint16_t t,
                                     const uint8_t*, uint32_t,
                                     uint64_t req) {
        retain.stage(prog.retainRegion());
        tplc::Status fs = retain.flush();
        if (fs.ok()) ctl.replyEmpty(fd, t, req);
        else ctl.errorReply(fd, t, fs.code, fs.msg.c_str(), req);
    });
    ctl.on(TPLC_OP_RETAIN_CLEAR, [&](int fd, uint16_t t,
                                     const uint8_t*, uint32_t,
                                     uint64_t req) {
        tplc::Status cs = retain.clear();
        if (cs.ok()) ctl.replyEmpty(fd, t, req);
        else ctl.errorReply(fd, t, cs.code, cs.msg.c_str(), req);
    });
    ctl.on(TPLC_OP_RETAIN_DUMP, [&](int fd, uint16_t t,
                                    const uint8_t*, uint32_t,
                                    uint64_t req) {
        ctl.reply(fd, t, prog.retainRegion(),
                  (uint32_t)prog.retainSize(), req);
    });
    ctl.on(TPLC_OP_RT_STATUS, [&](int fd, uint16_t t,
                                  const uint8_t*, uint32_t,
                                  uint64_t req) {
        TplcCtlRtStatus rs{};
        rs.proto_ver = TPLC_CTL_PROTO_VER;
        rs.uptime_s = (uint32_t)((mono_ns() - started_ns) / 1000000000ull);
        rs.prog_count = 1;
        rs.running_count = prog_state == PS_RUNNING ? 1 : 0;
        rs.master_status = tplc_image_master_status(&img);
        rs.plc_status = tplc_image_plc_status(&img);
        rs.master_hb_ns = tplc_image_master_hb_ns(&img);
        rs.plc_hb_ns = tplc_image_plc_hb_ns(&img);
        memcpy(rs.version, "tether-plc 0.1", 14);
        ctl.reply(fd, t, &rs, sizeof rs, req);
    });
    ctl.on(TPLC_OP_RT_SHUTDOWN, [&](int fd, uint16_t t,
                                    const uint8_t*, uint32_t,
                                    uint64_t req) {
        ctl.replyEmpty(fd, t, req);
        g_stop = 1;
    });

    /* ---------------- signals + main loop ----------------------- */
    signal(SIGTERM, on_sig);
    signal(SIGINT, on_sig);
    signal(SIGPIPE, SIG_IGN);

    uint64_t last_flush_ns = mono_ns();
    tplc_image_set_plc_status(&img, TPLC_IMG_PLC_RUN);
    fprintf(stderr, "tether-plc: running (module=%s image=%s ctl=%s)\n",
            module_path.c_str(), image_path.c_str(), ctl_path.c_str());

    while (!g_stop) {
        uint64_t now = mono_ns();

        /* master-loss policy surface (doc 03 §6.3): we keep cycling on
           held inputs — the latch returns last-good data; status goes
           degraded-announced via plc heartbeat continuing to tick     */
        uint32_t mst = tplc_image_master_status(&img);
        if (prog_state == PS_RUNNING &&
            (mst == TPLC_IMG_MASTER_RUN ||
             mst == TPLC_IMG_MASTER_DEGRADED ||
             mst == TPLC_IMG_MASTER_INIT)) {
            sched.tick(now, hooks);
        }

        /* retain: module dirty hints mark the store; staging happens
           here — between ticks, i.e. at task boundaries (doc 04 §5) */
        if (retain.attached()) {
            if (!prog.services().retainDirtyRanges().empty()) {
                prog.services().clearRetainDirty();
                retain.noteDirty();
            }
            if (retain.dirty() ||
                now - last_flush_ns >= retain_flush_ms * 1000000ull) {
                retain.stage(prog.retainRegion());
                retain.flush();
                last_flush_ns = now;
            }
        }

        tplc_image_plc_heartbeat(&img, now);

        /* wait: min(next task due, poll cap) */
        uint64_t nd = sched.nextDueNs();
        uint64_t wait_ns = poll_ns;
        if (nd != UINT64_MAX && nd > now)
            wait_ns = (nd - now) < poll_ns ? (nd - now) : poll_ns;
        int wait_ms = (int)(wait_ns / 1000000ull) + 1;
        ctl.poll(wait_ms);
    }

    fprintf(stderr, "tether-plc: stopping\n");
    prog_state = PS_STOPPED;
    tplc_image_set_plc_status(&img, TPLC_IMG_PLC_STOPPED);
    tplc_image_plc_heartbeat(&img, mono_ns());
    if (retain.attached()) {
        retain.stage(prog.retainRegion());
        retain.flush();               /* final persist on shutdown */
    }
    prog.stop();
    ctl.close();
    tplc_image_close(&img);
    return 0;
}
