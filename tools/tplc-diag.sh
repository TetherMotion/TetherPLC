#!/usr/bin/env bash
# tplc-diag.sh — diagnostics bundle: manifest + runtime state +
# event journal + audit tail + historian config into one directory
# (tar it yourself or pass -o for a .tar.gz).
#
#   tplc-diag.sh <ctl-socket> [-o out.tar.gz]
#
# Requires tplc-ctl on PATH or TPLC_CTL=/path/to/tplc-ctl.
set -uo pipefail

SOCK=${1:-}
OUT=""
[ "${2:-}" = "-o" ] && OUT=${3:-}
CTL=${TPLC_CTL:-}
if [ -z "$CTL" ]; then
    for c in tplc-ctl "$(dirname "$0")/../build/tplc-ctl" \
             "$(dirname "$0")/tplc-ctl"; do
        if command -v "$c" >/dev/null 2>&1 || [ -x "$c" ]; then
            CTL=$c; break
        fi
    done
fi
[ -n "$CTL" ] || { echo "tplc-ctl not found (set TPLC_CTL)" >&2; exit 2; }
[ -n "$SOCK" ] || { echo "usage: $0 <ctl-socket> [-o out.tar.gz]" >&2; exit 2; }

TS=$(date +%Y%m%d-%H%M%S)
DIR=$(mktemp -d "/tmp/tplc-diag-$TS.XXXXXX")
trap 'rm -rf "$DIR"' EXIT

dump() { # name, ctl args...
    local f="$DIR/$1"; shift
    "$CTL" "$SOCK" "$@" >"$f" 2>&1 || echo "(unavailable: $?)" >>"$f"
}

# manifest
{
    echo "tplc diagnostics bundle"
    echo "created_utc: $(date -u +%FT%TZ)"
    echo "socket:      $SOCK"
    echo "host:        $(hostname 2>/dev/null || echo '?')"
    uname -a 2>/dev/null | sed 's/^/uname:       /'
} > "$DIR/MANIFEST.txt"

# runtime + program state
dump prog.txt        prog
dump status.txt      status
dump image.txt       image
dump tasks.txt       list
dump pkg.txt         pkg
dump layout.txt      layout
dump retain.txt      retain-status
dump shadow.txt      shadow-status
dump map.txt         map-get

# event/alarm journal + history
dump events-live.txt evlog-list 128
dump events-stats.txt evlog-stats
dump events-hist.txt  evlog-history 512

# audit journal tail + runtime log if reachable
dump journal.txt     journal-tail 256

# historian config + recent trace frames
dump hist.txt        hist-list
dump trace.txt       trace-status
dump trace-csv.txt   trace-csv 0 512

# force/break/watch state
dump forces.txt      force-list
dump breaks.txt      break-list
dump watches.txt     watch-list

if [ -n "$OUT" ]; then
    tar -czf "$OUT" -C "$DIR" .
    echo "bundle: $OUT ($(du -h "$OUT" | cut -f1))"
else
    echo "bundle dir: $DIR"
    ls -1 "$DIR"
    echo "(copy or tar before exit — dir is removed on exit only with -o;"
    echo " without -o it persists until reboot)"
    # keep dir: neutralize trap
    trap - EXIT
fi
