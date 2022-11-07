//
// Copyright (c) 2016-2021, Hubert Figuière
//
// License: BSD-2-Clause
// See top-level LICENSE file.
//

pub use c::XmpError;

/// Error from XMP
#[derive(Debug, Eq, PartialEq)]
pub struct Error(pub XmpError);

/// Error trait to XmpError.
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<c::XmpError> for Error {
    fn from(e: c::XmpError) -> Error {
        Error(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as i32)
    }
}
