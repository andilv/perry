**fix(build): restore the musl `default-features = false` edge on `perry-codegen` (ARM/musl manual-build link failure).**

Upstream `f70bbaff8` ("refactor: replace utility dependencies with opt-in Perry helpers") restructured how the in-process LLVM is enabled: pre-merge, `perry` carried `llvm-inprocess` in its own `default` and its dependency edge disabled perry-codegen's defaults (`perry-codegen = { path = "../perry-codegen", default-features = false }` — introduced specifically so musl builds could strip it). The refactor replaced that with `perry-codegen.workspace = true`, leaving perry-codegen's own `default = ["llvm-inprocess"]` unconditionally on. Cargo ORs a dependency's default-features across all requirers, so **no CLI flag combination can strip it anymore** — and every musl manual-build leg died at link with exactly the documented failure:

```
/usr/bin/ld: libllvm_sys-*.rlib(JSON.cpp.o): undefined reference to symbol '__isoc23_strtoull@@GLIBC_2.38'
/lib/x86_64-linux-gnu/libc.so.6: error adding symbols: DSO missing from command line
```

(appt.llvm.org's LLVM 22 objects are glibc-built; a static musl binary cannot resolve their versioned glibc refs — the #7353 rationale for the edge.)

Three non-obvious details, all hit while fixing:

1. `cargo tree` confirmed inkwell resolves for `x86_64-unknown-linux-musl` even under `--no-default-features --features full-cli` post-merge.
2. Selecting `-p perry -p perry-codegen` together with `--no-default-features` does NOT help: the edge from `perry` still requests defaults, and defaults are unioned per package.
3. `{ workspace = true, default-features = false }` is **silently ignored** (cargo metadata shows `uses_default_features: true`) — the pre-merge direct `path` form is required.

Fix: restore the pre-merge edge in its direct-path form, and re-add `"llvm-inprocess"` to `perry`'s `default` so the default build is unchanged. `manual-build.yml` needs no change — its existing `--no-default-features --features full-cli` musl workaround works again. This is a fork-maintained divergence from upstream's manifest; the comment in the manifest explains what to re-apply after syncs.

Verified: `cargo tree` (musl + workaround flags → 0 inkwell; host/musl defaults → present); full `cargo build --profile release --target x86_64-unknown-linux-musl -p perry --no-default-features --features full-cli` → clean link, `static-pie linked, stripped`, zero `INTERP` program headers, no `__isoc23` references.
