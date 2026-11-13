/* tplc_fifo.h — TplcFifoService: FiFoAxes numeric setpoint FIFO
 * channel bridge for the Tc2_NcFifoAxes function blocks.
 *
 * A FiFoAxes channel is a synchronous numeric setpoint stream: the PLC
 * queues rows of LREAL target positions (one column per integrated
 * axis, group position = column) and the channel emits one row per
 * drive-clock tick as DS402 0x607A CSP targets. This is distinct from
 * the NC channel FIFO (nc.fifo), which queues G-code text blocks for
 * the interpreter.
 *
 * Two backends implement the vtable:
 *   - Tether IO (preferred): fifo.* params on tether_io_motion_server.
 *     The server owns the queues and consumes one row per mc.time
 *     tick; setpoints go straight to SimulatedCiA402Drive.
 *   - in-process: a C++ channel table in the host that consumes one
 *     row per fifo_poll call (scan-rate) and writes 0x607A through
 *     the in-process MotionService. Used when TPLC_MOTION_IO is not
 *     configured but tplc-motion resolves to the local backend.
 *
 * Wire params (tether_io_motion_server fifo.* group):
 *   fifo.chan       (U8 RW)  channel selector 0..3
 *   fifo.cmd        (U8 W)   1 start 2 stop 3 reset
 *   fifo.state      (U8 R)   0 idle 1 running 2 finished 3 error
 *   fifo.naxes      (U8 R)   integrated axis count
 *   fifo.maxrows    (U32 R)  row capacity (1000)
 *   fifo.queued     (U32 R)  queued rows
 *   fifo.seq        (U32 R)  rows consumed since start
 *   fifo.override   (F64 RW) override percent (recorded)
 *   fifo.integrate  (U32 W)  axis handle | (group pos << 16)
 *   fifo.disintegrate (U8 W) clear axes + queue
 *   fifo.write      (Bin W)  append: u32 nrows + nrows*8 f64 LE
 *   fifo.overwrite  (Bin W)  replace queue, same blob
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_FIFO_H
#define TPLC_FIFO_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_FIFO_ABI_VERSION 1u
#define TPLC_FIFO_SVC_NAME "tplc-fifo"
#define TPLC_FIFO_CHAN_MAX 4u
#define TPLC_FIFO_AXIS_MAX 8u
#define TPLC_FIFO_ROW_COLS 8u
#define TPLC_FIFO_MAX_ROWS 1000u

/* channel commands (fifo.cmd). */
#define TPLC_FIFO_CMD_START 1u
#define TPLC_FIFO_CMD_STOP  2u
#define TPLC_FIFO_CMD_RESET 3u

/* channel states (fifo.state). */
#define TPLC_FIFO_STATE_IDLE     0
#define TPLC_FIFO_STATE_RUNNING  1
#define TPLC_FIFO_STATE_FINISHED 2
#define TPLC_FIFO_STATE_ERROR    3

/* service return codes. */
#define TPLC_FIFO_OK        0
#define TPLC_FIFO_E_PARAM  -1
#define TPLC_FIFO_E_STATE  -2   /* bad channel / service inactive   */
#define TPLC_FIFO_E_IO     -3   /* transport or remote failure      */
#define TPLC_FIFO_E_FULL   -4   /* queue capacity exceeded          */

/* channel status snapshot (fifo_status). */
typedef struct TplcFifoStatus {
    int32_t  state;      /* TPLC_FIFO_STATE_*                    */
    int32_t  naxes;      /* integrated axis count                */
    uint32_t maxrows;    /* row capacity                         */
    uint32_t queued;     /* queued rows                          */
    uint32_t seq;        /* rows consumed since last start       */
    double   override;   /* channel override percent             */
    uint32_t reserved[5];
} TplcFifoStatus;        /* 56 bytes                             */

typedef struct TplcFifoService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* integrate an axis handle at a group position (column). */
    int32_t (*fifo_integrate)(void* svc, uint32_t chan,
                              uint32_t axis, uint32_t gpos);
    /* remove all integrated axes and queued rows. */
    int32_t (*fifo_disintegrate)(void* svc, uint32_t chan);
    /* append/replace row data: blob = u32 nrows + nrows*8 f64 LE,
       len = 4 + nrows*64. replace!=0 clears the queue first. */
    int32_t (*fifo_write)(void* svc, uint32_t chan,
                          const void* blob, uint32_t len,
                          int32_t replace);
    /* channel command (TPLC_FIFO_CMD_*). */
    int32_t (*fifo_command)(void* svc, uint32_t chan, uint32_t cmd);
    /* snapshot channel state. */
    int32_t (*fifo_status)(void* svc, uint32_t chan,
                           TplcFifoStatus* out);
    /* set the channel override percent. */
    int32_t (*fifo_set_override)(void* svc, uint32_t chan,
                                 double pct);
    /* advance the channel by one row when the backend is
       scan-driven (in-process); IO backends self-tick on the drive
       clock and return OK without consuming. */
    int32_t (*fifo_poll)(void* svc, uint32_t chan);
} TplcFifoService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_FIFO_H */
