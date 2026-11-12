/* tplc_sym.h — symbol table export + by-name access service.
 * Packages export `tplc_get_vars(uint32_t* count)` returning the
 * program's top-level variable table (name -> blob offset/size/
 * type).  The host resolves it optionally (a library package has
 * no table), and the tplc-sym service performs by-name reads and
 * writes against the live state blob — the transport behind
 * Tc2_DataExchange's FB_ReadAdsSymByName/FB_WriteAdsSymByName.
 * Vtable is fixed — append new fns at the end.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_SYM_H
#define TPLC_SYM_H

#include <stdint.h>

#define TPLC_SYMSVC_ABI_VERSION 1u
#define TPLC_SYM_NAME_MAX    32u
#define TPLC_SYM_MAX_VARS    256u

/* type codes match TplcIoType (tplc_abi.h) */
typedef struct TplcVarDesc {
    uint8_t  name[TPLC_SYM_NAME_MAX];
    uint32_t offset;      /* byte offset into the state blob */
    uint32_t size;        /* bytes (BOOL: 1)                 */
    uint8_t  type_code;   /* TPLC_IOT_*                      */
    uint8_t  is_at;       /* 1 = AT-bound (%I/%Q/%M)         */
    uint8_t  region;      /* TplcIoRegion when is_at         */
    uint8_t  unit;        /* bit index for BOOL AT vars      */
    uint32_t _pad;
} TplcVarDesc;
#ifdef __cplusplus
static_assert(sizeof(TplcVarDesc) == 48, "TplcVarDesc layout");
#else
_Static_assert(sizeof(TplcVarDesc) == 48, "TplcVarDesc layout");
#endif

/* package export: implemented by generated code when a PROGRAM is
 * present.  Returns the table (may be null) and writes the count. */
typedef const TplcVarDesc* (*TplcGetVarsFn)(uint32_t* count);

typedef struct TplcSymService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* count: number of top-level variables in the loaded program */
    uint32_t (*sym_count)(void*);

    /* at: enumerate — 0 and fills *out, or negative error */
    int32_t (*sym_at)(void*, uint32_t idx, TplcVarDesc* out);

    /* find: name -> 0 and fills *out; TPLC_SYM_E_NONAME otherwise */
    int32_t (*sym_find)(void*, const char* name, TplcVarDesc* out);

    /* read: copy the variable's live bytes into out (cap);
       *out_len gets the copied length. */
    int32_t (*sym_read)(void*, const char* name, uint8_t* out,
                        uint32_t cap, uint32_t* out_len);

    /* write: copy len bytes into the variable (bounded by its
       size; a short write is TPLC_SYM_E_SIZE). */
    int32_t (*sym_write)(void*, const char* name, const uint8_t* in,
                         uint32_t len);

    void* reserved[8];
} TplcSymService;

#define TPLC_SYM_OK        0
#define TPLC_SYM_E_PARAM  -1
#define TPLC_SYM_E_NONAME -2
#define TPLC_SYM_E_SIZE   -3
#define TPLC_SYM_E_NOBLOB -4

#endif /* TPLC_SYM_H */
