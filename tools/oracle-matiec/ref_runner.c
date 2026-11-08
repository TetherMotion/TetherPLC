/* ref_runner.c — matiec oracle harness (dev-only, GPL-quarantined).
 *
 * Compiled once per generated case together with the C that iec2c emits
 * for the matiec-dialect rendering. Communicates over stdio only:
 *
 *   in : "i=<hex %I bytes> t=<abs ns>\n"   one line per cycle
 *   out: "q=<hex %Q bytes>\n"             one line per cycle
 *
 * Each located variable gets its own storage cell; %I bits are unpacked
 * into cells before the scan and %Q cells packed back after it.
 * __CURRENT_TIME is set to the cycle's t_ns so TON/TOF/TP see the same
 * clock our interpreter/simulator do.
 *
 * Nothing here is linked into or shipped with Tether artifacts; the
 * produced trace is data consumed by diff_fuzz.py.
 */
#include <stdio.h>
#include <string.h>
#include "iec_types.h"
#include "accessor.h"
#include "iec_std_lib.h"

TIME __CURRENT_TIME = {0, 0};

#ifndef PROGNAME
#define PROGNAME FUZZ
#endif
#define PASTE2(a, b) a##b
#define PASTE(a, b) PASTE2(a, b)
#define PROG_data__ PASTE(PROGNAME, _data__)
#define PROG_init__ PASTE(PROGNAME, _init__)
#define PROG_body__ PASTE(PROGNAME, _body__)

/* one storage cell per located var; the generated code takes pointers */
#define __LOCATED_VAR(type, name, cls, mem, a, b) \
    static type name##_cell;                      \
    type *name = &name##_cell;
#include "LOCATED_VARIABLES.h"
#undef __LOCATED_VAR

typedef struct {
    const char *nm;
    char cls;
    int byte, bit;
    void *cell;
    int is_bool;
} loc_t;

#define __LOCATED_VAR(type, name, cls, mem, a, b) \
    {#name, #cls[0], a, b, &name##_cell, sizeof(type) == 1},
static loc_t locs[] = {
#include "LOCATED_VARIABLES.h"
};
#undef __LOCATED_VAR

#include "POUS.h"
#include "POUS.c"

static const int nloc = sizeof(locs) / sizeof(locs[0]);

static int hexbyte(const char *p)
{
    unsigned v = 0;
    sscanf(p, "%2x", &v);
    return (int)(unsigned char)v;
}

int main(void)
{
    PROG_data__ data;
    memset(&data, 0, sizeof data);   /* __INIT_RETAIN |= into flags — must start zeroed */
    PROG_init__(&data, 0);
    static char line[512];
    while (fgets(line, sizeof line, stdin)) {
        char ih[256] = "";
        unsigned long long tns = 0;
        if (sscanf(line, " i=%255s t=%llu", ih, &tns) != 2)
            continue;
        unsigned char ib[64] = {0};
        int nb = (int)strlen(ih) / 2;
        for (int i = 0; i < nb && i < 64; i++)
            ib[i] = (unsigned char)hexbyte(ih + 2 * i);
        __CURRENT_TIME.tv_sec = (long)(tns / 1000000000ULL);
        __CURRENT_TIME.tv_nsec = (long)(tns % 1000000000ULL);
        for (int k = 0; k < nloc; k++) {
            loc_t *L = &locs[k];
            if (L->cls == 'I' && L->is_bool)
                *(unsigned char *)L->cell =
                    (L->byte < nb) ? ((ib[L->byte] >> L->bit) & 1) : 0;
        }
        PROG_body__(&data);
        unsigned char qb[64] = {0};
        int maxb = 0;
        for (int k = 0; k < nloc; k++) {
            loc_t *L = &locs[k];
            if (L->cls == 'Q') {
                if (L->is_bool && *(unsigned char *)L->cell)
                    qb[L->byte] |= (unsigned char)(1 << L->bit);
                if (L->byte + 1 > maxb)
                    maxb = L->byte + 1;
            }
        }
        printf("q=");
        for (int i = 0; i < maxb; i++)
            printf("%02x", qb[i]);
        printf("\n");
        fflush(stdout);
    }
    return 0;
}
