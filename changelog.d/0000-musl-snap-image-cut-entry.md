**fix(runtime): `js_snap_image_cut` compiles on x86-64 Linux musl — gnu-only trampoline reached past its gate.**

The sync's snap-image cut entry (`js_snap_image_cut`, upstream Nov 16–17) gates its naked-asm offering trampoline on `all(linux, x86_64)` — but the `offering_entry_asm!` macro and its `gc::roots` re-export are gated `all(x86_64, linux, gnu)`. On a musl target the gate matches, the macro doesn't exist, and every musl leg dies:

```
error[E0433]: cannot find `offering_entry_asm` in `gc`
   --> crates/perry-runtime/src/snap_image.rs:123:16
note: found an item that was configured out ... gated behind the `gnu` feature
```

Upstream's own pattern for this split lives one file over (`gc/policy.rs::js_gc_collect`): the gnu build gets the naked frame-offering entry, every other configuration gets a plain fallback that runs the body without the offer. The cut entry got only half of that pattern — its gate forgot `target_env`.

Fix, mirroring `js_gc_collect` exactly: the naked variant tightens to `target_env = "gnu"`; a new plain fallback on `linux + x86_64 + not(gnu)` captures the baseline buffers (`take_pre`) and runs `cut_body()` with no walk origin — no frame to offer without the trampoline. `js_snap_image_cut_from` (the offer's continuation) stays gnu-only, since only the naked entry reaches it. Nothing outside linux-x86_64 changes: aarch64 and other targets still compile no cut entry at all.

Verified: reproduces the CI E0433 with a local musl `cargo check` of `perry-runtime`; after the fix both gnu and musl check clean, and the full manual-build musl leg (`--no-default-features --features full-cli`, release) links `static-pie` with 0 INTERP and compiles + runs a TS program.
