extern crate libc;

pub mod c;
pub mod xmp;
pub mod xmpstring;
pub mod xmpfile;
pub mod xmpiterator;

use std::ffi::{CString};
use std::cmp::Ordering;
pub use ::xmp::Xmp;
pub use ::xmpfile::XmpFile;
pub use ::xmpstring::XmpString;
pub use ::xmpiterator::XmpIterator;

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
                                       reg_prefix.as_mut_ptr()) }
}

pub fn namespace_prefix(uri: &str, prefix: &mut xmpstring::XmpString) -> bool {
    let s = CString::new(uri).unwrap();
    unsafe { c::xmp_namespace_prefix(s.as_ptr(), prefix.as_mut_ptr()) }
}

pub fn prefix_namespace(prefix: &str, uri: &mut xmpstring::XmpString) -> bool {
    let s = CString::new(prefix).unwrap();
    unsafe { c::xmp_prefix_namespace_uri(s.as_ptr(), uri.as_mut_ptr()) }
}

/// A wrapper around the C type XmpDateTime
pub struct XmpDateTime {
    c: c::XmpDateTime
}

impl XmpDateTime {
    /// construct from the native C type
    pub fn wrap(d: c::XmpDateTime) -> XmpDateTime {
        XmpDateTime { c: d }
    }
    pub fn as_ptr(&self) -> *const c::XmpDateTime {
        &self.c as *const c::XmpDateTime
    }
    pub fn as_mut_ptr(&mut self) -> *mut c::XmpDateTime {
        &mut self.c as *mut c::XmpDateTime
    }
}

impl PartialEq for XmpDateTime {
    fn eq(&self, other: &XmpDateTime) -> bool {
        unsafe {
            c::xmp_datetime_compare(&self.c as *const c::XmpDateTime,
                                    &other.c as *const c::XmpDateTime) == 0
        }
    }
}
impl PartialOrd for XmpDateTime {
    fn partial_cmp(&self, other: &XmpDateTime) -> Option<Ordering> {
        match unsafe {
            c::xmp_datetime_compare(&self.c as *const c::XmpDateTime,
                                    &other.c as *const c::XmpDateTime)
        } {
            0 => Some(Ordering::Equal),
            n if n < 0 => Some(Ordering::Less),
            n if n > 0 => Some(Ordering::Greater),
            _ => None
        }
    }
}
impl Eq for XmpDateTime {

}
impl Ord for XmpDateTime {
    fn cmp(&self, other: &XmpDateTime) -> Ordering {
        match unsafe {
            c::xmp_datetime_compare(&self.c as *const c::XmpDateTime,
                                    &other.c as *const c::XmpDateTime)
        } {
            n if n < 0 => Ordering::Less,
            n if n > 0 => Ordering::Greater,
            _ => Ordering::Equal
        }
    }
}

