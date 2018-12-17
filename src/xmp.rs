extern crate libc;

use super::Result;
use c;
use libc::c_char;
use std::ffi::CString;
use xmpstring::XmpString;
use DateTime;

pub mod flags {
    bitflags! {
        pub flags PropFlags: u32 {
            /// The property has no bit set.
            const PROP_NONE = 0,
            /// The value is a URI, use rdf:resource attribute. DISCOURAGED
            const PROP_VALUE_IS_URI     = 0x0000_0002u32,
        /** Options relating to qualifiers attached to a property. */
            /// The property has qualifiers, includes rdf:type and xml:lang.
        const PROP_HAS_QUALIFIERS   = 0x0000_0010u32,
            /// This is a qualifier, includes rdf:type and xml:lang.
        const PROP_IS_QUALIFIER     = 0x0000_0020u32,
            /// Implies `PROP_HAS_QUALIFIERS`, property has xml:lang.
        const PROP_HAS_LANG         = 0x0000_0040u32,
            /// Implies `PROP_HAS_QUALIFIERS`, property has rdf:type.
        const PROP_HAS_TYPE         = 0x0000_0080u32,

        /* Options relating to the data structure form. */
            /// The value is a structure with nested fields.
        const PROP_VALUE_IS_STRUCT = 0x0000_0100u32,
            /// The value is an array (RDF alt/bag/seq).
        const PROP_VALUE_IS_ARRAY  = 0x0000_0200u32,
            /// The item order does not matter.*/
        const PROP_ARRAY_IS_UNORDERED = PROP_VALUE_IS_ARRAY.bits,
            /// Implies `PROP_VALUE_IS_ARRAY`, item order matters.
        const PROP_ARRAY_IS_ORDERED = 0x0000_0400u32,
            /// Implies `PROP_ARRAY_IS_ORDERED`, items are alternates.
        const PROP_ARRAY_IS_ALT    = 0x0000_0800u32,
        /** Additional struct and array options. */
            /// Implies `PROP_ARRAY_IS_ALT`, items are localized text.
        const PROP_ARRAY_IS_ALTTEXT = 0x0000_1000u32,
            /// Used by array functions.
        const PROP_ARRAY_INSERT_BEFORE = 0x0000_4000u32,
            /// Used by array functions. */
        const PROP_ARRAY_INSERT_AFTER = 0x0000_8000u32,

        /* Other miscellaneous options. */
            /// This property is an alias name for another property.
        const PROP_IS_ALIAS         = 0x0001_0000u32,
            /// This property is the base value for a set of aliases.
        const PROP_HAS_ALIASES      = 0x0002_0000u32,
            /// This property is an "internal" property, owned by applications.
        const PROP_IS_INTERNAL      = 0x0004_0000u32,
            /// This property is not derived from the document content.
        const PROP_IS_STABLE        = 0x0010_0000u32,
            /// This property is derived from the document content.
        const PROP_IS_DERIVED       = 0x0020_0000u32,
        // kXMPUtil_AllowCommas   = 0x10000000u32,  ! Used by TXMPUtils::CatenateArrayItems and ::SeparateArrayItems.
        // kXMP_DeleteExisting    = 0x20000000u32,  ! Used by TXMPMeta::SetXyz functions to delete any pre-existing property.
        // kXMP_SchemaNode        = 0x80000000u32,  ! Returned by iterators - #define to avoid warnings

        /* Masks that are multiple flags. */
        const PROP_ARRAY_FORM_MASK  =
                PROP_VALUE_IS_ARRAY.bits | PROP_ARRAY_IS_ORDERED.bits
                | PROP_ARRAY_IS_ALT.bits | PROP_ARRAY_IS_ALTTEXT.bits,
            /// Is it simple or composite (array or struct)?
        const PROP_COMPOSITE_MASK   = PROP_VALUE_IS_STRUCT.bits
                | PROP_ARRAY_FORM_MASK.bits,
            /// Reserved for transient use by the implementation.
        const IMPL_RESERVED_MASK    = 0x7000_0000u32,

            /// Array is a ordered.
        const ARRAY_IS_ORDERED = 0x0000_0400u32,
            /// Array is alternate values
        const ARRAY_IS_ALT    = 0x0000_0800u32,
            /// Array is alternate text.
        const ARRAY_IS_ALTTEXT = 0x0000_1000u32,

        const ITEM_IS_STRUCT = 0x0000_0100u32,
            /// The value is an array (RDF alt/bag/seq).
        const ITEM_IS_ARRAY  = 0x0000_0200u32,
        }
    }

    bitflags! {
        pub flags SerialFlags: u32 {
            /// Omit the XML packet wrapper.
            const SERIAL_OMITPACKETWRAPPER   = 0x0010u32,
            /// Default is a writeable packet.
        const SERIAL_READONLYPACKET      = 0x0020u32,
            /// Use a compact form of RDF.
        const SERIAL_USECOMPACTFORMAT    = 0x0040u32,
            /// Include a padding allowance for a thumbnail image.
        const SERIAL_INCLUDETHUMBNAILPAD = 0x0100u32,
            /// The padding parameter is the overall packet length.
        const SERIAL_EXACTPACKETLENGTH   = 0x0200u32,
            /// Show aliases as XML comments.
        const SERIAL_WRITEALIASCOMMENTS  = 0x0400u32,
            /// Omit all formatting whitespace.
        const SERIAL_OMITALLFORMATTING   = 0x0800u32,

            /* ! Don't use directly, see the combined values below! */
        const _LITTLEENDIAN_BIT    = 0x0001u32,
        const _UTF16_BIT           = 0x0002u32,
        const _UTF32_BIT           = 0x0004u32,

        const SERIAL_ENCODINGMASK        = 0x0007u32,
            /// Serialize to UTF-8 (default)
        const SERIAL_ENCODEUTF8          = 0u32,
            /// Serialize to UTF-16 BE (big endian)
        const SERIAL_ENCODEUTF16BIG      = _UTF16_BIT.bits,
            /// Serialize to UTF-16 LE (little endian)
        const SERIAL_ENCODEUTF16LITTLE   =
                _UTF16_BIT.bits
                | _LITTLEENDIAN_BIT.bits,
            /// Serialize to UTF-32 BE (big endian)
        const SERIAL_ENCODEUTF32BIG      = _UTF32_BIT.bits,
            /// Serialize to UTF-32 LE (little endian)
        const SERIAL_ENCODEUTF32LITTLE   =
                _UTF32_BIT.bits
                | _LITTLEENDIAN_BIT.bits,
        }
    }
}

use self::flags::*;

pub struct Xmp {
    ptr: *mut c::Xmp,
}

impl Default for Xmp {
    fn default() -> Xmp {
        Xmp {
            ptr: unsafe { c::xmp_new_empty() },
        }
    }
}

impl Xmp {
    /// Construct from a native ptr. Will own it.
    pub fn from(ptr: *mut c::Xmp) -> Xmp {
        Xmp { ptr }
    }
    /// New Xmp object
    pub fn new() -> Xmp {
        Xmp::default()
    }
    /// New Xmp object a byte buffer.
    /// Return None if parsing failed.
    pub fn from_buffer(buf: &[u8]) -> Result<Xmp> {
        let ptr = unsafe { c::xmp_new(buf.as_ptr() as *const c_char, buf.len()) };
        if ptr.is_null() {
            return Err(super::get_error());
        }
        Ok(Xmp::from(ptr))
    }
    /// Parse buff into a Xmp
    pub fn parse(&mut self, buf: &[u8]) -> Result<()> {
        if unsafe { c::xmp_parse(self.ptr, buf.as_ptr() as *const c_char, buf.len()) } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Serialize the Xmp to an XmpString.
    pub fn serialize(&self, options: SerialFlags, padding: u32) -> Result<XmpString> {
        if self.is_null() {
            return Err(super::Error::BadObject);
        }
        let mut buffer = XmpString::new();
        if unsafe { c::xmp_serialize(self.ptr, buffer.as_mut_ptr(), options.bits(), padding) } {
            return Ok(buffer);
        }
        Err(super::get_error())
    }

    /// Serialize the Xmp to an XmpString with some formatting options.
    pub fn serialize_and_format(
        &self,
        options: SerialFlags,
        padding: u32,
        newline: &str,
        tab: &str,
        indent: i32,
    ) -> Result<XmpString> {
        if self.is_null() {
            return Err(super::Error::BadObject);
        }
        let s_newline = CString::new(newline).unwrap();
        let s_tab = CString::new(tab).unwrap();
        let mut buffer = XmpString::new();
        if unsafe {
            c::xmp_serialize_and_format(
                self.ptr,
                buffer.as_mut_ptr(),
                options.bits(),
                padding,
                s_newline.as_ptr(),
                s_tab.as_ptr(),
                indent,
            )
        } {
            return Ok(buffer);
        }
        Err(super::get_error())
    }

    /// Get property as a XmpString.
    pub fn get_property(
        &self,
        schema: &str,
        name: &str,
        propsbits: &mut PropFlags,
    ) -> Result<XmpString> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let mut raw_propsbits = 0u32;
        let mut property = XmpString::new();
        let result = unsafe {
            c::xmp_get_property(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                property.as_mut_ptr(),
                &mut raw_propsbits,
            )
        };
        *propsbits = PropFlags::from_bits(raw_propsbits).unwrap_or_else(PropFlags::empty);
        if result {
            Ok(property)
        } else {
            Err(super::get_error())
        }
    }

    /// Get DateTime property.
    pub fn get_property_date(
        &self,
        schema: &str,
        name: &str,
        propsbits: &mut PropFlags,
    ) -> Result<DateTime> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let mut raw_propsbits = 0u32;
        let mut property = DateTime::new();
        let result = unsafe {
            c::xmp_get_property_date(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                property.as_mut_ptr(),
                &mut raw_propsbits,
            )
        };
        *propsbits = PropFlags::from_bits(raw_propsbits).unwrap_or_else(PropFlags::empty);
        if result {
            Ok(property)
        } else {
            Err(super::get_error())
        }
    }

    /// Get float property
    pub fn get_property_float(
        &self,
        schema: &str,
        name: &str,
        propsbits: &mut PropFlags,
    ) -> Result<f64> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let mut raw_propsbits = 0u32;
        let mut property = 0f64;
        let result = unsafe {
            c::xmp_get_property_float(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                &mut property as *mut f64,
                &mut raw_propsbits,
            )
        };
        *propsbits = PropFlags::from_bits(raw_propsbits).unwrap_or_else(PropFlags::empty);
        if result {
            Ok(property)
        } else {
            Err(super::get_error())
        }
    }

    /// Get bool property
    pub fn get_property_bool(
        &self,
        schema: &str,
        name: &str,
        propsbits: &mut PropFlags,
    ) -> Result<bool> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let mut raw_propsbits = 0u32;
        let mut property = false;
        let result = unsafe {
            c::xmp_get_property_bool(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                &mut property as *mut bool,
                &mut raw_propsbits,
            )
        };
        *propsbits = PropFlags::from_bits(raw_propsbits).unwrap_or_else(PropFlags::empty);
        if result {
            Ok(property)
        } else {
            Err(super::get_error())
        }
    }

    /// Get i32 property
    pub fn get_property_i32(
        &self,
        schema: &str,
        name: &str,
        propsbits: &mut PropFlags,
    ) -> Result<i32> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let mut raw_propsbits = 0u32;
        let mut property = 0i32;
        let result = unsafe {
            c::xmp_get_property_int32(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                &mut property as *mut i32,
                &mut raw_propsbits,
            )
        };
        *propsbits = PropFlags::from_bits(raw_propsbits).unwrap_or_else(PropFlags::empty);
        if result {
            Ok(property)
        } else {
            Err(super::get_error())
        }
    }

    /// Get i64 property
    pub fn get_property_i64(
        &self,
        schema: &str,
        name: &str,
        propsbits: &mut PropFlags,
    ) -> Result<i64> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let mut raw_propsbits = 0u32;
        let mut property = 0i64;
        let result = unsafe {
            c::xmp_get_property_int64(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                &mut property as *mut i64,
                &mut raw_propsbits,
            )
        };
        *propsbits = PropFlags::from_bits(raw_propsbits).unwrap_or_else(PropFlags::empty);
        if result {
            Ok(property)
        } else {
            Err(super::get_error())
        }
    }

    /// Get array item property
    pub fn get_array_item(
        &self,
        schema: &str,
        name: &str,
        index: i32,
        propsbits: &mut PropFlags,
    ) -> Result<XmpString> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let mut raw_propsbits = 0u32;
        let mut property = XmpString::new();
        let result = unsafe {
            c::xmp_get_array_item(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                index,
                property.as_mut_ptr(),
                &mut raw_propsbits,
            )
        };
        *propsbits = PropFlags::from_bits(raw_propsbits).unwrap_or_else(PropFlags::empty);
        if result {
            Ok(property)
        } else {
            Err(super::get_error())
        }
    }

    /// Set a string property value
    pub fn set_property(
        &mut self,
        schema: &str,
        name: &str,
        value: &str,
        optionbits: PropFlags,
    ) -> Result<()> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let s_value = CString::new(value).unwrap();
        if unsafe {
            c::xmp_set_property(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                s_value.as_ptr(),
                optionbits.bits(),
            )
        } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Set a date property value
    pub fn set_property_date(
        &mut self,
        schema: &str,
        name: &str,
        value: &DateTime,
        optionbits: PropFlags,
    ) -> Result<()> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        if unsafe {
            c::xmp_set_property_date(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                value.as_ptr(),
                optionbits.bits(),
            )
        } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Set a float property value
    pub fn set_property_float(
        &mut self,
        schema: &str,
        name: &str,
        value: f64,
        optionbits: PropFlags,
    ) -> Result<()> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        if unsafe {
            c::xmp_set_property_float(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                value,
                optionbits.bits(),
            )
        } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Set a bool property value
    pub fn set_property_bool(
        &mut self,
        schema: &str,
        name: &str,
        value: bool,
        optionbits: PropFlags,
    ) -> Result<()> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        if unsafe {
            c::xmp_set_property_bool(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                value,
                optionbits.bits(),
            )
        } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Set an i32 property value
    pub fn set_property_i32(
        &mut self,
        schema: &str,
        name: &str,
        value: i32,
        optionbits: PropFlags,
    ) -> Result<()> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        if unsafe {
            c::xmp_set_property_int32(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                value,
                optionbits.bits(),
            )
        } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Set an i64 property value
    pub fn set_property_i64(
        &mut self,
        schema: &str,
        name: &str,
        value: i64,
        optionbits: PropFlags,
    ) -> Result<()> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        if unsafe {
            c::xmp_set_property_int64(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                value,
                optionbits.bits(),
            )
        } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Set an array item
    pub fn set_array_item(
        &mut self,
        schema: &str,
        name: &str,
        index: i32,
        value: &str,
        item_options: PropFlags,
    ) -> Result<()> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let s_value = CString::new(value).unwrap();
        if unsafe {
            c::xmp_set_array_item(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                index,
                s_value.as_ptr(),
                item_options.bits(),
            )
        } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    // XXX figure out the array options...
    /// Append an array item.
    pub fn append_array_item(
        &mut self,
        schema: &str,
        name: &str,
        array_options: PropFlags,
        value: &str,
        item_options: PropFlags,
    ) -> Result<()> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let s_value = CString::new(value).unwrap();
        if unsafe {
            c::xmp_append_array_item(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                array_options.bits(),
                s_value.as_ptr(),
                item_options.bits(),
            )
        } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Delete a property
    pub fn delete_property(&mut self, schema: &str, name: &str) -> Result<()> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        if unsafe { c::xmp_delete_property(self.ptr, s_schema.as_ptr(), s_name.as_ptr()) } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Has a property
    pub fn has_property(&self, schema: &str, name: &str) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe { c::xmp_has_property(self.ptr, s_schema.as_ptr(), s_name.as_ptr()) }
    }

    /// Get localized text.
    pub fn get_localized_text(
        &self,
        schema: &str,
        name: &str,
        gen_lang: &str,
        spec_lang: &str,
        propsbits: &mut PropFlags,
    ) -> Result<(XmpString, XmpString)> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let s_gen_lang = CString::new(gen_lang).unwrap();
        let s_spec_lang = CString::new(spec_lang).unwrap();

        let mut actual_lang = XmpString::new();
        let mut value = XmpString::new();

        let mut raw_propsbits = 0u32;
        let result = unsafe {
            c::xmp_get_localized_text(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                s_gen_lang.as_ptr(),
                s_spec_lang.as_ptr(),
                actual_lang.as_mut_ptr(),
                value.as_mut_ptr(),
                &mut raw_propsbits,
            )
        };
        *propsbits = PropFlags::from_bits(raw_propsbits).unwrap_or_else(PropFlags::empty);
        if result {
            Ok((actual_lang, value))
        } else {
            Err(super::get_error())
        }
    }

    /// Set localized text.
    pub fn set_localized_text(
        &mut self,
        schema: &str,
        name: &str,
        gen_lang: &str,
        spec_lang: &str,
        value: &str,
        propbits: PropFlags,
    ) -> Result<()> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let s_gen_lang = CString::new(gen_lang).unwrap();
        let s_spec_lang = CString::new(spec_lang).unwrap();
        let s_value = CString::new(value).unwrap();
        if unsafe {
            c::xmp_set_localized_text(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                s_gen_lang.as_ptr(),
                s_spec_lang.as_ptr(),
                s_value.as_ptr(),
                propbits.bits(),
            )
        } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Delete localize text.
    pub fn delete_localized_text(
        &mut self,
        schema: &str,
        name: &str,
        gen_lang: &str,
        spec_lang: &str,
    ) -> Result<()> {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let s_gen_lang = CString::new(gen_lang).unwrap();
        let s_spec_lang = CString::new(spec_lang).unwrap();
        if unsafe {
            c::xmp_delete_localized_text(
                self.ptr,
                s_schema.as_ptr(),
                s_name.as_ptr(),
                s_gen_lang.as_ptr(),
                s_spec_lang.as_ptr(),
            )
        } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Return if the native pointer is null.
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    /// Return the native pointer.
    pub fn as_ptr(&self) -> *const c::Xmp {
        self.ptr
    }

    /// Return the mutable native pointer.
    pub fn as_mut_ptr(&mut self) -> *mut c::Xmp {
        self.ptr
    }
}

impl Clone for Xmp {
    fn clone(&self) -> Self {
        if self.is_null() {
            // inside ptr is NULL. cloning a null object.
            return Xmp::from(self.ptr);
        }
        Xmp::from(unsafe { c::xmp_copy(self.ptr) })
    }
}

impl Drop for Xmp {
    /// Will release the Xmp native pointer on Drop.
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { c::xmp_free(self.ptr) };
        }
    }
}
