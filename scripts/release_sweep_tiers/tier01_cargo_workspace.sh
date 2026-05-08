#!/usr/bin/env bash
# Tier 1 — cargo_workspace
#
# What this should do:
#   cargo test --release --workspace with the host-appropriate UI exclusions
#   (per CLAUDE.md — exclude perry-ui-{ios,tvos,watchos,visionos,android,
#   windows,gtk4} on macOS; the matching set on Linux / Windows). Captures
#   the per-crate test result, fails if any crate fails.
#
# Filled in during step 2. Stub for now.

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$SCRIPT_DIR/../release_sweep_lib.sh"

OUT="${PERRY_RELEASE_SWEEP_OUTPUT:?PERRY_RELEASE_SWEEP_OUTPUT not set}"
sweep_tier_emit "$OUT" 1 "cargo_workspace" "NOT_IMPLEMENTED" 0 "stub — implement in step 2"
