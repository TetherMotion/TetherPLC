/* tplc_image.h — SHM process-image header and API (plan doc 03).
 * Single source of truth for the SHM file layout shared by the Tether
 * master side (image creator/writer of %I) and the tether-plc runtime
 * (attacher/writer of %Q).
 *
 * Header fields are plain integers; ALL access to shared fields goes
 * through the tplc_img_* accessors which apply __atomic_* builtins with
 * explicit memory orders — portable across C and C++ with semantics
 * identical to _Atomic fields (deviation recorded in
 * docs/plc/DECISIONS.md).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_IMAGE_H
#define TPLC_IMAGE_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#define _Static_assert static_assert
#endif

/* ------------------------------------------------------------------ */
/* File layout (plan doc 03 §3)                                        */
/* ------------------------------------------------------------------ */

#define TPLC_IMAGE_MAGIC   0x47414D49434C5054ull /* 'TPLCIMAG' LE      */
#define TPLC_IMAGE_VERSION 1u
#define TPLC_IMAGE_HDRSIZE 256u
#define TPLC_IMAGE_MAX_REGION_BYTES (64u * 1024u) /* v1 cap per area   */
#define TPLC_IMAGE_MAX_TRIES 8u                    /* seqlock bound    */

typedef struct TplcImageHeader {
    uint64_t magic;                     /* TPLC_IMAGE_MAGIC           */
    uint32_t version;
    uint32_t header_size;
    uint32_t layout_flags;              /* bit0: BE? must be 0 on v1  */
    uint32_t _rsvd;
    uint64_t image_epoch;               /* bump on master re-init     */
    uint64_t in_seq;                    /* seqlock: %I (master writes)*/
    uint64_t out_seq;                   /* seqlock: %Q (plc writes)   */
    uint64_t master_hb_ns;              /* monotonic heartbeat        */
    uint64_t plc_hb_ns;
    uint32_t master_status;             /* TplcImageMasterStatus      */
    uint32_t plc_status;                /* TplcImagePlcStatus         */
    uint64_t in_off, in_size;           /* absolute off/size of %I    */
    uint64_t out_off, out_size;         /* absolute off/size of %Q    */
    uint64_t in_writes;                 /* master write counter       */
    uint64_t out_writes;                /* plc write counter          */
    uint64_t in_retries;                /* latch retries (plc side)   */
    uint64_t out_retries;               /* latch retries (master)     */
    uint64_t last_fault_code;           /* plc writes; master reads   */
    uint64_t sim_time_ns;               /* sim clock (master writes,  */
                                        /* plc reads when CTXF_SIM)   */
    uint8_t  reserved[104];             /* expansion (per-prog table) */
} TplcImageHeader;
_Static_assert(sizeof(TplcImageHeader) == TPLC_IMAGE_HDRSIZE,
               "image header size");

/* ------------------------------------------------------------------ */
/* Status values                                                       */
/* ------------------------------------------------------------------ */

enum TplcImageMasterStatus {
    TPLC_IMG_MASTER_INIT     = 0,
    TPLC_IMG_MASTER_RUN      = 1,
    TPLC_IMG_MASTER_DEGRADED = 2,
    TPLC_IMG_MASTER_STOPPING = 3
};

enum TplcImagePlcStatus {
    TPLC_IMG_PLC_OPEN    = 0,
    TPLC_IMG_PLC_RUN     = 1,
    TPLC_IMG_PLC_FAULT   = 2,
    TPLC_IMG_PLC_STOPPED = 3,
    TPLC_IMG_PLC_SAFE    = 4
};

/* Image-layer return codes (negative; disjoint from ABI codes) */
enum {
    TPLC_E_IMG_MAGIC         = -100,
    TPLC_E_IMG_VERSION       = -101,
    TPLC_E_IMG_TOO_SMALL     = -102,
    TPLC_E_IMG_SEQ_EXHAUSTED = -103,
    TPLC_E_IMG_EPOCH         = -104,
    TPLC_E_IMG_RANGE         = -105,
    TPLC_E_IMG_IO            = -106,   /* syscall failure             */
    TPLC_E_IMG_STATE         = -107    /* wrong role/phase for op     */
};

/* Event codes (non-negative; journal/alarm identifiers, not rc) */
enum {
    TPLC_IMG_PLC_LOST    = 1,          /* master observes dead plc    */
    TPLC_IMG_MASTER_LOST = 2           /* plc observes dead master    */
};

/* ------------------------------------------------------------------ */
/* Inline atomic accessors (both roles; explicit memory orders)        */
/* ------------------------------------------------------------------ */

static inline uint64_t tplc_img_load_u64(const uint64_t* p) {
    return __atomic_load_n(p, __ATOMIC_ACQUIRE);
}
static inline void tplc_img_store_u64(uint64_t* p, uint64_t v) {
    __atomic_store_n(p, v, __ATOMIC_RELEASE);
}
static inline uint32_t tplc_img_load_u32(const uint32_t* p) {
    return __atomic_load_n(p, __ATOMIC_ACQUIRE);
}
static inline void tplc_img_store_u32(uint32_t* p, uint32_t v) {
    __atomic_store_n(p, v, __ATOMIC_RELEASE);
}

/* ------------------------------------------------------------------ */
/* Handle + API (implemented in runtime/image/tplc_image.c)            */
/* ------------------------------------------------------------------ */

typedef struct TplcImage {
    int      fd;
    uint8_t* base;                      /* mmap base                  */
    uint64_t map_size;
    int      is_creator;                /* master role iff nonzero    */
    TplcImageHeader* hdr;               /* == base                    */
    uint8_t* in_area;                   /* == base + hdr->in_off      */
    uint8_t* out_area;                  /* == base + hdr->out_off     */
} TplcImage;

/* Master role: create+size the file and publish the header.
 * Returns 0 or TPLC_E_IMG_*. path is a POSIX shm path ("/tether-...") */
int  tplc_image_create(TplcImage* img, const char* path,
                       uint32_t in_size, uint32_t out_size);

/* PLC role: open+map an existing image, validating header fields.
 * wait_ms < 0 blocks forever waiting for the file to appear; 0 = one
 * try; >0 polls that many ms. Returns 0 or TPLC_E_IMG_*. */
int  tplc_image_attach(TplcImage* img, const char* path, int wait_ms);

void tplc_image_close(TplcImage* img);
int  tplc_image_unlink(const char* path);   /* master only, teardown  */

/* Writer ops — single-writer per direction (plan doc 03 §4):
 *   write_inputs:    master writes a %I range under in_seq bracket
 *   commit_outputs:  plc writes a %Q range under out_seq bracket      */
int  tplc_image_write_inputs(TplcImage* img, const void* src,
                             uint64_t off, uint64_t len);
int  tplc_image_commit_outputs(TplcImage* img, const void* src,
                               uint64_t off, uint64_t len);

/* Reader ops — bounded-retry seqlock latch:
 *   latch_inputs:    plc copies a %I range (returns E_IMG_SEQ_EXHAUSTED
 *                    or E_IMG_EPOCH on epoch change mid-read)
 *   read_outputs:    master copies a %Q range                           */
int  tplc_image_latch_inputs(TplcImage* img, void* dst,
                             uint64_t off, uint64_t len);
int  tplc_image_read_outputs(TplcImage* img, void* dst,
                             uint64_t off, uint64_t len);

/* Heartbeats + status */
void     tplc_image_master_heartbeat(TplcImage* img, uint64_t now_ns);
void     tplc_image_plc_heartbeat(TplcImage* img, uint64_t now_ns);
void     tplc_image_set_master_status(TplcImage* img, uint32_t st);
void     tplc_image_set_plc_status(TplcImage* img, uint32_t st);
uint32_t tplc_image_master_status(const TplcImage* img);
uint32_t tplc_image_plc_status(const TplcImage* img);
uint64_t tplc_image_master_hb_ns(const TplcImage* img);
uint64_t tplc_image_plc_hb_ns(const TplcImage* img);
uint64_t tplc_image_epoch(const TplcImage* img);
void     tplc_image_set_fault(TplcImage* img, uint64_t code);

/* Counters */
uint64_t tplc_image_in_writes(const TplcImage* img);
uint64_t tplc_image_out_writes(const TplcImage* img);
uint64_t tplc_image_in_retries(const TplcImage* img);
uint64_t tplc_image_out_retries(const TplcImage* img);
uint64_t tplc_image_last_fault(const TplcImage* img);

/* Sizes */
uint64_t tplc_image_in_size(const TplcImage* img);
uint64_t tplc_image_out_size(const TplcImage* img);

/* Sim clock: master publishes sim_time_ns; a plc started with
   --sim (TPLC_CTXF_SIM) sources ctx->now_ns + svc->steady_ns from
   it instead of the wall clock (doc 14 §2 determinism).          */
static inline void tplc_image_sim_set_now(TplcImage* img,
                                          uint64_t ns) {
    tplc_img_store_u64(&img->hdr->sim_time_ns, ns);
}
static inline uint64_t tplc_image_sim_now(const TplcImage* img) {
    return tplc_img_load_u64(&img->hdr->sim_time_ns);
}

/* Staleness: helper for the §6.1 liveness rule. */
static inline int tplc_image_hb_stale(uint64_t hb_ns, uint64_t now_ns,
                                      uint64_t stale_after_ns) {
    return hb_ns == 0 || (now_ns > hb_ns &&
                          now_ns - hb_ns > stale_after_ns);
}

#ifdef __cplusplus
} /* extern "C" */
#endif
#endif /* TPLC_IMAGE_H */
