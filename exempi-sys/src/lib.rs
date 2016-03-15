
#[link(name = "exempi")]
extern crate libc;

use libc::{c_int, c_char, size_t};

pub enum Xmp {}
pub enum XmpFile {}
pub enum XmpString {}
pub enum XmpIterator {}

// the C defined struct.
#[repr(C)]
pub struct XmpDateTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
    pub tz_sign: i32,
    pub tz_hour: i32,
    pub tz_minute: i32,
    pub nano_second: i32,
}

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
    pub fn xmp_files_can_put_xmp(xf: *mut XmpFile, xmp: *const Xmp) -> bool;
    pub fn xmp_files_put_xmp(xf: *mut XmpFile, xmp: *const Xmp) -> bool;

    pub fn xmp_files_free(xf: *mut XmpFile) -> bool;

    pub fn xmp_files_get_file_info(xf: *mut XmpFile, fp: *mut XmpString,
                                   options: *mut c_int, format: *mut c_int,
                                   handler_flags: *mut c_int) -> bool;
    pub fn xmp_files_check_file_format(p: *const c_char) -> c_int;

    pub fn xmp_register_namespace(uri: *const c_char, prefix: *const c_char,
                                  reg_prefix: *mut XmpString) -> bool;
    pub fn xmp_namespace_prefix(ns: *const c_char,
                                prefix: *mut XmpString) -> bool;
    pub fn xmp_prefix_namespace_uri(prefix: *const c_char,
                                    ns: *mut XmpString) -> bool;

    pub fn xmp_new_empty() -> *mut Xmp;
    pub fn xmp_new(buffer: *const c_char, len: size_t) -> *mut Xmp;
    pub fn xmp_copy(xmp: *const Xmp) -> *mut Xmp;
    pub fn xmp_free(xmp: *mut Xmp) -> bool;
    pub fn xmp_parse(xmp: *mut Xmp, buffer: *const c_char, len: size_t) -> bool;
    pub fn xmp_serialize(xmp: *const Xmp, buf: *mut XmpString, options: c_int,
                         padding: u32) -> bool;
    pub fn xmp_serialize_and_format(xmp: *const Xmp, buf: *mut XmpString,
                                    options: u32, padding: u32,
                                    newline: *const c_char, tab: *const c_char,
                                    indent: i32) -> bool;

    // get properties
    pub fn xmp_get_property(xmp: *const Xmp, schema: *const c_char,
                            name: *const c_char, property: *mut XmpString,
                            propsbits: *mut u32) -> bool;
    pub fn xmp_get_property_date(xmp: *const Xmp, schema: *const c_char,
                                 name: *const c_char,
                                 property: *mut XmpDateTime,
                                 propsbits: *mut u32) -> bool;
    pub fn xmp_get_property_float(xmp: *const Xmp, schema: *const c_char,
                                  name: *const c_char,
                                  property: *mut f64,
                                  propsbits: *mut u32) -> bool;
    pub fn xmp_get_property_bool(xmp: *const Xmp, schema: *const c_char,
                                  name: *const c_char,
                                  property: *mut bool,
                                  propsbits: *mut u32) -> bool;
    pub fn xmp_get_property_int32(xmp: *const Xmp, schema: *const c_char,
                                  name: *const c_char,
                                  property: *mut i32,
                                  propsbits: *mut u32) -> bool;
    pub fn xmp_get_property_int64(xmp: *const Xmp, schema: *const c_char,
                                  name: *const c_char,
                                  property: *mut i64,
                                  propsbits: *mut u32) -> bool;
    pub fn xmp_get_array_item(xmp: *const Xmp,  schema: *const c_char,
                              name: *const c_char, index: i32,
                              property: *mut XmpString,
                              propsbits: *mut u32) -> bool;

    // set properies
    pub fn xmp_set_property(xmp: *mut Xmp, schema: *const c_char,
                            name: *const c_char, value: *const c_char,
                            optionbits: u32) -> bool;
    pub fn xmp_set_property_date(xmp: *mut Xmp, schema: *const c_char,
                            name: *const c_char, value: *const XmpDateTime,
                            optionbits: u32) -> bool;
    pub fn xmp_set_property_float(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, value: f64,
                                  optionbits: u32) -> bool;
    pub fn xmp_set_property_bool(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, value: bool,
                                  optionbits: u32) -> bool;
    pub fn xmp_set_property_int32(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, value: i32,
                                  optionbits: u32) -> bool;
    pub fn xmp_set_property_int64(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, value: i64,
                                  optionbits: u32) -> bool;
    pub fn xmp_set_array_item(xmp: *mut Xmp, schema: *const c_char,
                              name: *const c_char, index: i32,
                              value: *const c_char,
                              optionbits: *mut u32) -> bool;

    pub fn xmp_append_array_item(xmp: *mut Xmp, schema: *const c_char,
                                 name: *const c_char, array_options: u32,
                                 value: *const c_char,
                                 optionbits: u32) -> bool;
    pub fn xmp_delete_property(xmp: *mut Xmp, schema: *const c_char,
                               name: *const c_char) -> bool;
    pub fn xmp_has_property(xmp: *const Xmp, schema: *const c_char,
                            name: *const c_char) -> bool;

    pub fn xmp_get_localized_text(xmp: *const Xmp, schema: *const c_char,
                                 name: *const c_char, gen_lang: *const c_char,
                                 spec_lang: *const c_char,
                                 actual_lang: *mut XmpString,
                                 value: *mut XmpString,
                                 propbits: *mut u32) -> bool;
    pub fn xmp_set_localized_text(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, gen_lang: *const c_char,
                                  spec_lang: *const c_char,
                                  value: *const c_char,
                                  optionbits: u32) -> bool;
    pub fn xmp_delete_localized_text(xmp: *mut Xmp, schema: *const c_char,
                                     name: *const c_char,
                                     gen_lang: *const c_char,
                                     spec_lang: *const c_char) -> bool;

    pub fn xmp_string_new() -> *mut XmpString;
    pub fn xmp_string_free(s: *mut XmpString);
    pub fn xmp_string_cstr(s: *const XmpString) -> *const c_char;

    pub fn xmp_iterator_new() -> *mut XmpIterator;
    pub fn xmp_iterator_free(i: *mut XmpIterator) -> bool;
    pub fn xmp_iterator_next(i: *mut XmpIterator, schema: *mut XmpString,
                             name: *mut XmpString, value: *mut XmpString,
                             option: *mut u32) -> bool;
    pub fn xmp_iterator_skip(i: *mut XmpIterator, option: c_int) -> bool;

    pub fn xmp_datetime_compare(left: *const XmpDateTime,
                                right: *const XmpDateTime) -> c_int;
}

#[test]
fn native_call_works() {
    let inited = unsafe { xmp_init() };

    assert!(inited);

    let xf = unsafe { xmp_files_new() };

    assert!(!xf.is_null());
    assert!(unsafe { xmp_files_free(xf) });
    assert!(unsafe { xmp_get_error() } == 0);

    let xs = unsafe { xmp_string_new() };
    assert!(!xs.is_null());
    let s = unsafe { xmp_string_cstr(xs) };
    assert!(!s.is_null());
    unsafe { xmp_string_free(xs); }

    unsafe { xmp_terminate(); }
}
