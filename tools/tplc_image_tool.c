/* tplc-image — inspect/create SHM process images (plan doc 03 §8).
 *   tplc-image create <path> <in_size> <out_size>
 *   tplc-image dump   <path> [--watch <ms>]
 *   tplc-image unlink <path>
 * `dump` is read-only and safe on a live system.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/tplc_image.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static const char* master_status_str(uint32_t s) {
    switch (s) {
    case TPLC_IMG_MASTER_INIT:     return "init";
    case TPLC_IMG_MASTER_RUN:      return "run";
    case TPLC_IMG_MASTER_DEGRADED: return "degraded";
    case TPLC_IMG_MASTER_STOPPING: return "stopping";
    default: return "?";
    }
}
static const char* plc_status_str(uint32_t s) {
    switch (s) {
    case TPLC_IMG_PLC_OPEN:    return "open";
    case TPLC_IMG_PLC_RUN:     return "run";
    case TPLC_IMG_PLC_FAULT:   return "fault";
    case TPLC_IMG_PLC_STOPPED: return "stopped";
    case TPLC_IMG_PLC_SAFE:    return "safe";
    default: return "?";
    }
}

static void hexdump(const uint8_t* p, uint64_t base, uint64_t len,
                    uint64_t max) {
    if (len > max) len = max;
    for (uint64_t i = 0; i < len; i += 16) {
        printf("  %06lx  ", (unsigned long)(base + i));
        for (uint64_t j = i; j < i + 16 && j < len; j++)
            printf("%02x ", p[j]);
        printf(" ");
        for (uint64_t j = i; j < i + 16 && j < len; j++)
            putchar((p[j] >= 32 && p[j] < 127) ? p[j] : '.');
        putchar('\n');
    }
    if (len == max)
        printf("  ... (%lu bytes total)\n", (unsigned long)len);
}

static void dump(const TplcImage* img) {
    const TplcImageHeader* h = img->hdr;
    printf("magic        %016lx %s\n",
           (unsigned long)tplc_img_load_u64(&h->magic),
           tplc_img_load_u64(&h->magic) == TPLC_IMAGE_MAGIC
               ? "(TPLCIMAG)" : "(BAD)");
    printf("version      %u   header_size %u   flags %u\n",
           tplc_img_load_u32(&h->version),
           tplc_img_load_u32(&h->header_size),
           tplc_img_load_u32(&h->layout_flags));
    printf("epoch        %lu\n",
           (unsigned long)tplc_img_load_u64(&h->image_epoch));
    printf("in_seq       %lu %s\n",
           (unsigned long)tplc_img_load_u64(&h->in_seq),
           (tplc_img_load_u64(&h->in_seq) & 1) ? "(WRITE IN PROGRESS)"
                                               : "(stable)");
    printf("out_seq      %lu %s\n",
           (unsigned long)tplc_img_load_u64(&h->out_seq),
           (tplc_img_load_u64(&h->out_seq) & 1) ? "(WRITE IN PROGRESS)"
                                                : "(stable)");
    printf("master_hb    %lu ns   status %s\n",
           (unsigned long)tplc_image_master_hb_ns(img),
           master_status_str(tplc_image_master_status(img)));
    printf("plc_hb       %lu ns   status %s\n",
           (unsigned long)tplc_image_plc_hb_ns(img),
           plc_status_str(tplc_image_plc_status(img)));
    printf("in_area      off %lu size %lu\n",
           (unsigned long)tplc_img_load_u64(&h->in_off),
           (unsigned long)tplc_image_in_size(img));
    printf("out_area     off %lu size %lu\n",
           (unsigned long)tplc_img_load_u64(&h->out_off),
           (unsigned long)tplc_image_out_size(img));
    printf("writes       in %lu  out %lu\n",
           (unsigned long)tplc_image_in_writes(img),
           (unsigned long)tplc_image_out_writes(img));
    printf("retries      in %lu  out %lu\n",
           (unsigned long)tplc_image_in_retries(img),
           (unsigned long)tplc_image_out_retries(img));
    printf("last_fault   %lu\n",
           (unsigned long)tplc_image_last_fault(img));
}

static void usage(const char* argv0) {
    fprintf(stderr,
        "usage:\n"
        "  %s create <shm_path> <in_size> <out_size>\n"
        "  %s dump   <shm_path> [--watch ms] [--in N] [--out N]\n"
        "  %s unlink <shm_path>\n",
        argv0, argv0, argv0);
}

int main(int argc, char** argv) {
    if (argc < 3) { usage(argv[0]); return 2; }

    if (!strcmp(argv[1], "create") && argc == 5) {
        TplcImage img;
        int rc = tplc_image_create(&img, argv[2],
                                   (uint32_t)strtoul(argv[3], NULL, 0),
                                   (uint32_t)strtoul(argv[4], NULL, 0));
        if (rc) { fprintf(stderr, "create failed: %d\n", rc); return 1; }
        printf("created %s  in=%lu out=%lu\n", argv[2],
               (unsigned long)tplc_image_in_size(&img),
               (unsigned long)tplc_image_out_size(&img));
        tplc_image_close(&img);
        return 0;
    }

    if (!strcmp(argv[1], "unlink") && argc == 3)
        return tplc_image_unlink(argv[2]) ? 1 : 0;

    if (!strcmp(argv[1], "dump")) {
        long watch_ms = -1;
        uint64_t in_max = 64, out_max = 64;
        for (int i = 3; i < argc - 1; i++) {
            if (!strcmp(argv[i], "--watch")) watch_ms = atol(argv[++i]);
            else if (!strcmp(argv[i], "--in")) in_max = strtoul(argv[++i], NULL, 0);
            else if (!strcmp(argv[i], "--out")) out_max = strtoul(argv[++i], NULL, 0);
        }
        /* attach read-only-ish: reuse the attach path; map R/W but
           never write (dump is observationally read-only) */
        TplcImage img;
        int rc = tplc_image_attach(&img, argv[2], 5000);
        if (rc) { fprintf(stderr, "attach failed: %d\n", rc); return 1; }
        do {
            dump(&img);
            uint64_t in_sz = tplc_image_in_size(&img);
            uint64_t out_sz = tplc_image_out_size(&img);
            printf("--- %%I (first %lu of %lu) ---\n",
                   (unsigned long)(in_sz < in_max ? in_sz : in_max),
                   (unsigned long)in_sz);
            hexdump(img.in_area, 0, in_sz, in_max);
            printf("--- %%Q (first %lu of %lu) ---\n",
                   (unsigned long)(out_sz < out_max ? out_sz : out_max),
                   (unsigned long)out_sz);
            hexdump(img.out_area, 0, out_sz, out_max);
            if (watch_ms >= 0) {
                usleep((useconds_t)watch_ms * 1000);
                printf("\033[2J\033[H");   /* clear screen */
            }
        } while (watch_ms >= 0);
        tplc_image_close(&img);
        return 0;
    }

    usage(argv[0]);
    return 2;
}
