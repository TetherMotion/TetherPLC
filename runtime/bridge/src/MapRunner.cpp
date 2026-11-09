#include <tplc/bridge/MapRunner.hpp>
#include <tplc/bridge/ImageServer.hpp>
#include <tplc/tplc_image.h>
#include <cstdio>
#include <cstring>
#include <fcntl.h>
#include <unistd.h>
#include <sys/stat.h>

namespace tplc {

static bool tok(const std::string& s, const char* pfx,
                std::string& out) {
    size_t n = strlen(pfx);
    if (s.compare(0, n, pfx) == 0) { out = s.substr(n); return true; }
    return false;
}

Result<IoMap> parse_map(const std::string& text) {
    IoMap m;
    uint32_t line_no = 0;
    size_t pos = 0;
    auto bad = [&](const char* msg) {
        return Result<IoMap>::Err(
            Status::Err(-1, std::string("map line ") +
                            std::to_string(line_no) + ": " + msg));
    };
    while (pos < text.size()) {
        size_t eol = text.find('\n', pos);
        if (eol == std::string::npos) eol = text.size();
        std::string line = text.substr(pos, eol - pos);
        pos = eol + 1;
        line_no++;
        size_t h = line.find('#');
        if (h != std::string::npos) line.resize(h);
        /* split on ws */
        std::vector<std::string> t;
        size_t i = 0;
        while (i < line.size()) {
            while (i < line.size() && isspace((unsigned char)line[i])) i++;
            size_t b = i;
            while (i < line.size() && !isspace((unsigned char)line[i])) i++;
            if (b < i) t.push_back(line.substr(b, i - b));
        }
        if (t.empty()) continue;
        if (t[0] == "image") {
            for (size_t k = 1; k + 1 < t.size(); k += 2) {
                if (t[k] == "in")  m.in_size  = (uint32_t)strtoul(t[k+1].c_str(),0,0);
                if (t[k] == "out") m.out_size = (uint32_t)strtoul(t[k+1].c_str(),0,0);
            }
            continue;
        }
        if (t[0] == "bind") {
            if (t.size() < 3) return bad("bind <at> <key>");
            std::string at = t[1];
            if (at.size() >= 2 && at.front() == '"' && at.back() == '"')
                at = at.substr(1, at.size() - 2);
            m.bindings.push_back({at, t[2]});
            continue;
        }
        if (t[0] == "safety") {
            if (t.size() < 2) return bad("safety <key>");
            m.safety_keys.push_back(t[1]);
            continue;
        }
        if (t[0] == "hash") {
            if (t.size() < 2) return bad("hash <sha256-hex>");
            m.hash = t[1];
            continue;
        }
        if (t[0] != "in" && t[0] != "out")
            return bad("expected image|in|out|bind|safety");
        MapChannel c;
        c.is_input = t[0] == "in";
        bool saw_key = false, saw_pdo = false;
        for (size_t k = 1; k < t.size(); k++) {
            std::string v;
            if      (tok(t[k], "off=", v))  c.image_off = (uint32_t)strtoul(v.c_str(),0,0);
            else if (tok(t[k], "size=", v)) c.size      = (uint32_t)strtoul(v.c_str(),0,0);
            else if (tok(t[k], "key=", v))  { c.key = v; saw_key = true; }
            else if (tok(t[k], "pdo=", v))  { c.pdo = v; saw_pdo = true; }
            else if (tok(t[k], "type=", v)) c.type = v;
            else if (tok(t[k], "safe=", v)) {
                c.has_safe = true;
                if (v == "last")    c.safe = SAFE_LAST;
                else if (v == "defined") {
                    c.safe = SAFE_DEFINED;
                    if (k + 1 >= t.size()) return bad("safe=defined needs value");
                    c.safe_value = strtoull(t[++k].c_str(), 0, 0);
                }
            }
        }
        if (saw_key && saw_pdo)
            return bad("channel takes key= or pdo=, not both");
        if (saw_pdo) {
            /* bus channel: the simdir file is the pump's PDO image —
               TxPDO (.tx) feeds %I, RxPDO (.rx) is fed by %Q        */
            if (c.pdo.empty()) return bad("pdo= needs an endpoint name");
            c.key = c.pdo + (c.is_input ? ".tx" : ".rx");
        }
        if (c.size == 0 || c.key.empty())
            return bad("channel needs size>0 and key");
        m.channels.push_back(std::move(c));
    }
    return Result<IoMap>::Ok(std::move(m));
}

/* ---- validator (doc 09 §2.2) ------------------------------------ */
static uint32_t type_size(const std::string& t) {
    if (t == "bool" || t == "u8" || t == "i8")  return 1;
    if (t == "u16"  || t == "i16")              return 2;
    if (t == "u32"  || t == "i32" || t == "f32") return 4;
    if (t == "u64"  || t == "i64" || t == "f64" || t == "time")
        return 8;
    return 0;                            /* unknown → skip TYPE chk */
}

static bool at_offset(const std::string& at, char& space,
                      uint64_t& off) {
    /* %IX0.0 %IB4 %QW8 %MD2 … — return byte offset; X is bit-grain */
    if (at.size() < 4 || at[0] != '%' ||
        (at[1] != 'I' && at[1] != 'Q' && at[1] != 'M'))
        return false;
    space = at[1];
    char w = at[2];
    const char* p = at.c_str() + 3;
    char* end;
    double v = strtod(p, &end);
    if (end == p) return false;
    if (w == 'X' || w == 'B') off = (uint64_t)v;
    else if (w == 'W')        off = (uint64_t)v * 2;
    else if (w == 'D')        off = (uint64_t)v * 4;
    else if (w == 'L')        off = (uint64_t)v * 8;
    else return false;
    return true;
}

Result<BusCatalog> parse_bus_catalog(const std::string& text) {
    BusCatalog cat;
    size_t pos = 0;
    while (pos < text.size()) {
        size_t eol = text.find('\n', pos);
        if (eol == std::string::npos) eol = text.size();
        std::string line = text.substr(pos, eol - pos);
        pos = eol + 1;
        size_t h = line.find('#');
        if (h != std::string::npos) line.resize(h);
        std::vector<std::string> t;
        size_t i = 0;
        while (i < line.size()) {
            while (i < line.size() && isspace((unsigned char)line[i])) i++;
            size_t b = i;
            while (i < line.size() && !isspace((unsigned char)line[i])) i++;
            if (b < i) t.push_back(line.substr(b, i - b));
        }
        if (t.empty()) continue;
        if ((t[0] != "tx" && t[0] != "rx") || t.size() != 3)
            return Result<BusCatalog>::Err(Status::Err(
                -1, "bus catalog: expected 'tx|rx <endpoint> <size>'"));
        PdoEndpoint e;
        e.is_tx = t[0] == "tx";
        e.key   = t[1];
        e.size  = (uint32_t)strtoul(t[2].c_str(), 0, 0);
        cat.endpoints.push_back(std::move(e));
    }
    return Result<BusCatalog>::Ok(std::move(cat));
}

/* ---- canonical map hash (TPLC_E_MAP_EPOCH) --------------------------
   SHA-256 (FIPS 180-4) over a canonical serialization of the map's
   semantic content: image sizes, channels (direction/offset/size/
   file key/pdo endpoint/type/safe policy+value), bindings, safety
   keys — in declaration order.  Formatting, comments, and the `hash`
   directive itself are excluded, so restamping after a cosmetic edit
   yields the same digest.                                          */

namespace {

struct Sha256 {
    uint32_t h[8];
    uint64_t len = 0;
    uint8_t  buf[64];
    size_t   buflen = 0;

    static uint32_t rotr(uint32_t x, int n) { return (x >> n) | (x << (32 - n)); }

    void block(const uint8_t* p) {
        static const uint32_t K[64] = {
            0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,
            0x923f82a4,0xab1c5ed5,0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,
            0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,0xe49b69c1,0xefbe4786,
            0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
            0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,
            0x06ca6351,0x14292967,0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,
            0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,0xa2bfe8a1,0xa81a664b,
            0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
            0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,
            0x5b9cca4f,0x682e6ff3,0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,
            0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2 };
        uint32_t w[64];
        for (int i = 0; i < 16; i++)
            w[i] = (uint32_t)p[i*4] << 24 | (uint32_t)p[i*4+1] << 16 |
                   (uint32_t)p[i*4+2] << 8 | (uint32_t)p[i*4+3];
        for (int i = 16; i < 64; i++) {
            uint32_t s0 = rotr(w[i-15],7) ^ rotr(w[i-15],18) ^ (w[i-15] >> 3);
            uint32_t s1 = rotr(w[i-2],17) ^ rotr(w[i-2],19) ^ (w[i-2] >> 10);
            w[i] = w[i-16] + s0 + w[i-7] + s1;
        }
        uint32_t a=h[0],b=h[1],c=h[2],d=h[3],e=h[4],f=h[5],g=h[6],hh=h[7];
        for (int i = 0; i < 64; i++) {
            uint32_t S1 = rotr(e,6) ^ rotr(e,11) ^ rotr(e,25);
            uint32_t ch = (e & f) ^ (~e & g);
            uint32_t t1 = hh + S1 + ch + K[i] + w[i];
            uint32_t S0 = rotr(a,2) ^ rotr(a,13) ^ rotr(a,22);
            uint32_t mj = (a & b) ^ (a & c) ^ (b & c);
            uint32_t t2 = S0 + mj;
            hh=g; g=f; f=e; e=d+t1; d=c; c=b; b=a; a=t1+t2;
        }
        h[0]+=a; h[1]+=b; h[2]+=c; h[3]+=d; h[4]+=e; h[5]+=f; h[6]+=g; h[7]+=hh;
    }

    void update(const void* data, size_t n) {
        const uint8_t* p = static_cast<const uint8_t*>(data);
        len += n;
        while (n) {
            size_t take = 64 - buflen;
            if (take > n) take = n;
            std::memcpy(buf + buflen, p, take);
            buflen += take; p += take; n -= take;
            if (buflen == 64) { block(buf); buflen = 0; }
        }
    }

    void finish(uint8_t out[32]) {
        uint64_t bits = len * 8;
        uint8_t pad = 0x80;
        update(&pad, 1);
        uint8_t z = 0;
        while (buflen != 56) update(&z, 1);
        uint8_t lb[8];
        for (int i = 0; i < 8; i++) lb[i] = (uint8_t)(bits >> (56 - 8*i));
        update(lb, 8);
        for (int i = 0; i < 8; i++) {
            out[i*4]   = (uint8_t)(h[i] >> 24);
            out[i*4+1] = (uint8_t)(h[i] >> 16);
            out[i*4+2] = (uint8_t)(h[i] >> 8);
            out[i*4+3] = (uint8_t)h[i];
        }
    }

    Sha256() {
        h[0]=0x6a09e667; h[1]=0xbb67ae85; h[2]=0x3c6ef372; h[3]=0xa54ff53a;
        h[4]=0x510e527f; h[5]=0x9b05688c; h[6]=0x1f83d9ab; h[7]=0x5be0cd19;
    }
};

} /* namespace */

std::string map_hash(const IoMap& m) {
    std::string canon;
    canon += "tplc-map-v1\n";
    canon += "image in=" + std::to_string(m.in_size) +
             " out=" + std::to_string(m.out_size) + "\n";
    for (const auto& c : m.channels) {
        canon += c.is_input ? "in " : "out ";
        canon += "off=" + std::to_string(c.image_off) +
                 " size=" + std::to_string(c.size) +
                 " file=" + c.key +
                 " pdo=" + c.pdo +
                 " type=" + c.type +
                 " safe=" + std::to_string((int)c.safe) +
                 " sval=" + std::to_string(c.safe_value) + "\n";
    }
    for (const auto& b : m.bindings)
        canon += "bind " + b.at_name + " " + b.key + "\n";
    for (const auto& s : m.safety_keys)
        canon += "safety " + s + "\n";

    Sha256 s;
    s.update(canon.data(), canon.size());
    uint8_t d[32];
    s.finish(d);
    static const char* hex = "0123456789abcdef";
    std::string out(64, '0');
    for (int i = 0; i < 32; i++) {
        out[i*2]   = hex[d[i] >> 4];
        out[i*2+1] = hex[d[i] & 15];
    }
    return out;
}

std::vector<MapViolation> validate_map(const IoMap& m,
                                       const BusCatalog* bus) {
    std::vector<MapViolation> v;
    auto add = [&](int code, const char* rule,
                   const std::string& detail) {
        v.push_back({code, rule, detail});
    };

    /* epoch stamp: a declared `hash` must match the canonical hash of
       the map's current content — drift means the file was edited
       after stamping (TPLC_E_MAP_EPOCH)                        */
    if (!m.hash.empty() && m.hash != map_hash(m))
        add(TPLC_E_MAP_EPOCH, "MAP_EPOCH",
            "map hash " + m.hash.substr(0, 12) + "… != canonical " +
            map_hash(m).substr(0, 12) + "… (content drifted)");
    for (auto& c : m.channels) {
        uint32_t lim = c.is_input ? m.in_size : m.out_size;
        if (c.image_off + c.size > lim)
            add(TPLC_E_MAP_RANGE, "MAP_RANGE",
                "channel " + c.key + " outside image");
        uint32_t ts = type_size(c.type);
        if (!c.type.empty() && ts > c.size)
            add(TPLC_E_MAP_TYPE, "MAP_TYPE",
                "channel " + c.key + " type " + c.type +
                " wider than range");
        if (!c.is_input && !c.has_safe)
            add(TPLC_E_MAP_SAFE, "MAP_SAFE",
                "channel " + c.key + " has no safe policy");
        if (!c.is_input)
            for (auto& sk : m.safety_keys)
                if (sk == c.key)
                    add(TPLC_E_MAP_SAFETY, "MAP_SAFETY",
                        "safety endpoint " + c.key +
                        " bound to %Q");
        if (!c.is_input)
            for (auto& o : m.channels)
                if (!o.is_input && &o != &c &&
                    c.image_off < o.image_off + o.size &&
                    o.image_off < c.image_off + c.size)
                    add(TPLC_E_MAP_OWNER, "MAP_OWNER",
                        "%Q range of " + c.key +
                        " overlaps another writer");
        /* pdo= channels must name an endpoint that exists on the live
           bus with the matching direction and payload size — the pump's
           .bus catalog is the authority (TPLC_E_MAP_PDO).            */
        if (!c.pdo.empty()) {
            const PdoEndpoint* ep = nullptr;
            if (bus)
                for (auto& e : bus->endpoints)
                    if (e.key == c.pdo && e.is_tx == c.is_input)
                        ep = &e;
            if (!ep)
                add(TPLC_E_MAP_PDO, "MAP_PDO",
                    "channel " + c.key + " pdo endpoint " + c.pdo +
                    (bus ? " not on bus" : " has no bus catalog"));
            else if (ep->size != c.size)
                add(TPLC_E_MAP_PDO, "MAP_PDO",
                    "channel " + c.key + " size " +
                    std::to_string(c.size) + " != pdo " + c.pdo +
                    " size " + std::to_string(ep->size));
        }
    }
    for (auto& b : m.bindings) {
        char sp = 0;
        uint64_t off = 0;
        if (at_offset(b.at_name, sp, off)) {
            /* fixed AT: some channel must cover the byte offset */
            bool covered = false;
            for (auto& c : m.channels) {
                bool want_in = sp == 'I';
                if (c.is_input == want_in &&
                    off >= c.image_off && off < c.image_off + c.size)
                    covered = true;
            }
            if (!covered)
                add(TPLC_E_MAP_AT, "MAP_AT",
                    "fixed AT " + b.at_name +
                    " lands outside any channel");
        }
        if (b.key != "*") {
            const MapChannel* ch = nullptr;
            for (auto& c : m.channels)
                if (c.key == b.key) ch = &c;
            if (!ch)
                add(TPLC_E_MAP_BIND, "MAP_BIND",
                    "bind " + b.at_name + " → missing channel " +
                    b.key);
            else if (sp == 'I' || sp == 'Q' || sp == 'M') {
                /* direction: %I channels are inputs (written by the
                   bridge/master side), %Q channels are outputs; %M
                   markers are program scratch with no image channel */
                bool dir_ok = sp == 'I' ? ch->is_input
                                        : sp == 'Q' ? !ch->is_input
                                                    : false;
                if (!dir_ok)
                    add(TPLC_E_MAP_DIR, "MAP_DIR",
                        "bind " + b.at_name + " → channel " + b.key +
                        " direction mismatch");
            }
        }
    }
    return v;
}

MapRunner::MapRunner(const IoMap& map, ImageServer& img,
                     const std::string& sim_dir)
    : map_(map), img_(img), sim_dir_(sim_dir) {
    last_out_.resize(map.out_size, 0);
    if (!sim_dir_.empty()) ::mkdir(sim_dir_.c_str(), 0755);
}

void MapRunner::pump_inputs() {
    for (auto& c : map_.channels) {
        if (!c.is_input) continue;
        std::string p = sim_dir_ + "/" + c.key;
        uint8_t buf[256];
        if (c.size > sizeof buf) continue;
        int fd = ::open(p.c_str(), O_RDONLY);
        if (fd < 0) continue;               /* absent input → hold */
        ssize_t n = ::read(fd, buf, c.size);
        ::close(fd);
        if (n == (ssize_t)c.size)
            img_.writeInputs(buf, c.image_off, c.size);
    }
}

void MapRunner::pump_outputs(bool plc_ok) {
    /* single read of the %Q region, then per-channel safe policy */
    uint32_t n = map_.out_size;
    if (n == 0) return;
    if (n > last_out_.size()) last_out_.resize(n);
    std::vector<uint8_t> cur(n);
    if (img_.readOutputs(cur.data(), 0, n).ok() && plc_ok)
        last_out_ = cur;
    for (auto& c : map_.channels) {
        if (c.is_input) continue;
        std::string p = sim_dir_ + "/" + c.key;
        uint8_t buf[256];
        if (c.size > sizeof buf) continue;
        if (plc_ok) {
            memcpy(buf, last_out_.data() + c.image_off, c.size);
        } else switch (c.safe) {
            case SAFE_LAST:
                memcpy(buf, last_out_.data() + c.image_off, c.size);
                break;
            case SAFE_DEFINED:
                for (uint32_t i = 0; i < c.size; i++)
                    buf[i] = (uint8_t)(c.safe_value >> (i * 8));
                break;
            default:
                memset(buf, 0, c.size);
        }
        int fd = ::open(p.c_str(), O_WRONLY | O_CREAT | O_TRUNC, 0644);
        if (fd < 0) continue;
        ssize_t w = ::write(fd, buf, c.size);
        (void)w;
        ::close(fd);
    }
}

void MapRunner::cycle(bool plc_ok) {
    pump_inputs();
    pump_outputs(plc_ok);
    cycles_++;
}

} /* namespace tplc */
