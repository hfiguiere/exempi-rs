extern crate libc;

pub mod c;
pub mod xmp;
pub mod xmpstring;
pub mod xmpfile;

use std::ffi::{CString};

/// Initialize the library
pub fn init() -> bool {
    unsafe { c::xmp_init() }
}

/// Terminate the library
pub fn terminate() {
    unsafe { c::xmp_terminate() }
}

/// Get the last error code on the thread
pub fn get_error() -> i32 {
    unsafe { c::xmp_get_error() as i32 }
}

pub fn register_namespace(uri: &str, prefix: &str,
                          reg_prefix: &mut xmpstring::XmpString) -> bool {
    let s_uri = CString::new(uri).unwrap();
    let s_prefix = CString::new(prefix).unwrap();
    unsafe { c::xmp_register_namespace(s_uri.as_ptr(), s_prefix.as_ptr(),
                                       reg_prefix.as_ptr()) }
}

pub fn namespace_prefix(uri: &str, prefix: &mut xmpstring::XmpString) -> bool {
    let s = CString::new(uri).unwrap();
    unsafe { c::xmp_namespace_prefix(s.as_ptr(), prefix.as_ptr()) }
}

pub fn prefix_namespace(prefix: &str, uri: &mut xmpstring::XmpString) -> bool {
    let s = CString::new(prefix).unwrap();
    unsafe { c::xmp_prefix_namespace_uri(s.as_ptr(), uri.as_ptr()) }
}

