**fix(ci): manual-build stops building the removed `perry-ext-fastify` crate.**

Upstream `dcb8a760a` ("refactor(stdlib): remove qs/fastify/dayjs/date-fns/rate-limiter-flexible/node-cron native bindings") deleted `perry-ext-fastify`; the governed shipping set (`scripts/release_ext_packages.sh`) no longer carries it and `import fastify from "fastify"` now resolves through the stdlib. Upstream never touches `manual-build.yml` (fork-era workflow), so the legs died with the same error shape as the axios removal:

```
error: package ID specification `perry-ext-fastify` did not match any packages
```

Same three deletions as the `perry-ext-axios` fix (`0000-manual-build-drop-axios`):

1. the shared-tokio `cargo build --keep-going` invocation (no `-p perry-ext-fastify`),
2. the `SHARED_TOKIO` exclusion list for the CPU-only loop,
3. the tokio-hash verify loop (`for lib in net ws http …`).

The remaining crate set is exactly the wrapper crates that still exist in the governed list: net, ws, http, fetch, undici, mongodb, pg, mysql2, ioredis, nodemailer.
