**fix(build): single `[env]` table in `.cargo/config.toml` — duplicate key killed every cargo invocation.**

The remote sync merged upstream's `LIBSQLITE3_FLAGS` `[env]` table into the file below the fork's musl-fortify `CFLAGS_*` table. TOML forbids a duplicate `[env]` header, and the two regions are far enough apart that the merge produced no textual conflict — so every leg of the manual build died before compiling anything:

```
error: could not load Cargo configuration
Caused by: TOML parse error at line 102, column 2
102 | [env]
    | ^^^ duplicate key
```

The two tables are now one: the fork's four musl `CFLAGS_*`/`CXXFLAGS_*` entries plus upstream's `LIBSQLITE3_FLAGS`, with both comment blocks preserved and a note at the merge site.

Verified: `cargo check -p perry-runtime` parses the config and compiles clean under the freshly pinned `nightly-2026-10-04` toolchain.
