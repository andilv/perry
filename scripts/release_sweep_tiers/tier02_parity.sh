#!/usr/bin/env bash
# Tier 2 — parity
#
# What this should do:
#   Wrap ./run_parity_tests.sh and parse its output. Once step 2 lands the
#   JSON-summary patch on run_parity_tests.sh, this just reads the summary
#   file and emits PASS / FAIL with the (passed/failed/skipped) counts.
#
# Stub for now.

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$SCRIPT_DIR/../release_sweep_lib.sh"

OUT="${PERRY_RELEASE_SWEEP_OUTPUT:?PERRY_RELEASE_SWEEP_OUTPUT not set}"
sweep_tier_emit "$OUT" 2 "parity" "NOT_IMPLEMENTED" 0 "stub — wraps run_parity_tests.sh in step 2"
