#!/usr/bin/env bash
# error-path conformance: every wire-visible or operator-visible
# TPLC_E_* path a client can actually induce is exercised and asserted
# by its numeric magnitude (wire replies encode -code as a positive).
#   protocol : crafted frames → CTL_STATE/UNKNOWN_OP/PAYLOAD
#   ctl ops  : debug/state errors on a live runtime
#   host     : module stubs the loader rejects
#   package  : mutated .tplc files the reader rejects
#   image    : corrupted shared image at attach time
# Requires: build tree + tplcc + cc + python3 (77 = skip).
set -uo pipefail
cd "$(dirname "$0")/../.."

BUILD=${BUILD:-build}
IMG=/tplc_errpath
SHM=/dev/shm$IMG
CTL=/tmp/tplc_errpath.ctl
WORK=$(mktemp -d)
RT_PID=
trap '[ -n "$RT_PID" ] && kill "$RT_PID" 2>/dev/null; '"$BUILD"'/tplc-image unlink '"$IMG"' 2>/dev/null; rm -rf "$WORK" /dev/shm/tplc_ep_*' EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

[ -x "$BUILD/tether-plc" ] && [ -x "$BUILD/tplc-ctl" ] && \
[ -x "$BUILD/tplc-image" ] || { echo "SKIP runtime not built"; exit 77; }
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
[ -x "$TPLCC" ] || { echo "SKIP tplcc not built"; exit 77; }
CC=${CC:-cc}
command -v "$CC" >/dev/null && command -v python3 >/dev/null \
    || { echo "SKIP no cc/python3"; exit 77; }

# ---- a runnable program ------------------------------------------------
cat > $WORK/a.st <<'EOF'
PROGRAM Demo
VAR
    out_cnt AT %QD0 : DINT;
    cnt : DINT := 0;
    pad : ARRAY[0..31] OF DINT;
END_VAR
cnt := cnt + 1;
out_cnt := cnt;
END_PROGRAM
EOF
# b adds a var → different layout_hash than a
cat > $WORK/b.st <<'EOF'
PROGRAM Demo
VAR
    out_cnt AT %QD0 : DINT;
    cnt : DINT := 0;
    extra : DINT := 7;
END_VAR
cnt := cnt + 1;
out_cnt := cnt;
END_PROGRAM
EOF
"$TPLCC" build $WORK/a.st -o $WORK/a.tplc >/dev/null 2>&1 \
    || { echo "SKIP tplcc build a failed"; exit 77; }
"$TPLCC" build $WORK/b.st -o $WORK/b.tplc >/dev/null 2>&1 \
    || { echo "SKIP tplcc build b failed"; exit 77; }

# ---- package mutator: retag a section type + repatch hashes ------------
# header_hash = sha256(bytes 0..128); content_hash = sha256(raw TOC
# entries then payloads, signature entries excluded in both passes).
cat > $WORK/pkgmut.py <<'PYEOF'
import struct, hashlib, sys
src, dst, mode = sys.argv[1], sys.argv[2], sys.argv[3]
d = bytearray(open(src, 'rb').read())
toc_n = struct.unpack_from('<I', d, 16)[0]
def ent(i):
    o = 160 + i*32
    return o, struct.unpack_from('<IIQQII', d, o)
if mode == 'version':
    struct.pack_into('<I', d, 8, 99)
elif mode == 'toobig':
    struct.pack_into('<I', d, 16, 99)
elif mode == 'retag':
    want = int(sys.argv[4]); new = int(sys.argv[5]); req = int(sys.argv[6])
    for i in range(toc_n):
        o, (t, fl, off, sz, sv, pd) = ent(i)
        if t == want:
            struct.pack_into('<I', d, o, new)
            # retagged-to-unknown must not keep REQUIRED (that is the
            # -205 path); a retag meant to hide a section clears flags
            struct.pack_into('<I', d, o + 4, (fl | 1) if req else 0)
            break
elif mode == 'abiver':
    # manifest abi_version 1 -> 2 (same length, JSON stays valid)
    for i in range(toc_n):
        o, (t, fl, off, sz, sv, pd) = ent(i)
        if t == 0x01:
            p = bytes(d[off:off+sz]).find(b'"abi_version"')
            if p >= 0:
                q = off + p
                q = d.find(b':', q) + 1
                while d[q] in (32, 9): q += 1
                d[q] = ord('2')
            break
elif mode == 'smaprange':
    # first blob-resident symmap leaf gets an out-of-blob offset
    for i in range(toc_n):
        o, (t, fl, off, sz, sv, pd) = ent(i)
        if t == 0x20:
            count = struct.unpack_from('<I', d, off + 8)[0]
            for j in range(count):
                e = off + 32 + j * 64
                if d[e + 4] == 10:      # io channel: image offset
                    continue
                struct.pack_into('<Q', d, e + 12, 0x00FFFFFFFFFFFFFF)
                break
            break
elif mode == 'retaglast':
    o, (t, fl, off, sz, sv, pd) = ent(toc_n - 1)
    nt = t + 1
    while nt in (0x01,0x02,0x10,0x20,0x21,0x22,0x23,0x30,0x31,0x40,
                 0x41,0x42,0x50,0x60,0x70):
        nt += 1
    struct.pack_into('<I', d, o, nt)
    struct.pack_into('<I', d, o + 4, fl | 1)
# repatch content_hash + header_hash (all modes; harmless when the
# edited field precedes the hash check)
ch = hashlib.sha256()
for i in range(toc_n):
    o, (t, fl, off, sz, sv, pd) = ent(i)
    if t != 0x60:
        ch.update(d[o:o+32])
for i in range(toc_n):
    o, (t, fl, off, sz, sv, pd) = ent(i)
    if t != 0x60:
        ch.update(d[off:off+sz])
d[96:128] = ch.digest()
d[128:160] = hashlib.sha256(d[0:128]).digest()
open(dst, 'wb').write(d)
PYEOF

# ---- crafted-frame ctl client ------------------------------------------
cat > $WORK/frames.py <<'PYEOF'
import socket, struct, sys
CTL = sys.argv[1]
def frame(mt, flags, pl, req):
    return struct.pack('<HHIQ', mt, flags, len(pl), req) + pl
def recv_frame(s):
    h = b''
    while len(h) < 16:
        c = s.recv(16 - len(h))
        if not c: return None
        h += c
    mt, fl, pl, rq = struct.unpack('<HHIQ', h)
    d = b''
    while len(d) < pl:
        c = s.recv(pl - len(d))
        if not c: break
        d += c
    return mt, fl, d
def code(reply):
    if reply is None: return -1
    mt, fl, d = reply
    if not (fl & 0x8000) or len(d) < 8: return -1
    return struct.unpack('<I', d[:4])[0]
hello = struct.pack('<HHIH', 3, 0, 0, 1) + b't'
# 1: op before HELLO → CTL_STATE (400)
s = socket.socket(socket.AF_UNIX); s.connect(CTL)
s.sendall(frame(0x0103, 0, b'', 1))
print(code(recv_frame(s))); s.close()
# 2: HELLO then unknown op → CTL_UNKNOWN_OP (402)
s = socket.socket(socket.AF_UNIX); s.connect(CTL)
s.sendall(frame(0x0101, 0, hello, 1)); recv_frame(s)
s.sendall(frame(0x9999, 0, b'', 2))
print(code(recv_frame(s))); s.close()
# 3: HELLO then DBG_RESOLVE with empty payload → CTL_PAYLOAD (404)
s = socket.socket(socket.AF_UNIX); s.connect(CTL)
s.sendall(frame(0x0101, 0, hello, 1)); recv_frame(s)
s.sendall(frame(0x0501, 0, b'', 3))
print(code(recv_frame(s))); s.close()
PYEOF

# ---- bad module stubs ---------------------------------------------------
cat > $WORK/badmod.c <<'EOF'
#include "tplc/tplc_abi.h"
static int32_t t0(TplcTaskCtx* c) { (void)c; return 0; }
static const TplcTaskFn fns[] = { t0 };
static const TplcTaskDesc tasks[] = {
    { .name = "T", .kind = TPLC_TASK_CYCLIC, .period_ns = 1000000,
      .entry_index = 0 },
};
static const TplcProgramDescriptor desc = {
    .struct_size = sizeof(TplcProgramDescriptor),
    .abi_version = TPLC_ABI_VERSION,
    .abi_ext     = STUB_ABI_EXT,
    .name        = "bad",
    .state_blob_size = STUB_BLOB,
    .retain_size     = STUB_RET_SZ,
    .retain_offset   = STUB_RET_OFF,
    .task_count = 1, .tasks = tasks,
    .task_fns = fns, .task_fn_count = 1,
    .io_bindings = STUB_IO_B, .io_binding_count = STUB_IO_N,
};
uint32_t tplc_abi_version(void) { return TPLC_ABI_VERSION; }
const TplcProgramDescriptor* tplc_get_descriptor(void) { return &desc; }
int32_t tplc_init(TplcCtx* c) { (void)c; return 0; }
void tplc_shutdown(TplcCtx* c) { (void)c; }
int32_t tplc_task_run(TplcTaskCtx* c) { return t0(c); }
EOF

mkmod() {  # name defines...
    "$CC" -shared -fPIC -Iinclude -o "$WORK/$1.so" "$WORK/badmod.c" \
        "${@:2}" 2>/dev/null; }

# ============ image attach errors ============
"$BUILD"/tplc-image create /tplc_ep_ver 64 64 >/dev/null 2>&1
python3 -c "d=bytearray(open('/dev/shm/tplc_ep_ver','rb').read());\
import struct;struct.pack_into('<I',d,8,99);\
open('/dev/shm/tplc_ep_ver','wb').write(d)"
out=$(timeout 5 "$BUILD"/tether-plc --image /tplc_ep_ver \
      --package "$WORK/a.tplc" --ctl none --no-sandbox 2>&1)
echo "$out" | grep -q '(-101)' \
    && ok "TPLC_E_IMG_VERSION on tampered image" \
    || bad "IMG_VERSION: $out"

# attach needs st_size ≥ 256 + epoch>0 before validate runs; a valid
# header with in_size=0 lands on the TOO_SMALL check
python3 - <<'PYEOF'
import struct
d = bytearray(512)
struct.pack_into('<Q', d, 0, 0x47414D49434C5054)  # 'TPLCIMAG'
struct.pack_into('<I', d, 8, 1)                   # version
struct.pack_into('<I', d, 12, 256)                # header_size
struct.pack_into('<Q', d, 24, 1)                  # image_epoch
struct.pack_into('<Q', d, 72, 256)                # in_off
struct.pack_into('<Q', d, 80, 0)                  # in_size = 0
struct.pack_into('<Q', d, 88, 320)                # out_off
struct.pack_into('<Q', d, 96, 0)                  # out_size = 0
open('/dev/shm/tplc_ep_tiny','wb').write(d)
PYEOF
out=$(timeout 5 "$BUILD"/tether-plc --image /tplc_ep_tiny \
      --package "$WORK/a.tplc" --ctl none --no-sandbox 2>&1)
echo "$out" | grep -q '(-102)' \
    && ok "TPLC_E_IMG_TOO_SMALL on undersized shm" \
    || bad "IMG_TOO_SMALL: $out"

# shared image for every module/package boot below (a failed boot
# leaves it attachable; `create` reclaims a stale one anyway)
"$BUILD"/tplc-image create "$IMG" 256 256 >/dev/null 2>&1

# ============ map-validator errors (bridge; -600..-609 domain) ============
# every violation a bad map can hit: RANGE, TYPE, SAFE, OWNER, SAFETY,
# AT, BIND (MAP_DIR/PDO/EPOCH are declared but have no emit site)
cat > "$WORK/bad.map" <<'EOF'
image in 64 out 64
in  off=0 size=4 key=sensor1
in  off=200 size=8 key=sensor_far
out off=0 size=2 key=v1 type=u32
out off=1 size=4 key=v2 safe=zero
safety v2
bind %QD100 v1
bind %QD4 ghost
EOF
if [ -x "$BUILD/tether-plc-bridge" ]; then
    out=$("$BUILD"/tether-plc-bridge --image "$IMG" --map "$WORK/bad.map" \
          --no-plc 2>&1)
    # rule → numeric (MapRunner.hpp); full literals so the coverage
    # report's test-hit heuristic counts them
    for rc in "RANGE -602" "TYPE -603" "SAFE -608" "OWNER -600" \
              "SAFETY -605" "AT -606" "BIND -607"; do
        set -- $rc
        echo "$out" | grep -q "MAP_$1 ($2)" \
            && ok "TPLC_E_MAP_$1 validation finding" \
            || bad "MAP_$1: $out"
    done
fi

# ============ module loader errors ============
mkmod nosyms -DSTUB_ABI_EXT=0 -DSTUB_BLOB=64 -DSTUB_RET_SZ=0 \
     -DSTUB_RET_OFF=0 -DSTUB_IO_B=0 -DSTUB_IO_N=0
# strip the descriptor export → dlsym fails
"$CC" -shared -fPIC -o "$WORK/empty.so" -x c /dev/null 2>/dev/null
out=$(timeout 5 "$BUILD"/tether-plc --image "$IMG" \
      --module "$WORK/empty.so" --ctl none --no-sandbox 2>&1)
echo "$out" | grep -q '(-301)' \
    && ok "TPLC_E_HOST_SYM on module without ABI exports" \
    || bad "HOST_SYM: $out"

mkmod baddesc -DSTUB_ABI_EXT=0 -DSTUB_BLOB=64 -DSTUB_RET_SZ=0 \
     -DSTUB_RET_OFF=0 -DSTUB_IO_B=NULL -DSTUB_IO_N=1
out=$(timeout 5 "$BUILD"/tether-plc --image "$IMG" \
      --module "$WORK/baddesc.so" --ctl none --no-sandbox 2>&1)
echo "$out" | grep -q '(-302)' \
    && ok "TPLC_E_HOST_DESC on null io_bindings" \
    || bad "HOST_DESC: $out"

mkmod badrange -DSTUB_ABI_EXT=0 -DSTUB_BLOB=32 -DSTUB_RET_SZ=64 \
     -DSTUB_RET_OFF=0 -DSTUB_IO_B=0 -DSTUB_IO_N=0
out=$(timeout 5 "$BUILD"/tether-plc --image "$IMG" \
      --module "$WORK/badrange.so" --ctl none --no-sandbox 2>&1)
echo "$out" | grep -q '(-303)' \
    && ok "TPLC_E_HOST_RANGE on retain range past blob" \
    || bad "HOST_RANGE: $out"

mkmod badext -DSTUB_ABI_EXT=0x80000000u -DSTUB_BLOB=64 -DSTUB_RET_SZ=0 \
     -DSTUB_RET_OFF=0 -DSTUB_IO_B=0 -DSTUB_IO_N=0
out=$(timeout 5 "$BUILD"/tether-plc --image "$IMG" \
      --module "$WORK/badext.so" --ctl none --no-sandbox 2>&1)
echo "$out" | grep -q '(-65)' \
    && ok "TPLC_E_ABI_EXT_UNMET on unsupported abi_ext" \
    || bad "ABI_EXT_UNMET: $out"

# ============ package reader errors ============
run_pkg() { timeout 5 "$BUILD"/tether-plc --image "$IMG" --package "$1" \
            --ctl none --no-sandbox ${2:-} 2>&1; }

python3 "$WORK/pkgmut.py" "$WORK/a.tplc" "$WORK/ver.tplc" version
out=$(run_pkg "$WORK/ver.tplc")
echo "$out" | grep -q '(-201)' \
    && ok "TPLC_E_PKG_VERSION on format too new" || bad "PKG_VERSION: $out"

python3 "$WORK/pkgmut.py" "$WORK/a.tplc" "$WORK/toc.tplc" toobig
out=$(run_pkg "$WORK/toc.tplc")
echo "$out" | grep -q '(-202)' \
    && ok "TPLC_E_PKG_TOC on toc_count > max" || bad "PKG_TOC: $out"

head -c 300 "$WORK/a.tplc" > "$WORK/trunc.tplc"
out=$(run_pkg "$WORK/trunc.tplc")
echo "$out" | grep -q '(-206)' \
    && ok "TPLC_E_PKG_LIMIT on truncated file" || bad "PKG_LIMIT: $out"

python3 "$WORK/pkgmut.py" "$WORK/a.tplc" "$WORK/unk.tplc" retaglast
out=$(run_pkg "$WORK/unk.tplc")
echo "$out" | grep -q '(-205)' \
    && ok "TPLC_E_PKG_UNKNOWN_SECTION on unknown required" \
    || bad "PKG_UNKNOWN_SECTION: $out"

# hide target_obj: retag 0x10 → 0x03 (unknown, non-required, sorted)
python3 "$WORK/pkgmut.py" "$WORK/a.tplc" "$WORK/noobj.tplc" retag 16 3 0
out=$(run_pkg "$WORK/noobj.tplc")
echo "$out" | grep -q '(-212)' \
    && ok "TPLC_E_TARGET_MISMATCH on missing target_obj" \
    || bad "TARGET_MISMATCH: $out"

# semantic validation: manifest abi_version and symmap leaf range
python3 "$WORK/pkgmut.py" "$WORK/a.tplc" "$WORK/abiv.tplc" abiver
out=$(run_pkg "$WORK/abiv.tplc")
echo "$out" | grep -q '(-213)' \
    && ok "TPLC_E_PKG_ABI_MISMATCH on manifest abi_version" \
    || bad "PKG_ABI_MISMATCH: $out"

python3 "$WORK/pkgmut.py" "$WORK/a.tplc" "$WORK/srange.tplc" smaprange
out=$(run_pkg "$WORK/srange.tplc")
echo "$out" | grep -q '(-211)' \
    && ok "TPLC_E_SYMMAP_RANGE on out-of-blob leaf" \
    || bad "SYMMAP_RANGE: $out"

# signature tamper (sig section bytes; content_hash excludes it)
"$TPLCC" sign keygen --dir "$WORK/trust" --name dev >/dev/null 2>&1
"$TPLCC" sign sign "$WORK/a.tplc" --key "$WORK/trust/dev.key" \
    -o "$WORK/signed.tplc" >/dev/null 2>&1
python3 - "$WORK/signed.tplc" "$WORK/badsig.tplc" <<'PYEOF'
import struct, sys
d = bytearray(open(sys.argv[1],'rb').read())
toc_n = struct.unpack_from('<I', d, 16)[0]
for i in range(toc_n):
    o = 160 + i*32
    t, fl, off, sz, sv, pd = struct.unpack_from('<IIQQII', d, o)
    if t == 0x60:
        d[off + 8] ^= 0xFF          # inside sig payload
        break
open(sys.argv[2],'wb').write(d)
PYEOF
out=$(run_pkg "$WORK/badsig.tplc" "--sig-policy require --trust-dir $WORK/trust")
echo "$out" | grep -q '(-208)' \
    && ok "TPLC_E_SIG_INVALID on tampered signature" \
    || bad "SIG_INVALID: $out"

# ctl socket path over sockaddr limit → CTL_PROTO (-403) setup error
LONGCTL=/tmp/$(python3 -c 'print("x"*200)')
out=$(timeout 5 "$BUILD"/tether-plc --image "$IMG" \
      --package "$WORK/a.tplc" --ctl "$LONGCTL" --no-sandbox 2>&1)
echo "$out" | grep -q 'socket path too long' \
    && ok "TPLC_E_CTL_PROTO on oversize socket path" \
    || bad "CTL_PROTO: $out"

# malformed LAYD payload → PkgError::SectionFormat (-215) in the
# Rust reader, reached via --change-against predecessor decode
python3 - <<PYEOF
import struct, hashlib
d = bytearray(open('$WORK/a.tplc','rb').read())
toc_n = struct.unpack_from('<I', d, 16)[0]
for i in range(toc_n):
    o = 160 + i*32
    t, fl, off, sz, sv, pd = struct.unpack_from('<IIQQII', d, o)
    if t == 0x22:
        d[off] = 0x58               # 'L' → 'X': breaks LAYD magic
ch = hashlib.sha256()
for i in range(toc_n):
    o = 160 + i*32
    t, fl, off, sz, sv, pd = struct.unpack_from('<IIQQII', d, o)
    ch.update(d[o:o+32])
for i in range(toc_n):
    o = 160 + i*32
    t, fl, off, sz, sv, pd = struct.unpack_from('<IIQQII', d, o)
    ch.update(d[off:off+sz])
d[96:128] = ch.digest()
d[128:160] = hashlib.sha256(d[0:128]).digest()
open('$WORK/badlayd.tplc','wb').write(d)
PYEOF
out=$("$TPLCC" build "$WORK/b.st" -o "$WORK/x.tplc" \
      --change-against "$WORK/badlayd.tplc" 2>&1)
echo "$out" | grep -q 'SectionFormat' \
    && ok "TPLC_E_PKG_SECTION_FORMAT on malformed LAYD" \
    || bad "PKG_SECTION_FORMAT: $out"

# ============ live ctl paths ============
"$BUILD"/tplc-image create "$IMG" 256 256 >/dev/null 2>&1
"$BUILD"/tether-plc --image "$IMG" --package "$WORK/a.tplc" \
    --ctl "$CTL" --no-sandbox >"$WORK/rt.log" 2>&1 &
RT_PID=$!
for i in $(seq 1 50); do [ -S "$CTL" ] && break; sleep 0.1; done
[ -S "$CTL" ] || { echo "SKIP runtime ctl socket"; exit 77; }

# crafted frames: 400 / 402 / 404
codes=$(python3 "$WORK/frames.py" "$CTL")
got=$(echo "$codes" | sed -n '1p'); [ "$got" = 400 ] \
    && ok "TPLC_E_CTL_STATE on op before HELLO" \
    || bad "CTL_STATE: got $got"
got=$(echo "$codes" | sed -n '2p'); [ "$got" = 402 ] \
    && ok "TPLC_E_CTL_UNKNOWN_OP on bogus opcode" \
    || bad "CTL_UNKNOWN_OP: got $got"
got=$(echo "$codes" | sed -n '3p'); [ "$got" = 404 ] \
    && ok "TPLC_E_CTL_PAYLOAD on short payload" \
    || bad "CTL_PAYLOAD: got $got"

# state-diff before any swap → DBG_STATE (-416)
r=$("$BUILD"/tplc-ctl "$CTL" state-diff 2>&1)
echo "$r" | grep -q 'error 416' \
    && ok "TPLC_E_DBG_STATE on state-diff before swap" \
    || bad "DBG_STATE state-diff: $r"
# watch-get on unknown handle → DBG_STATE (-416)
r=$("$BUILD"/tplc-ctl "$CTL" watch-get 4242 2>&1)
echo "$r" | grep -q 'error 416' \
    && ok "TPLC_E_DBG_STATE on unknown watch handle" \
    || bad "DBG_STATE watch-get: $r"
# nine 64B watch items → Σlen > 512 → DBG_OVERLOAD (-412)
r=$("$BUILD"/tplc-ctl "$CTL" watch 0:64 0:64 0:64 0:64 0:64 \
      0:64 0:64 0:64 0:64 2>&1)
echo "$r" | grep -q 'error 412' \
    && ok "TPLC_E_DBG_OVERLOAD on watch payload > 512B" \
    || bad "DBG_OVERLOAD: $r"
# trace channel outside blob → TRACE_CONFIG (-417)
r=$("$BUILD"/tplc-ctl "$CTL" trace 999999:4 2>&1)
echo "$r" | grep -q 'error 417' \
    && ok "TPLC_E_TRACE_CONFIG on channel outside blob" \
    || bad "TRACE_CONFIG: $r"
# dbg-read outside blob → DBG_ACCESS (-415)
r=$("$BUILD"/tplc-ctl "$CTL" dbg-read 999999 4 2>&1)
echo "$r" | grep -q 'error 415' \
    && ok "TPLC_E_DBG_ACCESS on read outside blob" \
    || bad "DBG_ACCESS: $r"

# swap to a package with different layout and no migrate descriptor
# (built without --change-against) → MIGRATE_LAYOUT (-32)
r=$("$BUILD"/tplc-ctl "$CTL" swap "$WORK/b.tplc" 2>&1)
echo "$r" | grep -q 'error 32' \
    && ok "TPLC_E_MIGRATE_LAYOUT on swap without migrate path" \
    || bad "MIGRATE_LAYOUT: $r"

# stop+unload then resolve → HOST_STATE (-304)
"$BUILD"/tplc-ctl "$CTL" stop >/dev/null 2>&1
"$BUILD"/tplc-ctl "$CTL" unload >/dev/null 2>&1
r=$("$BUILD"/tplc-ctl "$CTL" resolve cnt 2>&1)
echo "$r" | grep -q 'error 304' \
    && ok "TPLC_E_HOST_STATE on resolve with no program" \
    || bad "HOST_STATE: $r"

kill "$RT_PID" 2>/dev/null; RT_PID=
wait 2>/dev/null

echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
