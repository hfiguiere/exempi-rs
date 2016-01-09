extern crate libc;

use ::c;
use std::ffi::CString;
use self::libc::{c_int};

pub struct XmpFile {
    ptr: * mut c::XmpFile
}

impl XmpFile {
    pub fn new() -> XmpFile {
        XmpFile { ptr: unsafe { c::xmp_files_new() } }
    }

    pub fn open_new(p: &str, options: i32) -> XmpFile {
        let pp: CString = CString::new(p).unwrap();
        XmpFile {
            ptr: unsafe {
                c::xmp_files_open_new(pp.as_ptr(), options as c_int)
            }
        }
    }

    pub fn open(&mut self, p: &str, options: i32) -> bool {
        if self.is_null() {
            return false;
        }
        let pp: CString = CString::new(p).unwrap();
        unsafe {
            c::xmp_files_open(self.ptr, pp.as_ptr(), options as c_int)
        }
    }

    pub fn close(&mut self, options: i32) -> bool {
        if self.is_null() {
            return false;
        }
        unsafe {
            c::xmp_files_close(self.ptr, options as c_int)
        }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

impl Drop for XmpFile {
    /// Drop the XmpFile.
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { c::xmp_files_free(self.ptr) };
        }
    }
}


#[test]
fn it_works() {
    let inited = ::init();

    assert!(inited);

    let xf = XmpFile::new();
    assert!(!xf.is_null());

    assert!(::get_error() == 0);

    ::terminate();
}
