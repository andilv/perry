#!/usr/bin/env bash
# Tier 3 — real_packages
#
# What this should do:
#   Compile + run each fixture under tests/release/packages/ with Perry,
#   compile + run the same fixture with Node, diff the output. Per the
#   user's preference, backends are mock/embedded (sqlite for drizzle,
#   redis-mock or local redis-server, MinIO binary for s3-lite) — no
#   Docker. Each fixture has its own _harness contract:
#     - perry compile <entry>.ts -o out
#     - ./out > perry-stdout.txt
#     - node <entry>.ts > node-stdout.txt
#     - diff perry-stdout.txt node-stdout.txt
#
# Filled in during step 3. Stub for now.

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$SCRIPT_DIR/../release_sweep_lib.sh"

OUT="${PERRY_RELEASE_SWEEP_OUTPUT:?PERRY_RELEASE_SWEEP_OUTPUT not set}"
sweep_tier_emit "$OUT" 3 "real_packages" "NOT_IMPLEMENTED" 0 "stub — implement in step 3 (highest-value new tier)"
