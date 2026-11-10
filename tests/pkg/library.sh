#!/usr/bin/env bash
# library.sh — tplcc build --library / --lib (plan 21 C0):
# library packages carry kind="library" manifests, POU object code,
# and ST sources; --lib pulls a package's ST_SOURCES into a program
# build. Rejections: PROGRAM / VAR_GLOBAL / TASK in library mode.
# Requires: tplcc, tplc-pkg (both under compiler/target/debug; skip 77).
set -uo pipefail
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-compiler/target/debug/tplcc}
PKG=${PKG:-compiler/target/debug/tplc-pkg}
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

[ -x "$TPLCC" ] || { echo "SKIP no tplcc"; exit 77; }
[ -x "$PKG" ]   || { echo "SKIP no tplc-pkg"; exit 77; }

LIB_ST="$WORK/libmath.st"
cat > "$LIB_ST" <<'EOF'
FUNCTION_BLOCK FB_Clamp
VAR_INPUT
    IN : DINT;
    LO : DINT;
    HI : DINT;
END_VAR
VAR_OUTPUT
    Q : DINT;
END_VAR
    Q := LIMIT(LO, IN, HI);
END_FUNCTION_BLOCK

FUNCTION F_Square : DINT
VAR_INPUT
    X : DINT;
END_VAR
    F_Square := X * X;
END_FUNCTION
EOF

# --- library build ---------------------------------------------------
"$TPLCC" build --library --name TcTestMath "$LIB_ST" \
    -o "$WORK/libmath.tplc" >/dev/null 2>&1 \
    || { bad "tplcc build --library failed"; \
         echo "== $pass pass, $fail fail"; exit 1; }
ok "tplcc build --library package built"

"$PKG" verify "$WORK/libmath.tplc" >/dev/null 2>&1 \
    && ok "tplc-pkg verify library" \
    || bad "tplc-pkg verify library"

INFO=$("$TPLCC" info "$WORK/libmath.tplc" 2>/dev/null)
echo "$INFO" | grep -q '"kind":"library"' \
    && ok "manifest kind=library" \
    || bad "manifest kind=library"
echo "$INFO" | grep -q 'type=0x0001' \
    && ok "manifest section present" \
    || bad "manifest section present"
# library packages have no program-state sections (LAYD 0x0022, IO_MAP
# 0x0030, SYMMAP 0x0020, TYPC 0x0021 must be absent)
echo "$INFO" | grep -q 'type=0x0022' \
    && bad "library must not carry LAYD" \
    || ok "no LAYD section"
echo "$INFO" | grep -q 'type=0x0010' \
    && ok "target_obj section present" \
    || bad "target_obj section present"

# POU symbols exported from the relocatable object inside the package
strings "$WORK/libmath.tplc" | grep -q '__tplc_pou_FB_Clamp' \
    && strings "$WORK/libmath.tplc" | grep -q '__tplc_pou_F_Square' \
    && ok "POU symbols in target_obj" \
    || bad "POU symbols in target_obj"

# --- rejections ------------------------------------------------------
cat > "$WORK/prog.st" <<'EOF'
PROGRAM NotALib
VAR x : INT; END_VAR
x := 1;
END_PROGRAM
EOF
"$TPLCC" build --library "$WORK/prog.st" -o "$WORK/bad1.tplc" >/dev/null 2>&1 \
    && bad "--library accepted PROGRAM" \
    || ok "--library rejects PROGRAM"

cat > "$WORK/gvl.st" <<'EOF'
FUNCTION F_UseG : INT
VAR_EXTERNAL g : INT; END_VAR
    F_UseG := g;
END_FUNCTION
VAR_GLOBAL g : INT := 41; END_VAR
EOF
"$TPLCC" build --library "$WORK/gvl.st" -o "$WORK/bad2.tplc" >/dev/null 2>&1 \
    && bad "--library accepted VAR_GLOBAL" \
    || ok "--library rejects VAR_GLOBAL"

# --- --lib consumption -----------------------------------------------
cat > "$WORK/main.st" <<'EOF'
PROGRAM Demo
VAR
    inp AT %ID0 : DINT;
    outv AT %QD0 : DINT;
    sq   AT %QD4 : DINT;
    c    : FB_Clamp;
END_VAR
    c(IN := inp, LO := -100, HI := 100);
    outv := c.Q;
    sq := F_Square(inp);
END_PROGRAM
EOF
"$TPLCC" build "$WORK/main.st" --lib "$WORK/libmath.tplc" \
    -o "$WORK/demo.tplc" >/dev/null 2>&1 \
    || { bad "tplcc build --lib failed"; \
         echo "== $pass pass, $fail fail"; exit 1; }
ok "program build with --lib succeeded"

"$PKG" verify "$WORK/demo.tplc" >/dev/null 2>&1 \
    && ok "tplc-pkg verify consumer" \
    || bad "tplc-pkg verify consumer"

INFO2=$("$TPLCC" info "$WORK/demo.tplc" 2>/dev/null)
echo "$INFO2" | grep -q '"name":"Demo"' \
    && ok "consumer manifest is a program" \
    || bad "consumer manifest is a program"
echo "$INFO2" | grep -q 'type=0x0022' \
    && ok "consumer carries LAYD" \
    || bad "consumer carries LAYD"
# the library FB instance must appear in the program's symbol map —
# its qualified field name is embedded in SMAP payload bytes
strings "$WORK/demo.tplc" | grep -q 'c\.Q\|FB_Clamp' \
    && ok "library FB visible in consumer symbols" \
    || bad "library FB visible in consumer symbols"

echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
