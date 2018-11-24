extern crate libc;
extern crate exempi_sys as c;
#[macro_use]
extern crate bitflags;

mod xmp;
mod xmpstring;
mod xmpfile;
mod xmpiterator;

use std::ffi::{CString};
use std::cmp::Ordering;
use std::mem::transmute;
use std::result;
use std::sync::{Once, ONCE_INIT};

pub use xmp::Xmp as Xmp;
pub use xmpfile::XmpFile as XmpFile;
pub use xmpstring::XmpString as XmpString;
pub use xmpiterator::XmpIterator as XmpIterator;
pub use c::FileType as FileType;
pub use c::XmpError as Error;
pub use c::consts::*;
pub use c::TzSign as XmpTzSign;
// all the flags.
pub use xmpfile::flags::*;
pub use xmp::flags::*;
pub use xmpiterator::flags::*;

/// Result type
pub type Result<T> = result::Result<T, Error>;

static START: Once = ONCE_INIT;

/// Initialize the library.
///
/// This will ensure xmp_init() is called only once so that
/// concurrent calls are exclusive.
pub fn init() -> bool {
    let mut inited = true;
    START.call_once(|| {
        // run initialization here
        inited = unsafe { c::xmp_init() };
    });
    inited
}

/// Force initialize the library.
///
/// Can be safely called more than once or after `init()`
/// but can't be called concurrently from different threads.
pub fn force_init() -> bool {
    unsafe { c::xmp_init() }
}

/// Terminate the library. (use is discouraged)
///
/// As a side effect, you can't counter balance it with a `init()`
/// hence its use is discouraged.
pub fn terminate() {
    unsafe { c::xmp_terminate() }
}

/// Get the last error code on the thread
/// Set when a function return false or None.
pub fn get_error() -> Error {
    let err = unsafe { c::xmp_get_error() };
    match err {
        -15...0 |
        -110...-101 |
        -211...-201 => unsafe { transmute(err) },
        _ => Error::TBD
    }
}

/// Register namespace with uri and suggested prefix
/// Returns the actual registered prefix.
pub fn register_namespace(uri: &str, prefix: &str) -> Option<XmpString> {
    let s_uri = CString::new(uri).unwrap();
    let s_prefix = CString::new(prefix).unwrap();
    let mut reg_prefix = XmpString::new();
    if unsafe { c::xmp_register_namespace(s_uri.as_ptr(), s_prefix.as_ptr(),
                                          reg_prefix.as_mut_ptr()) } {
        Some(reg_prefix)
    } else {
        None
    }
}

/// Return the prefix for the namespace uri.
pub fn namespace_prefix(uri: &str) -> Option<XmpString> {
    let s = CString::new(uri).unwrap();
    let mut prefix = XmpString::new();
    if unsafe { c::xmp_namespace_prefix(s.as_ptr(), prefix.as_mut_ptr()) } {
        Some(prefix)
    } else {
        None
    }
}

/// Return the namespace uri for the prefix.
pub fn prefix_namespace(prefix: &str) -> Option<XmpString> {
    let s = CString::new(prefix).unwrap();
    let mut uri = XmpString::new();
    if unsafe { c::xmp_prefix_namespace_uri(s.as_ptr(), uri.as_mut_ptr()) } {
        Some(uri)
    } else {
        None
    }
}

/// A wrapper around the C type DateTime
#[derive(Clone, Debug, Default)]
pub struct DateTime {
    c: c::XmpDateTime
}

impl DateTime {
    pub fn new() -> Self {
        DateTime::default()
    }
    /// Construct from the native C type
    pub fn from(d: c::XmpDateTime) -> DateTime {
        DateTime { c: d }
    }
    /// Return the native pointer
    pub fn as_ptr(&self) -> *const c::XmpDateTime {
        &self.c as *const c::XmpDateTime
    }
    /// Return the native mutable pointer
    pub fn as_mut_ptr(&mut self) -> *mut c::XmpDateTime {
        &mut self.c as *mut c::XmpDateTime
    }
    /// Set date
    pub fn set_date(&mut self, year: i32, month: i32, day: i32) {
        self.c.year = year;
        self.c.month = month;
        self.c.day = day;
        self.c.has_date = 1;
    }
    /// Set time
    pub fn set_time(&mut self, hour: i32, min: i32, sec: i32) {
        self.c.hour = hour;
        self.c.minute = min;
        self.c.second = sec;
        self.c.has_time = 1;
    }
    /// Set Timezone
    pub fn set_timezone(&mut self, sign: XmpTzSign, hour: i32, min: i32) {
        self.c.tz_sign = sign;
        self.c.tz_hour = hour;
        self.c.tz_minute = min;
        self.c.has_tz = 1;
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &DateTime) -> bool {
        unsafe {
            c::xmp_datetime_compare(&self.c as *const c::XmpDateTime,
                                    &other.c as *const c::XmpDateTime) == 0
        }
    }
}
impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &DateTime) -> Option<Ordering> {
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
impl Eq for DateTime {

}
impl Ord for DateTime {
    fn cmp(&self, other: &DateTime) -> Ordering {
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

