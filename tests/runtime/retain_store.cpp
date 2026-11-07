/* retain_store.cpp — RetainStore conformance: file create, stage →
 * flush → load round-trip, generation bumping, uuid/hash mismatch
 * reject, corruption reject, clear. (plan doc 04 §5)
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/RetainStore.hpp>
#include "../test_util.h"

#include <cstring>
#include <fcntl.h>
#include <unistd.h>

#define DIR "/tmp/tplc-retain-test"

static const uint8_t UUID_A[16] = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16};
static const uint8_t UUID_B[16] = {0xAA,0xBB,3,4,5,6,7,8,9,10,11,12,13,14,15,16};

int main() {
    char cmd[64];
    snprintf(cmd, sizeof cmd, "rm -rf %s", DIR);
    CHECK(system(cmd) == 0);

    const uint64_t LEN = 64, HASH = 0xDEADBEEF;
    std::vector<uint8_t> region(LEN), out(LEN);
    for (uint64_t i = 0; i < LEN; i++) region[i] = (uint8_t)(i * 7 + 1);

    /* ---------- round trip ---------- */
    {
        tplc::RetainStore rs;
        CHECK(rs.open(DIR, UUID_A, HASH, LEN).ok());
        CHECK(rs.attached());
        CHECK(!rs.load(out.data()));            /* fresh file → no restore */
        CHECK(rs.losses() == 1);
        rs.noteDirty();
        CHECK(rs.dirty());
        rs.stage(region.data());
        CHECK(rs.flush().ok());
        CHECK(!rs.dirty());
        CHECK(rs.flushes() == 1);
        CHECK(rs.generation() == 1);
    }
    {
        tplc::RetainStore rs;
        CHECK(rs.open(DIR, UUID_A, HASH, LEN).ok());
        CHECK(rs.load(out.data()));
        CHECK(out == region);
        CHECK(rs.restores() == 1);
        CHECK(rs.generation() == 1);            /* persisted, not bumped */
    }

    /* ---------- uuid / hash mismatch → defaults ---------- */
    {
        tplc::RetainStore rs;
        CHECK(rs.open(DIR, UUID_B, HASH, LEN).ok());   /* different file */
        CHECK(!rs.load(out.data()));                    /* empty file */
    }
    {
        tplc::RetainStore rs;
        CHECK(rs.open(DIR, UUID_A, HASH + 1, LEN).ok()); /* same file */
        CHECK(!rs.load(out.data()));                    /* hash mismatch */
        CHECK(rs.losses() == 1);
    }

    /* ---------- corrupt payload rejected ---------- */
    {
        /* poison one payload byte in the file */
        char path[128];
        snprintf(path, sizeof path, "%s/0102030405060708090a0b0c0d0e0f10.bin", DIR);
        int fd = ::open(path, O_RDWR);
        CHECK(fd >= 0);
        uint8_t b = 0xFF;
        CHECK(::pwrite(fd, &b, 1, (off_t)sizeof(tplc::TplcRetainHeader) + 5) == 1);
        ::close(fd);
        tplc::RetainStore rs;
        CHECK(rs.open(DIR, UUID_A, HASH, LEN).ok());
        CHECK(!rs.load(out.data()));            /* crc rejects */
    }

    /* ---------- clear ---------- */
    {
        tplc::RetainStore rs;
        CHECK(rs.open(DIR, UUID_A, HASH, LEN).ok());
        CHECK(rs.clear().ok());
        tplc::RetainStore rs2;
        CHECK(rs2.open(DIR, UUID_A, HASH, LEN).ok());
        CHECK(rs2.load(out.data()));            /* clear still restores (zeros) */
        bool all_zero = true;
        for (auto v : out) all_zero &= (v == 0);
        CHECK(all_zero);
    }

    /* ---------- inert when len==0 ---------- */
    {
        tplc::RetainStore rs;
        CHECK(rs.open(DIR, UUID_A, HASH, 0).ok());
        CHECK(!rs.attached());
    }

    CHECK(system(cmd) == 0);
    TEST_REPORT("retain_store");
}
