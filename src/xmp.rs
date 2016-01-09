
use ::c;

pub struct Xmp {
    ptr: *mut c::Xmp
}



impl Xmp {
    pub fn from_ptr(ptr: *mut c::Xmp) -> Xmp {
        Xmp { ptr: ptr }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
    pub fn c_ptr(&self) -> *mut c::Xmp {
        self.ptr
    }
}

// TODO implement clone

impl Drop for Xmp {
    /// Drop the Xmp.
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { c::xmp_free(self.ptr) };
        }
    }
}



