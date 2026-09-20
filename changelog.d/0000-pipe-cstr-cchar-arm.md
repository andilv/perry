**fix(ext/net): stream `pipe()` compiles on ARM — E0308 `*const i8` vs `*const u8` at the `js_dynamic_object_get_property` extern.**

Upstream `950d152ff` ("fix: net.Socket pending/destroyed timing") added `crates/perry-ext-net/src/pipe.rs`, whose `extern "C"` block hardcoded the name-pointer parameter as `*const i8` while its three call sites pass `c"…".as_ptr()`. `CStr::as_ptr` returns `*const c_char`, whose signedness is target-defined: `i8` on x86_64 (compiles) but `u8` on aarch64/arm — where every leg fails with:

```
error[E0308]: mismatched types
  --> crates/perry-ext-net/src/pipe.rs:84:61
   |
84 |         let write_fn = js_dynamic_object_get_property(dest, c"write".as_ptr(), 5);
   |                                                              ^^^^^^^^^^^^^^^^^ expected `*const i8`, found `*const u8`
```

Same root family as the earlier `unsafe-libyaml` ARM break (a hardcoded x86-ish `c_char` assumption), but an independent site in a different crate — `pipe.rs` is the only ext-crate file with a hardcoded `*const i8` extern parameter; the node-api `c"…"` uses in `perry-runtime` are already `c_char`-correct.

Fix: declare the parameter as `*const std::ffi::c_char` so the declaration matches what `CStr::as_ptr` produces on every target. The runtime-side definition (`perry-runtime/src/value/dynamic_object.rs`) keeps its own annotation — pointee signedness is not part of the C symbol ABI, and that file already reinterprets the bytes as `*const u8` internally.

Verified: `cargo check --target aarch64-unknown-linux-gnu -p perry-ext-net` and host x86_64, both clean; `cargo fmt --check` clean.
