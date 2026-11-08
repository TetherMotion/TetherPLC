/* sandbox_probe.cpp — applies the tether-plc sandbox then attempts
 * one socket() family, argv[1] = "unix" | "inet". Under the filter,
 * AF_INET must kill the process with SIGSYS; AF_UNIX must succeed.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#include <tplc/host/Sandbox.hpp>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>
#include <cstring>
#include <cstdio>

int main(int argc, char** argv) {
    if (argc < 2) return 64;
    tplc::Status st = tplc::installSandbox();
    if (!st.ok()) { std::fprintf(stderr, "sandbox: %s\n", st.msg.c_str()); return 65; }

    int dom = std::strcmp(argv[1], "inet") == 0 ? AF_INET : AF_UNIX;
    int fd = socket(dom, SOCK_STREAM, 0);   /* AF_INET → SIGSYS here */
    if (fd >= 0) { close(fd); return 0; }
    return 66;                             /* errno, not a kill     */
}
