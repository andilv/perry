#!/usr/bin/env bash
# Tier 0 — build_matrix
#
# What this should do (step 1, the orchestrator skeleton, leaves this stub):
#   `cargo build --release` for every target triple Perry ships:
#     - host (macos/linux/windows)
#     - aarch64-apple-ios / -tvos / -visionos / -watchos      (macOS host only)
#     - aarch64-linux-android, x86_64-linux-android           (macOS + Linux)
#     - x86_64-pc-windows-msvc / -gnu                         (Windows host)
#   Plus the perry-ui-* crates that are excluded on non-target hosts via the
#   --exclude list in CLAUDE.md, where the cross-toolchain allows it.
#
# To be filled in during step 2 of the rollout. The stub emits NOT_IMPLEMENTED
# so the orchestrator still produces a real report.

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$SCRIPT_DIR/../release_sweep_lib.sh"

OUT="${PERRY_RELEASE_SWEEP_OUTPUT:?PERRY_RELEASE_SWEEP_OUTPUT not set}"
sweep_tier_emit "$OUT" 0 "build_matrix" "NOT_IMPLEMENTED" 0 "stub — implement in step 2"
