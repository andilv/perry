**chore(ci): manual-build uploads skip the zip envelope (`archive: false`).**

`actions/upload-artifact` v7 (current major; **v8 does not exist** — only `download-artifact` is at v8) gained an `archive` input: `false` uploads the file as-is instead of wrapping it in a zip. Every manual-build artifact is already exactly one self-contained archive (`.tar.gz` on Unix, `.zip` on Windows), so the default wrapped each in a second compression layer — the classic double-zip: wasted upload time for zero size win on already-compressed payloads.

All four upload steps now pass `archive: false`. Consequences, per the action's contract:

- The `name` input is **ignored** in this mode — the filename becomes the artifact name. Artifacts on a run page are now `perry-<target>.tar.gz` / `perry-<target>-lite.tar.gz` / `perry-<target>.zip` (extension included) instead of the bare `perry-<target>`. Nothing in-repo consumes these artifacts by name (manual-build has no download steps), so the rename is self-contained.
- The lite uploads' `*-lite.tar.gz` wildcard is pinned to the exact filename — `archive: false` permits only a single matched file, and the exact path makes that guarantee explicit.
