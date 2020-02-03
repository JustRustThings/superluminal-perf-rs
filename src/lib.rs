#![allow(unused_variables)]

use std::ffi::CStr;
#[cfg(all(feature = "enable", target_os = "windows"))]
use superluminal_perf_sys as ffi;

/// Check if the API is enabled
pub const fn enabled() -> bool {
    cfg!(all(feature = "enable", target_os = "windows"))
}

/// Begin an instrumentation event with the specified ID
///
/// The ID for a specific scope must be the same over the lifetime of the program
pub fn begin_event(id: &'static [u8]) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        let id_cstr = CStr::from_bytes_with_nul(id)
            .expect("Invalid ID string, must be null-terminated and not contain interior null");
        ffi::PerformanceAPI_BeginEvent(id_cstr.as_ptr(), std::ptr::null(), ffi::DEFAULT_COLOR)
    }
}

pub fn begin_event_cstr(id: &'static CStr) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_BeginEvent(id.as_ptr(), std::ptr::null(), ffi::DEFAULT_COLOR)
    }
}

/// Begin an instrumentation event with the specified ID and runtime data
///
/// The ID for a specific scope must be the same over the lifetime of the program.
/// The data can vary for each invocation of this scope and is intended to hold information that is only available at runtime.
pub fn begin_event_with_data(id: &'static [u8], data: &[u8]) {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        let id_cstr = CStr::from_bytes_with_nul(id)
            .expect("Invalid ID string, must be null-terminated and not contain interior null");
        let data_cstr = CStr::from_bytes_with_nul(data)
            .expect("Invalid data string, must be null-terminated and not contain interior null");
        ffi::PerformanceAPI_BeginEvent(id_cstr.as_ptr(), data_cstr.as_ptr(), ffi::DEFAULT_COLOR)
    }
}

/// End an instrumentation event. Must be matched with a call to BeginEvent within the same function
pub fn end_event() {
    #[cfg(all(feature = "enable", target_os = "windows"))]
    unsafe {
        ffi::PerformanceAPI_EndEvent();
    }
}

/// Set the name of the current thread to the specified thread name
pub fn set_current_thread_name(_name: &str) {}
