#!/usr/bin/env bash
# Tier 6 — doc_tests
#
# What this should do:
#   Wrap scripts/run_doc_tests.sh on macOS / Linux and scripts/run_doc_tests.ps1
#   on Windows. The doc-tests harness already compiles + runs every TS snippet
#   in docs/examples/, so this tier mostly handles per-host script selection
#   and result aggregation.
#
# Stub for now.

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$SCRIPT_DIR/../release_sweep_lib.sh"

OUT="${PERRY_RELEASE_SWEEP_OUTPUT:?PERRY_RELEASE_SWEEP_OUTPUT not set}"
sweep_tier_emit "$OUT" 6 "doc_tests" "NOT_IMPLEMENTED" 0 "stub — wraps run_doc_tests.{sh,ps1} in step 2"
