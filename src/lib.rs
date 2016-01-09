
pub mod c;
pub mod xmp;
pub mod xmpstring;
pub mod xmpfile;

/// Initialize the library
pub fn init() -> bool {
    unsafe { c::xmp_init() }
}

/// Terminate the library
pub fn terminate() {
    unsafe { c::xmp_terminate() }
}

/// Get the last error code on the thread
pub fn get_error() -> i32 {
    unsafe { c::xmp_get_error() as i32 }
}
