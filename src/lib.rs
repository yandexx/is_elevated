//! **is_elevated** is a simple Windows-only crate that lets you determine
//! whether the current process is running as elevated (also known “as
//! administrator,” or integrity level High), or not (integrity level Medium
//! or lower).
//!
//! ## Example
//! ```rust
//! use is_elevated::is_elevated;
//!
//! if !is_elevated() {
//!		println!(
//!			"Warning: the program isn’t running as elevated; some functionality may not work."
//!		);
//! }
//! ```
//!
//!
#![cfg(windows)]
#![doc(html_root_url = "https://docs.rs/is_elevated/0.1.1")]
#![deny(missing_docs)]

extern crate winapi;
use std::mem;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::LPVOID;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::TokenElevation;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::TOKEN_ELEVATION;
use winapi::um::winnt::TOKEN_QUERY;

/// Returns a boolean value, indicating whether the current process is elevated.
/// ## Example
/// ```rust
/// use is_elevated::is_elevated;
///
/// if !is_elevated() {
///		println!(
///			"Warning: the program isn’t running as elevated; some functionality may not work."
///		);
/// }
/// ```
pub fn is_elevated() -> bool {
    // based on https://stackoverflow.com/a/8196291
    unsafe {
        let mut current_token_ptr: HANDLE = mem::zeroed();
        let mut token_elevation: TOKEN_ELEVATION = mem::zeroed();
        let token_elevation_type_ptr: *mut TOKEN_ELEVATION = &mut token_elevation;
        let mut size: DWORD = 0;

        let result = OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut current_token_ptr);

        if result != 0 {
            let result = GetTokenInformation(
                current_token_ptr,
                TokenElevation,
                token_elevation_type_ptr as LPVOID,
                mem::size_of::<winapi::um::winnt::TOKEN_ELEVATION_TYPE>() as u32,
                &mut size,
            );
            if result != 0 {
                return token_elevation.TokenIsElevated != 0;
            }
        }
    }
    false
}
