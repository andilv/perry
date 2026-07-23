fix(build): aarch64 Linux PTY ioctl type mismatch — cast TIOC{SCTTY,SWINSZ} to `libc::Ioctl` instead of `libc::c_ulong`

`libc::c_ulong` is `u64` on aarch64, but `libc::ioctl`'s request parameter type (`Ioctl`) is `c_int` (`i32`) on aarch64 Linux. The `as libc::c_ulong` cast produced a `u64` where `i32` was expected, causing E0308. Using `libc::Ioctl` (which is `c_int` on aarch64 and `c_ulong` on x86_64) fixes both platforms.
