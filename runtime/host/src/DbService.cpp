/* DbService.cpp — tplc-db SQLite-backed SQL service.
 * Statements are compiled and run with the SQLite C API; query
 * results are flattened to tab-separated rows in a bounded
 * buffer (never unbounded allocation).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/DbService.hpp>

#include <sqlite3.h>

#include <cstring>
#include <string>

namespace tplc {

struct DbService::Impl {
    sqlite3* db[TPLC_DB_MAX_HANDLES]{};
    std::string last_error;

    int32_t slot(uint32_t h) {
        if (h == 0 || h > TPLC_DB_MAX_HANDLES || !db[h - 1])
            return -1;
        return (int32_t)h - 1;
    }
};

DbService::DbService() : impl_(new Impl) {
    svc_.struct_size = sizeof(TplcDbService);
    svc_.abi_version = TPLC_DB_ABI_VERSION;
    svc_.host = this;
    svc_.db_open = [](void* s, const char* p, uint32_t* o)
        { return self(s)->open(s, p, o); };
    svc_.db_close = [](void* s, uint32_t h)
        { return self(s)->close(s, h); };
    svc_.db_exec = [](void* s, uint32_t h, const char* q, uint32_t* c)
        { return self(s)->exec(s, h, q, c); };
    svc_.db_query = [](void* s, uint32_t h, const char* q, uint8_t* b,
                       uint32_t n, uint32_t* r, uint32_t* c)
        { return self(s)->query(s, h, q, b, n, r, c); };
    svc_.db_error = [](void* s, uint32_t h, uint8_t* o, uint32_t n,
                       uint32_t* l)
        { return self(s)->error(s, h, o, n, l); };
}

DbService::~DbService() {
    for (uint32_t i = 0; i < TPLC_DB_MAX_HANDLES; i++)
        if (impl_->db[i]) sqlite3_close(impl_->db[i]);
}

int32_t DbService::open(void*, const char* path, uint32_t* out_handle) {
    if (!path || !out_handle) return TPLC_DB_E_PARAM;
    int slot = -1;
    for (uint32_t i = 0; i < TPLC_DB_MAX_HANDLES; i++)
        if (!impl_->db[i]) { slot = (int)i; break; }
    if (slot < 0) return TPLC_DB_E_PARAM;
    sqlite3* db = nullptr;
    if (sqlite3_open(path, &db) != SQLITE_OK) {
        if (db) {
            impl_->last_error = sqlite3_errmsg(db);
            sqlite3_close(db);
        }
        return TPLC_DB_E_OPEN;
    }
    impl_->db[slot] = db;
    *out_handle = (uint32_t)slot + 1;
    return TPLC_DB_OK;
}

int32_t DbService::close(void*, uint32_t h) {
    int32_t s = impl_->slot(h);
    if (s < 0) return TPLC_DB_E_HANDLE;
    sqlite3_close(impl_->db[s]);
    impl_->db[s] = nullptr;
    return TPLC_DB_OK;
}

int32_t DbService::exec(void*, uint32_t h, const char* sql,
                        uint32_t* out_changes) {
    int32_t s = impl_->slot(h);
    if (s < 0) return TPLC_DB_E_HANDLE;
    if (!sql) return TPLC_DB_E_PARAM;
    char* err = nullptr;
    if (sqlite3_exec(impl_->db[s], sql, nullptr, nullptr, &err) !=
        SQLITE_OK) {
        impl_->last_error = err ? err : "exec failed";
        if (err) sqlite3_free(err);
        return TPLC_DB_E_SQL;
    }
    if (out_changes)
        *out_changes =
            (uint32_t)sqlite3_changes(impl_->db[s]);
    return TPLC_DB_OK;
}

int32_t DbService::query(void*, uint32_t h, const char* sql,
                         uint8_t* buf, uint32_t cap,
                         uint32_t* out_rows, uint32_t* out_cols) {
    int32_t s = impl_->slot(h);
    if (s < 0) return TPLC_DB_E_HANDLE;
    if (!sql || !buf || cap == 0) return TPLC_DB_E_PARAM;
    sqlite3_stmt* st = nullptr;
    if (sqlite3_prepare_v2(impl_->db[s], sql, -1, &st, nullptr) !=
        SQLITE_OK) {
        impl_->last_error = sqlite3_errmsg(impl_->db[s]);
        return TPLC_DB_E_SQL;
    }
    uint32_t used = 0, rows = 0;
    uint32_t cols = (uint32_t)sqlite3_column_count(st);
    auto put = [&](const char* p, size_t n) {
        for (size_t i = 0; i < n && used + 1 < cap; i++)
            buf[used++] = (uint8_t)p[i];
    };
    int rc;
    while ((rc = sqlite3_step(st)) == SQLITE_ROW) {
        for (uint32_t c = 0; c < cols; c++) {
            if (c) put("\t", 1);
            const unsigned char* v = sqlite3_column_text(st, (int)c);
            if (v) put((const char*)v, std::strlen((const char*)v));
        }
        put("\n", 1);
        rows++;
        if (used + 1 >= cap) break;   /* bounded: stop early */
    }
    sqlite3_finalize(st);
    buf[used] = 0;
    if (out_rows) *out_rows = rows;
    if (out_cols) *out_cols = cols;
    if (rc != SQLITE_ROW && rc != SQLITE_DONE) {
        impl_->last_error = sqlite3_errmsg(impl_->db[s]);
        return TPLC_DB_E_SQL;
    }
    return TPLC_DB_OK;
}

int32_t DbService::error(void*, uint32_t h, uint8_t* out,
                         uint32_t cap, uint32_t* out_len) {
    if (!out || cap == 0) return TPLC_DB_E_PARAM;
    std::string msg = impl_->last_error;
    if (impl_->slot(h) >= 0)
        msg = sqlite3_errmsg(impl_->db[impl_->slot(h)]);
    uint32_t n = (uint32_t)msg.size();
    if (n >= cap) n = cap - 1;
    std::memcpy(out, msg.data(), n);
    out[n] = 0;
    if (out_len) *out_len = n;
    return TPLC_DB_OK;
}

} /* namespace tplc */
