//! Stable `extern "C"` shims that perry-ffi declares for use by
//! external native binding crates (#466 Phase 1 + 5).
//!
//! perry-ffi can't depend on perry-stdlib's internal Rust modules
//! (`crate::common::async_bridge::*`) because that would force every
//! external wrapper to take a workspace dep on perry-stdlib —
//! defeating the whole point of an ABI-stable surface. Instead, the
//! contract goes through C ABI:
//!
//! 1. perry-stdlib (this file) defines `#[no_mangle] extern "C"`
//!    shims wrapping `async_bridge`'s public Rust functions.
//! 2. perry-ffi declares those symbols as `extern "C"` and exposes
//!    safe Rust wrappers (`JsPromise`, `spawn_blocking`).
//! 3. External wrappers depend only on perry-ffi. At final link
//!    time, the `perry_ffi_*` undefined references they carry get
//!    resolved by perry-stdlib's archive — same mechanism as every
//!    other `_js_*` symbol that perry-stdlib exports today.
//!
//! Symbol naming uses the `perry_ffi_` prefix (vs. perry-stdlib's
//! existing `js_*`) so the contract is unambiguously bound to
//! perry-ffi's semver — not perry-stdlib's. A breaking change to
//! one of these signatures bumps perry-ffi major.

use std::ffi::c_void;

use crate::common::async_bridge;

/// `perry_ffi_promise_new()` — allocate a fresh Promise.
///
/// Thin pass-through to perry-runtime's allocator. Returned pointer
/// is owned by the runtime arena; resolution / rejection is what
/// transfers it to the awaiter.
#[no_mangle]
pub extern "C" fn perry_ffi_promise_new() -> *mut perry_runtime::Promise {
    perry_runtime::js_promise_new()
}

/// `perry_ffi_promise_resolve_bits(promise, bits)` — resolve the
/// promise with a NaN-boxed JSValue, supplied as raw bits.
///
/// Caller is responsible for the bits being a valid encoded value
/// (e.g. a `STRING_TAG`-tagged pointer for strings, the bit pattern
/// of `1.0` / `0.0` for booleans). perry-ffi's safe wrappers handle
/// the encoding so external authors don't write the tag values
/// directly.
#[no_mangle]
pub extern "C" fn perry_ffi_promise_resolve_bits(
    promise: *mut perry_runtime::Promise,
    bits: u64,
) {
    async_bridge::queue_promise_resolution(promise as usize, true, bits);
}

/// `perry_ffi_promise_reject_bits(promise, bits)` — reject with a
/// JSValue. Same encoding contract as resolve.
#[no_mangle]
pub extern "C" fn perry_ffi_promise_reject_bits(
    promise: *mut perry_runtime::Promise,
    bits: u64,
) {
    async_bridge::queue_promise_resolution(promise as usize, false, bits);
}

/// `perry_ffi_spawn_blocking(ctx, invoke)` — run `invoke(ctx)` on
/// the global tokio runtime's blocking pool. The caller is expected
/// to box a closure into `ctx` before calling, and write a thin
/// trampoline that decodes the closure inside `invoke`. perry-ffi's
/// safe `spawn_blocking` does exactly that.
///
/// `invoke` must take ownership of `ctx` (drop the box inside) —
/// this function does not free `ctx` itself.
///
/// Why blocking-pool: most native bindings that need async (bcrypt,
/// argon2, fs, http) are CPU-bound or make synchronous I/O calls
/// that would stall a tokio worker. The blocking pool is the
/// recommended pattern; pure-async tasks can use this same shim
/// (the closure can run an `async {}` block via
/// `tokio::runtime::Handle::current().block_on`).
#[no_mangle]
pub extern "C" fn perry_ffi_spawn_blocking(
    ctx: *mut c_void,
    invoke: extern "C" fn(*mut c_void),
) {
    // SAFETY of the raw `ctx` pointer is the caller's; we only
    // forward it across the spawn boundary to `invoke`. Wrapping
    // pointers in a `usize` lets us cross the closure boundary
    // because raw pointers are not `Send`.
    let ctx_addr = ctx as usize;
    async_bridge::runtime().spawn_blocking(move || {
        invoke(ctx_addr as *mut c_void);
    });
}
