#!/usr/bin/env bash
# Tier 9 — sim_watchos
#
# What this should do:
#   Mirror run_simctl_tests.sh for the watchOS Simulator. watchOS sim apps
#   are launched via xcrun simctl with a paired iPhone runtime; the gate is
#   that an Apple Watch device is bootable and a paired iPhone is present.
#
# Macros only.

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$SCRIPT_DIR/../release_sweep_lib.sh"

OUT="${PERRY_RELEASE_SWEEP_OUTPUT:?PERRY_RELEASE_SWEEP_OUTPUT not set}"
sweep_tier_emit "$OUT" 9 "sim_watchos" "NOT_IMPLEMENTED" 0 "stub — new run_watchsim_tests.sh in step 5"
