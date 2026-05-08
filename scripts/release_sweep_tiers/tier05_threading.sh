#!/usr/bin/env bash
# Tier 5 — threading
#
# What this should do:
#   Wrap scripts/run_thread_tests.sh. Once step 2 adds the JSON summary, the
#   tier reads that and emits PASS / FAIL with per-test breakdown.
#
# Stub for now.

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$SCRIPT_DIR/../release_sweep_lib.sh"

OUT="${PERRY_RELEASE_SWEEP_OUTPUT:?PERRY_RELEASE_SWEEP_OUTPUT not set}"
sweep_tier_emit "$OUT" 5 "threading" "NOT_IMPLEMENTED" 0 "stub — wraps run_thread_tests.sh in step 2"
