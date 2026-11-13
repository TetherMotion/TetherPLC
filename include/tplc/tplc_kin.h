/* tplc_kin.h — TplcKinService: kinematic transformation object bridge
 * for the Tc2_NcKinematicTransformation function blocks.
 *
 * A kinematic object binds a configured forward-kinematics chain
 * (cartesian, planar 2R, articulated 3R, SCARA, serial 6/7 DH,
 * gantry 5, delta, Stewart platform) to optional ACS (joint) and MCS
 * (Cartesian) axis lists plus an NC channel id. The transform itself
 * is computed by the Tether kinematics engine: forward maps joint
 * values to a Pose6D, backward runs deterministic damped
 * least-squares position IK (orientation unconstrained).
 *
 * Two backends implement the vtable:
 *   - Tether IO (preferred): kin.* params on
 *     tether_io_motion_server, which owns the ForwardKinematicsBase
 *     models. Synchronous: kin.cmd performs the transform inside the
 *     param write.
 *   - in-process: the same Tether kinematics headers compiled into
 *     the host (available when the Tether checkout is present), so
 *     deterministic sim runs need no peer.
 *
 * Wire params (tether_io_motion_server kin.* group, kin.obj selects
 * the object 0..7):
 *   kin.type      (I32 RW)  0 cartesian 1 planar2d 2 articulated3
 *                           3 scara 4 serial6 5 serial7 6 gantry5
 *                           7 delta 8 stewart (-1 unconfigured)
 *   kin.preset    (U8 RW)   serial6: 0 UR5 1 KUKA; serial7 Panda
 *   kin.cfg       (Bin RW)  u32 n + n f64 geometry (DH: 4/joint)
 *   kin.dof       (U8 R)    chain DOF
 *   kin.locked    (U8 RW)   1 = type/preset/cfg writes rejected
 *   kin.in        (Bin RW)  u32 n + n f64: joints (fwd) or
 *                           target x,y,z + seed joints (inv)
 *   kin.out       (Bin R)   f64[8]: x,y,z,roll,pitch,yaw,ikOk,0
 *   kin.q         (Bin R)   f64[8]: last joints / IK solution
 *   kin.cmd       (U8 W)    1 fwd 2 inv 3 reset 4 rot-save
 *                           5 rot-restore
 *   kin.state     (U8 R)    0 idle 1 ready 2 error
 *   kin.err       (U16 R)   0 none 1 no model 2 bad DOF
 *                           3 IK no convergence 4 bad cmd
 *   kin.acs/mcs   (Bin RW)  u32 n + n u32 slave indices
 *   kin.nacs/nmcs (U8 R)    configured axis counts
 *   kin.rot       (Bin RW)  f64[3] rotation preset rpy [rad]
 *   kin.chn       (U8 RW)   NC channel id of the kinematic group
 *   kin.chnstate  (U8 R)    channel op state
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_KIN_H
#define TPLC_KIN_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_KIN_ABI_VERSION 1u
#define TPLC_KIN_SVC_NAME "tplc-kin"
#define TPLC_KIN_OBJ_MAX 8u
#define TPLC_KIN_MAX_DIM 8u
#define TPLC_KIN_MAX_CFG 32u

/* chain types (kin.type). */
#define TPLC_KIN_T_CARTESIAN   0
#define TPLC_KIN_T_PLANAR2D    1
#define TPLC_KIN_T_ARTICULATED3 2
#define TPLC_KIN_T_SCARA       3
#define TPLC_KIN_T_SERIAL6     4
#define TPLC_KIN_T_SERIAL7     5
#define TPLC_KIN_T_GANTRY5     6
#define TPLC_KIN_T_DELTA       7
#define TPLC_KIN_T_STEWART     8

/* serial6 presets (kin.preset). */
#define TPLC_KIN_PRESET_UR5  0u
#define TPLC_KIN_PRESET_KUKA 1u

/* commands (kin.cmd). */
#define TPLC_KIN_CMD_FWD         1u
#define TPLC_KIN_CMD_INV         2u
#define TPLC_KIN_CMD_RESET       3u
#define TPLC_KIN_CMD_ROT_SAVE    4u
#define TPLC_KIN_CMD_ROT_RESTORE 5u

/* rotation ops (kin_rotation op). */
#define TPLC_KIN_ROT_GET      0u
#define TPLC_KIN_ROT_PRESET   1u
#define TPLC_KIN_ROT_SAVE     2u
#define TPLC_KIN_ROT_RESTORE  3u

/* object states (kin.state / TplcKinStatus.state). */
#define TPLC_KIN_STATE_IDLE  0
#define TPLC_KIN_STATE_READY 1
#define TPLC_KIN_STATE_ERROR 2

/* service return codes. */
#define TPLC_KIN_OK        0
#define TPLC_KIN_E_PARAM  -1
#define TPLC_KIN_E_STATE  -2   /* bad object / service inactive      */
#define TPLC_KIN_E_IO     -3   /* transport or remote failure       */
#define TPLC_KIN_E_LOCKED -4   /* object parameter lock engaged     */

/* object status snapshot (kin_status). */
typedef struct TplcKinStatus {
    int32_t  state;      /* TPLC_KIN_STATE_*                     */
    int32_t  type;       /* TPLC_KIN_T_* or -1 unconfigured      */
    int32_t  dof;        /* chain degrees of freedom             */
    int32_t  nacs;       /* ACS (joint) axis count               */
    int32_t  nmcs;       /* MCS (Cartesian) axis count           */
    uint32_t locked;     /* parameter lock flag                  */
    int32_t  err;        /* last error code                      */
    int32_t  chnstate;   /* kinematic channel op state           */
    int32_t  chn;        /* NC channel id                        */
    uint32_t reserved[4];
} TplcKinStatus;          /* 64 bytes                             */

typedef struct TplcKinService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* configure chain type/preset + geometry blob (f64[ncfg]). */
    int32_t (*kin_configure)(void* svc, uint32_t obj, int32_t type,
                             uint32_t preset, const double* cfg,
                             uint32_t ncfg);
    /* set ACS/MCS axis slave lists (NULL/0 = clear that side). */
    int32_t (*kin_set_axes)(void* svc, uint32_t obj,
                            const uint32_t* acs, uint32_t nacs,
                            const uint32_t* mcs, uint32_t nmcs);
    /* synchronous command (TPLC_KIN_CMD_*). */
    int32_t (*kin_command)(void* svc, uint32_t obj, uint32_t cmd);
    /* load transform input: fwd = dof joints; inv = target xyz +
       optional seed joints. n doubles, <= TPLC_KIN_MAX_DIM. */
    int32_t (*kin_calc_input)(void* svc, uint32_t obj,
                              const double* in, uint32_t n);
    /* read results: out8 = pose (x,y,z,roll,pitch,yaw,ikOk,0),
       q8 = joints. Either may be NULL. */
    int32_t (*kin_result)(void* svc, uint32_t obj,
                          double* out8, double* q8);
    /* parameter lock (FB_KinUnlock/LockTrafoParam). */
    int32_t (*kin_set_locked)(void* svc, uint32_t obj,
                              uint32_t locked);
    /* rotation preset/state: op TPLC_KIN_ROT_*; preset passes rot3,
       get writes rot3. */
    int32_t (*kin_rotation)(void* svc, uint32_t obj, uint32_t op,
                            const double* rot3, double* out3);
    /* object + channel status snapshot. */
    int32_t (*kin_status)(void* svc, uint32_t obj,
                          TplcKinStatus* out);
    /* associate the object with an NC channel id (0 = none). */
    int32_t (*kin_set_channel)(void* svc, uint32_t obj,
                               uint32_t chn);
    /* read back the configured ACS/MCS axis lists. acs/mcs receive up
       to TPLC_KIN_MAX_DIM u32s; nacs/nmcs the counts (each side may
       be skipped by passing NULL for both its pointers). */
    int32_t (*kin_get_axes)(void* svc, uint32_t obj,
                            uint32_t* acs, uint32_t* nacs,
                            uint32_t* mcs, uint32_t* nmcs);
} TplcKinService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_KIN_H */
