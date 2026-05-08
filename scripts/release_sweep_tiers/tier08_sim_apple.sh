#!/usr/bin/env bash
# Tier 8 — sim_apple
#
# What this should do:
#   Extend scripts/run_simctl_tests.sh from iOS-only to a triple-platform
#   matrix (iOS / tvOS / visionOS) by parameterizing PLATFORM and DEVICE.
#   Each platform's pass/fail is its own bucket inside the tier's log.
#
# Macros only — Xcode + simctl required. Skipped on Linux / Windows by the
# orchestrator's gate.

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$SCRIPT_DIR/../release_sweep_lib.sh"

OUT="${PERRY_RELEASE_SWEEP_OUTPUT:?PERRY_RELEASE_SWEEP_OUTPUT not set}"
sweep_tier_emit "$OUT" 8 "sim_apple" "NOT_IMPLEMENTED" 0 "stub — generalizes run_simctl_tests.sh in step 5"
