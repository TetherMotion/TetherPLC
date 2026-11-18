/* Golden-capture codec test: decodes the real p-net IODRead
 * request/response frames captured 2026-08-30 (S7-1500 CPU <-> p-net
 * device, testdata/alarm/*.hex in the pnio reference crate) and checks
 * the recovered fields against the capture's documented identity.
 * Pure codec — no network. Exit 0 on success. */
#include <tplc/host/PnioCm.hpp>
#include <cstdio>
#include <cctype>
#include <cstring>
#include <string>
#include <vector>
#include <fstream>

using namespace tplc::host;

static std::vector<uint8_t> loadHex(const char* path) {
    std::ifstream f(path);
    std::string line, all;
    while (std::getline(f, line)) {
        if (line.empty() || line[0] == '#') continue;
        all += line;
    }
    std::string h;
    for (char c : all)
        if (std::isxdigit((unsigned char)c)) h += c;
    std::vector<uint8_t> out;
    out.reserve(h.size() / 2);
    for (size_t i = 0; i + 1 < h.size(); i += 2)
        out.push_back(uint8_t(std::stoul(h.substr(i, 2), 0, 16)));
    return out;
}

static int fails = 0;
#define CHECK(cond, msg) do { if (!(cond)) { \
    printf("FAIL %s:%d %s\n", __FILE__, __LINE__, msg); ++fails; } } while (0)

int main(int argc, char** argv) {
    const char* dir = argc > 1 ? argv[1] : "golden";
    const std::string p = std::string(dir) + "/";

    /* ---- IODRead request (CPU -> device, opnum 2, ptype Request) ---- */
    auto req = loadHex((p + "im0_read_req.hex").c_str());
    CHECK(req.size() == 206, "req frame size");
    PnioRpcHeader rh;
    CHECK(pnioDecRpcHeader(req.data() + 42, req.size() - 42, rh), "req rpc hdr");
    CHECK(rh.ptype == PnioPtype::Request && rh.opnum == 2, "req ptype/opnum");
    const uint8_t* rb = req.data() + 142; /* 42 eth/ip/udp + 80 rpc + 20 ndr */
    CHECK(uint16_t(rb[0] << 8 | rb[1]) == kBlkIodReadReqHeader, "req block type");
    {
        const uint8_t* b = rb + 6;
        const size_t bl = (uint16_t(rb[2] << 8 | rb[3])) - 2;
        CHECK(bl == 58, "req body len");
        CHECK(uint16_t(b[0] << 8 | b[1]) == 6, "req seq echo");
        CHECK(uint16_t(b[28] << 8 | b[29]) == kPnioIndexIm0, "req index AFF0");
        CHECK(uint16_t(b[22] << 8 | b[23]) == 0 &&
              uint16_t(b[24] << 8 | b[25]) == 1, "req slot/subslot");
    }

    /* ---- IODRead response (device -> CPU, ptype Response) ---- */
    auto res = loadHex((p + "im0_read_res.hex").c_str());
    CHECK(res.size() == 266, "res frame size");
    CHECK(pnioDecRpcHeader(res.data() + 42, res.size() - 42, rh), "res rpc hdr");
    CHECK(rh.ptype == PnioPtype::Response && rh.opnum == 2, "res ptype/opnum");
    std::vector<PnioReadResRecord> rr;
    CHECK(pnioDecReadRes(res.data() + 142, res.size() - 142, rr), "decReadRes");
    CHECK(rr.size() == 1, "one res record");
    if (!rr.empty()) {
        const auto& r = rr[0];
        CHECK(r.req.seq == 6 && r.req.index == kPnioIndexIm0 &&
              r.req.slot == 0 && r.req.subslot == 1, "res echo fields");
        CHECK(r.data.size() == 60, "res record len (6 hdr + 54 body)");
        PnioIm0View v;
        CHECK(pnioDecIm0(r.data.data(), r.data.size(), v), "decIm0");
        CHECK(v.vendorId == 0x0493, "vendor 0x0493");
        CHECK(v.orderId == "12345 Abcdefghijk", "order id");
        CHECK(v.serialNumber == "007", "serial");
        CHECK(v.hardwareRevision == 0x0003, "hw rev");
        CHECK(v.swPrefix == 'V' && v.swFunctional == 0 &&
              v.swBugFix == 2 && v.swInternal == 0, "sw rev V.0.2.0");
        CHECK(v.supported == kPnioImSupportedDap, "supported DAP mask");
    }

    /* ---- implicit-read variant response ---- */
    auto resif = loadHex((p + "im0_read_res_if.hex").c_str());
    std::vector<PnioReadResRecord> rr3;
    CHECK(pnioDecReadRes(resif.data() + 142, resif.size() - 142, rr3) &&
          rr3.size() == 1, "res_if decReadRes");
    if (!rr3.empty()) {
        PnioIm0View v3;
        CHECK(pnioDecIm0(rr3[0].data.data(), rr3[0].data.size(), v3) &&
              v3.vendorId == 0x0493 && v3.orderId == "12345 Abcdefghijk",
              "res_if IM0 identity");
    }

    /* ---- truncated / wrong-type negatives ---- */
    {
        std::vector<PnioReadResRecord> bad;
        CHECK(!pnioDecReadRes(res.data() + 142, 10, bad), "truncated res");
        uint8_t wrong[8] = {0x00, 0x10, 0x00, 0x08, 1, 0, 0, 0};
        CHECK(!pnioDecReadRes(wrong, sizeof wrong, bad), "wrong block type");
    }

    if (fails) { printf("golden: %d FAILURES\n", fails); return 1; }
    printf("golden: all checks passed\n");
    return 0;
}
