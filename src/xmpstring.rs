extern crate libc;

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

    pub fn ptr(&self) -> *mut c::XmpString {
        self.ptr
    }

    // XXX properly deal with the utf8 error
    pub fn to_str(&self) -> &str {
        let s = unsafe { CStr::from_ptr(c::xmp_string_cstr(self.ptr)) };
        let utf8 = str::from_utf8(s.to_bytes());
        if utf8.is_ok() {
            return utf8.unwrap();
        }
        ""
    }
}

impl Drop for XmpString {
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { c::xmp_string_free(self.ptr) };
        }
    }
}
