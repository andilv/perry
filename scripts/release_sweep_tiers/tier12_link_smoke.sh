#!/usr/bin/env bash
# Tier 12 — link_smoke
#
# What this should do:
#   For every target triple Perry supports as `--target`, compile a tiny
#   `App({ body: Text("ok") })` fixture and verify that the linker produces
#   a non-empty artifact. We don't run the artifact (the matching simulator
#   tier handles that); the assertion is purely "the cross-compile + link
#   pipeline still works end-to-end."
#
#   Targets per CLAUDE.md:
#     - host (macos / linux / windows)
#     - ios-simulator, ios, tvos-simulator, tvos       (macOS only)
#     - visionos-simulator, visionos                   (macOS only)
#     - android (aarch64-linux-android)                (macOS + Linux)
#     - watchos-simulator, watchos                     (macOS only)
#
# Stub for now.

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$SCRIPT_DIR/../release_sweep_lib.sh"

OUT="${PERRY_RELEASE_SWEEP_OUTPUT:?PERRY_RELEASE_SWEEP_OUTPUT not set}"
sweep_tier_emit "$OUT" 12 "link_smoke" "NOT_IMPLEMENTED" 0 "stub — implement in step 4 (fast feedback on platform regressions)"
