**fix(ci): manual-build derives its ext-crate lists from the governance inventory.**

Every upstream binding removal (axios `a97f72b25`, fastify `dcb8a760a`, fetch/pg/mysql2 `b77aba634`) broke `manual-build.yml` the same way — three hardcoded crate lists that upstream never maintains, each requiring an identical 3-line prune after the fact. The workflow now derives everything from `scripts/release_ext_packages.sh`, the same governance inventory upstream's own release workflow consumes, so ext crates added or removed upstream flow through without workflow edits.

Changes:

1. The "shared-tokio" + "CPU-only" build split collapses into one derived loop: every governed crate is built with `-p perry-runtime -p perry-runtime-static -p perry-stdlib -p perry-stdlib-static` alongside it (upstream's #7358 pattern). #507 is satisfied structurally — cargo unifies tokio features over stdlib+wrapper in each invocation — instead of by a hand-maintained tokio-vs-CPU crate list. The loop runs on Windows legs too (the old shared-tokio step did); the per-crate `|| skip` preserves `--keep-going`'s best-effort semantics.
2. The #507 tokio-hash verify loop iterates the same governed list (warning per missing archive, comparing tokio hashes per present archive, tolerating no-tokio wrappers) instead of hardcoded lib names.
3. Latent fix: `cargo build --${{ inputs.profile }}` produced `cargo build --dev` for `profile: dev` dispatches, which cargo rejects — every `dev` dispatch died at the first build step. All three invocation sites now use `--profile ${{ inputs.profile }}`, which is valid for both `release` and `dev`. (CI never surfaced this because the input defaults to `release`.)

Verified locally by simulating the exact shipped `run:` scripts (extracted from the YAML, `${{ }}` substituted):

- governed list: all 20 packages resolve (`cargo pkgid -p`) — the "did not match any packages" failure class is structurally gone;
- build loop (`cargo check` substituted for `cargo build` to skip 20× linking): 0 skips, all 20 ext crates + 4 core crates compile, ~2 min first crate then 1–4 s cache hits each;
- verify script against synthetic archives: matching markers → pass over 6 archives with per-missing-lib warnings; a mismatched `Cs…_5tokio` hash → `::error::` + exit 1; a no-tokio wrapper (undici shape) → tolerated.
