use c;
use std::ffi::CStr;
use std::fmt;
use std::str;

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
/// if let Ok(ref xmpstring) = xmp.get_property("http://rust.figuiere.net/ns/rust/", "rust", &mut flags) {
///    println!("property value is {}, flags {}", xmpstring, flags.bits());
///    println!("string len: {}", xmpstring.len());
///    let s = String::from(xmpstring.to_str());
///    println!("converted to std::String: {}", s);
/// }
/// ```
#[derive(Debug)]
pub struct XmpString(*mut c::XmpString);

impl Default for XmpString {
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
    pub fn to_str(&self) -> &str {
        unsafe {
            let s = CStr::from_ptr(c::xmp_string_cstr(self.0));
            // we are supposed to receive UTF8 from the library.
            str::from_utf8_unchecked(s.to_bytes())
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
        write!(f, "{}", self.to_str())
    }
}
