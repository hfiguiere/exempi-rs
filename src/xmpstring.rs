
use ::c;
use std::str;
use std::ffi::{CStr};

pub struct XmpString {
    ptr: *mut c::XmpString
}

impl XmpString {
    pub fn new() -> XmpString {
        XmpString { ptr: unsafe { c::xmp_string_new() } }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn as_ptr(&self) -> *const c::XmpString {
        self.ptr
    }

    pub fn as_mut_ptr(&mut self) -> *mut c::XmpString {
        self.ptr
    }

    // XXX properly deal with the utf8 error
    pub fn to_str(&self) -> &str {
        unsafe {
            let s = CStr::from_ptr(c::xmp_string_cstr(self.ptr));
            // we are supposed to receive UTF8 from the library.
            str::from_utf8_unchecked(s.to_bytes())
        }
    }
}

impl Drop for XmpString {
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
