
#[link(name = "exempi")]

extern crate libc;
use self::libc::{c_int, c_char};

pub enum Xmp {}
pub enum XmpFile {}
pub enum XmpString {}
pub enum XmpIterator {}

pub const XMP_OPEN_NOOPTION: c_int = 0x00000000;        /**< No open option */
pub const XMP_OPEN_READ: c_int = 0x00000001;            /**< Open for read-only access. */
pub const XMP_OPEN_FORUPDATE: c_int = 0x00000002;       /**< Open for reading and writing. */
pub const XMP_OPEN_ONLYXMP: c_int = 0x00000004;         /**< Only the XMP is wanted, allows space/time optimizations. */
pub const XMP_OPEN_CACHETNAIL: c_int = 0x00000008;      /**< Cache thumbnail if possible, GetThumbnail will be called. */
pub const XMP_OPEN_STRICTLY: c_int = 0x00000010;        /**< Be strict about locating XMP and reconciling with other forms. */
pub const XMP_OPEN_USESMARTHANDLER: c_int = 0x00000020; /**< Require the use of a smart handler. */
pub const XMP_OPEN_USEPACKETSCANNING: c_int = 0x00000040; /**< Force packet scanning, don't use a smart handler. */
pub const XMP_OPEN_LIMITSCANNING: c_int = 0x00000080;     /**< Only packet scan files "known" to need scanning. */
pub const XMP_OPEN_REPAIR_FILE: c_int = 0x00000100; /**< Attempt to repair a file opened for update, default is to not open (throw an exception). */
pub const XMP_OPEN_OPTIMIZEFILELAYOUT: c_int = 0x00000200; /**< Optimize MPEG4 to support stream when updating This can take some time */
pub const XMP_OPEN_INBACKGROUND: c_int = 0x10000000; /**< Set if calling from background thread. */



pub const XMP_CLOSE_NOOPTION: c_int = 0x0000;  /**< No close option */
pub const XMP_CLOSE_SAFEUPDATE: c_int = 0x0001; /**< Write into a temporary file and swap for crash safety. */

extern "C" {
    pub fn xmp_init() -> bool;
    pub fn xmp_terminate();

    pub fn xmp_get_error() -> c_int;

    pub fn xmp_files_new() -> *mut XmpFile;
    pub fn xmp_files_open_new(p: *const c_char, options: c_int) -> *mut XmpFile;
    pub fn xmp_files_open(xf: *mut XmpFile, p:  *const c_char,
                          options: c_int) -> bool;
    pub fn xmp_files_close(xf: *mut XmpFile, options: c_int) -> bool;

    pub fn xmp_files_get_new_xmp(xf: *mut XmpFile) -> *mut Xmp;
    pub fn xmp_files_get_xmp(xf: *mut XmpFile, xmp: *mut Xmp) -> bool;
    pub fn xmp_files_can_put_xmp(xf: *mut XmpFile, xmp: *mut Xmp) -> bool;
    pub fn xmp_files_put_xmp(xf: *mut XmpFile, xmp: *mut Xmp) -> bool;

    pub fn xmp_files_free(xf: *mut XmpFile) -> bool;

    pub fn xmp_files_get_file_info(xf: *mut XmpFile, fp: *mut XmpString,
                                   options: *mut c_int, format: *mut c_int,
                                   handler_flags: *mut c_int) -> bool;
    pub fn xmp_files_check_file_format(p: *const c_char) -> c_int;

    pub fn xmp_free(xmp: *mut Xmp) -> bool;

    pub fn xmp_string_new() -> *mut XmpString;
    pub fn xmp_string_free(s: *mut XmpString);
    pub fn xmp_string_cstr(s: *mut XmpString) -> *const c_char;
}

#[test]
fn it_works() {
    let inited = unsafe { xmp_init() };

    assert!(inited);

    let xf = unsafe { xmp_files_new() };

    assert!(!xf.is_null());
    assert!(unsafe { xmp_files_free(xf) });
    assert!(unsafe { xmp_get_error() } == 0);

    unsafe { xmp_terminate() };
}
