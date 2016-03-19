
#[link(name = "exempi")]
extern crate libc;

use libc::{c_int, c_char, size_t};
use std::ops::BitOr;
use std::mem::transmute;

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

#[derive(Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum OpenFlags {
    /// No open option
    None = 0x00000000,
    /// Open for read-only access.
    Read = 0x00000001,
    /// Open for reading and writing.
    ForUpdate = 0x00000002,
    /// Only the XMP is wanted, allows space/time optimizations.
    OnlyXmp = 0x00000004,
    /// Cache thumbnail if possible, GetThumbnail will be called.
    CacheThumbnail = 0x00000008,
    /// Be strict about locating XMP and reconciling with other forms.
    Strictly = 0x00000010,
    /// Require the use of a smart handler.
    UseSmartHandler = 0x00000020,
    /// Force packet scanning, don't use a smart handler.
    UsePacketScanning = 0x00000040,
    /// Only packet scan files "known" to need scanning.
    LimitedScanning = 0x00000080,
    /// Attempt to repair a file opened for update, default is to not open (throw an exception).
    RepairFile = 0x00000100,
    /// Optimize MPEG4 to support stream when updating This can take some time
    OptimizeFileLayout = 0x00000200,
    /// Set if calling from background thread.
    InBackground = 0x10000000,
}

impl BitOr for OpenFlags {
    type Output = OpenFlags;
    fn bitor(self, rhs: OpenFlags) -> Self::Output {
        unsafe { transmute((self as u32) | (rhs as u32)) }
    }
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

#[derive(Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum FormatOptionBits {
    CanInjectXMP = 0x00000001,
    CanExpand = 0x00000002,
    CanRewrite = 0x00000004,
    PrefersInPlace = 0x00000008,
    CanReconcile = 0x00000010,
    AllowsOnlyXMP = 0x00000020,
    ReturnsRawPacket = 0x00000040,
    HandlerOwnsFile = 0x00000100,
    AllowSafeUpdate = 0x00000200,
    NeedsReadOnlyPacket = 0x00000400,
    UseSidecarXMP = 0x00000800,
    FolderBasedFormat = 0x00001000,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum CloseFlags {
    /// No close option
    None = 0x0000,
    /// Write into a temporary file and swap for crash safety.
    SafeUpdate = 0x0001,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u32)]
///The bits passed to the properties getter and setter.
pub enum PropBits {
    None = 0,
    /** Options relating to the XML string form of the property value. */
    /// The value is a URI, use rdf:resource attribute. DISCOURAGED
    ValueIsUri = 0x00000002u32,
    /** Options relating to qualifiers attached to a property. */
    /// The property has qualifiers, includes rdf:type and xml:lang.
    HasQualifiers = 0x00000010u32,
    /// This is a qualifier, includes rdf:type and xml:lang.
    IsQualifier = 0x00000020u32,
    /// Implies XMP_PropHasQualifiers, property has xml:lang.
    HasLang = 0x00000040u32,
    /// Implies XMP_PropHasQualifiers, property has rdf:type.
    HasType = 0x00000080u32,

    /* Options relating to the data structure form. */
    /// The value is a structure with nested fields.
    IsStruct = 0x00000100u32,
    /// The value is an array(RDF alt/bag/seq).
    /// Is an array, the item order does not matter.
    /// Default for arrays.
    IsArrayUnordered = 0x00000200u32,
    /// Is an array, the item order matters.
    IsArrayOrdered = 0x00000400u32,
    /// Implies IsArrayOrdered, items are alternates.
    IsArrayAlt = 0x00000800u32,
    /// Implies IsArrayAlt, items are localized text.
    IsArrayAltText = 0x00001000u32,

    /** Additional struct and array options. */
    /// Used by array functions.
    ArrayInsertBefore = 0x00004000u32,
    /// Used by array functions.
    ArrayInsertAfter = 0x00008000u32,

    /* Other miscellaneous options. */
    /// This property is an alias name for another property.
    IsAlias = 0x00010000u32,
    /// This property is the base value for a set of aliases.
    HasAliases = 0x00020000u32,
    /// This property is an "internal" property, owned by applications.
    IsInternal = 0x00040000u32,
    /// This property is not derived from the document content.
    IsStable = 0x00100000u32,
    /// This property is derived from the document content.
    IsDerived = 0x00200000u32,
    /* kXMPUtil_AllowCommas   = 0x10000000u32,  ! Used by
       TXMPUtils::CatenateArrayItems and ::SeparateArrayItems. */
    /* kXMP_DeleteExisting    = 0x20000000u32,  ! Used by TXMPMeta::SetXyz
       functions to delete any pre-existing property. */
    /* kXMP_SchemaNode        = 0x80000000u32,  ! Returned by iterators - #define
       to avoid warnings */

    /// Bit mask for "is array"
    ArrayMask = 0x1e00u32,
    /// Bit mask for is struct or arry
    CompositeMask = 0x1f00u32,

    /// Reserved for transient use by the implementation.
    ImplReservedMask = 0x70000000u32,
}

impl BitOr for PropBits {
    type Output = PropBits;
    fn bitor(self, rhs: PropBits) -> Self::Output {
        unsafe { transmute((self as u32) | (rhs as u32)) }
    }
}

#[derive(Clone, Copy)]
#[repr(u32)]
/// Option bits for XmpIterator::next
pub enum IterBits {
    /// The low 8 bits are an enum of what data structure to iterate.
    ClassMask = 0x00FFu32,
    /// Iterate the property tree of a Xmp object.
    Properties = 0x0000u32,
    /// Iterate the global alias table.
    Aliases = 0x0001u32,
    /// Iterate the global namespace table.
    Namespaces = 0x0002u32,
    /// Just do the immediate children of the root, default is subtree.
    JustChildren = 0x0100u32,
    /// Just do the leaf nodes, default is all nodes in the subtree.
    JustLeafNodes = 0x0200u32,
    /// Return just the leaf part of the path, default is the full path.
    JustLeafName = 0x0400u32,
    /// Include aliases, default is justactual properties.
    IncludeAliases = 0x0800u32,
    /// Omit all qualifiers.
    OmitQualifiers = 0x1000u32,
}

impl BitOr for IterBits {
    type Output = IterBits;
    fn bitor(self, rhs: IterBits) -> Self::Output {
        unsafe { transmute((self as u32) | (rhs as u32)) }
    }
}

#[derive(Clone, Copy)]
#[repr(u32)]
/// Option bits to XmpIterator::skip. Combine them with |
pub enum IterSkipBits {
    None = 0,
    /// Skip the subtree below the current node.
    SkipSubtree = 0x0001u32,
    /// Skip the subtree below and remaining siblings of the current node.
    SkipSiblings = 0x0002u32,
}

impl BitOr for IterSkipBits {
    type Output = IterSkipBits;
    fn bitor(self, rhs: IterSkipBits) -> Self::Output {
        unsafe { transmute((self as u32) | (rhs as u32)) }
    }
}

#[derive(Clone, Copy)]
#[repr(u32)]
/// Option bits to pass to Xmp::serialize functions.
/// You can combine them with |
pub enum SerializeBits {
    /* Options for xmp_serialize */
    /// Omit the XML packet wrapper.
    OmitPacketWrapper = 0x0010u32,
    /// Default is a writeable packet.
    ReadOnlyPacket = 0x0020u32,
    /// Use a compact form of RDF.
    UseCompactFormat = 0x0040u32,
    /// Include a padding allowance for a thumbnail image.
    IncludeThumnailPad = 0x0100u32,
    /// The padding parameter is the overall packet length.
    ExactPacketLenght = 0x0200u32,
    /// Show aliases as XML comments.
    WriateAliasComments = 0x0400u32,
    /// Omit all formatting whitespace. */
    OmitAllFormatting = 0x0800u32,

    //    XMP_SERIAL_ENCODINGMASK = 0x0007u32,
    /// Default: UTF8 encoded
    EncodeUTF8 = 0u32,
    /// UTF16 BE
    EncodeUTF16Big = 0x0002u32,
    /// UTF16 LE
    EncodeUTF16Little = 0x0002u32 + 0x0001u32,
    /// UTF32 BE
    EncodeUTF32Big = 0x0004u32,
    /// UTF32 LE
    EncodeUTF32Little = 0x0004u32 + 0x0001u32,
}

impl BitOr for SerializeBits {
    type Output = SerializeBits;
    fn bitor(self, rhs: SerializeBits) -> Self::Output {
        unsafe { transmute((self as u32) | (rhs as u32)) }
    }
}


extern "C" {
    pub fn xmp_init() -> bool;
    pub fn xmp_terminate();

    pub fn xmp_get_error() -> c_int;

    pub fn xmp_files_new() -> *mut XmpFile;
    pub fn xmp_files_open_new(p: *const c_char,
                              options: OpenFlags) -> *mut XmpFile;
    pub fn xmp_files_open(xf: *mut XmpFile, p:  *const c_char,
                          options: OpenFlags) -> bool;
    pub fn xmp_files_close(xf: *mut XmpFile, options: CloseFlags) -> bool;

    pub fn xmp_files_get_new_xmp(xf: *mut XmpFile) -> *mut Xmp;
    pub fn xmp_files_get_xmp(xf: *mut XmpFile, xmp: *mut Xmp) -> bool;
    pub fn xmp_files_can_put_xmp(xf: *mut XmpFile, xmp: *const Xmp) -> bool;
    pub fn xmp_files_put_xmp(xf: *mut XmpFile, xmp: *const Xmp) -> bool;

    pub fn xmp_files_free(xf: *mut XmpFile) -> bool;

    pub fn xmp_files_get_file_info(xf: *mut XmpFile, fp: *mut XmpString,
                                   options: *mut OpenFlags,
                                   format: *mut FileType,
                                   handler_flags: *mut FormatOptionBits)
                                   -> bool;
    pub fn xmp_files_check_file_format(path: *const c_char) -> FileType;

    pub fn xmp_files_get_format_info(format: FileType,
                                     options: *mut FormatOptionBits) -> bool;

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
                         options: SerializeBits, padding: u32) -> bool;
    pub fn xmp_serialize_and_format(xmp: *const Xmp, buf: *mut XmpString,
                                    options: SerializeBits, padding: u32,
                                    newline: *const c_char, tab: *const c_char,
                                    indent: i32) -> bool;

    // get properties
    pub fn xmp_get_property(xmp: *const Xmp, schema: *const c_char,
                            name: *const c_char, property: *mut XmpString,
                            propsbits: *mut PropBits) -> bool;
    pub fn xmp_get_property_date(xmp: *const Xmp, schema: *const c_char,
                                 name: *const c_char,
                                 property: *mut XmpDateTime,
                                 propsbits: *mut PropBits) -> bool;
    pub fn xmp_get_property_float(xmp: *const Xmp, schema: *const c_char,
                                  name: *const c_char,
                                  property: *mut f64,
                                  propsbits: *mut PropBits) -> bool;
    pub fn xmp_get_property_bool(xmp: *const Xmp, schema: *const c_char,
                                  name: *const c_char,
                                  property: *mut bool,
                                  propsbits: *mut PropBits) -> bool;
    pub fn xmp_get_property_int32(xmp: *const Xmp, schema: *const c_char,
                                  name: *const c_char,
                                  property: *mut i32,
                                  propsbits: *mut PropBits) -> bool;
    pub fn xmp_get_property_int64(xmp: *const Xmp, schema: *const c_char,
                                  name: *const c_char,
                                  property: *mut i64,
                                  propsbits: *mut PropBits) -> bool;
    pub fn xmp_get_array_item(xmp: *const Xmp,  schema: *const c_char,
                              name: *const c_char, index: i32,
                              property: *mut XmpString,
                              propsbits: *mut PropBits) -> bool;

    // set properies
    pub fn xmp_set_property(xmp: *mut Xmp, schema: *const c_char,
                            name: *const c_char, value: *const c_char,
                            optionbits: PropBits) -> bool;
    pub fn xmp_set_property_date(xmp: *mut Xmp, schema: *const c_char,
                            name: *const c_char, value: *const XmpDateTime,
                            optionbits: PropBits) -> bool;
    pub fn xmp_set_property_float(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, value: f64,
                                  optionbits: PropBits) -> bool;
    pub fn xmp_set_property_bool(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, value: bool,
                                  optionbits: PropBits) -> bool;
    pub fn xmp_set_property_int32(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, value: i32,
                                  optionbits: PropBits) -> bool;
    pub fn xmp_set_property_int64(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, value: i64,
                                  optionbits: PropBits) -> bool;
    pub fn xmp_set_array_item(xmp: *mut Xmp, schema: *const c_char,
                              name: *const c_char, index: i32,
                              value: *const c_char,
                              optionbits: PropBits) -> bool;

    pub fn xmp_append_array_item(xmp: *mut Xmp, schema: *const c_char,
                                 name: *const c_char, array_options: u32,
                                 value: *const c_char,
                                 optionbits: PropBits) -> bool;
    pub fn xmp_delete_property(xmp: *mut Xmp, schema: *const c_char,
                               name: *const c_char) -> bool;
    pub fn xmp_has_property(xmp: *const Xmp, schema: *const c_char,
                            name: *const c_char) -> bool;

    pub fn xmp_get_localized_text(xmp: *const Xmp, schema: *const c_char,
                                 name: *const c_char, gen_lang: *const c_char,
                                 spec_lang: *const c_char,
                                 actual_lang: *mut XmpString,
                                 value: *mut XmpString,
                                 propbits: *mut PropBits) -> bool;
    pub fn xmp_set_localized_text(xmp: *mut Xmp, schema: *const c_char,
                                  name: *const c_char, gen_lang: *const c_char,
                                  spec_lang: *const c_char,
                                  value: *const c_char,
                                  optionbits: PropBits) -> bool;
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
                             option: *mut IterBits) -> bool;
    pub fn xmp_iterator_skip(i: *mut XmpIterator, option: IterSkipBits) -> bool;

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
