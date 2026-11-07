/* test_util.h — minimal check macros for the conformance suites.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_TEST_UTIL_H
#define TPLC_TEST_UTIL_H

#include <stdio.h>

static int tplc_test_failures = 0;
static int tplc_test_checks = 0;

#define CHECK(cond) do {                                        \
    tplc_test_checks++;                                         \
    if (!(cond)) {                                              \
        tplc_test_failures++;                                   \
        fprintf(stderr, "FAIL %s:%d: %s\n", __FILE__, __LINE__, \
                #cond);                                         \
    }                                                           \
} while (0)

#define CHECK_EQ(a, b) do {                                     \
    tplc_test_checks++;                                         \
    long long _va = (long long)(a), _vb = (long long)(b);       \
    if (_va != _vb) {                                           \
        tplc_test_failures++;                                   \
        fprintf(stderr, "FAIL %s:%d: %s=%lld != %lld\n",        \
                __FILE__, __LINE__, #a, _va, _vb);              \
    }                                                           \
} while (0)

#define TEST_REPORT(name) do {                                  \
    fprintf(stderr, "%s: %d checks, %d failures\n", name,       \
            tplc_test_checks, tplc_test_failures);              \
    return tplc_test_failures ? 1 : 0;                          \
} while (0)

#endif /* TPLC_TEST_UTIL_H */
