/* XmlsrvService.cpp — tplc-xmlsrv file-backed XML value store.
 * The document is a flat element map under a <root>:
 *   <root><key>value</key></root>
 * XPaths reduce to their last segment.  Reads/writes are plain
 * file operations; the file is rewritten atomically (tmp+rename)
 * so a crash cannot leave a half-written document.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/XmlsrvService.hpp>

#include <cstdio>
#include <cstring>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

namespace tplc {

namespace {

/* last segment of "a/b/c" -> "c" */
std::string key_of(const char* xpath) {
    std::string p = xpath ? xpath : "";
    auto slash = p.find_last_of('/');
    if (slash != std::string::npos) p = p.substr(slash + 1);
    return p;
}

std::string xml_escape(const std::string& s) {
    std::string o;
    for (char c : s) {
        switch (c) {
        case '<': o += "&lt;"; break;
        case '>': o += "&gt;"; break;
        case '&': o += "&amp;"; break;
        default: o += c;
        }
    }
    return o;
}

std::string xml_unescape(const std::string& s) {
    std::string o;
    for (size_t i = 0; i < s.size(); i++) {
        if (s[i] == '&') {
            if (s.compare(i, 4, "&lt;") == 0) { o += '<'; i += 3; continue; }
            if (s.compare(i, 4, "&gt;") == 0) { o += '>'; i += 3; continue; }
            if (s.compare(i, 5, "&amp;") == 0) { o += '&'; i += 4; continue; }
        }
        o += s[i];
    }
    return o;
}

struct Entry { std::string key, val; };

bool parse_doc(const std::string& doc, std::vector<Entry>& out) {
    size_t i = 0;
    while (true) {
        size_t a = doc.find('<', i);
        if (a == std::string::npos) break;
        size_t b = doc.find('>', a);
        if (b == std::string::npos) return false;
        std::string tag = doc.substr(a + 1, b - a - 1);
        if (tag.empty() || tag[0] == '/' || tag[0] == '?') { i = b + 1; continue; }
        /* the root is a container: descend into it, don't skip it */
        if (tag == "root") { i = b + 1; continue; }
        std::string close = "</" + tag + ">";
        size_t e = doc.find(close, b + 1);
        if (e == std::string::npos) return false;
        out.push_back({tag, xml_unescape(doc.substr(b + 1, e - b - 1))});
        i = e + close.size();
    }
    return true;
}

std::string render_doc(const std::vector<Entry>& entries) {
    std::string d = "<root>";
    for (const auto& e : entries)
        d += "<" + e.key + ">" + xml_escape(e.val) + "</" + e.key + ">";
    d += "</root>\n";
    return d;
}

}  // namespace

struct XmlsrvService::Impl {};

XmlsrvService::XmlsrvService() : impl_(new Impl) {
    svc_.struct_size = sizeof(TplcXmlsrvService);
    svc_.abi_version = TPLC_XMLSRV_ABI_VERSION;
    svc_.host = this;
    svc_.xml_read = [](void* s, const char* p, const char* x, uint8_t* o,
                       uint32_t n, uint32_t* l)
        { return self(s)->read(s, p, x, o, n, l); };
    svc_.xml_write = [](void* s, const char* p, const char* x,
                        const uint8_t* v, uint32_t n)
        { return self(s)->write(s, p, x, v, n); };
}
XmlsrvService::~XmlsrvService() = default;

int32_t XmlsrvService::read(void*, const char* path, const char* xpath,
                            uint8_t* out, uint32_t cap,
                            uint32_t* out_len) {
    if (!path || !xpath || !out || cap == 0) return TPLC_XMLSRV_E_PARAM;
    std::ifstream f(path);
    if (!f) return TPLC_XMLSRV_E_IO;
    std::stringstream ss;
    ss << f.rdbuf();
    std::vector<Entry> entries;
    if (!parse_doc(ss.str(), entries)) return TPLC_XMLSRV_E_XML;
    std::string key = key_of(xpath);
    for (const auto& e : entries) {
        if (e.key != key) continue;
        uint32_t n = (uint32_t)e.val.size();
        if (n >= cap) n = cap - 1;
        std::memcpy(out, e.val.data(), n);
        out[n] = 0;
        if (out_len) *out_len = n;
        return TPLC_XMLSRV_OK;
    }
    if (out_len) *out_len = 0;
    return TPLC_XMLSRV_E_NOKEY;
}

int32_t XmlsrvService::write(void*, const char* path, const char* xpath,
                             const uint8_t* value, uint32_t value_len) {
    if (!path || !xpath) return TPLC_XMLSRV_E_PARAM;
    std::string key = key_of(xpath);
    if (key.empty()) return TPLC_XMLSRV_E_PARAM;
    std::vector<Entry> entries;
    {
        std::ifstream f(path);
        if (f) {
            std::stringstream ss;
            ss << f.rdbuf();
            if (!parse_doc(ss.str(), entries)) entries.clear();
        }
    }
    std::string val((const char*)value, value ? value_len : 0);
    bool found = false;
    for (auto& e : entries)
        if (e.key == key) { e.val = val; found = true; break; }
    if (!found) entries.push_back({key, val});
    std::string tmp = std::string(path) + ".tmp";
    {
        std::ofstream o(tmp, std::ios::trunc);
        if (!o) return TPLC_XMLSRV_E_IO;
        o << render_doc(entries);
        if (!o.good()) return TPLC_XMLSRV_E_IO;
    }
    if (std::rename(tmp.c_str(), path) != 0) {
        std::remove(tmp.c_str());
        return TPLC_XMLSRV_E_IO;
    }
    return TPLC_XMLSRV_OK;
}

} /* namespace tplc */
