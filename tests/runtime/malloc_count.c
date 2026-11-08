/* malloc interpose counter (doc 14 §5): LD_PRELOAD shim counting
 * malloc/calloc/realloc/aligned_alloc calls in the host process.
 * Writes the running total to $TPLC_MC_FILE every 100 ms so a test
 * can snapshot, let the plc scan, and diff — steady-state scanning
 * must not allocate (the "no allocation after init" property).
 *
 * build: cc -shared -fPIC -O2 -o malloc_count.so malloc_count.c -ldl
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#define _GNU_SOURCE
#include <dlfcn.h>
#include <fcntl.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static atomic_ulong g_count;
static void* (*real_malloc)(size_t);
static void* (*real_calloc)(size_t, size_t);
static void* (*real_realloc)(void*, size_t);
static void* (*real_aligned_alloc)(size_t, size_t);

static int g_in_hook; /* TLS-free recursion guard (early dlsym allocs) */

static void resolve(void) {
    if (real_malloc)
        return;
    real_malloc = dlsym(RTLD_NEXT, "malloc");
    real_calloc = dlsym(RTLD_NEXT, "calloc");
    real_realloc = dlsym(RTLD_NEXT, "realloc");
    real_aligned_alloc = dlsym(RTLD_NEXT, "aligned_alloc");
}

void* malloc(size_t n) {
    resolve();
    if (!g_in_hook) {
        g_in_hook = 1;
        atomic_fetch_add_explicit(&g_count, 1, memory_order_relaxed);
        g_in_hook = 0;
    }
    return real_malloc(n);
}

void* calloc(size_t n, size_t sz) {
    resolve();
    if (!g_in_hook) {
        g_in_hook = 1;
        atomic_fetch_add_explicit(&g_count, 1, memory_order_relaxed);
        g_in_hook = 0;
    }
    /* before real_calloc resolves, dlsym itself may calloc — give it a
       tiny static arena so it can't recurse into us */
    if (!real_calloc) {
        static char early[4096];
        static atomic_uint early_off;
        size_t need = n * sz;
        unsigned o = atomic_fetch_add(&early_off, (unsigned)((need + 15) & ~15ul));
        if (o + need <= sizeof early)
            return early + o;
        return NULL;
    }
    return real_calloc(n, sz);
}

void* realloc(void* p, size_t n) {
    resolve();
    if (!g_in_hook) {
        g_in_hook = 1;
        atomic_fetch_add_explicit(&g_count, 1, memory_order_relaxed);
        g_in_hook = 0;
    }
    return real_realloc(p, n);
}

void* aligned_alloc(size_t a, size_t n) {
    resolve();
    if (!g_in_hook) {
        g_in_hook = 1;
        atomic_fetch_add_explicit(&g_count, 1, memory_order_relaxed);
        g_in_hook = 0;
    }
    return real_aligned_alloc(a, n);
}

/* reporter: dumps the counter to $TPLC_MC_FILE every 100 ms.
   Raw syscalls only — stdio would allocate through our own hooks and
   pollute the measurement. */
static void* report_fn(void* unused) {
    (void)unused;
    const char* path = getenv("TPLC_MC_FILE");
    if (!path)
        return NULL;
    char buf[32];
    for (;;) {
        int len = snprintf(buf, sizeof buf, "%lu\n",
                           atomic_load_explicit(&g_count, memory_order_relaxed));
        int fd = open(path, O_WRONLY | O_CREAT | O_TRUNC, 0644);
        if (fd >= 0) {
            ssize_t w = write(fd, buf, (size_t)len);
            (void)w;
            close(fd);
        }
        struct timespec ts = {0, 100000000};
        nanosleep(&ts, NULL);
    }
}

__attribute__((constructor)) static void init(void) {
    resolve();
    pthread_t t;
    if (pthread_create(&t, NULL, report_fn, NULL) == 0)
        pthread_detach(t);
}
