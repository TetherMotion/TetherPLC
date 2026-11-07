/* CatalogExporter — symmap + type catalog → tether_io catalog
 * entries (doc 09 §6): vars become parameters, io channels and
 * counters become signals, retain vars land under the read-only
 * retain namespace. v1 emits JSON for the Tether app to ingest.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#pragma once

#include <tplc/host/Status.hpp>
#include <cstdint>
#include <string>
#include <vector>

namespace tplc {

struct CatalogEntry {
    uint64_t    id;              /* FNV-1a 64 of name              */
    std::string name;            /* plc.<prog>.<qualified>         */
    std::string group;
    std::string kind;            /* "param" | "signal"             */
    uint8_t     value_type = 0;  /* tether_io ValueType numeric    */
    uint8_t     element_type = 0;
    bool        writable = false;
    std::string description;
};

/* parse the SMAP + TYPC payloads (as stored in the .tplc sections)
   into catalog entries; task_names feed plc.<prog>.task.<n>.stats */
Result<std::vector<CatalogEntry>> catalog_export(
    const uint8_t* smap, uint32_t smap_len,
    const uint8_t* typc, uint32_t typc_len,
    const std::string& prog_name,
    const std::vector<std::string>& task_names);

std::string catalog_json(const std::vector<CatalogEntry>& entries);

} /* namespace tplc */
