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
    SafePolicy  safe = SAFE_ZERO;
    uint64_t    safe_value = 0;       /* SAFE_DEFINED byte pattern  */
};

struct IoMap {
    uint32_t in_size = 0, out_size = 0;
    std::vector<MapChannel> channels;
};

/* line format (v1 dev map):
     image in <in_size> out <out_size>
     in  off=<n> size=<n> key=<name> [safe=zero|last|defined <hex>]
     out off=<n> size=<n> key=<name> [safe=…]
   '#' comments; %Q writer overlap + range checked (doc 09 §2.2).   */
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
