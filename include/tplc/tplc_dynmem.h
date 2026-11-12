/* tplc_dynmem.h — C ABI for the tplc-dynmem host service.
 * Bounded dynamic-memory arena for the Tc3_DynamicMemory
 * surface (FB_DynMem_Buffer / FB_DynMem_Manager).  The arena
 * is a fixed compile-time region — allocation is deterministic,
 * bounded, and never touches the host heap.  Blocks are
 * addressed by small integer handles (never raw pointers), so
 * nothing leaks across the PLC ABI.
 * Vtable is fixed — append new fns at the end.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_DYNMEM_H
#define TPLC_DYNMEM_H

#include <stdint.h>

#define TPLC_DYNMEM_ABI_VERSION 1u

/* arena limits (deterministic worst case) */
#define TPLC_DYNMEM_MAX_BLOCKS   64u
#define TPLC_DYNMEM_MAX_BLOCK    (64u*1024u)
#define TPLC_DYNMEM_ARENA_BYTES  (1024u*1024u)

typedef struct TplcDynmemStats {
    uint32_t live_blocks;    /* blocks currently allocated */
    uint32_t live_bytes;     /* bytes currently allocated  */
    uint32_t total_allocs;   /* lifetime alloc calls       */
    uint32_t total_frees;    /* lifetime free calls        */
    uint32_t arena_bytes;    /* total arena size           */
    uint32_t arena_used;     /* high-water bytes in use    */
    uint8_t  reserved[32];
} TplcDynmemStats;

typedef struct TplcDynmemService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* alloc: size>0 -> block id (1..N) or negative error.
       reset!=0 zero-fills the new block. */
    int32_t (*dynmem_alloc)(void*, uint32_t size, uint32_t reset);

    /* free: id -> 0 or negative error; the block's bytes are
       returned to the arena. */
    int32_t (*dynmem_free)(void*, uint32_t id);

    /* resize: id -> 0 or negative error.  preserve!=0 keeps the
       min(old,new) prefix; reset!=0 zero-fills first. */
    int32_t (*dynmem_resize)(void*, uint32_t id, uint32_t new_size,
                             uint32_t preserve, uint32_t reset);

    /* read:  (id, offset) -> byte 0..255 or negative error */
    int32_t (*dynmem_read)(void*, uint32_t id, uint32_t offset);

    /* write: (id, offset, byte) -> 0 or negative error */
    int32_t (*dynmem_write)(void*, uint32_t id, uint32_t offset,
                            uint32_t byte);

    /* stats: -> 0 and fills *out, or negative error */
    int32_t (*dynmem_stats)(void*, TplcDynmemStats* out);

    void* reserved[8];
} TplcDynmemService;

#define TPLC_DYNMEM_OK        0
#define TPLC_DYNMEM_E_PARAM  -1
#define TPLC_DYNMEM_E_HANDLE -2
#define TPLC_DYNMEM_E_FULL   -3   /* arena exhausted          */
#define TPLC_DYNMEM_E_RANGE  -4   /* offset past block end    */

#endif /* TPLC_DYNMEM_H */
