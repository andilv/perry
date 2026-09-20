**fix(ci): manual-build stops building the removed `perry-ext-axios` crate.**

Upstream `a97f72b25` ("chore(bindings): remove axios native binding, compile real axios from source") deleted `perry-ext-axios` — `import axios from "axios"` now compiles the real npm package from source, and the governed shipping set (`scripts/release_ext_packages.sh`) no longer carries an axios wrapper. Upstream never touched `manual-build.yml` (fork-era workflow), so every leg died immediately with:

```
error: package ID specification `perry-ext-axios` did not match any packages
```

Three deletions in `manual-build.yml`, all list members of the existing #507 shared-tokio mechanism:

1. the shared-tokio `cargo build --keep-going` invocation (no `-p perry-ext-axios`),
2. the `SHARED_TOKIO` exclusion list for the CPU-only loop,
3. the tokio-hash verify loop (`for lib in net ws http …`).

Nothing else needed changing: packaging globs `libperry_ext_*.a`, and the shared-tokio crate set now mirrors `binding_needs_shared_tokio` in `crates/perry/src/commands/compile/optimized_libs/freshness.rs` exactly (net, ws, http, fetch, undici, fastify, mongodb, pg, mysql2, ioredis, nodemailer).
