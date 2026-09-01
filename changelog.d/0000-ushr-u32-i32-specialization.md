**fix(codegen): `>>>` u32-range values no longer poison the i32 specialization paths.**

Two unsound bare `fptosi double→i32` conversions (LLVM poison for values ≥ 2^31, while JS `>>>` values legitimately occupy `[0, 2^32)`) corrupted 32-bit arithmetic on u32-range data:

1. `collectors/integer_locals.rs` — the integer-local proof treated `x >>> 0` as signed-i32-producing, so a local reassigned a u32-range value stayed specialized to i32 slots/ABI and the callsite poisoned. `>>> 0` is now excluded from the proof (only `| 0` is a signed ToInt32); the two `BitOr | UShr` match arms and the two bitwise catch-alls drop `UShr`.

2. `expr/i32_fast_path.rs` `lower_expr_native_i32` — the fallback `_` arm (plus the LocalGet and non-clamp Call arms) emitted an unguarded `fptosi double→i32` when lowering an expression to i32, reached by plain JS array element reads like `S[x >>> 24]` (the bcryptjs S-box shape). All three now gate on `is_known_i32_range` → `toint32_fast`/`toint32`, mirroring the existing guarded F64 arm.

Discovered building a pure-TS BIP340 Schnorr sign/verify (SHA-256 digest assembly) — SHA-256 state words are u32-range, so the digest loop collapsed every word ≥ 2^31 to `0x80000000`. With both fixes the full BIP340 test-vector suite (19/19) matches Node.
