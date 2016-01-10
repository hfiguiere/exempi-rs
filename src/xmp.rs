extern crate libc;

use ::c;
use self::libc::c_char;
use ::xmpstring::XmpString;
use std::ffi::{CString};

pub struct Xmp {
    ptr: *mut c::Xmp
}

impl Xmp {
    pub fn from_ptr(ptr: *mut c::Xmp) -> Xmp {
        Xmp { ptr: ptr }
    }
    pub fn new() -> Xmp {
        Xmp { ptr: unsafe { c::xmp_new_empty() } }
    }
    pub fn from_buffer(buf: &[u8]) -> Xmp {
        Xmp { ptr: unsafe {
            c::xmp_new(buf.as_ptr() as *const c_char, buf.len())
        } }
    }
    // XXX &mut self ?
    pub fn parse(&self, buf: &[u8]) -> bool {
        unsafe {
            c::xmp_parse(self.ptr,
                         buf.as_ptr() as *const c_char,
                         buf.len())
        }
    }
    pub fn serialize(&self, buffer: &XmpString,
                     options: i32, padding: u32) -> bool {
        if self.is_null() || buffer.is_null() {
            return false;
        }
        unsafe { c::xmp_serialize(self.ptr,
                                  buffer.as_ptr(), options, padding) }
    }
    pub fn serialize_and_format(&self, buffer: &XmpString, options: u32,
                                padding: u32, newline: &str, tab: &str,
                                indent: i32) -> bool {
        if self.is_null() || buffer.is_null() {
            return false;
        }
        let s_newline = CString::new(newline).unwrap();
        let s_tab = CString::new(tab).unwrap();
        unsafe { c::xmp_serialize_and_format(self.ptr, buffer.as_ptr(),
                                             options, padding,
                                             s_newline.as_ptr(), s_tab.as_ptr(),
                                             indent) }
    }
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
    pub fn as_ptr(&self) -> *mut c::Xmp {
        self.ptr
    }

}

impl Clone for Xmp {
    fn clone(&self) -> Self {
        if self.is_null() {
            // inside ptr is NULL. cloning a null object.
            return Xmp { ptr: self.ptr };
        }
        Xmp { ptr: unsafe { c::xmp_copy(self.ptr) } }
    }
}

impl Drop for Xmp {
    /// Drop the Xmp.
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { c::xmp_free(self.ptr) };
        }
    }
}



