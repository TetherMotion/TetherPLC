/* tplc_image.c — SHM process-image implementation (plan doc 03).
 * Both roles in one lib: master (create + %I writer + %Q reader) and
 * plc (attach + %I reader + %Q writer). Seqlock per doc 03 §4:
 * wait-free writers, bounded-retry readers, crash-safe (no locks).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/tplc_image.h>

#include <errno.h>
#include <fcntl.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#define ALIGN64(v) (((v) + 63u) & ~63ull)

static uint64_t now_ns(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

/* ------------------------------------------------------------------ */
/* Seqlock primitives (doc 03 §4, normative pattern)                   */
/* ------------------------------------------------------------------ */

static void seq_write_begin(uint64_t* seq) {
    /* odd increment: relaxed add + release fence — readers observing
       odd know a write is in progress and must not trust the payload */
    __atomic_add_fetch(seq, 1, __ATOMIC_RELAXED);
    __atomic_thread_fence(__ATOMIC_RELEASE);
}

static void seq_write_end(uint64_t* seq) {
    /* even increment: release — payload ordered before the store */
    __atomic_add_fetch(seq, 1, __ATOMIC_RELEASE);
}

/* Read one stable epoch+payload under seqlock.
 * Returns 0 on success, TPLC_E_IMG_SEQ_EXHAUSTED on bound hit,
 * TPLC_E_IMG_EPOCH on observed epoch change mid-read. */
static int seq_read(TplcImage* img, uint64_t* seq, uint8_t* area,
                    void* dst, uint64_t off, uint64_t len,
                    uint64_t* retry_ctr) {
    uint64_t s1, s2, e1, e2;
    uint32_t tries = 0;
    for (;;) {
        s1 = tplc_img_load_u64(seq);
        if (s1 & 1u) {                      /* write in progress */
            if (++tries >= TPLC_IMAGE_MAX_TRIES)
                goto exhausted;
            continue;
        }
        e1 = tplc_img_load_u64(&img->hdr->image_epoch);
        __atomic_thread_fence(__ATOMIC_ACQUIRE);
        memcpy(dst, area + off, len);
        __atomic_thread_fence(__ATOMIC_ACQUIRE);
        s2 = tplc_img_load_u64(seq);
        e2 = tplc_img_load_u64(&img->hdr->image_epoch);
        if (s1 == s2 && !(s1 & 1u)) {
            if (e1 != e2) return TPLC_E_IMG_EPOCH;
            return 0;
        }
        if (++tries >= TPLC_IMAGE_MAX_TRIES)
            goto exhausted;
    }
exhausted:
    __atomic_add_fetch(retry_ctr, 1, __ATOMIC_RELAXED);
    return TPLC_E_IMG_SEQ_EXHAUSTED;
}

/* ------------------------------------------------------------------ */
/* Create / attach / close                                             */
/* ------------------------------------------------------------------ */

int tplc_image_create(TplcImage* img, const char* path,
                      uint32_t in_size, uint32_t out_size) {
    memset(img, 0, sizeof *img);
    img->fd = -1;
    if (in_size == 0 || out_size == 0 ||
        in_size > TPLC_IMAGE_MAX_REGION_BYTES ||
        out_size > TPLC_IMAGE_MAX_REGION_BYTES)
        return TPLC_E_IMG_TOO_SMALL;

    uint64_t in_off  = TPLC_IMAGE_HDRSIZE;
    uint64_t out_off = ALIGN64(in_off + in_size);
    uint64_t total   = out_off + ALIGN64(out_size);

    int fd = shm_open(path, O_RDWR | O_CREAT | O_EXCL, 0660);
    if (fd < 0 && errno == EEXIST) {
        /* stale image from a previous run: reclaim it */
        if (shm_unlink(path) != 0) return TPLC_E_IMG_IO;
        fd = shm_open(path, O_RDWR | O_CREAT | O_EXCL, 0660);
    }
    if (fd < 0) return TPLC_E_IMG_IO;
    if (ftruncate(fd, (off_t)total) != 0) {
        close(fd); shm_unlink(path); return TPLC_E_IMG_IO;
    }

    uint8_t* base = mmap(NULL, total, PROT_READ | PROT_WRITE,
                         MAP_SHARED, fd, 0);
    if (base == MAP_FAILED) {
        close(fd); shm_unlink(path); return TPLC_E_IMG_IO;
    }
    memset(base, 0, total);

    img->fd = fd; img->base = base; img->map_size = total;
    img->is_creator = 1;
    img->hdr = (TplcImageHeader*)base;
    img->in_area = base + in_off;
    img->out_area = base + out_off;

    TplcImageHeader* h = img->hdr;
    tplc_img_store_u64(&h->magic, TPLC_IMAGE_MAGIC);
    tplc_img_store_u32(&h->version, TPLC_IMAGE_VERSION);
    tplc_img_store_u32(&h->header_size, TPLC_IMAGE_HDRSIZE);
    tplc_img_store_u32(&h->layout_flags, 0);
    tplc_img_store_u64(&h->in_off, in_off);
    tplc_img_store_u64(&h->in_size, in_size);
    tplc_img_store_u64(&h->out_off, out_off);
    tplc_img_store_u64(&h->out_size, out_size);
    tplc_img_store_u64(&h->in_seq, 0);
    tplc_img_store_u64(&h->out_seq, 0);
    tplc_img_store_u32(&h->master_status, TPLC_IMG_MASTER_INIT);
    tplc_img_store_u32(&h->plc_status, TPLC_IMG_PLC_OPEN);
    /* epoch bump LAST: attachers validate epoch>0 as "header complete" */
    tplc_img_store_u64(&h->master_hb_ns, now_ns());
    tplc_img_store_u64(&h->image_epoch, 1);
    return 0;
}

static int validate_header(TplcImage* img) {
    TplcImageHeader* h = img->hdr;
    if (tplc_img_load_u64(&h->magic) != TPLC_IMAGE_MAGIC)
        return TPLC_E_IMG_MAGIC;
    if (tplc_img_load_u32(&h->version) != TPLC_IMAGE_VERSION)
        return TPLC_E_IMG_VERSION;
    if (tplc_img_load_u32(&h->header_size) != TPLC_IMAGE_HDRSIZE)
        return TPLC_E_IMG_VERSION;
    if (tplc_img_load_u32(&h->layout_flags) & 1u)
        return TPLC_E_IMG_VERSION;              /* BE not supported v1 */
    uint64_t in_off  = tplc_img_load_u64(&h->in_off);
    uint64_t in_size = tplc_img_load_u64(&h->in_size);
    uint64_t out_off = tplc_img_load_u64(&h->out_off);
    uint64_t out_size= tplc_img_load_u64(&h->out_size);
    if (in_size == 0 || out_size == 0 ||
        in_size > TPLC_IMAGE_MAX_REGION_BYTES ||
        out_size > TPLC_IMAGE_MAX_REGION_BYTES)
        return TPLC_E_IMG_TOO_SMALL;
    if (in_off != TPLC_IMAGE_HDRSIZE || (out_off & 63u) != 0)
        return TPLC_E_IMG_MAGIC;
    uint64_t need = out_off + ALIGN64(out_size);
    if (need > img->map_size) return TPLC_E_IMG_TOO_SMALL;
    img->in_area = img->base + in_off;
    img->out_area = img->base + out_off;
    return 0;
}

int tplc_image_attach(TplcImage* img, const char* path, int wait_ms) {
    memset(img, 0, sizeof *img);
    img->fd = -1;

    int fd = -1;
    long waited = 0;
    for (;;) {
        fd = shm_open(path, O_RDWR, 0);
        if (fd >= 0) break;
        if (errno != ENOENT) return TPLC_E_IMG_IO;
        if (wait_ms == 0) return TPLC_E_IMG_IO;
        if (wait_ms > 0 && waited >= wait_ms) return TPLC_E_IMG_IO;
        struct timespec ts = {0, 10 * 1000 * 1000};
        nanosleep(&ts, NULL);
        waited += 10;
    }

    /* wait for creator to finish publishing (epoch > 0), bounded */
    struct stat st;
    int rc = TPLC_E_IMG_IO;
    for (int i = 0; i < 100; i++) {
        if (fstat(fd, &st) == 0 && st.st_size >= (off_t)TPLC_IMAGE_HDRSIZE)
            break;
        struct timespec ts = {0, 5 * 1000 * 1000};
        nanosleep(&ts, NULL);
        if (i == 99) { close(fd); return TPLC_E_IMG_IO; }
    }

    uint8_t* base = mmap(NULL, (size_t)st.st_size,
                         PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (base == MAP_FAILED) { close(fd); return TPLC_E_IMG_IO; }

    img->fd = fd; img->base = base; img->map_size = (uint64_t)st.st_size;
    img->is_creator = 0;
    img->hdr = (TplcImageHeader*)base;

    /* bounded wait for header-complete (epoch > 0) */
    int ok = 0;
    for (int i = 0; i < 100; i++) {
        if (tplc_img_load_u64(&img->hdr->image_epoch) > 0) { ok = 1; break; }
        struct timespec ts = {0, 5 * 1000 * 1000};
        nanosleep(&ts, NULL);
    }
    if (ok) rc = validate_header(img);
    if (!ok || rc != 0) {
        munmap(base, (size_t)st.st_size);
        close(fd);
        memset(img, 0, sizeof *img);
        img->fd = -1;
        return ok ? rc : TPLC_E_IMG_MAGIC;
    }
    return 0;
}

void tplc_image_close(TplcImage* img) {
    if (img->base) munmap(img->base, (size_t)img->map_size);
    if (img->fd >= 0) close(img->fd);
    memset(img, 0, sizeof *img);
    img->fd = -1;
}

int tplc_image_unlink(const char* path) {
    return shm_unlink(path) == 0 ? 0 : TPLC_E_IMG_IO;
}

/* ------------------------------------------------------------------ */
/* Writers                                                             */
/* ------------------------------------------------------------------ */

int tplc_image_write_inputs(TplcImage* img, const void* src,
                            uint64_t off, uint64_t len) {
    if (!img->is_creator) return TPLC_E_IMG_STATE;
    if (off + len > tplc_img_load_u64(&img->hdr->in_size))
        return TPLC_E_IMG_RANGE;
    uint64_t* seq = &img->hdr->in_seq;
    seq_write_begin(seq);
    memcpy(img->in_area + off, src, len);
    seq_write_end(seq);
    __atomic_add_fetch(&img->hdr->in_writes, 1, __ATOMIC_RELAXED);
    return 0;
}

int tplc_image_commit_outputs(TplcImage* img, const void* src,
                              uint64_t off, uint64_t len) {
    if (img->is_creator) return TPLC_E_IMG_STATE;
    if (off + len > tplc_img_load_u64(&img->hdr->out_size))
        return TPLC_E_IMG_RANGE;
    uint64_t* seq = &img->hdr->out_seq;
    seq_write_begin(seq);
    memcpy(img->out_area + off, src, len);
    seq_write_end(seq);
    __atomic_add_fetch(&img->hdr->out_writes, 1, __ATOMIC_RELAXED);
    return 0;
}

/* ------------------------------------------------------------------ */
/* Readers                                                             */
/* ------------------------------------------------------------------ */

int tplc_image_latch_inputs(TplcImage* img, void* dst,
                            uint64_t off, uint64_t len) {
    if (img->is_creator) return TPLC_E_IMG_STATE;
    if (off + len > tplc_img_load_u64(&img->hdr->in_size))
        return TPLC_E_IMG_RANGE;
    return seq_read(img, &img->hdr->in_seq, img->in_area, dst, off, len,
                    &img->hdr->in_retries);
}

int tplc_image_read_outputs(TplcImage* img, void* dst,
                            uint64_t off, uint64_t len) {
    if (!img->is_creator) return TPLC_E_IMG_STATE;
    if (off + len > tplc_img_load_u64(&img->hdr->out_size))
        return TPLC_E_IMG_RANGE;
    return seq_read(img, &img->hdr->out_seq, img->out_area, dst, off, len,
                    &img->hdr->out_retries);
}

/* ------------------------------------------------------------------ */
/* Heartbeats, status, counters                                        */
/* ------------------------------------------------------------------ */

void tplc_image_master_heartbeat(TplcImage* img, uint64_t ns) {
    tplc_img_store_u64(&img->hdr->master_hb_ns, ns);
}
void tplc_image_plc_heartbeat(TplcImage* img, uint64_t ns) {
    tplc_img_store_u64(&img->hdr->plc_hb_ns, ns);
}
void tplc_image_set_master_status(TplcImage* img, uint32_t st) {
    tplc_img_store_u32(&img->hdr->master_status, st);
}
void tplc_image_set_plc_status(TplcImage* img, uint32_t st) {
    tplc_img_store_u32(&img->hdr->plc_status, st);
}
uint32_t tplc_image_master_status(const TplcImage* img) {
    return tplc_img_load_u32(&img->hdr->master_status);
}
uint32_t tplc_image_plc_status(const TplcImage* img) {
    return tplc_img_load_u32(&img->hdr->plc_status);
}
uint64_t tplc_image_master_hb_ns(const TplcImage* img) {
    return tplc_img_load_u64(&img->hdr->master_hb_ns);
}
uint64_t tplc_image_plc_hb_ns(const TplcImage* img) {
    return tplc_img_load_u64(&img->hdr->plc_hb_ns);
}
uint64_t tplc_image_epoch(const TplcImage* img) {
    return tplc_img_load_u64(&img->hdr->image_epoch);
}
void tplc_image_set_fault(TplcImage* img, uint64_t code) {
    tplc_img_store_u64(&img->hdr->last_fault_code, code);
}

uint64_t tplc_image_in_writes(const TplcImage* img) {
    return tplc_img_load_u64(&img->hdr->in_writes);
}
uint64_t tplc_image_out_writes(const TplcImage* img) {
    return tplc_img_load_u64(&img->hdr->out_writes);
}
uint64_t tplc_image_in_retries(const TplcImage* img) {
    return tplc_img_load_u64(&img->hdr->in_retries);
}
uint64_t tplc_image_out_retries(const TplcImage* img) {
    return tplc_img_load_u64(&img->hdr->out_retries);
}
uint64_t tplc_image_last_fault(const TplcImage* img) {
    return tplc_img_load_u64(&img->hdr->last_fault_code);
}
uint64_t tplc_image_in_size(const TplcImage* img) {
    return tplc_img_load_u64(&img->hdr->in_size);
}
uint64_t tplc_image_out_size(const TplcImage* img) {
    return tplc_img_load_u64(&img->hdr->out_size);
}
