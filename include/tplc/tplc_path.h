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
#define TPLC_PATH_SEG_CIRC   2u
#define TPLC_PATH_SEG_BLEND  4u

/* Circular segment modes (TwinCAT MC_CIRC_MODE subset):
   TPLC_PATH_CIRC_CENTER — aux[] holds the circle center; it must
     be (nearly) equidistant from the segment start and end.
   TPLC_PATH_CIRC_BORDER — aux[] holds a point on the circle; the
     center is the circumcenter of start/aux/end.
   TPLC_PATH_SEG_BLEND — junction transition segment (TwinCAT
     MC_MoveBlendingPreparation): holds the path end position for
     dwell_ms milliseconds. Real velocity blending/lookahead is
     not modeled; TransitionParameter[0] supplies the time.
   The circle is evaluated in the plane of position entries 0 and
   1; entries >= 2 interpolate linearly. path_choice: 1 =
   counter-clockwise, 2 = clockwise (TwinCAT
   MC_CIRC_PATHCHOICE). */
#define TPLC_PATH_CIRC_CENTER 1u
#define TPLC_PATH_CIRC_BORDER 2u

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
    /* append a circular segment (appended at the end of the vtable
       for ABI compatibility). mode is TPLC_PATH_CIRC_*; aux[]
       holds the center (CENTER) or a border point (BORDER);
       end[] holds the absolute target positions; path_choice is
       1=CCW, 2=CW. vel/acc are trapezoid limits on arc length. */
    int32_t (*path_add_circ)(void* host, uint32_t path,
                             uint32_t mode, const double* aux,
                             uint32_t naux, const double* end,
                             uint32_t nend, double vel, double acc,
                             double path_choice);
} TplcPathService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_PATH_H */
