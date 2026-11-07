/* ImageServer — the master-side half of the process image (doc 03,
 * doc 09 §4): creates/owns the shm image, publishes master status +
 * heartbeat, and offers the PDO↔region copies the MapRunner drives.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#pragma once

#include <tplc/tplc_image.h>
#include <tplc/host/Status.hpp>
#include <string>

namespace tplc {

class ImageServer {
public:
    ImageServer() = default;
    ~ImageServer() { close(); }
    ImageServer(const ImageServer&) = delete;
    ImageServer& operator=(const ImageServer&) = delete;

    /* create the image; also accepts an already-created file */
    Status open(const std::string& path, uint32_t in_size,
                uint32_t out_size);
    void   close();                     /* STOPPING + close (no unlink) */
    Status unlink();                    /* close + unlink the shm */

    void   heartbeat(uint64_t now_ns) {
        tplc_image_master_heartbeat(&img_, now_ns);
    }
    void   setStatus(uint32_t st) { tplc_image_set_master_status(&img_, st); }

    /* master→%I copy; PLC latches it on its next scan boundary */
    Status writeInputs(const void* src, uint32_t off, uint32_t len) {
        int rc = tplc_image_write_inputs(&img_, src, off, len);
        return rc ? Status::Err(rc, "write_inputs") : Status::Ok();
    }
    /* %Q→master copy; PLC committed it at its last boundary */
    Status readOutputs(void* dst, uint32_t off, uint32_t len) {
        int rc = tplc_image_read_outputs(&img_, dst, off, len);
        return rc ? Status::Err(rc, "read_outputs") : Status::Ok();
    }

    uint32_t plcStatus()  const { return tplc_image_plc_status(&img_); }
    uint64_t plcHbNs()    const { return tplc_image_plc_hb_ns(&img_); }
    uint32_t inSize()     const { return tplc_image_in_size(&img_); }
    uint32_t outSize()    const { return tplc_image_out_size(&img_); }
    bool     open()       const { return img_.hdr != nullptr; }
    TplcImage* raw()            { return &img_; }

private:
    TplcImage img_{};
};

} /* namespace tplc */
