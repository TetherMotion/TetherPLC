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
        if (t[0] != "in" && t[0] != "out")
            return bad("expected image|in|out");
        MapChannel c;
        c.is_input = t[0] == "in";
        for (size_t k = 1; k < t.size(); k++) {
            std::string v;
            if      (tok(t[k], "off=", v))  c.image_off = (uint32_t)strtoul(v.c_str(),0,0);
            else if (tok(t[k], "size=", v)) c.size      = (uint32_t)strtoul(v.c_str(),0,0);
            else if (tok(t[k], "key=", v))  c.key = v;
            else if (tok(t[k], "safe=", v)) {
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
        uint32_t lim = c.is_input ? m.in_size : m.out_size;
        if (c.image_off + c.size > lim)
            return bad("channel outside image");
        if (!c.is_input) {
            /* doc 09 §2.2: exactly one %Q writer — no overlap */
            for (auto& o : m.channels)
                if (!o.is_input &&
                    c.image_off < o.image_off + o.size &&
                    o.image_off < c.image_off + c.size)
                    return bad("overlapping %Q writer (MAP_OWNER)");
        }
        m.channels.push_back(std::move(c));
    }
    return Result<IoMap>::Ok(std::move(m));
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
