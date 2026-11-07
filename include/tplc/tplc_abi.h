/* tplc_abi.h — TetherPLC runtime ABI, version 1
 * Contract between host runtime (tether-plc) and a compiled program
 * module. C, not C++; no exceptions; stable across compilers.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_ABI_H
#define TPLC_ABI_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
/* _Static_assert is a C keyword; C++ spells it static_assert */
#define _Static_assert static_assert
#endif

/* ------------------------------------------------------------------ */
/* Versioning                                                          */
/* ------------------------------------------------------------------ */

#define TPLC_ABI_VERSION        1u
#define TPLC_ABI_VERSION_MAJOR(v) ((v) >> 16)
#define TPLC_ABI_VERSION_MINOR(v) ((v) & 0xFFFFu)
#define TPLC_ABI_MAKE(maj,min)    (((maj) << 16) | (min))

#define TPLC_TASK_NAME_MAX      32u
#define TPLC_PROG_NAME_MAX      64u
#define TPLC_SERVICE_NAME_MAX   32u

/* ------------------------------------------------------------------ */
/* Return codes (ABI entries return int32_t; <0 = error)               */
/* ------------------------------------------------------------------ */

enum {
    TPLC_OK                  = 0,

    /* tplc_init */
    TPLC_E_INIT_LAYOUT       = -1,   /* blob size/layout mismatch      */
    TPLC_E_INIT_INVALID      = -2,   /* bad ctx / null required field  */
    TPLC_E_INIT_VALUE        = -3,   /* init value out of range        */
    TPLC_E_INIT_RESOURCE     = -4,   /* declared resource unsatisfiable*/

    /* tplc_task_run */
    TPLC_TASK_E_FAULT        = -16,  /* generic task fault             */
    TPLC_TASK_E_PANIC        = -17,  /* module panic handler fired     */
    TPLC_TASK_E_BOUNDS       = -18,  /* internal bounds check          */
    TPLC_TASK_E_DIVZERO      = -19,  /* checked int division by zero   */
    TPLC_TASK_E_OVERFLOW     = -20,  /* checked-mode int overflow      */
    TPLC_TASK_E_RANGE        = -21,  /* subrange/enum/string range     */
    TPLC_TASK_E_ABORTED      = -22,  /* cooperative abort via ckpt     */
    TPLC_TASK_E_OVERRUN      = -23,  /* WCET budget hard-exceeded      */

    /* tplc_migrate */
    TPLC_E_MIGRATE_LAYOUT    = -32,  /* desc/blob inconsistency        */
    TPLC_E_MIGRATE_DENY      = -33,  /* deny rule; see out_status      */
    TPLC_E_MIGRATE_DESCINVAL = -34,  /* migrate_desc malformed         */
    TPLC_E_MIGRATE_BLOBRANGE = -35,  /* blob access outside declared   */

    /* loader/host surface (never returned by module code itself) */
    TPLC_E_ABI_MISMATCH      = -64,  /* abi_version != host's          */
    TPLC_E_ABI_EXT_UNMET     = -65,  /* required caps not in host caps */

    /* reserved */
    TPLC_E_MODULE_MIN        = -999,
    TPLC_E_MODULE_MAX        = -1999 /* module-defined range           */
};

/* Event/alarm codes (non-negative; never a return value) */
enum {
    TPLC_PROG_FAULT = 16             /* program faulted; doc 02 §4.6   */
};

/* TplcMigrateStatus — written into TplcMigrateCtx.out_status */
enum TplcMigrateStatus {
    TPLC_MIGRATE_OK             = 0,
    TPLC_MIGRATE_OK_DEGRADED    = 1,
    TPLC_MIGRATE_DENY           = 2,
    TPLC_MIGRATE_ERR_DESC       = 3,
    TPLC_MIGRATE_ERR_BLOB_RANGE = 4
};

/* ------------------------------------------------------------------ */
/* Flags and enums                                                     */
/* ------------------------------------------------------------------ */

/* TplcCtx.ctx_flags */
enum {
    TPLC_CTXF_SIM             = 1u << 0, /* sim host (diagnostics only) */
    TPLC_CTXF_DEBUG_SVC       = 1u << 1, /* debug named services up    */
    TPLC_CTXF_RETAIN_HINT     = 1u << 2, /* retain_dirty supported     */
    TPLC_CTXF_RETAIN_RESTORED = 1u << 3  /* retain region warm-loaded  */
};

/* TplcTaskDesc.flags */
enum {
    TPLC_TASKF_ENABLED_AT_START = 1u << 0,
    TPLC_TASKF_WATCHDOGGED      = 1u << 1
};

/* TplcTaskCtx.task_flags */
enum {
    TPLC_TASKF_FIRST_RUN  = 1u << 0,
    TPLC_TASKF_LAST_RUN   = 1u << 1,
    TPLC_TASKF_RESUME_OC  = 1u << 2  /* first run after online change  */
};

/* TplcTaskDesc.kind — TplcTaskKind */
enum TplcTaskKind {
    TPLC_TASK_CYCLIC       = 0,
    TPLC_TASK_FREEWHEELING = 1,
    TPLC_TASK_EVENT        = 2,   /* v1: declared, unimplemented      */
    TPLC_TASK_RESERVED_SFC = 3,   /* iteration-2 sequencing layer     */
    TPLC_TASK_STATUS       = 4
};

/* TplcProgramDescriptor.abi_ext capability bits */
enum TplcAbiExt {
    TPLC_EXT_MT_TASKS            = 1u << 0,
    TPLC_EXT_EVENT_TASKS         = 1u << 1,
    TPLC_EXT_SVC_TRACE           = 1u << 2,
    TPLC_EXT_MOVABLE_BETWEEN     = 1u << 3, /* always on in v1         */
    TPLC_EXT_RETAIN_ASYNC        = 1u << 4,
    TPLC_EXT_FORCE_ATOMIC64      = 1u << 5
};

/* TplcIoBinding.region */
enum TplcIoRegion {
    TPLC_REGION_IMAGE_IN   = 0,  /* %I: image→shadow before task       */
    TPLC_REGION_IMAGE_OUT  = 1,  /* %Q: shadow→image after task        */
    TPLC_REGION_MARKER     = 2   /* %M: direct map, no copy needed     */
};

/* TplcIoBinding.flags */
enum { TPLC_IOB_FORCEABLE = 1u << 0 };

/* TplcIoBinding.type_code — primitive codes for host copy semantics */
enum TplcIoType {
    TPLC_IOT_BOOL  = 0,   /* bit copy: image_offset in bits          */
    TPLC_IOT_U8  = 1, TPLC_IOT_S8  = 2,
    TPLC_IOT_U16 = 3, TPLC_IOT_S16 = 4,
    TPLC_IOT_U32 = 5, TPLC_IOT_S32 = 6,
    TPLC_IOT_U64 = 7, TPLC_IOT_S64 = 8,
    TPLC_IOT_F32 = 9, TPLC_IOT_F64 = 10,
    TPLC_IOT_RAW = 15   /* size-byte copy (structs/arrays/strings)   */
};

/* ------------------------------------------------------------------ */
/* Tables (module-const)                                               */
/* ------------------------------------------------------------------ */

typedef struct TplcIoBinding {
    uint32_t blob_offset;   /* shadow var location in state blob       */
    uint8_t  region;        /* TplcIoRegion                            */
    uint8_t  type_code;     /* TplcIoType                              */
    uint8_t  flags;
    uint8_t  unit;          /* bits: type_code==TPLC_IOT_BOOL uses     */
    uint32_t image_offset;  /* offset in region's image (bytes; bits   */
                            /* for BOOL: byte offset, unit = bit idx)  */
    uint32_t size;          /* bytes (BOOL: 1)                         */
    uint32_t _pad;
} TplcIoBinding;
_Static_assert(sizeof(TplcIoBinding) == 20, "TplcIoBinding layout");

typedef struct TplcTaskDesc {
    uint8_t  name[TPLC_TASK_NAME_MAX];
    uint8_t  kind;          /* TplcTaskKind                            */
    uint8_t  flags;
    uint8_t  priority;      /* 0..255, lower = earlier                 */
    uint8_t  _pad0;
    uint64_t period_ns;     /* CYCLIC only                             */
    uint64_t wcet_hint_ns;  /* 0 = unspecified                         */
    uint32_t entry_index;   /* into task_fns[]                         */
    uint32_t task_local_size;
} TplcTaskDesc;
_Static_assert(sizeof(TplcTaskDesc) == 64, "TplcTaskDesc layout");

/* forward decls */
typedef struct TplcCtx        TplcCtx;
typedef struct TplcTaskCtx    TplcTaskCtx;
typedef struct TplcMigrateCtx TplcMigrateCtx;
typedef int32_t (*TplcTaskFn)(TplcTaskCtx*);

typedef struct TplcProgramDescriptor {
    uint32_t struct_size;         /* sizeof(TplcProgramDescriptor)     */
    uint32_t abi_version;
    uint32_t abi_ext;             /* required caps (host must superset)*/
    uint32_t flags;               /* bit0: has_migrate                 */
    uint8_t  program_uuid[16];
    uint8_t  layout_hash[32];
    uint8_t  name[TPLC_PROG_NAME_MAX];

    uint64_t state_blob_size;
    uint64_t retain_size;
    uint64_t retain_offset;       /* within blob                       */
    uint64_t task_scratch_size;
    uint32_t image_in_bytes;
    uint32_t image_out_bytes;
    uint32_t marker_bytes;
    uint32_t _pad0;

    uint32_t task_count;
    uint32_t _pad1;
    const TplcTaskDesc* tasks;

    const TplcTaskFn* task_fns;
    uint32_t task_fn_count;
    uint32_t _pad2;

    const TplcIoBinding* io_bindings;
    uint32_t io_binding_count;
    uint32_t _pad3;
} TplcProgramDescriptor;

/* ------------------------------------------------------------------ */
/* Services (host-provided; callable from scan path, nonblocking)      */
/* ------------------------------------------------------------------ */

typedef struct TplcServices {
    uint32_t struct_size;
    uint32_t caps;                /* TplcAbiExt host supports          */

    void     (*log)(void* host, uint8_t level, uint32_t code,
                    const void* packed_args, uint32_t packed_len);
    void     (*diag_counter)(void* host, uint32_t id, int64_t delta);
    uint64_t (*steady_ns)(void* host);
    void     (*retain_dirty)(void* host, uint64_t off, uint64_t len);
    int32_t  (*raise_event)(void* host, uint32_t event_mask);
    void*    (*get_service)(void* host, const char* name, uint32_t len);

    void*    host;                /* cookie — first arg of the above   */
} TplcServices;
_Static_assert(offsetof(TplcServices, host) == 7 * sizeof(void*),
               "TplcServices layout");

/* ------------------------------------------------------------------ */
/* Contexts (host-allocated; pointers valid for call duration only)    */
/* ------------------------------------------------------------------ */

struct TplcCtx {
    uint32_t struct_size;
    uint32_t ctx_flags;           /* TPLC_CTXF_*                       */

    void*    state_blob;
    void*    retain_region;       /* == state_blob + retain_offset     */
    void*    image_in;
    void*    image_out;
    void*    markers;

    uint64_t now_ns;
    uint64_t cycle_no;
    int64_t  last_cycle_overrun_ns;

    const TplcServices* svc;
    void*    host_private;

    /* module-visible control fields (host-written, codegen-read) */
    volatile uint32_t abort_flag; /* cooperative-abort request         */
    uint32_t  _pad0;
    uint64_t  budget_deadline_ns; /* hard WCET bound for ckpt          */
};

struct TplcTaskCtx {
    TplcCtx* prog;
    uint32_t task_index;
    uint32_t task_flags;          /* TPLC_TASKF_*                      */
    void*    task_local;
    uint64_t scheduled_ns;
    uint64_t deadline_ns;
    uint64_t dt_ns;
};

struct TplcMigrateCtx {
    TplcCtx*    new_prog;

    const void* old_blob;
    uint64_t    old_blob_size;
    uint8_t     old_layout_hash[32];
    uint32_t    old_abi_version;
    uint32_t    _pad0;

    const void* migrate_desc;
    uint64_t    migrate_desc_size;

    uint32_t    out_status;       /* TplcMigrateStatus                 */
    uint32_t    out_first_error;
};

/* ------------------------------------------------------------------ */
/* Exported symbol table (the complete ABI surface)                    */
/* ------------------------------------------------------------------ */

uint32_t                      tplc_abi_version(void);
const TplcProgramDescriptor*  tplc_get_descriptor(void);
int32_t                       tplc_init(TplcCtx* ctx);
void                          tplc_shutdown(TplcCtx* ctx);
int32_t                       tplc_task_run(TplcTaskCtx* tctx);
int32_t                       tplc_migrate(TplcMigrateCtx* mctx);

/* symbol names as strings for loader tables */
#define TPLC_SYM_ABI_VERSION   "tplc_abi_version"
#define TPLC_SYM_DESCRIPTOR    "tplc_get_descriptor"
#define TPLC_SYM_INIT          "tplc_init"
#define TPLC_SYM_SHUTDOWN      "tplc_shutdown"
#define TPLC_SYM_TASK_RUN      "tplc_task_run"
#define TPLC_SYM_MIGRATE       "tplc_migrate"

#ifdef __cplusplus
} /* extern "C" */
#endif
#endif /* TPLC_ABI_H */
