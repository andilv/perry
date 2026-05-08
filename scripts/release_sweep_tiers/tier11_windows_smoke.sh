#!/usr/bin/env bash
# Tier 11 — windows_smoke
#
# What this should do:
#   Compile a tiny Win32 app via `perry compile --target windows`, launch it
#   under PERRY_UI_TEST_MODE=1, scrape stdout for the exit-after-first-frame
#   signal, assert clean exit. Implemented natively as smoke_windows_app.ps1
#   in step 7; this shell entry is a thin shim that execs the .ps1 and reads
#   back its emitted result.json.
#
# Gate=windows in the orchestrator → SKIP on macOS / Linux automatically.

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$SCRIPT_DIR/../release_sweep_lib.sh"

OUT="${PERRY_RELEASE_SWEEP_OUTPUT:?PERRY_RELEASE_SWEEP_OUTPUT not set}"
sweep_tier_emit "$OUT" 11 "windows_smoke" "NOT_IMPLEMENTED" 0 "stub — new smoke_windows_app.ps1 in step 7"
