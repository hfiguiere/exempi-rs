
#[link(name = "exempi")]
extern crate libc;

use libc::{c_int, c_char, size_t};

pub enum Xmp {}
pub enum XmpFile {}
pub enum XmpString {}
pub enum XmpIterator {}

#[derive(Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum TzSign {
    /// West of UTC
    West = -1,
    /// UTC
    UTC = 0,
    /// East of UTC
    East = 1,
}

// the C defined struct.
#[repr(C)]
pub struct XmpDateTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
    pub tz_sign: TzSign,
    pub tz_hour: i32,
    pub tz_minute: i32,
    pub nano_second: i32,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u32)]
/// Public file formats.
pub enum FileType {
    PDF = 0x50444620u32, /* 'PDF ' */
    PS = 0x50532020u32,  /* 'PS  ', general PostScript following DSC
                                  conventions. */
    EPS = 0x45505320u32, /* 'EPS ', encapsulated PostScript. */

    JPEG = 0x4A504547u32,   /* 'JPEG' */
    JPEG2K = 0x4A505820u32, /* 'JPX ', ISO 15444-1 */
    TIFF = 0x54494646u32,   /* 'TIFF' */
    GIF = 0x47494620u32,    /* 'GIF ' */
    PNG = 0x504E4720u32,    /* 'PNG ' */

    SWF = 0x53574620u32, /* 'SWF ' */
    FLA = 0x464C4120u32, /* 'FLA ' */
    FLV = 0x464C5620u32, /* 'FLV ' */

    MOV = 0x4D4F5620u32,   /* 'MOV ', Quicktime */
    AVI = 0x41564920u32,   /* 'AVI ' */
    CIN = 0x43494E20u32,   /* 'CIN ', Cineon */
    WAV = 0x57415620u32,   /* 'WAV ' */
    MP3 = 0x4D503320u32,   /* 'MP3 ' */
    SES = 0x53455320u32,   /* 'SES ', Audition session */
    CEL = 0x43454C20u32,   /* 'CEL ', Audition loop */
    MPEG = 0x4D504547u32,  /* 'MPEG' */
    MPEG2 = 0x4D503220u32, /* 'MP2 ' */
    MPEG4 = 0x4D503420u32, /* 'MP4 ', ISO 14494-12 and -14 */
    WMAV = 0x574D4156u32,  /* 'WMAV', Windows Media Audio and Video */
    AIFF = 0x41494646u32,  /* 'AIFF' */

    HTML = 0x48544D4Cu32, /* 'HTML' */
    XML = 0x584D4C20u32,  /* 'XML ' */
    TEXT = 0x74657874u32, /* 'text' */

    /* Adobe application file formats. */
    Photoshop = 0x50534420u32,   /* 'PSD ' */
    Illustrator = 0x41492020u32, /* 'AI  ' */
    InDesign = 0x494E4444u32,    /* 'INDD' */
    AEProject = 0x41455020u32,   /* 'AEP ' */
    AEProjTemplate =
        0x41455420u32, /* 'AET ', After Effects Project Template */
    AEFilterPreset = 0x46465820u32,  /* 'FFX ' */
    EncoreProject = 0x4E434F52u32,   /* 'NCOR' */
    PremiereProject = 0x5052504Au32, /* 'PRPJ' */
    PremiereTitle = 0x5052544Cu32,   /* 'PRTL' */
    UCFFile = 0x55434620u32,         /* 'UCF ' */
    /* Catch all. */
    Unknown = 0x20202020u32 /* '    ' */
}

extern "C" {
    pub fn xmp_init() -> bool;
    pub fn xmp_terminate();

    pub fn xmp_get_error() -> c_int;

    pub fn xmp_files_new() -> *mut XmpFile;
    pub fn xmp_files_open_new(p: *const c_char,
                              options: u32 /*OpenFlags*/) -> *mut XmpFile;
    pub fn xmp_files_open(xf: *mut XmpFile, p:  *const c_char,
                          options: u32 /*OpenFlags*/) -> bool;
    pub fn xmp_files_close(xf: *mut XmpFile,
                           options: u32 /*CloseFlags*/) -> bool;

    pub fn xmp_files_get_new_xmp(xf: *mut XmpFile) -> *mut Xmp;
    pub fn xmp_files_get_xmp(xf: *mut XmpFile, xmp: *mut Xmp) -> bool;
    pub fn xmp_files_can_put_xmp(xf: *mut XmpFile, xmp: *const Xmp) -> bool;
    pub fn xmp_files_put_xmp(xf: *mut XmpFile, xmp: *const Xmp) -> bool;

    pub fn xmp_files_free(xf: *mut XmpFile) -> bool;

    pub fn xmp_files_get_file_info(xf: *mut XmpFile, fp: *mut XmpString,
                                   options: *mut u32, /*OpenFlags*/
                                   format: *mut FileType,
                                   handler_flags: *mut u32 /*FormatOptionFlags*/)
                                   -> bool;
    pub fn xmp_files_check_file_format(path: *const c_char) -> FileType;

    pub fn xmp_files_get_format_info(format: FileType,
                                     options: *mut u32 /*FormatOptionFlags*/)
                                     -> bool;

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
    pub fn xmp_serialize(xmp: *const Xmp, buf: *mut XmpString,
                         options: u32 /*SerialFlags*/, padding: u32) -> bool;
    pub fn xmp_serialize_and_format(xmp: *const Xmp, buf: *mut XmpString,
                                    options: u32 /*SerialFlags*/,
                                    padding: u32, newline: *const c_char,
                                    tab: *const c_char, indent: i32) -> bool;

    // get properties
    pub fn xmp_get_property(xmp: *const Xmp, schema: *const c_char,
                            name: *const c_char, property: *mut XmpString,
                            propsbits: *mut u32 /*PropFlags*/) -> bool;
    pub fn xmp_get_property_date(xmp: *const Xmp, schema: *const c_char,
                                 name: *const c_char,
                                 property: *mut XmpDateTime,
                                 propsbits: *mut u32 /*PropFlags*/) -> bool;
    pub fn xmp_get_property_float(xmp: *const Xmp, schema: *const c_char,
                                  name: *const c_char,
                                  property: *mut f64,
                                  propsbits: *mut u32 /*PropFlags*/) -> bool;
    pub fn xmp_get_property_bool(xmp: *const Xmp, schema: *const c_char,
                                  name: *const c_char,
                                  property: *mut bool,
                                  propsbits: *mut u32 /*PropFlags*/) -> bool;
    pub fn xmp_get_property_int32(xmp: *const Xmp, schema: *const c_char,
                                  name: *const c_char,
                                  property: *mut i32,
                                  propsbits: *mut u32 /*PropFlags*/) -> bool;
    pub fn xmp_get_property_int64(xmp: *const Xmp, schema: *const c_char,
                                  name: *const c_char,
                                  property: *mut i64,
                                  propsbits: *mut u32 /*PropFlags*/) -> bool;
    pub fn xmp_get_array_item(xmp: *const Xmp,  schema: *const c_char,
                              name: *const c_char, index: i32,
                              property: *mut XmpString,
                              propsbits: *mut u32 /*PropFlags*/) -> bool;

    // set properies
    pub fn xmp_set_property(xmp: *mut Xmp, schema: *const c_char,
                            name: *const c_char, value: *const c_char,
                            optionbits: u32 /*PropFlags*/) -> bool;
    pub fn xmp_set_property_date(xmp: *mut Xmp, schema: *const c_char,
                            name: *const c_char, value: *const XmpDateTime,
                            optionbits: u32 /*PropFlags*/) -> bool;
    pub fn xmp_set_property_float(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, value: f64,
                                  optionbits: u32 /*PropFlags*/) -> bool;
    pub fn xmp_set_property_bool(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, value: bool,
                                  optionbits: u32 /*PropFlags*/) -> bool;
    pub fn xmp_set_property_int32(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, value: i32,
                                  optionbits: u32 /*PropFlags*/) -> bool;
    pub fn xmp_set_property_int64(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, value: i64,
                                  optionbits: u32 /*PropFlags*/) -> bool;
    pub fn xmp_set_array_item(xmp: *mut Xmp, schema: *const c_char,
                              name: *const c_char, index: i32,
                              value: *const c_char,
                              optionbits: u32 /*PropFlags*/) -> bool;

    pub fn xmp_append_array_item(xmp: *mut Xmp, schema: *const c_char,
                                 name: *const c_char, array_options: u32,
                                 value: *const c_char,
                                 optionbits: u32 /*PropFlags*/) -> bool;
    pub fn xmp_delete_property(xmp: *mut Xmp, schema: *const c_char,
                               name: *const c_char) -> bool;
    pub fn xmp_has_property(xmp: *const Xmp, schema: *const c_char,
                            name: *const c_char) -> bool;

    pub fn xmp_get_localized_text(xmp: *const Xmp, schema: *const c_char,
                                 name: *const c_char, gen_lang: *const c_char,
                                 spec_lang: *const c_char,
                                 actual_lang: *mut XmpString,
                                 value: *mut XmpString,
                                 propbits: *mut u32 /*PropFlags*/) -> bool;
    pub fn xmp_set_localized_text(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, gen_lang: *const c_char,
                                  spec_lang: *const c_char,
                                  value: *const c_char,
                                  optionbits: u32 /*PropFlags*/) -> bool;
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
                             option: *mut u32 /*IterFlags*/) -> bool;
    pub fn xmp_iterator_skip(i: *mut XmpIterator,
                             option: u32 /*IterSkipFlags*/) -> bool;

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