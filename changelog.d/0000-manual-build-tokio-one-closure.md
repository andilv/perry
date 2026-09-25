**fix(ci): manual-build builds the governed ext set in ONE cargo closure (tokio-hash regression).**

PR #19's derived build was right about the crate list and wrong about the closure shape: it swapped the original single invocation for a per-wrapper loop (each carrying `-p perry-runtime/-stdlib`), assuming upstream's #7358 pattern satisfies #507. It does not. Cargo unions tokio features **per invocation**, and the resulting artifact's `Cs<hash>_5tokio` mangling is closure-dependent — so a per-wrapper loop re-mints tokio+stdlib on every call and the shipped `libperry_stdlib.a` ends up carrying the *last* crate's hash, diverging from earlier wrappers. The first real dispatch failed exactly there:

```
error: libperry_ext_http.a tokio (CslJ0BC8TjXRP_5tokio) != stdlib tokio (Cs6OU60Uxqifa_5tokio)
       — CONTEXT TLS mismatch, runtime panic (#507)
```

(The verify step caught it — that gate did precisely its job.)

Fix: back to ONE `cargo build --keep-going` invocation over the stdlib/runtime crates **plus the entire governed ext set**, with the list still derived from `scripts/release_ext_packages.sh` (the no-workflow-edits-on-removal property survives; only the closure shape changed). `--keep-going` keeps the original best-effort semantics — a wrapper that cannot build on the host is skipped, packaging ships what was produced, and the verify step warns per missing governed archive.

Verified locally on the workspace (dev profile, host target):

- measured the defect directly: `perry-ext-http` solo mints tokio hash `Cs6ciKSAag4T7_5tokio`, but in a stdlib+http+net closure the same crate mints `Csi5TboBIr4x7_5tokio` — closure-dependent, matching the CI failure mode;
- one invocation over core + all 20 governed crates: every produced `libperry_ext_*.a` shares the stdlib hash (zero mismatches; three crates were killed by local disk exhaustion — itself an exercise of the `--keep-going` path);
- the shipped verify step, extracted from the final YAML and run against those real artifacts: passes over 8 archives, one shared hash, warnings for the locally missing ones.
