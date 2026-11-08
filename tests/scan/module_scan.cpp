/* module_scan — post-link scanner conformance (doc 15 §4.3).
 * Asserts the ElfScan verdicts for hand-crafted module fixtures:
 * kernel-gate opcodes, smuggled imports and forbidden patterns inside
 * immediates. Fixture paths arrive as argv[1..N] in pairs:
 *   <path> <expect:ok|reject> */
#include <tplc/host/ElfScan.hpp>
#include <tplc/tplc_pkg.h>

#include <cstdio>
#include <cstring>

int main(int argc, char** argv) {
    int fails = 0;
    for (int i = 1; i + 1 < argc; i += 2) {
        bool wantOk = std::strcmp(argv[i + 1], "ok") == 0;
        tplc::Status st = tplc::scanModuleFile(argv[i]);
        bool pass = st.ok() == wantOk &&
                    (wantOk || st.code == TPLC_E_PKG_SYSCALL);
        std::printf("%s %-32s code=%d %s\n",
                    pass ? "PASS" : "FAIL", argv[i], st.code,
                    st.msg.c_str());
        if (!pass) fails++;
    }
    if (fails) std::printf("module_scan: %d failures\n", fails);
    return fails ? 1 : 0;
}
