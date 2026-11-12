/* DbService.hpp — tplc-db SQLite-backed SQL service.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_DB_SERVICE_HPP
#define TPLC_DB_SERVICE_HPP

#include <tplc/tplc_db.h>
#include <memory>

namespace tplc {

class DbService {
public:
    DbService();
    ~DbService();
    const TplcDbService& table() const { return svc_; }

private:
    static DbService* self(void* s) {
        return static_cast<DbService*>(
            reinterpret_cast<TplcDbService*>(s)->host);
    }

    struct Impl;
    std::unique_ptr<Impl> impl_;
    TplcDbService svc_{};

    int32_t open(void*, const char* path, uint32_t* out_handle);
    int32_t close(void*, uint32_t h);
    int32_t exec(void*, uint32_t h, const char* sql,
                 uint32_t* out_changes);
    int32_t query(void*, uint32_t h, const char* sql, uint8_t* buf,
                  uint32_t cap, uint32_t* out_rows,
                  uint32_t* out_cols);
    int32_t error(void*, uint32_t h, uint8_t* out, uint32_t cap,
                  uint32_t* out_len);
};

} /* namespace tplc */
#endif /* TPLC_DB_SERVICE_HPP */
