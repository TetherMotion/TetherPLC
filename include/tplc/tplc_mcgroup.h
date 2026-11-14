/* tplc_mcgroup.h — TetherPLC coordinated-motion group service ABI.
 *
 * TwinCAT AXES_GROUP_REF objects are PLC-side bookkeeping: axis
 * membership, enable/disable, override, setpoint source and the
 * active coordinate transform. The service keeps a fixed table of
 * groups; motion execution (MC_MovePath and friends) is layered on
 * top by the trajectory engine in a later tranche.
 *
 * Group handles are 1-based indices into the table; handle 0 is
 * invalid. Axis handles reuse the tplc-motion UDINT handles.
 * Group states approximate the TwinCAT mcGroupState enum:
 *   0 undefined/empty, 1 disabled, 2 standby (enabled, no move),
 *   3 moving, 4 error-disabled, 5 group-error.
 */
#ifndef TPLC_MCGROUP_H
#define TPLC_MCGROUP_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_MCGROUP_ABI_VERSION 1u
#define TPLC_MCGROUP_MAX_GROUPS  8u
#define TPLC_MCGROUP_MAX_AXES    16u

#define TPLC_MCGROUP_OK        0
#define TPLC_MCGROUP_E_PARAM  -1
#define TPLC_MCGROUP_E_STATE  -2   /* bad group/axis handle        */
#define TPLC_MCGROUP_E_FULL   -3   /* group table or axis list full */
#define TPLC_MCGROUP_E_STATEG -4   /* illegal group state for op    */

/* group states (approximation of TwinCAT mcGroupState) */
#define TPLC_MCGROUP_ST_EMPTY     0
#define TPLC_MCGROUP_ST_DISABLED  1
#define TPLC_MCGROUP_ST_STANDBY   2
#define TPLC_MCGROUP_ST_MOVING    3
#define TPLC_MCGROUP_ST_ERRDIS    4
#define TPLC_MCGROUP_ST_GRPERR    5

/* setpoint sources (approximation of MC3_SETPOINT_SOURCE) */
/* ECmSetpointSource mapping: Group=1->0, MCS=2->1, ACS=3->2 */
#define TPLC_MCGROUP_SRC_GROUP   0   /* MC_MovePath setpoints     */
#define TPLC_MCGROUP_SRC_MCS     1   /* Cartesian (IK active)      */
#define TPLC_MCGROUP_SRC_ACS     2   /* axis-direct (FK active)    */

typedef struct TplcMcGroupService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* ensure a group exists (idempotent); out = handle */
    int32_t (*grp_ensure)(void* host, uint32_t grp, uint32_t* out);
    /* membership ops; ident = IDENT_IN_GROUP interpretation */
    int32_t (*grp_add_axis)(void* host, uint32_t grp, uint32_t axis,
                            uint32_t ident);
    int32_t (*grp_remove_axis)(void* host, uint32_t grp, uint32_t axis,
                               uint32_t ident);
    int32_t (*grp_ungroup)(void* host, uint32_t grp);
    /* 1 = enable, 0 = disable, 2 = reset-to-disabled */
    int32_t (*grp_set_state)(void* host, uint32_t grp, int32_t op);
    /* read: state, member count, enabled flag, error id */
    int32_t (*grp_status)(void* host, uint32_t grp, int32_t* state,
                          int32_t* n_axes, int32_t* enabled,
                          int32_t* err_id);
    /* override: set requested, read back actual (clamped 0..2) */
    int32_t (*grp_set_override)(void* host, uint32_t grp, double ovr,
                                double* actual);
    /* setpoint source + coordinate transform id */
    int32_t (*grp_set_source)(void* host, uint32_t grp, int32_t src);
    int32_t (*grp_set_transform)(void* host, uint32_t grp,
                                 uint32_t coord);
    /* readback for status FBs */
    int32_t (*grp_get_source)(void* host, uint32_t grp, int32_t* src);
    int32_t (*grp_get_transform)(void* host, uint32_t grp,
                                 uint32_t* coord);
    int32_t (*grp_get_override)(void* host, uint32_t grp, double* ovr);
    /* membership snapshot: fills axes[] up to cap, returns count */
    int32_t (*grp_axes)(void* host, uint32_t grp, uint32_t* axes,
                        uint32_t cap, int32_t* n);
} TplcMcGroupService;

#ifdef __cplusplus
}
#endif
#endif /* TPLC_MCGROUP_H */
