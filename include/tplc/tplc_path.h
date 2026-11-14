/* tplc_path.h — TetherPLC coordinated path service ABI.
 *
 * TwinCAT MC_PATH_DATA_REF objects are PLC-side bookkeeping: an
 * ordered list of path segments (linear moves and dwells) that
 * MC_MovePath executes on an AXES_GROUP_REF. The service keeps a
 * fixed table of path objects; each object holds up to
 * TPLC_PATH_MAX_SEGS segments with up to TPLC_PATH_MAX_AXES
 * position values (axis user units, f64 storage).
 *
 * Segment types:
 *   TPLC_PATH_SEG_LINEAR — trapezoidal-velocity linear move to the
 *     stored absolute position vector (rest-to-rest per segment).
 *   TPLC_PATH_SEG_DWELL  — hold position for dwell_ms milliseconds.
 *
 * The first segment's start positions are supplied by the motion
 * FB at execute time via path_set_start (current axis positions);
 * later segments chain from the previous segment's target, which
 * gives TwinCAT's relative-to-path-start semantics for linear
 * absolute segments. Circular/blending/blocker segment types are
 * not implemented.
 *
 * Path handles are 1-based indices; handle 0 is invalid.
 */
#ifndef TPLC_PATH_H
#define TPLC_PATH_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_PATH_ABI_VERSION 1u
#define TPLC_PATH_MAX_PATHS  8u
#define TPLC_PATH_MAX_SEGS  32u
#define TPLC_PATH_MAX_AXES  16u

#define TPLC_PATH_SEG_LINEAR 1u
#define TPLC_PATH_SEG_DWELL  3u

#define TPLC_PATH_OK        0
#define TPLC_PATH_E_PARAM  -1
#define TPLC_PATH_E_STATE  -2   /* bad path handle            */
#define TPLC_PATH_E_FULL   -3   /* path/segment table full    */
#define TPLC_PATH_E_EMPTY  -4   /* eval/info on empty path    */

typedef struct TplcPathService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* allocate a path object; out = handle */
    int32_t (*path_create)(void* host, uint32_t* out);
    /* drop all segments and the start positions */
    int32_t (*path_clear)(void* host, uint32_t path);
    /* append a segment. pos[] holds n absolute target positions
       (user units) for LINEAR, ignored for DWELL; vel/acc are the
       trapezoid limits in user units/s and /s^2; dwell_ms applies
       to DWELL segments */
    int32_t (*path_add)(void* host, uint32_t path, uint32_t type,
                        const double* pos, uint32_t n, double vel,
                        double acc, double dwell_ms);
    /* set the segment-0 start positions (called once by the motion
       FB when the path is executed) */
    int32_t (*path_set_start)(void* host, uint32_t path,
                              const double* pos, uint32_t n);
    /* evaluate the path at time t seconds: fills pos[0..cap) with
       absolute axis positions, *seg with the active segment index,
       *done = 1 when t is past the path end. Returns the number of
       meaningful position entries via *n_out */
    int32_t (*path_eval)(void* host, uint32_t path, double t,
                         double* pos, uint32_t cap, uint32_t* seg,
                         int32_t* done, uint32_t* n_out);
    /* readback: segment count + total duration in seconds */
    int32_t (*path_info)(void* host, uint32_t path, uint32_t* nseg,
                         double* duration);
} TplcPathService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_PATH_H */
