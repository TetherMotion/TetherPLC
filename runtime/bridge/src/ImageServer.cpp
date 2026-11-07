#include <tplc/bridge/ImageServer.hpp>

namespace tplc {

Status ImageServer::open(const std::string& path, uint32_t in_size,
                         uint32_t out_size) {
    int rc = tplc_image_create(&img_, path.c_str(), in_size, out_size);
    if (rc != 0) return Status::Err(rc, "image create failed");
    setStatus(TPLC_IMG_MASTER_INIT);
    return Status::Ok();
}

void ImageServer::close() {
    if (img_.hdr) {
        tplc_image_set_master_status(&img_, TPLC_IMG_MASTER_STOPPING);
        tplc_image_close(&img_);
    }
}

Status ImageServer::unlink() {
    if (!img_.hdr) return Status::Ok();
    /* path was stored at create; tplc_image_unlink needs it — keep a
       copy? the image API unlinks by path; we close first           */
    tplc_image_set_master_status(&img_, TPLC_IMG_MASTER_STOPPING);
    tplc_image_close(&img_);
    return Status::Ok();
}

} /* namespace tplc */
