/* tplc_serial.h — tplc-serial service ABI.
 * Returned by TplcServices.get_service("tplc-serial"). All ops are
 * nonblocking and bounded: they run on the scan path under the task
 * WCET budget. Termios configuration happens once at open; reads and
 * writes are single nonblocking syscalls per call.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_SERIAL_H
#define TPLC_SERIAL_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_SERIAL_SERVICE_NAME "tplc-serial"
#define TPLC_SERIAL_ABI_VERSION  1u

/* serial handle: service-assigned u32, 0 = invalid                    */
typedef uint32_t TplcSerialHandle;
#define TPLC_SERIAL_HANDLE_INVALID 0u

/* op result codes (mirrored into FB nErrId on failure)                */
enum {
    TPLC_SERIAL_OK          = 0,
    TPLC_SERIAL_E_PARAM     = -1,  /* bad args (null buf, len, dev)   */
    TPLC_SERIAL_E_NOSERVICE = -2,  /* service unavailable            */
    TPLC_SERIAL_E_OPEN      = -3,  /* open()/termios failed          */
    TPLC_SERIAL_E_IO        = -4,  /* read/write syscall failed      */
    TPLC_SERIAL_E_CLOSED    = -5,  /* peer/handle closed             */
    TPLC_SERIAL_E_AGAIN     = -6,  /* would block — retry next scan  */
    TPLC_SERIAL_E_STATE     = -7,  /* op not valid in current state  */
};

/* parity codes for serial_open                                          */
enum {
    TPLC_SERIAL_PARITY_NONE = 0,
    TPLC_SERIAL_PARITY_EVEN = 1,
    TPLC_SERIAL_PARITY_ODD  = 2,
};

/* open flags                                                              */
#define TPLC_SERIAL_F_RAW       0x01  /* raw mode (default when set)  */
#define TPLC_SERIAL_F_RS485     0x02  /* RS485 half-duplex direction  */
#define TPLC_SERIAL_F_NOCTTY    0x04  /* do not acquire controlling tty */

typedef struct TplcSerialService {
    uint32_t struct_size;      /* sizeof(TplcSerialService)          */
    uint32_t abi_version;      /* TPLC_SERIAL_ABI_VERSION            */
    void*    host;             /* service-private cookie             */

    /* open device_path (e.g. "/dev/ttyS0", "/dev/pts/3") with the
       given termios parameters. baud in bit/s (e.g. 9600), databits
       5..8, parity TPLC_SERIAL_PARITY_*, stopbits 1 or 2, flags
       TPLC_SERIAL_F_*. Returns the handle via *out. The port is
       configured and opened O_NONBLOCK before this returns.        */
    int32_t (*serial_open)(void* svc, const char* device_path,
                           uint32_t baud, uint8_t databits,
                           uint8_t parity, uint8_t stopbits,
                           uint32_t flags, TplcSerialHandle* out);

    /* single nonblocking write of up to len bytes; *done receives
       the actual count. TPLC_SERIAL_E_AGAIN = nothing moved this
       scan (retry); OK with *done>0 = progress; *done==0 = nothing
       to send. Partial progress is reported, never buffered.      */
    int32_t (*serial_write)(void* svc, TplcSerialHandle h,
                            const void* buf, uint32_t len,
                            uint32_t* done);

    /* single nonblocking read of up to cap bytes into buf;
       *done receives the count (0 = no data this scan).          */
    int32_t (*serial_read)(void* svc, TplcSerialHandle h,
                           void* buf, uint32_t cap, uint32_t* done);

    /* discard queued input and/or output (tcflush). purge is a
       bitmask: 1 = input, 2 = output, 3 = both.                    */
    int32_t (*serial_flush)(void* svc, TplcSerialHandle h,
                            uint8_t purge);

    /* RS485 direction control on an already-open handle (no-op on
       ports without RS485 hardware; still returns OK).             */
    int32_t (*serial_rs485)(void* svc, TplcSerialHandle h,
                            uint8_t enable);

    /* close handle; idempotent (OK on invalid handle)               */
    int32_t (*serial_close)(void* svc, TplcSerialHandle h);
} TplcSerialService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_SERIAL_H */
