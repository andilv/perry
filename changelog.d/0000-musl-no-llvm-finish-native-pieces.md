**fix(codegen): `finish_native_pieces` compiles without `llvm-inprocess` (E0425 on musl manual-build legs).**

Upstream's fast-emit pieces refactor (#10586) added `linker::finish_native_pieces` as an unguarded `pub(crate)` fn whose body calls `finish_native_emission` — which is `#[cfg(feature = "llvm-inprocess")]`. Every `--no-default-features` build of `perry-codegen` therefore fails resolution:

```
error[E0425]: cannot find function `finish_native_emission` in this scope
  --> crates/perry-codegen/src/linker.rs:749:22
```

Upstream never compiles the no-LLVM variant (their release musl legs build a musl-native LLVM in an Alpine container with the feature on), so only the fork's manual-build musl legs — which build `--no-default-features --features full-cli` against the glibc-host LLVM (#7353, restored in 0000-musl-llvm-inprocess-edge) — exercise this configuration.

Fix follows upstream's own stub pattern (see `compile_ll_inprocess_in`): the real body is gated behind `llvm-inprocess`; a `#[cfg(not(feature = "llvm-inprocess"))]` stub `bail!`s with "the in-process native backend requires the `llvm-inprocess` feature". It is unreachable in that configuration — `inprocess_requested()` returns `false` without the feature, routing compiles through the clang-subprocess text path.

Verified: `cargo check -p perry-codegen` in both variants (no-defaults 22s, default 5s), `cargo fmt` clean, and the full manual-build musl leg reproduced locally — `cargo build --profile release --target x86_64-unknown-linux-musl -p perry --no-default-features --features full-cli` links a `static-pie` binary with zero `INTERP` headers, which then compiles and runs a TypeScript program (`hello-musl`, exit 0) through the text path.
