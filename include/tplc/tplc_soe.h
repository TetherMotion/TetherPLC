/* tplc_soe.h — C ABI for the tplc-soe EtherCAT SoE (Sercos-over-
 * EtherCAT) host service.  Bridges the ec.soe.* params exposed by a
 * Tether IO endpoint (tether_io_motion_server sim contract; a real
 * master would bridge the drive's SoE mailbox services S-0-xxxx under
 * the same names).  Endpoint from TPLC_EC_IO, falling back to
 * TPLC_MOTION_IO.  SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_SOE_H
#define TPLC_SOE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_SOE_ABI_VERSION 1u
#define TPLC_SOE_DIAG_LIST 8u
#define TPLC_SOE_DIAGMSG 64u

/* decoded drive state snapshot (ec.soe.state blob) */
typedef struct TplcSoeState {
    uint16_t status;            /* S-0-0135: bit0 ready, bit1 enabled,
                                   bit2 fault, bit3 warning */
    uint16_t cmd_state;         /* 0 idle, 1 running, 2 done, 3 error */
    uint32_t password;          /* S-0-0267 readback */
    uint16_t diag_num;          /* diagnosis number */
    uint16_t classx;            /* class-X diag flags */
    uint16_t diag_count;        /* valid entries in diag_list */
    uint16_t diag_list[TPLC_SOE_DIAG_LIST];
    float    amp_temp;          /* amplifier temperature */
    float    motor_temp;        /* motor temperature */
    float    dc_volt;           /* DC bus voltage */
    float    dc_curr;           /* DC bus current */
    float    ax_mains;          /* AX5000 P-0-0200 mains voltage */
    uint16_t ax_ctrl;           /* AX5000 motor ctrl word readback */
    uint16_t ax_park;           /* AX5000 park state (0/1) */
    uint16_t ax_fw_rev;         /* AX5000 firmware revision */
    uint16_t ax_fw_build;       /* AX5000 firmware build */
    uint16_t cmd_ctrl;          /* last command-control write */
    char     diagmsg[TPLC_SOE_DIAGMSG]; /* S-0-0095 Sercos string */
} TplcSoeState;

/* ---- service vtable ---- */
typedef struct TplcSoeService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* read the full state snapshot for drive slave_addr (0..15) */
    int32_t (*soe_read)(void*, uint32_t slave_addr, TplcSoeState* st);

    /* mailbox writes (op ids match the ec.soe.cmd blob) */
    int32_t (*soe_write_password)(void*, uint32_t slave_addr,
                                  uint32_t password);
    int32_t (*soe_write_cmd_ctrl)(void*, uint32_t slave_addr,
                                  uint32_t cmd_ctrl);
    int32_t (*soe_exec_command)(void*, uint32_t slave_addr,
                                uint32_t command_id);
    int32_t (*soe_reset)(void*, uint32_t slave_addr);
    int32_t (*soe_write_ax_ctrl)(void*, uint32_t slave_addr,
                                 uint32_t ctrl_word);
    int32_t (*soe_write_ax_park)(void*, uint32_t slave_addr,
                                 uint32_t park);

    void* reserved[6];
} TplcSoeService;

/* status codes */
#define TPLC_SOE_OK          0
#define TPLC_SOE_E_PARAM    -1
#define TPLC_SOE_E_NOEP     -2   /* no endpoint configured/reachable */
#define TPLC_SOE_E_IO       -3

#ifdef __cplusplus
}
#endif
#endif /* TPLC_SOE_H */
