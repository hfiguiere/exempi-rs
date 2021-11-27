//
// Copyright (c) 2016-2021, Hubert Figuière
//
// License: BSD-2-Clause
// See top-level LICENSE file.
//

extern crate exempi_sys as c;
#[macro_use]
extern crate bitflags;

mod error;
mod xmp;
mod xmpfile;
mod xmpiterator;
mod xmpstring;

use std::cmp::Ordering;
use std::ffi::CString;
use std::mem::transmute;
use std::result;
use std::sync::Once;

pub use c::consts::*;
pub use c::FileType;
pub use c::TzSign;
pub use error::Error;
pub use xmp::{PropFlags, SerialFlags, Xmp};
pub use xmpfile::{CloseFlags, FormatOptionFlags, OpenFlags, XmpFile};
pub use xmpiterator::{IterFlags, IterSkipFlags, XmpIterator};
pub use xmpstring::XmpString;

/// Result type
pub type Result<T> = result::Result<T, Error>;

static START: Once = Once::new();

/// Initialize the library.
///
/// This will ensure xmp_init() is called only once so that
/// concurrent calls are exclusive.
///
/// It is automatically called by any entry point to the library.
pub(crate) fn init() -> bool {
    let mut inited = true;
    START.call_once(|| {
        // run initialization here
        inited = unsafe { c::xmp_init() };
    });
    inited
}

/// Get the last error code on the thread
/// Set when a function return false or None.
pub(crate) fn get_error() -> Error {
    let err = unsafe { c::xmp_get_error() };
    Error::from(match err {
        -15..=0 | -110..=-101 | -211..=-201 => unsafe { transmute(err) },
        _ => c::XmpError::TBD,
    })
}

/// Register namespace with uri and suggested prefix
/// Returns the actual registered prefix.
pub fn register_namespace(uri: &str, prefix: &str) -> Result<XmpString> {
    init();
    let s_uri = CString::new(uri).unwrap();
    let s_prefix = CString::new(prefix).unwrap();
    let mut reg_prefix = XmpString::new();
    if unsafe {
        c::xmp_register_namespace(s_uri.as_ptr(), s_prefix.as_ptr(), reg_prefix.as_mut_ptr())
    } {
        Ok(reg_prefix)
    } else {
        Err(get_error())
    }
}

/// Return the prefix for the namespace uri.
pub fn namespace_prefix<U: AsRef<[u8]>>(uri: U) -> Result<XmpString> {
    init();
    let s = CString::new(uri.as_ref()).unwrap();
    let mut prefix = XmpString::new();
    if unsafe { c::xmp_namespace_prefix(s.as_ptr(), prefix.as_mut_ptr()) } {
        Ok(prefix)
    } else {
        Err(get_error())
    }
}

/// Return the namespace uri for the prefix.
pub fn prefix_namespace<P: AsRef<[u8]>>(prefix: P) -> Result<XmpString> {
    init();
    let s = CString::new(prefix.as_ref()).unwrap();
    let mut uri = XmpString::new();
    if unsafe { c::xmp_prefix_namespace_uri(s.as_ptr(), uri.as_mut_ptr()) } {
        Ok(uri)
    } else {
        Err(get_error())
    }
}

/// A wrapper around the C type DateTime
#[derive(Clone, Debug, Default)]
pub struct DateTime(c::XmpDateTime);

impl DateTime {
    /// Create a new DateTime
    pub fn new() -> Self {
        DateTime::default()
    }
    /// Return the native pointer
    pub fn as_ptr(&self) -> *const c::XmpDateTime {
        &self.0 as *const c::XmpDateTime
    }
    /// Return the native mutable pointer
    pub fn as_mut_ptr(&mut self) -> *mut c::XmpDateTime {
        &mut self.0 as *mut c::XmpDateTime
    }
    /// Set date
    pub fn set_date(&mut self, year: i32, month: i32, day: i32) {
        self.0.year = year;
        self.0.month = month;
        self.0.day = day;
        self.0.has_date = 1;
    }
    /// Set time
    pub fn set_time(&mut self, hour: i32, min: i32, sec: i32) {
        self.0.hour = hour;
        self.0.minute = min;
        self.0.second = sec;
        self.0.has_time = 1;
    }
    /// Set nano_second
    pub fn set_nano_second(&mut self, nano_second: i32) {
        self.0.nano_second = nano_second;
    }
    /// Set Timezone
    pub fn set_timezone(&mut self, sign: TzSign, hour: i32, min: i32) {
        self.0.tz_sign = sign;
        self.0.tz_hour = hour;
        self.0.tz_minute = min;
        self.0.has_tz = 1;
    }
    /// Year of the DateTime
    pub fn year(&self) -> i32 {
        self.0.year
    }
    /// Month of the DateTime
    pub fn month(&self) -> i32 {
        self.0.month
    }
    /// Day of the DateTime
    pub fn day(&self) -> i32 {
        self.0.day
    }
    /// Hour of the DateTime
    pub fn hour(&self) -> i32 {
        self.0.hour
    }
    /// Minute of the DateTime
    pub fn minute(&self) -> i32 {
        self.0.minute
    }
    /// Seconds of the DateTime
    pub fn second(&self) -> i32 {
        self.0.second
    }
    /// nano seconds of the DateTime
    pub fn nano_second(&self) -> i32 {
        self.0.nano_second
    }
}

impl From<c::XmpDateTime> for DateTime {
    /// Construct from the native C type
    fn from(d: c::XmpDateTime) -> DateTime {
        DateTime(d)
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &DateTime) -> bool {
        unsafe {
            c::xmp_datetime_compare(
                &self.0 as *const c::XmpDateTime,
                &other.0 as *const c::XmpDateTime,
            ) == 0
        }
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &DateTime) -> Option<Ordering> {
        match unsafe {
            c::xmp_datetime_compare(
                &self.0 as *const c::XmpDateTime,
                &other.0 as *const c::XmpDateTime,
            )
        } {
            0 => Some(Ordering::Equal),
            n if n < 0 => Some(Ordering::Less),
            n if n > 0 => Some(Ordering::Greater),
            _ => None,
        }
    }
}

impl Eq for DateTime {}

impl Ord for DateTime {
    fn cmp(&self, other: &DateTime) -> Ordering {
        match unsafe {
            c::xmp_datetime_compare(
                &self.0 as *const c::XmpDateTime,
                &other.0 as *const c::XmpDateTime,
            )
        } {
            n if n < 0 => Ordering::Less,
            n if n > 0 => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}
