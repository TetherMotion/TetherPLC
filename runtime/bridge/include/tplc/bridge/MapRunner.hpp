/* MapRunner — per-cycle map application (doc 09 §4.1): input channels
 * copy PDO→%I, output channels copy %Q→PDO with safe-state
 * substitution when the PLC is not RUNNING. v1 transport is `sim`:
 * each channel's key is a file under a sim directory — inputs are
 * re-read and outputs re-written every cycle. SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */

#pragma once

#include <tplc/host/Status.hpp>
#include <cstdint>
#include <string>
#include <vector>

namespace tplc {

class ImageServer;

enum SafePolicy : uint8_t { SAFE_ZERO = 0, SAFE_LAST = 1, SAFE_DEFINED = 2 };

struct MapChannel {
    bool        is_input;             /* in: pdo→%I / out: %Q→pdo   */
    uint32_t    image_off = 0;
    uint32_t    size = 0;
    std::string key;                  /* sim transport: file name   */
    std::string pdo;                  /* bus endpoint name (pdo=)   */
    std::string type;                 /* image view type (bool/u8…) */
    SafePolicy  safe = SAFE_ZERO;
    uint64_t    safe_value = 0;       /* SAFE_DEFINED byte pattern  */
    bool        has_safe = false;     /* explicit safe= seen        */
};

/* bind <at> <key> — program AT binding: fixed at_name (%IX/%QX/etc)
   must land inside a channel; key may be a channel key or '*'      */
struct MapBinding {
    std::string at_name;
    std::string key;
};

struct IoMap {
    uint32_t in_size = 0, out_size = 0;
    std::vector<MapChannel> channels;
    std::vector<MapBinding> bindings;
    std::vector<std::string> safety_keys;  /* safety-classified eps */
    std::string hash;               /* declared `hash <sha256>` stamp */
};

/* map validation (doc 09 §2.2): each violation is reported with its
   stable code — a map is deployable only when the list is empty.
   Codes live at -600..: the -500 block belongs to wire-side
   TPLC_E_MGMT_* (tplc_ctl.h) and MAP codes must not alias it.      */
struct MapViolation {
    int         code;                 /* TPLC_E_MAP_*               */
    std::string rule;                 /* MAP_RANGE/OWNER/…          */
    std::string detail;
};
constexpr int TPLC_E_MAP_OWNER  = -600;
constexpr int TPLC_E_MAP_DIR    = -601;
constexpr int TPLC_E_MAP_RANGE  = -602;
constexpr int TPLC_E_MAP_TYPE   = -603;
constexpr int TPLC_E_MAP_PDO    = -604;
constexpr int TPLC_E_MAP_SAFETY = -605;
constexpr int TPLC_E_MAP_AT     = -606;
constexpr int TPLC_E_MAP_BIND   = -607;
constexpr int TPLC_E_MAP_SAFE   = -608;
constexpr int TPLC_E_MAP_EPOCH  = -609;

/* Bus catalog: the live bus configuration a `pdo=` channel is
   validated against (TPLC_E_MAP_PDO).  Published by the master-side
   pump as <simdir>/.bus:
       tx <endpoint> <size>     TxPDO, slave -> master (map `in`)
       rx <endpoint> <size>     RxPDO, master -> slave (map `out`)
   <endpoint> is the pump's PDO key (e.g. "el1004.c1"); a map channel
   `in … pdo=el1004.c1` resolves to file key "el1004.c1.tx".          */
struct PdoEndpoint {
    bool        is_tx;                /* true: TxPDO (in), false: Rx */
    std::string key;
    uint32_t    size = 0;
};

struct BusCatalog {
    std::vector<PdoEndpoint> endpoints;
};

Result<BusCatalog> parse_bus_catalog(const std::string& text);

/* Canonical map hash (TPLC_E_MAP_EPOCH): SHA-256 over a canonical
   serialization of {image, channels, bindings, safety} — cosmetic
   formatting and the `hash` directive itself are excluded.  Stamp a
   map by inserting `hash <map_hash output>`; validate_map then
   rejects the map when its content drifts from the stamp.        */
std::string map_hash(const IoMap& m);

/* @p bus may be nullptr: pdo= channels then always violate MAP_PDO
   (no live bus config to validate against).                        */
std::vector<MapViolation> validate_map(const IoMap& m,
                                       const BusCatalog* bus = nullptr);

/* line format (v1 dev map):
     image in <in_size> out <out_size>
     in  off=<n> size=<n> key=<name> [safe=zero|last|defined <hex>]
     out off=<n> size=<n> key=<name> [safe=…]
     in|out off=<n> size=<n> pdo=<endpoint> [safe=…]   (bus channel)
     hash <sha256-hex>                  (epoch stamp, optional)
   '#' comments; %Q writer overlap + range checked (doc 09 §2.2).
   pdo= and key= are mutually exclusive on a channel.               */
Result<IoMap> parse_map(const std::string& text);

class MapRunner {
public:
    MapRunner(const IoMap& map, ImageServer& img,
              const std::string& sim_dir);

    /* one exchange: inputs (pdo→%I), outputs (%Q→pdo, safe-substituted
       when `plc_ok` is false). Call at the cyclic phase per doc 03. */
    void cycle(bool plc_ok);

    uint64_t cycles() const { return cycles_; }

private:
    void pump_inputs();
    void pump_outputs(bool plc_ok);

    const IoMap&   map_;
    ImageServer&   img_;
    std::string    sim_dir_;
    uint64_t       cycles_ = 0;
    std::vector<uint8_t> last_out_;   /* SAFE_LAST backing bytes      */
};

} /* namespace tplc */
