/* image_conformance.c — SHM process-image protocol conformance
 * (plan doc 03 §13 fixtures). In-process: one create handle (master
 * role) + one attach handle (plc role) over the same shm path.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/tplc_image.h>
#include "../test_util.h"

#include <string.h>

#define PATH "/tplc-test-image-001"

int main(void) {
    TplcImage master, plc;
    int rc;

    /* --- create + attach ------------------------------------------- */
    tplc_image_unlink(PATH);
    rc = tplc_image_create(&master, PATH, 256, 256);
    CHECK_EQ(rc, 0);
    CHECK_EQ(tplc_image_in_size(&master), 256);
    CHECK_EQ(tplc_image_out_size(&master), 256);
    CHECK(tplc_image_epoch(&master) == 1);
    CHECK_EQ(tplc_image_master_status(&master), TPLC_IMG_MASTER_INIT);
    CHECK_EQ(tplc_image_plc_status(&master), TPLC_IMG_PLC_OPEN);

    rc = tplc_image_attach(&plc, PATH, 1000);
    CHECK_EQ(rc, 0);
    CHECK_EQ(tplc_image_in_size(&plc), 256);
    CHECK_EQ(tplc_image_out_size(&plc), 256);

    /* --- %I round trip (master writes, plc latches) ----------------- */
    uint8_t in_data[16];
    for (int i = 0; i < 16; i++) in_data[i] = (uint8_t)(0xA0 + i);
    rc = tplc_image_write_inputs(&master, in_data, 0, 16);
    CHECK_EQ(rc, 0);
    CHECK_EQ(tplc_image_in_writes(&master), 1);

    uint8_t latched[16] = {0};
    rc = tplc_image_latch_inputs(&plc, latched, 0, 16);
    CHECK_EQ(rc, 0);
    CHECK(memcmp(latched, in_data, 16) == 0);
    CHECK_EQ(tplc_image_in_retries(&plc), 0);

    /* --- %Q round trip (plc commits, master reads) ------------------- */
    uint8_t out_data[8];
    for (int i = 0; i < 8; i++) out_data[i] = (uint8_t)(0x50 + i);
    rc = tplc_image_commit_outputs(&plc, out_data, 8, 8);  /* offset 8 */
    CHECK_EQ(rc, 0);
    CHECK_EQ(tplc_image_out_writes(&plc), 1);

    uint8_t got[8] = {0};
    rc = tplc_image_read_outputs(&master, got, 8, 8);
    CHECK_EQ(rc, 0);
    CHECK(memcmp(got, out_data, 8) == 0);
    /* bytes outside the committed range stay zero */
    uint8_t z[4] = {0xEE, 0xEE, 0xEE, 0xEE};
    rc = tplc_image_read_outputs(&master, z, 0, 4);
    CHECK_EQ(rc, 0);
    CHECK_EQ(z[0], 0);

    /* --- role enforcement -------------------------------------------- */
    CHECK_EQ(tplc_image_write_inputs(&plc, in_data, 0, 1),
             TPLC_E_IMG_STATE);
    CHECK_EQ(tplc_image_commit_outputs(&master, out_data, 0, 1),
             TPLC_E_IMG_STATE);
    CHECK_EQ(tplc_image_latch_inputs(&master, latched, 0, 1),
             TPLC_E_IMG_STATE);
    CHECK_EQ(tplc_image_read_outputs(&plc, got, 0, 1),
             TPLC_E_IMG_STATE);

    /* --- range enforcement ------------------------------------------- */
    CHECK_EQ(tplc_image_write_inputs(&master, in_data, 255, 16),
             TPLC_E_IMG_RANGE);
    CHECK_EQ(tplc_image_commit_outputs(&plc, out_data, 250, 8),
             TPLC_E_IMG_RANGE);

    /* --- heartbeats + status ------------------------------------------ */
    tplc_image_master_heartbeat(&master, 123456789);
    tplc_image_plc_heartbeat(&plc, 987654321);
    CHECK_EQ(tplc_image_master_hb_ns(&plc), 123456789);
    CHECK_EQ(tplc_image_plc_hb_ns(&master), 987654321);
    tplc_image_set_master_status(&master, TPLC_IMG_MASTER_RUN);
    tplc_image_set_plc_status(&plc, TPLC_IMG_PLC_RUN);
    CHECK_EQ(tplc_image_master_status(&plc), TPLC_IMG_MASTER_RUN);
    CHECK_EQ(tplc_image_plc_status(&master), TPLC_IMG_PLC_RUN);
    tplc_image_set_fault(&plc, 42);
    CHECK_EQ(tplc_image_last_fault(&master), 42);

    /* --- staleness helper --------------------------------------------- */
    CHECK(!tplc_image_hb_stale(1000, 2000, 5000));
    CHECK(tplc_image_hb_stale(1000, 9000, 5000));
    CHECK(tplc_image_hb_stale(0, 100, 5000));

    /* --- seq-odd stuck writer → bounded read ---------------------------- */
    /* simulate a writer that died mid-write (seq left odd) */
    tplc_img_store_u64(&master.hdr->in_seq, 7u);   /* odd */
    uint64_t tries_before = tplc_image_in_retries(&plc);
    rc = tplc_image_latch_inputs(&plc, latched, 0, 16);
    CHECK_EQ(rc, TPLC_E_IMG_SEQ_EXHAUSTED);
    CHECK(tplc_image_in_retries(&plc) > tries_before);
    /* restore parity and confirm reads work again */
    tplc_img_store_u64(&master.hdr->in_seq, 8u);
    rc = tplc_image_latch_inputs(&plc, latched, 0, 16);
    CHECK_EQ(rc, 0);
    CHECK(memcmp(latched, in_data, 16) == 0);

    /* --- epoch: creator reclaim bumps epoch ----------------------------- */
    tplc_image_set_plc_status(&plc, TPLC_IMG_PLC_STOPPED);
    tplc_image_close(&plc);
    tplc_image_close(&master);
    rc = tplc_image_create(&master, PATH, 128, 128);  /* reclaim stale */
    CHECK_EQ(rc, 0);
    CHECK_EQ(tplc_image_epoch(&master), 1);           /* fresh file   */
    tplc_image_close(&master);

    /* --- attach-wait on missing path fails after bound ------------------- */
    tplc_image_unlink(PATH);
    rc = tplc_image_attach(&plc, PATH, 50);
    CHECK_EQ(rc, TPLC_E_IMG_IO);

    tplc_image_unlink(PATH);
    TEST_REPORT("image_conformance");
}
