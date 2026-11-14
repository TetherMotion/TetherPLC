#!/usr/bin/env bash
# make-deb.sh — build a .deb of the TetherPLC toolchain.
#
# Packages: tplcc/tplc-pkg (Rust compiler), tether-plc + tether-plc-sim
# + tplc-* (C++ runtime/control), the Atc_TempCtrl + Tc2/Tc3 compat
# library sources, and a prebuilt atc_tempctrl.tplc package.
#
# Prerequisites:
#   cmake --build build -j8          # C++ binaries (RelWithDebInfo ok)
#   cargo build --release -p tplcc -p tplc-pkg --manifest-path compiler/Cargo.toml
#   dpkg-deb                          # Debian/Ubuntu packaging tool
#
# Usage: scripts/make-deb.sh [output-dir] [version]
#   default output: build/deb, version: 0.1.0-1
set -euo pipefail
cd "$(dirname "$0")/.."

OUT=${1:-build/deb}
VER=${2:-0.1.0-1}
PKG=tetherplc
ARCH=amd64
ROOT=$OUT/${PKG}_${VER}_${ARCH}
BIN=build
RUST=compiler/target/release

for b in tplcc tplc-pkg; do
    [ -x "$RUST/$b" ] || { echo "missing $RUST/$b — cargo build --release first" >&2; exit 1; }
done
for b in tether-plc tether-plc-sim tplc-io tplc-ctl tplc-ctl-proxy tplc-image \
         tplc-io-probe tplc-io-cli; do
    [ -x "$BIN/$b" ] || { echo "missing $BIN/$b — cmake --build build first" >&2; exit 1; }
done
command -v dpkg-deb >/dev/null || { echo "dpkg-deb not found" >&2; exit 1; }

rm -rf "$ROOT"
mkdir -p "$ROOT/DEBIAN" "$ROOT/usr/bin" "$ROOT/usr/share/$PKG" \
         "$ROOT/usr/share/doc/$PKG" "$ROOT/usr/share/$PKG/lib"

# --- binaries (stripped copies; build/ keeps the originals) -------------
for b in tplcc tplc-pkg; do
    strip -o "$ROOT/usr/bin/$b" "$RUST/$b"
done
for b in tether-plc tether-plc-sim tplc-io tplc-ctl tplc-ctl-proxy \
         tplc-image tplc-io-probe tplc-io-cli; do
    strip -o "$ROOT/usr/bin/$b" "$BIN/$b"
done

# --- IDE proxy wrapper: sane defaults for the installed layout -----------
cat > "$ROOT/usr/bin/tplc-proxy" <<'WRAP'
#!/bin/sh
# TetherPLC IDE proxy with installed paths: tplcc for /compile,
# library dirs for /blocks + /libsrc. Extra args pass through.
exec tplc-ctl-proxy \
    --tplcc "$(command -v tplcc)" \
    --libdir /usr/share/tetherplc/libraries/tempctrl \
    --libdir /usr/share/tetherplc/compat/tc2 \
    --libdir /usr/share/tetherplc/compat/tc3 \
    "$@"
WRAP
chmod 755 "$ROOT/usr/bin/tplc-proxy"

# --- library sources ------------------------------------------------------
mkdir -p "$ROOT/usr/share/$PKG/libraries/tempctrl" \
         "$ROOT/usr/share/$PKG/compat/tc2" "$ROOT/usr/share/$PKG/compat/tc3"
cp libraries/tempctrl/*.st "$ROOT/usr/share/$PKG/libraries/tempctrl/"
cp compat/tc2/*.st "$ROOT/usr/share/$PKG/compat/tc2/" 2>/dev/null || true
cp compat/tc3/*.st "$ROOT/usr/share/$PKG/compat/tc3/" 2>/dev/null || true

# --- prebuilt temperature library package --------------------------------
if "$RUST/tplcc" build --library --name Atc_TempCtrl \
        libraries/tempctrl/atc_tempctrl.st \
        -o "$ROOT/usr/share/$PKG/lib/atc_tempctrl.tplc" >/dev/null 2>&1; then
    echo "built atc_tempctrl.tplc"
else
    rm -f "$ROOT/usr/share/$PKG/lib/atc_tempctrl.tplc"
    echo "warn: atc_tempctrl.tplc build failed — sources still shipped" >&2
fi

# --- docs -----------------------------------------------------------------
cp docs/plc/atc-tempctrl.md docs/plc/toolchain.md docs/plc/web-ide.md \
   "$ROOT/usr/share/doc/$PKG/" 2>/dev/null || true
cat > "$ROOT/usr/share/doc/$PKG/README" <<'DOC'
TetherPLC — IEC 61131-3 toolchain + EtherCAT temperature-control runtime

Binaries:  tplcc (compiler), tplc-pkg (package verify/sign),
           tether-plc (runtime), tether-plc-sim (simulator),
           tplc-io / tplc-ctl / tplc-ctl-proxy / tplc-image (services),
           tplc-proxy (WebUI proxy wrapper with installed libdirs)

Libraries: /usr/share/tetherplc/libraries/tempctrl/atc_tempctrl.st
           /usr/share/tetherplc/compat/{tc2,tc3}/*.st
           /usr/share/tetherplc/lib/atc_tempctrl.tplc (prebuilt)

Quick start:
  tplcc build my_program.st --lib /usr/share/tetherplc/lib/atc_tempctrl.tplc \
      -o my_program.tplc
  tplc-pkg verify my_program.tplc
DOC
gzip -9 "$ROOT/usr/share/doc/$PKG/"*.md 2>/dev/null || true

# --- control ---------------------------------------------------------------
cat > "$ROOT/DEBIAN/control" <<CTL
Package: $PKG
Version: $VER
Section: electronics
Priority: optional
Architecture: $ARCH
Maintainer: TetherMotion <packages@tethermotion.example>
Depends: libc6, libstdc++6, libssl3, zlib1g, libffi8, libzstd1
Description: IEC 61131-3 PLC toolchain and EtherCAT runtime
 TetherPLC compiles ST/FBD/LD programs (tplcc) into signed .tplc
 packages executed by the tether-plc runtime. Ships the Atc_TempCtrl
 temperature-control library (PID, cascade, Smith, LQR/LQG, autotuners,
 profile programmer) and Tc2/Tc3 compatibility sources.
CTL

dpkg-deb --build "$ROOT" "$OUT/${PKG}_${VER}_${ARCH}.deb"
echo "==> $OUT/${PKG}_${VER}_${ARCH}.deb"
ls -la "$OUT/${PKG}_${VER}_${ARCH}.deb"
