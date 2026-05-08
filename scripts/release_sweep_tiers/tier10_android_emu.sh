#!/usr/bin/env bash
# Tier 10 — android_emu
#
# What this should do:
#   Spin up an Android emulator (via avdmanager + emulator), wait for adb
#   boot complete, install + launch each docs/examples/ui/* that declares
#   `android` in its targets banner, scrape adb logcat for the Perry
#   PERRY_UI_TEST_MODE exit signal, assert a clean exit.
#
# Runs on macOS + Linux (Android Studio SDK on both); skipped on Windows
# because the gate sets gate=macos,linux. Windows-host Android testing is
# tracked separately.

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$SCRIPT_DIR/../release_sweep_lib.sh"

OUT="${PERRY_RELEASE_SWEEP_OUTPUT:?PERRY_RELEASE_SWEEP_OUTPUT not set}"
sweep_tier_emit "$OUT" 10 "android_emu" "NOT_IMPLEMENTED" 0 "stub — new run_android_emu_tests.sh in step 6"
