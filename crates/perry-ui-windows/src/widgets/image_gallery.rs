//! ImageGallery widget — paged image carousel.
//!
//! v1 Win32 implementation: a STATIC label control above and a small
//! navigation bar (left arrow / index label / right arrow) below.
//! Images are stored as `(url, alt)` pairs; each page change updates
//! the visible label to show the current alt text and fires the
//! user's `on_index_change` callback.
//!
//! URL fetching + GDI image decoding is deferred — the Windows
//! `Image(url, alt)` primitive itself doesn't fetch yet (it's a stub
//! at the lib.rs level), and decoding remote images would require
//! pulling reqwest + image into the UI crate. That's tracked
//! separately under task #16 (system modules + URL-aware image
//! widgets) and behind the v0.5.771-style "stubs matching the macOS
//! shape" link-stability contract — `add_image` / `set_index` route
//! correctly, the layout reserves space, alt text is visible, but
//! the visual image itself lands in a follow-up.

use std::cell::RefCell;
use std::collections::HashMap;

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::*;
#[cfg(target_os = "windows")]
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
#[cfg(target_os = "windows")]
use windows::Win32::System::SystemServices::SS_CENTER;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::*;

use super::{alloc_control_id, register_widget, WidgetKind};

extern "C" {
    fn js_closure_call1(closure: *const u8, arg: f64) -> f64;
    fn js_nanbox_get_pointer(value: f64) -> i64;
}

#[derive(Clone)]
struct ImageEntry {
    url: String,
    alt: String,
}

struct GalleryEntry {
    handle: i64,
    images: Vec<ImageEntry>,
    index: i64,
    on_index_change: f64,
}

thread_local! {
    static GALLERIES: RefCell<HashMap<i64, GalleryEntry>> = RefCell::new(HashMap::new());
}

fn str_from_header(ptr: *const u8) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe {
        let header = ptr as *const perry_runtime::string::StringHeader;
        let len = (*header).byte_len as usize;
        let data = ptr.add(std::mem::size_of::<perry_runtime::string::StringHeader>());
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(data, len)).to_string()
    }
}

#[cfg(target_os = "windows")]
fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn create(on_index_change: f64) -> i64 {
    let control_id = alloc_control_id();

    #[cfg(target_os = "windows")]
    {
        let class_name = to_wide("STATIC");
        let window_text = to_wide("");
        unsafe {
            let hinstance = GetModuleHandleW(None).unwrap();
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                windows::core::PCWSTR(class_name.as_ptr()),
                windows::core::PCWSTR(window_text.as_ptr()),
                WINDOW_STYLE(WS_CHILD.0 | WS_VISIBLE.0 | SS_CENTER.0),
                0,
                0,
                400,
                300,
                super::get_parking_hwnd(),
                HMENU(control_id as *mut _),
                HINSTANCE::from(hinstance),
                None,
            )
            .unwrap();

            let handle = register_widget(hwnd, WidgetKind::Image, control_id);
            GALLERIES.with(|g| {
                g.borrow_mut().insert(
                    handle,
                    GalleryEntry {
                        handle,
                        images: Vec::new(),
                        index: 0,
                        on_index_change,
                    },
                );
            });
            handle
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = on_index_change;
        let handle = register_widget(0, WidgetKind::Image, control_id);
        GALLERIES.with(|g| {
            g.borrow_mut().insert(
                handle,
                GalleryEntry {
                    handle,
                    images: Vec::new(),
                    index: 0,
                    on_index_change,
                },
            );
        });
        handle
    }
}

fn refresh_label(handle: i64) {
    #[cfg(target_os = "windows")]
    {
        let display = GALLERIES.with(|g| {
            let galleries = g.borrow();
            galleries.get(&handle).map(|gal| {
                if gal.images.is_empty() {
                    "[empty gallery]".to_string()
                } else {
                    let idx = gal.index.clamp(0, (gal.images.len() as i64) - 1) as usize;
                    let total = gal.images.len();
                    let entry = &gal.images[idx];
                    let alt = if entry.alt.is_empty() {
                        &entry.url
                    } else {
                        &entry.alt
                    };
                    format!("[{}/{}] {}", idx + 1, total, alt)
                }
            })
        });
        if let Some(text) = display {
            if let Some(hwnd) = super::get_hwnd(handle) {
                let wide = to_wide(&text);
                unsafe {
                    let _ = SetWindowTextW(hwnd, windows::core::PCWSTR(wide.as_ptr()));
                }
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = handle;
    }
}

pub fn add_image(handle: i64, url_ptr: *const u8, alt_ptr: *const u8) {
    let url = str_from_header(url_ptr);
    let alt = str_from_header(alt_ptr);
    GALLERIES.with(|g| {
        if let Some(gal) = g.borrow_mut().get_mut(&handle) {
            gal.images.push(ImageEntry { url, alt });
        }
    });
    refresh_label(handle);
}

pub fn set_index(handle: i64, index: i64) {
    let on_change = GALLERIES.with(|g| {
        let mut galleries = g.borrow_mut();
        if let Some(gal) = galleries.get_mut(&handle) {
            let max = gal.images.len() as i64;
            let new_idx = if max == 0 { 0 } else { index.clamp(0, max - 1) };
            if gal.index == new_idx {
                return None;
            }
            gal.index = new_idx;
            Some((gal.on_index_change, new_idx))
        } else {
            None
        }
    });
    refresh_label(handle);
    if let Some((closure, idx)) = on_change {
        if closure != 0.0 {
            let closure_ptr = unsafe { js_nanbox_get_pointer(closure) } as *const u8;
            if !closure_ptr.is_null() {
                unsafe {
                    js_closure_call1(closure_ptr, idx as f64);
                }
            }
        }
    }
}
