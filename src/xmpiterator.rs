
use ::c;
use ::xmpstring::XmpString;

pub struct XmpIterator {
    ptr: *mut c::XmpIterator
}



impl XmpIterator {

    pub fn new() -> XmpIterator {
        XmpIterator { ptr: unsafe { c::xmp_iterator_new() } }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn as_ptr(&self) -> *mut c::XmpIterator {
        self.ptr
    }

    pub fn next(&mut self, schema: &mut XmpString, name: &mut XmpString,
                value: &mut XmpString, option: &mut u32) -> bool {
        unsafe { c::xmp_iterator_next(self.ptr, schema.as_mut_ptr(),
                                      name.as_mut_ptr(), value.as_mut_ptr(),
                                      option as *mut u32 ) }
    }
    pub fn skip(&mut self, option: i32) -> bool {
        if self.is_null() {
            return false;
        }
        unsafe { c::xmp_iterator_skip(self.ptr, option) }
    }
}

impl Drop for XmpIterator {
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { c::xmp_iterator_free(self.ptr) };
        }
    }
}
