mod loot;
mod conditions;
mod weather;
mod random;
mod string_format;

use std::ffi::{CString};
use std::os::raw::{c_char, c_int};
use std::sync::Once;
pub use weather::violet_get_weather_state;
pub use random::{
    violet_urand, violet_irand, violet_urand32, violet_frand,
    violet_rand_chance, violet_rand_norm
};
pub use string_format::{
    violet_format_mmap_path,
    violet_format_mmap_path_for_tile,
    violet_format_mmap_path_for_terrain
};

static INIT: Once = Once::new();

/// on a server start once (World.cpp)
#[unsafe(no_mangle)]
pub extern "C" fn violet_core_attach() -> c_int {
    INIT.call_once(|| {
        println!("\n\x1b[95m[VioletCore]\x1b[0m Assimilation protocol initialized.");
        println!("\x1b[95m[VioletCore]\x1b[0m Runtime: Rust 1.94.0 Nightly | Mode: Active\n");
    });

    0 // success in C++
}

// (const char*).
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn violet_on_message_receive(raw_ptr: *const c_char) {
//     if raw_ptr.is_null() {
//         return;
//     }
//
//     let c_str = unsafe { CStr::from_ptr(raw_ptr) };
//
//     if let Ok(message) = c_str.to_str() {
//         if message.contains(".violet") {
//             println!("\x1b[95m[VioletCore]\x1b[0m Shadow command detected: {}", message);
//         }
//     }
// }

#[unsafe(no_mangle)]
pub extern "C" fn violet_get_status() -> *mut c_char {
    let status = CString::new("Violet System: Active. Core integrated.").unwrap();
    status.into_raw()
}

/// clean memory for Rust in С++
#[unsafe(no_mangle)]
pub unsafe extern "C" fn violet_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}