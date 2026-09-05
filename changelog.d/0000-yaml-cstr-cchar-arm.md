**fix(runtime): YAML error path builds on ARM (`aarch64` musl/gnu) — E0308 `*const i8` vs `*const u8`.**

`bun_compat/cli_utils.rs:950` passed `parser.problem` (an `unsafe-libyaml` `yaml_parser_t` field) directly to `std::ffi::CStr::from_ptr`. That compiled on x86_64 and broke every ARM target:

1. `unsafe-libyaml` 0.2.11 vendors a private `mod libc` with a **hardcoded** `i8 as c_char` (its `src/lib.rs:50`), so `problem` is `*const i8` on every target regardless of platform.
2. `CStr::from_ptr` takes the *real* target `c_char`, which is target-defined: `i8` on x86_64, **`u8`** on aarch64/arm/riscv64 (C's `char` is unsigned there). On ARM the call fails with E0308 "expected `*const u8`, found `*const i8`".

First surfaced as the `aarch64-unknown-linux-musl` leg of `manual-build.yml` (native on `ubuntu-24.04-arm`); by extension every aarch64 leg (gnu, release-packages bottles, build-cross) was equally broken, while all x86_64 legs and local dev builds passed — which is why it went unnoticed.

Fix is one line: `CStr::from_ptr(parser.problem.cast())`. `ptr::cast` infers the pointee from `from_ptr`'s parameter — the target-correct `c_char` — so the pointer becomes `*const u8` on ARM and stays `*const i8` on x86_64. Layout-identical (1-byte pointee, NUL-terminated C string), signedness irrelevant for byte-wise `to_string_lossy`; no behavior change.

Verified: `cargo check --target aarch64-unknown-linux-gnu -p perry-runtime` (reproduced the exact E0308 before the fix, clean after) and host x86_64 `cargo check -p perry-runtime` (unchanged).
