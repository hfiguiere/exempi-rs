
use c;
use std::fmt;
use std::str;
use std::ffi::{CStr};

/// The string wrapper from Exempi. It is meant to be used for output parameter.
/// But gives you ownership of the string.
/// Because of the way the C API of Exempi is implemented, we provide
/// this type instead of using std::string::String to avoid copying of strings
/// until needed.
/// They are mostly returned in an Option<XmpString> enum.
/// XmpString support several of the standard traits.
///
/// ```no_run
/// use exempi::{Xmp,PropFlags};
///
/// let xmp = Xmp::new();
/// let mut flags = PropFlags::empty();
///
/// if let Some(ref xmpstring) = xmp.get_property("http://rust.figuiere.net/ns/rust/", "rust", &mut flags) {
///    println!("property value is {}, flags {}", xmpstring, flags.bits());
///    println!("string len: {}", xmpstring.len());
///    let s = String::from(xmpstring.to_str());
///    println!("converted to std::String: {}", s);
/// }
/// ```
pub struct XmpString {
    ptr: *mut c::XmpString
}

impl XmpString {
    /// Create a new XmpString
    pub fn new() -> XmpString {
        XmpString { ptr: unsafe { c::xmp_string_new() } }
    }

    /// Native pointer is NULL
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

    /// Return the length of the string
    pub fn len(&self) -> usize {
        if self.ptr.is_null() {
            return 0;
        }
        unsafe { c::xmp_string_len(self.ptr) }
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

impl fmt::Display for XmpString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
