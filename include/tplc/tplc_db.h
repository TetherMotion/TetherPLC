/* tplc_db.h — C ABI for the tplc-db host service.
 * SQL access for the Tc2_Database / Tc3_Database surface,
 * backed by an embedded SQLite engine (no ODBC driver manager,
 * no TwinCAT Database Server).  Statements are plain SQL text;
 * query results cross the ABI as tab-separated rows (one row
 * per '\n') in a bounded buffer.
 * Vtable is fixed — append new fns at the end.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_DB_H
#define TPLC_DB_H

#include <stdint.h>

#define TPLC_DB_ABI_VERSION 1u

#define TPLC_DB_MAX_HANDLES 8u
#define TPLC_DB_BUF_CAP     255u

typedef struct TplcDbService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* open: database path (created if absent) -> handle or
       negative error.  ":memory:" opens a private in-memory db. */
    int32_t (*db_open)(void*, const char* path, uint32_t* out_handle);

    /* close: handle -> 0 or negative error */
    int32_t (*db_close)(void*, uint32_t h);

    /* exec: non-SELECT statement -> 0 and *out_changes (rows
       affected), or negative error. */
    int32_t (*db_exec)(void*, uint32_t h, const char* sql,
                       uint32_t* out_changes);

    /* query: SELECT statement -> 0 and fills the buffer with
       tab-separated values, '\n' between rows; *out_rows and
       *out_cols report the result shape.  Bounded: the buffer
       is truncated at cap (row/col counts still report what
       was produced). */
    int32_t (*db_query)(void*, uint32_t h, const char* sql,
                        uint8_t* out_buf, uint32_t cap,
                        uint32_t* out_rows, uint32_t* out_cols);

    /* error: last engine error text -> 0 and *out_len */
    int32_t (*db_error)(void*, uint32_t h, uint8_t* out,
                        uint32_t cap, uint32_t* out_len);

    void* reserved[8];
} TplcDbService;

#define TPLC_DB_OK         0
#define TPLC_DB_E_PARAM   -1
#define TPLC_DB_E_HANDLE  -2
#define TPLC_DB_E_OPEN    -3
#define TPLC_DB_E_SQL     -4

#endif /* TPLC_DB_H */
