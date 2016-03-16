
use ::c;
use std::str;
use std::ffi::{CStr};

/// The string wrapper from Exempi.
/// It is meant to be used for output parameter.
/// But gives you ownership of the string.
pub struct XmpString {
    ptr: *mut c::XmpString
}

impl XmpString {
    /// Create a new XmpString
    pub fn new() -> XmpString {
        XmpString { ptr: unsafe { c::xmp_string_new() } }
    }

    /// Underlying string is NULL
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    /// Return the native pointer
    pub fn as_ptr(&self) -> *const c::XmpString {
        self.ptr
    }

    /// Return the mutable native pointer
    pub fn as_mut_ptr(&mut self) -> *mut c::XmpString {
        self.ptr
    }

    /// Convert to a str
    // XXX properly deal with the utf8 error
    pub fn to_str(&self) -> &str {
        unsafe {
            let s = CStr::from_ptr(c::xmp_string_cstr(self.ptr));
            // we are supposed to receive UTF8 from the library.
            str::from_utf8_unchecked(s.to_bytes())
        }
    }
}

impl ToString for XmpString {
    fn to_string(&self) -> String {
        String::from(self.to_str())
    }
}

impl Drop for XmpString {
    /// Will deallocate properly the underlying object
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { c::xmp_string_free(self.ptr) };
        }
    }
}

impl Eq for XmpString {

}
impl PartialEq for XmpString {
    fn eq(&self, other: &XmpString) -> bool {
        self.to_str() == other.to_str()
    }
}
