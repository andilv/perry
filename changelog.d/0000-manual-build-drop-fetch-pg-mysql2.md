**fix(ci): manual-build stops building the turnloop-removed ext crates (`fetch`, `pg`, `mysql2`).**

Upstream `b77aba634` ("turnloop: replace tokio as Perry's event loop (P0–P8) (#10354)") deleted `perry-ext-fetch`, `perry-ext-pg`, and `perry-ext-mysql2` — the tokio-using wrappers whose transport moved into the turnloop world. The governed shipping set (`scripts/release_ext_packages.sh`) is down to 20 crates, none of them these. `manual-build.yml` (fork-era, unmaintained upstream) kept naming them, so every leg died with:

```
error: package ID specification `perry-ext-fetch` did not match any packages
  help: a package with a similar name exists: `perry-ext-net`
```

Same three deletions as the axios (`0000-manual-build-drop-axios`) and fastify (`0000-manual-build-drop-fastify`) fixes:

1. the shared-tokio `cargo build --keep-going` invocation (no `-p perry-ext-fetch` / `-p perry-ext-pg` / `-p perry-ext-mysql2`; also de-duplicated a `-p perry-ext-undici` the fastify fix accidentally left on two continuation lines),
2. the `SHARED_TOKIO` exclusion list for the CPU-only loop,
3. the tokio-hash verify loop (`for lib in net ws http …`).

The shared-tokio set is now net, ws, http, undici, mongodb, ioredis, nodemailer — every one still a real crate with a tokio dep (undici rides the shared build per `binding_needs_shared_tokio`'s comment in `crates/perry/src/commands/compile/optimized_libs/freshness.rs`), so the #507 CONTEXT-TLS verify remains meaningful post-turnloop. The CPU-only loop and packaging steps iterate globs and needed no change.
