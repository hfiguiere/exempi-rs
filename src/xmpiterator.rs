
use c;
use c::{IterBits, IterSkipBits};
use xmpstring::XmpString;

pub struct XmpIterator {
    ptr: *mut c::XmpIterator
}

impl XmpIterator {

    /// Construct a new `XmpIterator` from a native pointer
    pub fn new() -> XmpIterator {
        XmpIterator { ptr: unsafe { c::xmp_iterator_new() } }
    }

    /// Whether native pointer is null
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    /// Return native pointer.
    pub fn as_ptr(&self) -> *mut c::XmpIterator {
        self.ptr
    }

    /// Iterate to the next element following the option bitset from `IterBits`
    pub fn next(&mut self, schema: &mut XmpString, name: &mut XmpString,
                value: &mut XmpString, option: &mut IterBits) -> bool {
        unsafe { c::xmp_iterator_next(self.ptr, schema.as_mut_ptr(),
                                      name.as_mut_ptr(), value.as_mut_ptr(),
                                      option) }
    }

    /// Skip the poperties following the option bitset from `IterSkipBits`
    pub fn skip(&mut self, option: IterSkipBits) -> bool {
        if self.is_null() {
            return false;
        }
        unsafe { c::xmp_iterator_skip(self.ptr, option) }
    }
}

/// `XmpIterator` implements the `Drop` trait to release the memory
/// from the native object.
impl Drop for XmpIterator {
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { c::xmp_iterator_free(self.ptr) };
        }
    }
}
