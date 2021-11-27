//
// Copyright (c) 2016-2021, Hubert Figuière
//
// License: BSD-2-Clause
// See top-level LICENSE file.
//

use std::ffi::CStr;
use std::fmt;
use std::str;

/// The string wrapper from Exempi.
///
/// It is meant to be used for output parameter. But gives you ownership of the string.
/// Because of the way the C API of Exempi is implemented, we provide
/// this type instead of using std::string::String to avoid copying of strings
/// until needed.
/// They are mostly returned in an Option<XmpString> enum.
/// XmpString support several of the standard traits.
///
/// ```no_run
/// use exempi2::{Xmp,PropFlags};
///
/// let xmp = Xmp::new();
/// let mut flags = PropFlags::empty();
///
/// if let Ok(ref xmpstring) = xmp.get_property("http://rust.figuiere.net/ns/rust/", "rust", &mut flags) {
///    println!("property value is {}, flags {}", xmpstring, flags.bits());
///    println!("string len: {}", xmpstring.len());
///    let s = String::from(xmpstring);
///    println!("converted to std::String: {}", s);
/// }
/// ```
#[derive(Debug)]
pub struct XmpString(*mut c::XmpString);

impl Default for XmpString {
    /// Default XmpString is allocated and ready to use
    fn default() -> XmpString {
        XmpString(unsafe { c::xmp_string_new() })
    }
}

impl XmpString {
    /// Create a new XmpString
    pub fn new() -> XmpString {
        XmpString::default()
    }

    /// Native pointer is NULL
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Return the native pointer
    pub fn as_ptr(&self) -> *const c::XmpString {
        self.0
    }

    /// Return the mutable native pointer
    pub fn as_mut_ptr(&mut self) -> *mut c::XmpString {
        self.0
    }

    /// Return the length of the string
    pub fn len(&self) -> usize {
        if self.0.is_null() {
            return 0;
        }
        unsafe { c::xmp_string_len(self.0) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Convert to a str
    // XXX properly deal with the utf8 error
    pub fn to_str(&self) -> Option<&str> {
        unsafe {
            let s = CStr::from_ptr(c::xmp_string_cstr(self.0));
            // we are supposed to receive UTF8 from the library.
            str::from_utf8(s.to_bytes()).ok()
        }
    }
}

impl From<&XmpString> for String {
    /// Convert an XmpString safely to string.
    /// The result is lossy in it is not utf-8.
    fn from(s: &XmpString) -> String {
        unsafe {
            let s = CStr::from_ptr(c::xmp_string_cstr(s.0));
            // we are supposed to receive UTF8 from the library.
            // be we'll play it safe.
            String::from_utf8_lossy(s.to_bytes()).to_string()
        }
    }
}

impl Drop for XmpString {
    /// Will deallocate properly the underlying object
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { c::xmp_string_free(self.0) };
        }
    }
}

impl Eq for XmpString {}
impl PartialEq for XmpString {
    fn eq(&self, other: &XmpString) -> bool {
        self.to_str() == other.to_str()
    }
}

impl fmt::Display for XmpString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

#[cfg(test)]
mod test {
    use super::XmpString;

    #[test]
    fn test_xmpstring() {
        let s = XmpString::default();

        assert!(!s.is_null());
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);

        let s2 = XmpString::new();
        assert_eq!(s, s2);

        let string = String::from(&s2);
        assert_eq!(string, "".to_owned());
    }
}
