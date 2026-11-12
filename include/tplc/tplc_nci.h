/* tplc_nci.h — TplcNciService: NCI channel bridge for the TC2 NC
 * interpreter function blocks.
 *
 * G-code interpretation lives in Tether: a tether_io_motion_server
 * compiles program text with GCode::PlanningSegmentBuilder, builds a
 * MotionPlanner::MotionPlan (corner blending + velocity profile), and
 * streams CSP setpoints to the first three CiA402 axes (0x607A) on the
 * drive clock. This vtable only transports channel commands and status
 * between the PLC and that server; there is no in-process interpreter,
 * so the service is only active when TPLC_MOTION_IO is configured.
 *
 * Channel params on the wire (see tether_io_motion_server):
 *   nc.prog / nc.prog_path (String W)   program text / file path
 *   nc.cmd      (U8 RW)  command code below
 *   nc.state    (U8 R)   state values below (GCode::InterpreterState)
 *   nc.line nc.total nc.errcode (U32 R)
 *   nc.progress nc.duration nc.scale nc.feedovr (F64)
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_NCI_H
#define TPLC_NCI_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_NCI_ABI_VERSION 1u
#define TPLC_NCI_SVC_NAME "tplc-nci"
#define TPLC_NCI_CHAN_MAX 4u

/* channel commands (nc.cmd). */
#define TPLC_NCI_CMD_RUN    1u
#define TPLC_NCI_CMD_STEP   2u
#define TPLC_NCI_CMD_PAUSE  3u
#define TPLC_NCI_CMD_RESUME 4u
#define TPLC_NCI_CMD_STOP   5u
#define TPLC_NCI_CMD_RESET  6u
#define TPLC_NCI_CMD_VERIFY 7u

/* channel states, matching GCode::InterpreterState values. */
#define TPLC_NCI_STATE_IDLE     0
#define TPLC_NCI_STATE_READY    1
#define TPLC_NCI_STATE_RUNNING  2
#define TPLC_NCI_STATE_PAUSED   3
#define TPLC_NCI_STATE_FINISHED 4
#define TPLC_NCI_STATE_ERROR    5
#define TPLC_NCI_STATE_STOPPED  6

/* service return codes. */
#define TPLC_NCI_OK        0
#define TPLC_NCI_E_PARAM  -1
#define TPLC_NCI_E_STATE  -2   /* bad channel / service inactive   */
#define TPLC_NCI_E_IO     -3   /* transport or remote failure      */

/* channel status snapshot (nci_status). */
typedef struct TplcNciStatus {
    int32_t  state;        /* TPLC_NCI_STATE_*                     */
    int32_t  finished;     /* nonzero when FINISHED/ERROR/STOPPED  */
    int32_t  errcode;      /* channel error code                   */
    uint32_t line;         /* executing source line                */
    uint32_t total;        /* parsed block count                   */
    int32_t  progress_ms;  /* plan time elapsed, milliseconds      */
    int32_t  duration_ms;  /* total plan duration, milliseconds    */
    uint32_t mfunc;        /* pending M-code, 0 = none             */
    uint32_t fifolen;      /* queued FIFO blocks                   */
    uint32_t fifoseq;      /* FIFO entries consumed this run       */
    uint32_t reserved[6];
} TplcNciStatus;           /* 64 bytes                             */

typedef struct TplcNciService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* upload program text to the channel (compiles on the server). */
    int32_t (*nci_load_text)(void* svc, uint32_t chan,
                             const char* text, uint32_t len);
    /* compile a program file that lives on the server host. */
    int32_t (*nci_load_file)(void* svc, uint32_t chan,
                             const char* path, uint32_t len);
    /* issue a channel command (TPLC_NCI_CMD_*). */
    int32_t (*nci_command)(void* svc, uint32_t chan, uint32_t cmd);
    /* snapshot channel state. */
    int32_t (*nci_status)(void* svc, uint32_t chan, TplcNciStatus* out);
    /* append one G-code block to the channel FIFO queue (nc.fifo). */
    int32_t (*nci_fifo)(void* svc, uint32_t chan,
                      const char* text, uint32_t len);
    /* confirm a pending M-function (nc.confirm); resumes a held
       channel. */
    int32_t (*nci_confirm)(void* svc, uint32_t chan);
    /* read/write one R-parameter register (nc.ridx/nc.rval);
       write!=0 stores *value, else reads into *value. */
    int32_t (*nci_rparam)(void* svc, uint32_t chan, uint32_t idx,
                          int32_t write, double* value);
    /* read/write the zero-shift (G92) offsets; write!=0 stores the
       three values, else reads them. Applied on next compile. */
    int32_t (*nci_zshift)(void* svc, uint32_t chan, int32_t write,
                          double* xyz);
} TplcNciService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_NCI_H */
