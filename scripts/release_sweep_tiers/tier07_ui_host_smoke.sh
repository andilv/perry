#!/usr/bin/env bash
# Tier 7 — ui_host_smoke
#
# What this should do:
#   Run scripts/run_ui_styling_matrix.sh, then for each docs/examples/ui/* that
#   declares the host platform in its targets banner, compile + launch under
#   PERRY_UI_TEST_MODE=1 PERRY_UI_TEST_EXIT_AFTER_MS=500. Each backend
#   (AppKit on macOS, GTK4 on Linux, Win32 on Windows) gets its own bucket.
#
# Stub for now.

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$SCRIPT_DIR/../release_sweep_lib.sh"

OUT="${PERRY_RELEASE_SWEEP_OUTPUT:?PERRY_RELEASE_SWEEP_OUTPUT not set}"
sweep_tier_emit "$OUT" 7 "ui_host_smoke" "NOT_IMPLEMENTED" 0 "stub — wraps run_ui_styling_matrix.sh + headless launch in step 2"
