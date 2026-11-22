/* tether-plc support helpers — split from main.cpp (signals-adjacent
 * utilities, auth/update machinery, deploy upload, runbook, supervise). */
#pragma once

#include <cstdint>
#include <string>
#include <string_view>
#include <unordered_map>
#include <vector>
#include <csignal>

extern volatile sig_atomic_t g_stop;

uint64_t mono_ns();
uint64_t realtime_ns();
std::string manifest_str(const std::string& j, const char* key);
void usage(const char* a0);
int op_min_role(uint16_t t);
int role_by_name(const std::string& role);

struct UpdateAuthRule {
    std::string policy;
    std::string trust_dir;
};
std::unordered_map<uint32_t, int>
load_authz(const std::string& path, bool* ok);

void uuid_hex(const uint8_t u[16], char out[33]);
bool uuid_or_path(const uint8_t* p, uint32_t len,
                  const std::string& dir, std::string& out);
bool shadow_path(std::string_view p, uint32_t* idx,
                 std::string_view* sym);
bool hex_uuid(const std::string& hex, uint8_t u[16]);
void sha256_file(int fd, uint8_t out[32]);

struct DeployUpload {
    bool        active = false;
    uint64_t    id = 0, size = 0, received = 0;
    uint64_t    last_ns = 0;   /* idle timeout: drop stalled uploads */
    uint8_t     sha[32]{};
    uint8_t     mode = 0;
    int         fd = -1;
    std::string tmp_path;
    void drop();
};

void runbook_write(const std::string& dir,
                   const std::string& instance,
                   const char* package, const uint8_t uuid[16],
                   const char* state);
int supervise(char** argv, int sep);
