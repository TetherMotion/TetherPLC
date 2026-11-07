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
        if (t[0] != "in" && t[0] != "out")
            return bad("expected image|in|out|bind|safety");
        MapChannel c;
        c.is_input = t[0] == "in";
        for (size_t k = 1; k < t.size(); k++) {
            std::string v;
            if      (tok(t[k], "off=", v))  c.image_off = (uint32_t)strtoul(v.c_str(),0,0);
            else if (tok(t[k], "size=", v)) c.size      = (uint32_t)strtoul(v.c_str(),0,0);
            else if (tok(t[k], "key=", v))  c.key = v;
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

std::vector<MapViolation> validate_map(const IoMap& m) {
    std::vector<MapViolation> v;
    auto add = [&](int code, const char* rule,
                   const std::string& detail) {
        v.push_back({code, rule, detail});
    };
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
            bool found = false;
            for (auto& c : m.channels)
                if (c.key == b.key) found = true;
            if (!found)
                add(TPLC_E_MAP_BIND, "MAP_BIND",
                    "bind " + b.at_name + " → missing channel " +
                    b.key);
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
