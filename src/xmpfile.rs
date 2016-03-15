extern crate libc;

use ::c;
use ::xmp::Xmp;
use ::xmpstring::XmpString;
use std::ffi::{CString};
use self::libc::{c_int};

pub struct XmpFile {
    ptr: *mut c::XmpFile,
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

    pub fn get_new_xmp(&self) -> Xmp {
        Xmp::from_ptr(unsafe {
            c::xmp_files_get_new_xmp(self.ptr)
        })
    }

    pub fn get_xmp(&self, xmp: &mut Xmp) -> bool {
        if self.is_null() || xmp.is_null() {
            return false;
        }
        unsafe { c::xmp_files_get_xmp(self.ptr, xmp.as_mut_ptr()) }
    }

    pub fn can_put_xmp(&self, xmp: &Xmp) -> bool {
        if self.is_null() || xmp.is_null() {
            return false;
        }
        unsafe { c::xmp_files_can_put_xmp(self.ptr, xmp.as_ptr()) }
    }
    pub fn put_xmp(&mut self, xmp: &Xmp) -> bool {
        if self.is_null() || xmp.is_null() {
            return false;
        }
        unsafe { c::xmp_files_put_xmp(self.ptr, xmp.as_ptr()) }
    }

    pub fn get_file_info(&self, file_path: &mut String,
                         options: &mut i32, format: &mut i32,
                         handler_flags: &mut i32) -> bool {
        if self.is_null() {
            return false;
        }
        let mut s: XmpString = XmpString::new();

        let result = unsafe {
            c::xmp_files_get_file_info(self.ptr, s.as_mut_ptr(),
                                       options as *mut c_int,
                                       format as *mut c_int,
                                       handler_flags as *mut c_int)
        };

        file_path.push_str(s.to_str());
        result
    }

    pub fn check_file_format(p: &str) -> i32 {
        let pp: CString = CString::new(p).unwrap();
        unsafe { c::xmp_files_check_file_format(pp.as_ptr()) }
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
