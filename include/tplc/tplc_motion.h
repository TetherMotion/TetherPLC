/* tplc_motion.h — TplcMotionService: simulated CiA402/DS402-shaped
 * axis backend for the PLCopen MC2 motion function blocks.
 *
 * Each axis is a simulated servo drive exposing the DS402 object
 * subset the MC layer needs: controlword/statusword state machine,
 * modes of operation, and target/actual position/velocity/torque.
 * The model mirrors Tether's CiA402Defs (object indices, control-
 * word/statusword bits, operating-mode values) so a later adapter
 * can map 1:1 onto tether/motion (Cia402MotionController /
 * DS402MachineAdapter) without reworking the PLC surface.
 *
 * Dynamics are deterministic: motion_axis_poll advances every
 * axis by the elapsed host clock (HostServices::steadyNs), so
 * --sim steps replay exactly. Profile modes integrate actual
 * position/velocity/torque toward the configured targets with
 * velocity/acceleration limits; cyclic-sync modes (CSP/CSV/CST)
 * apply the target directly each cycle.
 *
 * Access model: motion_axis_create returns a bounded axis handle;
 * motion_obj_read/motion_obj_write address the DS402 object
 * dictionary by index (0x6040 controlword, 0x6041 statusword,
 * 0x6060 mode, 0x6064 actual pos, 0x607A target pos, ...). Values
 * are int32; each object's native width is sign/zero-extended per
 * the DS402 definition (controlword/statusword u16, mode i8,
 * position/velocity i32, torque i16).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_MOTION_H
#define TPLC_MOTION_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_MOTION_ABI_VERSION 1u
#define TPLC_MOTION_SVC_NAME "tplc-motion"
#define TPLC_MOTION_AXIS_INVALID 0u
#define TPLC_MOTION_AXIS_MAX 16u

/* DS402 object indices exposed by the simulated axis (subset). */
#define TPLC_MOTION_OBJ_ERROR_CODE     0x603Fu  /* i32  RO */
#define TPLC_MOTION_OBJ_CONTROLWORD    0x6040u  /* u16  RW */
#define TPLC_MOTION_OBJ_STATUSWORD     0x6041u  /* u16  RO */
#define TPLC_MOTION_OBJ_MODES_OP       0x6060u  /* i8   RW */
#define TPLC_MOTION_OBJ_MODES_DISP     0x6061u  /* i8   RO */
#define TPLC_MOTION_OBJ_POS_ACTUAL     0x6064u  /* i32  RO */
#define TPLC_MOTION_OBJ_VEL_ACTUAL     0x606Cu  /* i32  RO */
#define TPLC_MOTION_OBJ_FOLLOW_WIN     0x6065u  /* u32  RW */
#define TPLC_MOTION_OBJ_TORQUE_TARGET  0x6071u  /* i16  RW */
#define TPLC_MOTION_OBJ_POS_TARGET     0x607Au  /* i32  RW */
#define TPLC_MOTION_OBJ_VEL_TARGET     0x60FFu  /* i32  RW */
#define TPLC_MOTION_OBJ_TORQUE_ACTUAL  0x6077u  /* i16  RO */
#define TPLC_MOTION_OBJ_HOME_OFFSET    0x607Cu  /* i32  RW (sim ext) */
#define TPLC_MOTION_OBJ_PROBE_ABORT    0x2001u  /* i32  RW (sim ext) */
#define TPLC_MOTION_OBJ_POS_OFFSET     0x2002u  /* i32  RW (sim ext) */
#define TPLC_MOTION_OBJ_BRAKE_CMD      0x2003u  /* i32  RW (sim ext) */
#define TPLC_MOTION_OBJ_PARK_CMD       0x2004u  /* i32  RW (sim ext) */
#define TPLC_MOTION_OBJ_COE_CMD        0x2005u  /* i32  RW (sim ext) */
#define TPLC_MOTION_OBJ_SAW_CHAR0      0x2006u  /* i32  RW (sim ext) */

/* CiA402 operating-mode values (0x6060). */
#define TPLC_MOTION_MODE_NONE   0
#define TPLC_MOTION_MODE_PP     1   /* profile position        */
#define TPLC_MOTION_MODE_VL     2   /* velocity                */
#define TPLC_MOTION_MODE_PV     3   /* profile velocity        */
#define TPLC_MOTION_MODE_TQ     4   /* profile torque          */
#define TPLC_MOTION_MODE_HM     6   /* homing                  */
#define TPLC_MOTION_MODE_CSP    8   /* cyclic sync position    */
#define TPLC_MOTION_MODE_CSV    9   /* cyclic sync velocity    */
#define TPLC_MOTION_MODE_CST    10  /* cyclic sync torque      */

/* controlword bits (0x6040). */
#define TPLC_MOTION_CW_SWITCHON        0x0001u
#define TPLC_MOTION_CW_ENABLE_VOLTAGE  0x0002u
#define TPLC_MOTION_CW_QUICKSTOP       0x0004u  /* 0 = quick stop */
#define TPLC_MOTION_CW_ENABLE_OP       0x0008u
#define TPLC_MOTION_CW_NEW_SETPOINT    0x0010u
#define TPLC_MOTION_CW_CHANGE_SET_IMM  0x0020u
#define TPLC_MOTION_CW_ABS_REL         0x0040u
#define TPLC_MOTION_CW_FAULT_RESET     0x0080u
#define TPLC_MOTION_CW_HALT            0x0100u

/* statusword bits (0x6041). */
#define TPLC_MOTION_SW_READY_TO_SWITCH 0x0001u
#define TPLC_MOTION_SW_SWITCHED_ON     0x0002u
#define TPLC_MOTION_SW_OP_ENABLED      0x0004u
#define TPLC_MOTION_SW_FAULT           0x0008u
#define TPLC_MOTION_SW_VOLTAGE_ENABLED 0x0010u
#define TPLC_MOTION_SW_QUICKSTOP       0x0020u
#define TPLC_MOTION_SW_SWITCHON_DIS    0x0040u
#define TPLC_MOTION_SW_TARGET_REACHED  0x0400u
#define TPLC_MOTION_SW_FOLLOWING_ERR   0x2000u

/* axis config (motion_axis_create). All limits in user units;
   velocity units/sec, acceleration units/sec^2. pos_min/pos_max
   are software position limits (ignored unless flags bit0). */
typedef struct TPLC_MotionAxisCfg {
    int32_t  units_per_rev;    /* position units per revolution   */
    int32_t  max_velocity;     /* units/sec, >0                   */
    int32_t  max_accel;        /* units/sec^2, >0                */
    int32_t  max_decelerate;   /* units/sec^2, >0                */
    int32_t  pos_min;          /* software position limit low    */
    int32_t  pos_max;          /* software position limit high   */
    int32_t  home_offset;      /* homing target (HM mode)        */
    int32_t  follow_window;    /* target-reached window, units   */
    uint32_t flags;            /* bit0 = enforce pos limits      */
    uint32_t reserved[7];
} TPLC_MotionAxisCfg;          /* 64 bytes                       */

/* service return codes */
#define TPLC_MOTION_OK        0
#define TPLC_MOTION_E_PARAM  -1
#define TPLC_MOTION_E_STATE  -2   /* bad axis handle / object       */
#define TPLC_MOTION_E_RANGE  -3   /* value out of range / limit     */
#define TPLC_MOTION_E_FULL   -4   /* axis pool exhausted            */
#define TPLC_MOTION_E_MODE   -5   /* mode not supported by axis     */

/* scalar axis status snapshot (motion_axis_info) */
typedef struct TPLC_MotionAxisInfo {
    int32_t  status;        /* DS402 state (CiA402State value) */
    int32_t  mode;          /* modes-of-operation-display      */
    int32_t  fault;         /* nonzero = fault code active     */
    int32_t  target_reached;/* statusword bit10                */
    int32_t  busy;          /* motion in progress              */
    int32_t  pos_actual;    /* position actual value           */
    int32_t  vel_actual;    /* velocity actual value           */
    uint32_t reserved[9];
} TPLC_MotionAxisInfo;       /* 64 bytes                        */

typedef struct TplcMotionService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* create a simulated axis; returns TPLC_MOTION_OK and a live
       axis handle (1..TPLC_MOTION_AXIS_MAX). */
    int32_t (*motion_axis_create)(void* svc,
                                  const TPLC_MotionAxisCfg* cfg,
                                  uint32_t* out_axis);
    /* destroy an axis, releasing its handle. */
    int32_t (*motion_axis_destroy)(void* svc, uint32_t axis);
    /* advance every axis's dynamics by the elapsed host clock.
       Call once per scan (the MC layer does this); also invoked
       implicitly by reads/writes so standalone callers stay
       correct. */
    int32_t (*motion_axis_poll)(void* svc);
    /* scalar status snapshot for an axis. */
    int32_t (*motion_axis_info)(void* svc, uint32_t axis,
                                TPLC_MotionAxisInfo* info);
    /* read a DS402 object by index; returns the sign/zero-extended
       value per the object's native width. */
    int32_t (*motion_obj_read)(void* svc, uint32_t axis,
                               uint32_t index, int32_t* out);
    /* write a DS402 object by index (controlword, mode, targets,
       follow window, home offset). Returns TPLC_MOTION_OK or a
       negative code. */
    int32_t (*motion_obj_write)(void* svc, uint32_t axis,
                                uint32_t index, int32_t value);
} TplcMotionService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_MOTION_H */
