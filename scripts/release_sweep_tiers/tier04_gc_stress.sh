#!/usr/bin/env bash
# Tier 4 — gc_stress
#
# What this should do:
#   Wrap scripts/run_memory_stability_tests.sh and run it through every GC
#   mode combo from CLAUDE.md:
#     - default                     (generational, no evac, no codegen WB)
#     - PERRY_GEN_GC=0              (full mark-sweep, bisection mode)
#     - PERRY_GEN_GC_EVACUATE=1     (copying evacuation pass)
#     - PERRY_WRITE_BARRIERS=1      (codegen-emitted write barriers)
#     - all-on combo
#   Asserts each combo passes the existing RSS / correctness gates.
#
# Stub for now.

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$SCRIPT_DIR/../release_sweep_lib.sh"

OUT="${PERRY_RELEASE_SWEEP_OUTPUT:?PERRY_RELEASE_SWEEP_OUTPUT not set}"
sweep_tier_emit "$OUT" 4 "gc_stress" "NOT_IMPLEMENTED" 0 "stub — extends run_memory_stability_tests.sh matrix in step 2"
