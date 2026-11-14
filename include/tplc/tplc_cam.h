/* tplc_cam.h — TetherPLC cam-object and cam-coupling service ABI.
 *
 * TwinCAT Type_CouplingFunction1D / MC_CAM_REF objects are PLC-side
 * bookkeeping: a cam table (master position -> slave position
 * points), periodicity/continuity flags and the binding of a slave
 * axis to a master axis through that table. The service keeps a
 * fixed table of cam objects and a fixed table of slave bindings;
 * axis motion stays in tplc-motion / Tether IO.
 *
 * Cam object handles are 1-based indices; handle 0 is invalid.
 * Axis handles reuse the tplc-motion UDINT handles.
 *
 * A slave may hold at most one binding: multi-cam summation
 * (MC_CamAdd on an already-coupled slave, plate blending) is not
 * implemented and cam_bind returns TPLC_CAM_E_BOUND.
 */
#ifndef TPLC_CAM_H
#define TPLC_CAM_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_CAM_ABI_VERSION 1u
#define TPLC_CAM_MAX_OBJS    16u
#define TPLC_CAM_MAX_PTS     64u
#define TPLC_CAM_MAX_BIND    16u

#define TPLC_CAM_OK        0
#define TPLC_CAM_E_PARAM  -1
#define TPLC_CAM_E_STATE  -2   /* bad cam/slave/master handle  */
#define TPLC_CAM_E_FULL   -3   /* object or binding table full */
#define TPLC_CAM_E_BOUND  -4   /* slave already has a binding   */
#define TPLC_CAM_E_NOCAM  -5   /* slave has no binding          */

/* cam_config flags */
#define TPLC_CAM_F_PERIODIC 0x1u
#define TPLC_CAM_F_CONTMASK 0x6u   /* continuity requirement << 1 */

typedef struct TplcCamService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* allocate a cam object; out = handle */
    int32_t (*cam_create)(void* host, uint32_t* out);
    /* reset point count (keeps the object allocated) */
    int32_t (*cam_clear)(void* host, uint32_t cam);
    /* write one table point; idx 0..TPLC_CAM_MAX_PTS-1; also grows
       the recorded point count to idx+1 */
    int32_t (*cam_write)(void* host, uint32_t cam, uint32_t idx,
                         double mpos, double spos);
    /* set table size + flags (periodic/continuity); npts may be 0 to
       keep the count recorded by cam_write */
    int32_t (*cam_config)(void* host, uint32_t cam, uint32_t npts,
                          uint32_t flags);
    /* interpolate the table at master position mpos (table units);
       applies periodic wrap and end clamping; result in *spos */
    int32_t (*cam_eval)(void* host, uint32_t cam, double mpos,
                        double* spos);
    /* readback: point count + flags */
    int32_t (*cam_info)(void* host, uint32_t cam, uint32_t* npts,
                        uint32_t* flags);
    /* bind slave->master through cam with coupling scalings
       (mscale/moff applied before the table lookup,
        sscale/soff after); fails E_BOUND if slave is bound */
    int32_t (*cam_bind)(void* host, uint32_t slave, uint32_t master,
                        uint32_t cam, double mscale, double moff,
                        double sscale, double soff);
    /* remove the slave's binding; cam must match when nonzero */
    int32_t (*cam_unbind)(void* host, uint32_t slave, uint32_t cam);
    /* query the slave's binding: cam/master handles + scalings;
       returns TPLC_CAM_OK with *cam = 0 when unbound */
    int32_t (*cam_bound)(void* host, uint32_t slave, uint32_t* cam,
                         uint32_t* master, double* mscale,
                         double* moff, double* sscale, double* soff);
    /* update the scalings of an existing binding */
    int32_t (*cam_scaling)(void* host, uint32_t slave, double mscale,
                           double moff, double sscale, double soff);
} TplcCamService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_CAM_H */
