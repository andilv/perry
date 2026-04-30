//! HarmonyOS callback registry for ArkUI → Perry NAPI bridge (Phase 2 v2).
//!
//! ArkUI renders pages emitted by `perry-codegen-arkts`. When the user
//! authors `Button("Save", () => { count++ })` in TypeScript, the harvest
//! pass:
//!
//! 1. Captures the closure expression, assigns it slot `idx`
//! 2. Emits `Button('Save').onClick(() => perryEntry.invokeCallback(<idx>))`
//!    in the .ets
//! 3. Injects a `perry_arkts_register_callback(<idx>, <closure>)` call
//!    into Perry's `main()` so the closure pointer ends up in this table
//!
//! On `main()` startup the closures get registered. When the user later
//! taps Save in ArkUI, the .ets `onClick` fires `perryEntry.invokeCallback(0)`
//! through NAPI; that lands in `perry_arkts_invoke_callback` here, which
//! looks up slot 0, unboxes the closure pointer, and calls
//! `js_closure_call0` — running the original Perry TS closure body.
//!
//! Phase 2 v2 only supports 0-arg closures (Button.onClick). Toggle's
//! `(isOn: boolean) => ...`, TextField's `(value: string) => ...`, and
//! Slider's `(value: number) => ...` need NaN-box marshaling for the
//! arg and are deferred to v2.5.
//!
//! GC: registered closure pointers are scanned via
//! `arkts_callbacks_root_scanner`, registered in `gc_init`, so the
//! generational mark-sweep doesn't reclaim them between callbacks.

use std::sync::Mutex;

use crate::closure::{js_closure_call0, ClosureHeader};
use crate::value::{POINTER_MASK, TAG_UNDEFINED};

// POINTER_TAG is private to the value module; redeclare the constant here
// so we can match against it. Must stay in sync with value.rs.
const POINTER_TAG_BITS: u64 = 0x7FFD_0000_0000_0000;

static CALLBACKS: Mutex<Vec<f64>> = Mutex::new(Vec::new());

/// Register a Perry closure (NaN-boxed f64) at the given slot. Slots
/// beyond the current Vec length are filled with TAG_UNDEFINED so the
/// caller can register slots in any order.
#[no_mangle]
pub extern "C" fn perry_arkts_register_callback(idx: i64, closure_d: f64) {
    let mut cbs = CALLBACKS.lock().unwrap();
    let i = idx as usize;
    while cbs.len() <= i {
        cbs.push(f64::from_bits(TAG_UNDEFINED));
    }
    cbs[i] = closure_d;
}

/// Invoke a registered closure by slot. Returns NaN-boxed `undefined` if
/// the slot is out of range, never registered, or holds a non-pointer
/// value (defensive — should never happen with codegen-emitted shape).
#[no_mangle]
pub extern "C" fn perry_arkts_invoke_callback(idx: i64) -> f64 {
    // Snapshot under lock then drop so the closure body can re-enter
    // (e.g. a button handler that itself registers another callback).
    let closure_d = {
        let cbs = CALLBACKS.lock().unwrap();
        let i = idx as usize;
        if i >= cbs.len() {
            return f64::from_bits(TAG_UNDEFINED);
        }
        cbs[i]
    };
    let bits = closure_d.to_bits();
    if (bits & !POINTER_MASK) != POINTER_TAG_BITS {
        return f64::from_bits(TAG_UNDEFINED);
    }
    let raw = (bits & POINTER_MASK) as *const ClosureHeader;
    if raw.is_null() {
        return f64::from_bits(TAG_UNDEFINED);
    }
    js_closure_call0(raw)
}

/// GC root scanner. Marks each registered closure pointer as live so the
/// generational mark-sweep doesn't reclaim closure bodies between taps.
pub fn arkts_callbacks_root_scanner(mark: &mut dyn FnMut(f64)) {
    if let Ok(cbs) = CALLBACKS.try_lock() {
        for &c in cbs.iter() {
            mark(c);
        }
    }
}
