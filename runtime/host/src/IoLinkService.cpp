/* IoLinkService.cpp — tplc-iolink: IO-Link master data-link protocol
 * (IEC 61131-9 / SDCI) over per-port TCP byte transports.
 *
 * Wire format implemented (IO-Link Interface Spec, data-link layer):
 *   master frame : [MC][CKT][PD-out octets][OD octets]
 *   device frame : [PD-in octets][OD octets][CKS]
 *   MC octet     : bit7 = read(1)/write(0), bits6-5 = channel
 *                  (PROCESS 0x00, PAGE 0x20, DIAGNOSIS 0x40, ISDU 0x60),
 *                  bits4-0 = address / ISDU FlowCTRL.
 *   CKT octet    : bits7-6 = M-sequence type, bits5-0 = A.1.6 checksum.
 *   CKS octet    : bit7 = event pending, bit6 = PD-in invalid,
 *                  bits5-0 = A.1.6 checksum.
 *   Checksum A.1.6: XOR all octets (seed 0x52) with the checksum octet
 *   itself entering with bits 0-5 zeroed, then fold 8->6 bits.
 *
 * Transport: one TCP endpoint per port from TPLC_IOLINK_IO (comma-
 * separated tcp://host:port list). Real IO-Link devices need 24 V
 * signaling — deploy behind a PHY-equipped adapter; this stack speaks
 * the data-link/application protocol. Synchronous per call.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/IoLinkService.hpp>

#include <arpa/inet.h>
#include <cerrno>
#include <cstring>
#include <fcntl.h>
#include <netdb.h>
#include <poll.h>
#include <string>
#include <sys/socket.h>
#include <unistd.h>
#include <cstdlib>

namespace tplc {

struct IoLinkService::Port {
    int fd = -1;
    std::string endpoint;
    uint32_t pst = TPLC_IL_ST_CLOSED;
    uint32_t errors = 0;
    uint32_t frames = 0;
    uint32_t cksFlags = 0;
    uint8_t dpp1[16] = {};
    bool haveIdent = false;
};

namespace {

constexpr uint8_t MC_READ = 0x80;
constexpr uint8_t CH_PROCESS = 0x00;
constexpr uint8_t CH_PAGE = 0x20;
constexpr uint8_t CH_ISDU = 0x60;
constexpr uint8_t FLOWCTRL_IDLE = 0x11;
constexpr uint8_t MSEQ_T2V = 6;
constexpr uint8_t WAKE_BYTE = 0x55;
constexpr int IO_TIMEOUT_MS = 100;

uint8_t checksum6(const uint8_t* d, size_t n) {
    uint8_t ck = 0x52;
    for (size_t i = 0; i < n; ++i) ck ^= d[i];
    const auto b = [&](int i) { return (ck >> i) & 1; };
    uint8_t r = 0;
    r |= uint8_t((b(7) ^ b(5) ^ b(3) ^ b(1)) << 5);
    r |= uint8_t((b(6) ^ b(4) ^ b(2) ^ b(0)) << 4);
    r |= uint8_t((b(7) ^ b(6)) << 3);
    r |= uint8_t((b(5) ^ b(4)) << 2);
    r |= uint8_t((b(3) ^ b(2)) << 1);
    r |= uint8_t(b(1) ^ b(0));
    return r;
}

/* build CKT: type bits | checksum over [MC][CKT=typebits][rest] */
uint8_t buildCkt(uint8_t mc, uint8_t mseqType,
                 const uint8_t* rest, size_t restLen,
                 uint8_t* frame) {
    frame[0] = mc;
    frame[1] = uint8_t(mseqType << 6); /* checksum bits zeroed */
    if (restLen) memcpy(frame + 2, rest, restLen);
    const uint8_t ck = checksum6(frame, 2 + restLen);
    frame[1] = uint8_t((mseqType << 6) | ck);
    return frame[1];
}

int tcpConnect(const std::string& ep, int timeoutMs) {
    std::string s = ep;
    const std::string p = "tcp://";
    if (s.rfind(p, 0) == 0) s = s.substr(p.size());
    const size_t colon = s.rfind(':');
    if (colon == std::string::npos) return -1;
    const std::string host = s.substr(0, colon);
    const int port = std::stoi(s.substr(colon + 1));
    addrinfo hints{}, *res = nullptr;
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;
    if (getaddrinfo(host.c_str(), std::to_string(port).c_str(), &hints,
                    &res) != 0 || !res)
        return -1;
    int fd = -1;
    for (auto* ai = res; ai && fd < 0; ai = ai->ai_next) {
        fd = socket(ai->ai_family, ai->ai_socktype, ai->ai_protocol);
        if (fd < 0) continue;
        /* nonblocking connect with timeout */
        const int flags = fcntl(fd, F_GETFL, 0);
        fcntl(fd, F_SETFL, flags | O_NONBLOCK);
        if (connect(fd, ai->ai_addr, ai->ai_addrlen) != 0) {
            if (errno != EINPROGRESS) { close(fd); fd = -1; continue; }
            pollfd pfd{fd, POLLOUT, 0};
            if (poll(&pfd, 1, timeoutMs) <= 0 ||
                !(pfd.revents & POLLOUT)) {
                close(fd); fd = -1; continue;
            }
            int soerr = 0; socklen_t sl = sizeof(soerr);
            getsockopt(fd, SOL_SOCKET, SO_ERROR, &soerr, &sl);
            if (soerr) { close(fd); fd = -1; continue; }
        }
        fcntl(fd, F_SETFL, flags); /* back to blocking */
        struct timeval tv{0, IO_TIMEOUT_MS * 1000};
        setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv));
        setsockopt(fd, SOL_SOCKET, SO_SNDTIMEO, &tv, sizeof(tv));
    }
    freeaddrinfo(res);
    return fd;
}

ssize_t xwrite(int fd, const uint8_t* d, size_t n) {
    size_t off = 0;
    while (off < n) {
        ssize_t w = write(fd, d + off, n - off);
        if (w <= 0) return -1;
        off += size_t(w);
    }
    return ssize_t(off);
}

/* read exactly n bytes (blocking w/ SO_RCVTIMEO) */
int xread(int fd, uint8_t* d, size_t n) {
    size_t off = 0;
    while (off < n) {
        ssize_t r = read(fd, d + off, n - off);
        if (r <= 0) return -1;
        off += size_t(r);
    }
    return 0;
}

IoLinkService* self(void* s) { return static_cast<IoLinkService*>(s); }

} // namespace

IoLinkService::IoLinkService() {
    /* TPLC_IOLINK_IO: comma-separated tcp://host:port per port */
    const char* env = std::getenv("TPLC_IOLINK_IO");
    std::vector<std::string> eps;
    if (env && *env) {
        std::string all(env), cur;
        for (char c : all) {
            if (c == ',') { if (!cur.empty()) eps.push_back(cur); cur.clear(); }
            else cur += c;
        }
        if (!cur.empty()) eps.push_back(cur);
    }
    if (eps.empty()) {
        /* fall back to the shared EC endpoint for port 0 */
        const char* fb = std::getenv("TPLC_CANOPEN_IO");
        if (!fb || !*fb) fb = std::getenv("TPLC_EC_IO");
        if (!fb || !*fb) fb = std::getenv("TPLC_MOTION_IO");
        if (fb && *fb) eps.push_back(fb);
    }
    for (size_t i = 0; i < eps.size() && i < TPLC_IOLINK_MAX_PORTS; ++i) {
        auto p = std::make_unique<Port>();
        p->endpoint = eps[i];
        ports_.push_back(std::move(p));
    }
    active_ = !ports_.empty();

    svc_.struct_size = sizeof(TplcIoLinkService);
    svc_.abi_version = TPLC_IOLINK_ABI_VERSION;
    svc_.host = this;
    svc_.il_state = [](void* s) { return self(s)->state(s); };
    svc_.il_open = [](void* s, uint32_t p) { return self(s)->open(s, p); };
    svc_.il_close = [](void* s, uint32_t p) { return self(s)->close(s, p); };
    svc_.il_ident = [](void* s, uint32_t p, uint8_t* o, uint32_t c,
                       uint32_t* l) {
        return self(s)->ident(s, p, o, c, l);
    };
    svc_.il_pd_exchange = [](void* s, uint32_t p, uint32_t m,
                             const uint8_t* pd, uint32_t pl,
                             uint8_t* pi, uint32_t pc,
                             uint32_t* pl2, uint32_t* cf) {
        return self(s)->pdExchange(s, p, m, pd, pl, pi, pc, pl2, cf);
    };
    svc_.il_isdu_read = [](void* s, uint32_t p, uint8_t a,
                           const uint8_t* od, uint32_t ol,
                           uint8_t* r, uint32_t rc, uint32_t* rl) {
        return self(s)->isduRead(s, p, a, od, ol, r, rc, rl);
    };
    svc_.il_isdu_write = [](void* s, uint32_t p, uint8_t a,
                            const uint8_t* od, uint32_t ol) {
        return self(s)->isduWrite(s, p, a, od, ol);
    };
    svc_.il_port_info = [](void* s, uint32_t p, uint32_t* st,
                           uint32_t* e, uint32_t* f) {
        return self(s)->portInfo(s, p, st, e, f);
    };
}

IoLinkService::~IoLinkService() = default;

int32_t IoLinkService::state(void*) { return active_ ? 1 : 0; }

IoLinkService::Port* IoLinkService::portAt(uint32_t i) {
    if (i >= ports_.size()) return nullptr;
    return ports_[i].get();
}

int32_t IoLinkService::open(void*, uint32_t port) {
    Port* p = portAt(port);
    if (!p) return TPLC_IL_E_PARAM;
    if (p->fd >= 0) return TPLC_IL_OK;
    p->fd = tcpConnect(p->endpoint, 2000);
    if (p->fd < 0) { p->pst = TPLC_IL_ST_ERROR; ++p->errors; return TPLC_IL_E_IO; }
    /* wakeup: burst of wake bytes puts a sleeping device on COM1;
       on an always-listening device they are inert padding */
    uint8_t wake[8];
    memset(wake, WAKE_BYTE, sizeof(wake));
    if (xwrite(p->fd, wake, sizeof(wake)) < 0) {
        ::close(p->fd); p->fd = -1; p->pst = TPLC_IL_ST_ERROR;
        ++p->errors; return TPLC_IL_E_IO;
    }
    p->pst = TPLC_IL_ST_OPEN;
    p->haveIdent = false;
    return TPLC_IL_OK;
}

int32_t IoLinkService::close(void*, uint32_t port) {
    Port* p = portAt(port);
    if (!p) return TPLC_IL_E_PARAM;
    if (p->fd >= 0) { ::close(p->fd); p->fd = -1; }
    p->pst = TPLC_IL_ST_CLOSED;
    return TPLC_IL_OK;
}

/* send one master frame, read+validate one device reply.
   tx: full frame bytes; rxLen: expected reply octets before CKS.
   Returns TPLC_IL_OK and fills reply/cks on success. */
int32_t IoLinkService::exchange(Port* p, const uint8_t* tx, size_t txLen,
                 uint8_t* rx, size_t rxLen, uint8_t* cksOut,
                 uint32_t* cksFlags) {
    if (p->fd < 0) return TPLC_IL_E_NOEP;
    if (xwrite(p->fd, tx, txLen) < 0) { ++p->errors; return TPLC_IL_E_IO; }
    uint8_t buf[80];
    if (xread(p->fd, buf, rxLen + 1) != 0) { ++p->errors; return TPLC_IL_E_IO; }
    const uint8_t cks = buf[rxLen];
    /* verify: checksum6 over reply octets with CKS bits0-5 zeroed */
    uint8_t chk[80];
    memcpy(chk, buf, rxLen);
    chk[rxLen] = uint8_t(cks & 0xC0); /* flags only, checksum bits 0 */
    const uint8_t want = checksum6(chk, rxLen + 1);
    if (want != (cks & 0x3F)) { ++p->errors; return TPLC_IL_E_CKS; }
    if (cksOut) *cksOut = cks;
    if (cksFlags) *cksFlags = cks & 0xC0;
    if (rxLen) memcpy(rx, buf, rxLen);
    ++p->frames;
    p->cksFlags = cks & 0xC0;
    return TPLC_IL_OK;
}

/* page-channel read: one octet of the device parameter page.
   MC = read|PAGE|addr, type-0 frame, reply [OD][CKS]. */
int32_t IoLinkService::pageRead(Port* p, uint8_t addr, uint8_t* od) {
    uint8_t tx[4];
    buildCkt(uint8_t(MC_READ | CH_PAGE | addr), 0, nullptr, 0, tx);
    uint8_t rx[2]; uint8_t cks = 0;
    int32_t rc = exchange(p, tx, 2, rx, 1, &cks, nullptr);
    if (rc != TPLC_IL_OK) return rc;
    *od = rx[0];
    return TPLC_IL_OK;
}

int32_t IoLinkService::ident(void*, uint32_t port, uint8_t* out,
                             uint32_t cap, uint32_t* out_len) {
    Port* p = portAt(port);
    if (!p) return TPLC_IL_E_PARAM;
    if (p->fd < 0 || p->pst < TPLC_IL_ST_OPEN)
        return TPLC_IL_E_STATE;
    if (p->haveIdent && out_len) {
        if (cap >= 16 && out) memcpy(out, p->dpp1, 16);
        *out_len = 16;
        return TPLC_IL_OK;
    }
    /* DPP1 record: 16 page reads at addresses 0x00-0x0F */
    for (uint8_t a = 0; a < 16; ++a) {
        int32_t rc = pageRead(p, a, &p->dpp1[a]);
        if (rc != TPLC_IL_OK) { ++p->errors; return rc; }
    }
    p->haveIdent = true;
    p->pst = TPLC_IL_ST_IDENT;
    if (out_len) *out_len = cap < 16 ? cap : 16;
    if (out && cap) memcpy(out, p->dpp1, *out_len < 16 ? *out_len : 16);
    return TPLC_IL_OK;
}

/* one cyclic frame, M-seq type 2_V (PD both ways + optional OD).
   tx: [MC][CKT][PDO][OD]; rx: [PDI][OD][CKS]. */
int32_t IoLinkService::pdExchange(void*, uint32_t port, uint32_t mseq,
                                  const uint8_t* pdo, uint32_t pdo_len,
                                  uint8_t* pdi, uint32_t pdi_cap,
                                  uint32_t* pdi_len, uint32_t* cks_flags) {
    (void)mseq; /* codec currently emits T2V; other types staged */
    Port* p = portAt(port);
    if (!p) return TPLC_IL_E_PARAM;
    if (p->fd < 0 || p->pst < TPLC_IL_ST_IDENT)
        return TPLC_IL_E_STATE;
    if (pdo_len > TPLC_IOLINK_MAX_PD || pdi_cap > TPLC_IOLINK_MAX_PD +
        TPLC_IOLINK_MAX_OD + 8)
        return TPLC_IL_E_PARAM;
    /* negotiated sizes come from the DPP1 when known; until the AL
       negotiation lands we use the caller's lengths */
    const uint32_t pdi_len_dev = pdi_cap; /* reply carries pdi_cap octets */
    uint8_t tx[80];
    /* rest = PDO octets then OD octets (none for cyclic PD) */
    buildCkt(uint8_t((pdo_len ? 0 : MC_READ) | CH_PROCESS | 0),
             MSEQ_T2V, pdo, pdo_len, tx);
    const size_t txLen = 2 + pdo_len;
    /* device reply: pdi_len_dev PD-in octets + 0 OD + CKS — we don't
       know the device's PD-in length a priori; use the DPP1 pd-in
       descriptor when identified, else pdi_cap */
    uint32_t wantPdi = pdi_len_dev;
    if (p->haveIdent) {
        const uint8_t desc = p->dpp1[5];
        const bool isBytes = (desc & 0x80) != 0;
        wantPdi = isBytes ? (desc & 0x1F) : ((desc & 0x1F) + 7) / 8;
        if (wantPdi > pdi_cap) wantPdi = pdi_cap;
    }
    uint8_t rx[80];
    int32_t rc = exchange(p, tx, txLen, rx, wantPdi, nullptr, cks_flags);
    if (rc != TPLC_IL_OK) return rc;
    if (pdi && wantPdi) memcpy(pdi, rx, wantPdi);
    if (pdi_len) *pdi_len = wantPdi;
    if (p->pst == TPLC_IL_ST_IDENT) p->pst = TPLC_IL_ST_OPERATE;
    return TPLC_IL_OK;
}

/* Type-0 ISDU exchange: [MC=rw|ISDU|flowctrl][CKT] (+OD on write)
   -> [OD][CKS]. Single-octet services (segmentation is staged). */
int32_t IoLinkService::isduRead(void*, uint32_t port, uint8_t addr,
                                const uint8_t* od, uint32_t od_len,
                                uint8_t* resp, uint32_t resp_cap,
                                uint32_t* resp_len) {
    (void)od; (void)od_len;
    Port* p = portAt(port);
    if (!p) return TPLC_IL_E_PARAM;
    if (p->fd < 0 || p->pst < TPLC_IL_ST_IDENT)
        return TPLC_IL_E_STATE;
    uint8_t tx[8];
    buildCkt(uint8_t(MC_READ | CH_ISDU | (addr & 0x0F)), 0,
             nullptr, 0, tx);
    uint8_t rx[2]; uint8_t cks = 0;
    int32_t rc = exchange(p, tx, 2, rx, 1, &cks, nullptr);
    if (rc != TPLC_IL_OK) return rc;
    if (resp && resp_cap) resp[0] = rx[0];
    if (resp_len) *resp_len = 1;
    return TPLC_IL_OK;
}

int32_t IoLinkService::isduWrite(void*, uint32_t port, uint8_t addr,
                                 const uint8_t* od, uint32_t od_len) {
    Port* p = portAt(port);
    if (!p) return TPLC_IL_E_PARAM;
    if (p->fd < 0 || p->pst < TPLC_IL_ST_IDENT)
        return TPLC_IL_E_STATE;
    if (!od || od_len != 1) return TPLC_IL_E_PARAM; /* single-octet svc */
    uint8_t tx[8];
    buildCkt(uint8_t(CH_ISDU | (addr & 0x0F)), 0, od, 1, tx);
    /* write expects an OD echo + CKS reply */
    uint8_t rx[2];
    return exchange(p, tx, 3, rx, 1, nullptr, nullptr);
}

int32_t IoLinkService::portInfo(void*, uint32_t port, uint32_t* st,
                                uint32_t* errors, uint32_t* frames) {
    Port* p = portAt(port);
    if (!p) return TPLC_IL_E_PARAM;
    if (st) *st = p->pst;
    if (errors) *errors = p->errors;
    if (frames) *frames = p->frames;
    return TPLC_IL_OK;
}

} // namespace tplc
