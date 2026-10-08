**fix(runtime): `perry_memory_profile.c` compiles on Ubuntu musl-gcc (no kernel UAPI in its include path).**

The sync added `src/ffi/perry_memory_profile.c`, which includes `<linux/prctl.h>` for `PR_GET_THP_DISABLE`/`PR_SET_THP_DISABLE`. Ubuntu's `musl-gcc` specs search only musl's own include tree, so the kernel UAPI header — shipped by `linux-libc-dev` into `/usr/include/linux` and by Alpine's `linux-headers` into `/usr/include` — is invisible there, and every manual-build musl leg dies in the cc-rs build script:

```
warning: perry-runtime@0.5.1659: src/ffi/perry_memory_profile.c:5:10: fatal error: linux/prctl.h: No such file or directory
error: failed to run custom build command for `perry-runtime v0.5.1659`
```

Upstream never sees this: its release-pipeline musl legs build inside Alpine, where `linux-headers` populates the same include tree musl uses. The glibc `<sys/prctl.h>` on line 4 already resolves fine (musl ships it).

Fix: guard the UAPI include with `__has_include` and fall back to the fixed kernel ABI values (`PR_SET_THP_DISABLE` = 41, `PR_GET_THP_DISABLE` = 42, both stable since Linux 4.5), extending the file's existing `PR_THP_DISABLE_EXCEPT_ADVISED` ifndef pattern. glibc and Alpine musl still take the real header; behavior is unchanged everywhere the header exists.

Verified: reproduces the exact CI failure with local `musl-gcc -std=c11` against the same mimalloc include dir; the patched file compiles under both `musl-gcc` and glibc `gcc`; and `cargo build --profile release --target x86_64-unknown-linux-musl -p perry-runtime` passes the build script end to end.
