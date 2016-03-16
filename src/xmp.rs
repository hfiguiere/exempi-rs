extern crate libc;

use ::c;
use self::libc::c_char;
use ::xmpstring::XmpString;
use ::XmpDateTime;
use std::ffi::{CString};

pub struct Xmp {
    ptr: *mut c::Xmp
}

impl Xmp {
    /// Construct from a native ptr. Will own it.
    pub fn from(ptr: *mut c::Xmp) -> Xmp {
        Xmp { ptr: ptr }
    }
    /// New Xmp object
    pub fn new() -> Xmp {
        Xmp { ptr: unsafe { c::xmp_new_empty() } }
    }
    /// New Xmp object a byte buffer.
    /// Return None if parsing failed.
    pub fn from_buffer(buf: &[u8]) -> Option<Xmp> {
        let ptr = unsafe {
            c::xmp_new(buf.as_ptr() as *const c_char, buf.len())
        };
        if ptr.is_null() {
            return None;
        }
        Some(Xmp::from(ptr))
    }
    /// Parse buff into a Xmp
    pub fn parse(&mut self, buf: &[u8]) -> bool {
        unsafe {
            c::xmp_parse(self.ptr,
                         buf.as_ptr() as *const c_char,
                         buf.len())
        }
    }
    pub fn serialize(&self, buffer: &mut XmpString,
                     options: i32, padding: u32) -> bool {
        if self.is_null() || buffer.is_null() {
            return false;
        }
        unsafe { c::xmp_serialize(self.ptr,
                                  buffer.as_mut_ptr(), options, padding) }
    }
    pub fn serialize_and_format(&self, buffer: &mut XmpString, options: u32,
                                padding: u32, newline: &str, tab: &str,
                                indent: i32) -> bool {
        if self.is_null() || buffer.is_null() {
            return false;
        }
        let s_newline = CString::new(newline).unwrap();
        let s_tab = CString::new(tab).unwrap();
        unsafe { c::xmp_serialize_and_format(self.ptr, buffer.as_mut_ptr(),
                                             options, padding,
                                             s_newline.as_ptr(), s_tab.as_ptr(),
                                             indent) }
    }

    pub fn get_property(&self, schema: &str, name: &str,
                        property: &mut XmpString, propsbits: &mut u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe {
            c::xmp_get_property(self.ptr, s_schema.as_ptr(), s_name.as_ptr(),
                                property.as_mut_ptr(), propsbits as *mut u32)
        }
    }

    pub fn get_property_date(&self, schema: &str, name: &str,
                             property: &mut XmpDateTime,
                             propsbits: &mut u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe {
            c::xmp_get_property_date(self.ptr, s_schema.as_ptr(),
                                     s_name.as_ptr(),
                                     property.as_mut_ptr(),
                                     propsbits as *mut u32)
        }
    }

    pub fn get_property_float(&self, schema: &str, name: &str,
                              property: &mut f64, propsbits: &mut u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe {
            c::xmp_get_property_float(self.ptr, s_schema.as_ptr(),
                                      s_name.as_ptr(),
                                      property as *mut f64,
                                      propsbits as *mut u32)
        }
    }

    pub fn get_property_bool(&self, schema: &str, name: &str,
                             property: &mut bool, propsbits: &mut u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe {
            c::xmp_get_property_bool(self.ptr, s_schema.as_ptr(),
                                      s_name.as_ptr(),
                                      property as *mut bool,
                                      propsbits as *mut u32)
        }
    }
    pub fn get_property_int32(&self, schema: &str, name: &str,
                              property: &mut i32, propsbits: &mut u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe {
            c::xmp_get_property_int32(self.ptr, s_schema.as_ptr(),
                                      s_name.as_ptr(),
                                      property as *mut i32,
                                      propsbits as *mut u32)
        }
    }
    pub fn get_property_int64(&self, schema: &str, name: &str,
                              property: &mut i64, propsbits: &mut u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe {
            c::xmp_get_property_int64(self.ptr, s_schema.as_ptr(),
                                      s_name.as_ptr(),
                                      property as *mut i64,
                                      propsbits as *mut u32)
        }
    }
    pub fn get_array_item(&self, schema: &str, name: &str, index: i32,
                                   property: &mut XmpString,
                                   propsbits: &mut u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe {
            c::xmp_get_array_item(self.ptr, s_schema.as_ptr(),
                                           s_name.as_ptr(), index,
                                           property.as_mut_ptr(),
                                           propsbits as *mut u32)
        }
    }

    pub fn set_property(&mut self, schema: &str, name: &str,
                        value: &str, optionbits: u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let s_value = CString::new(value).unwrap();
        unsafe {
            c::xmp_set_property(self.ptr, s_schema.as_ptr(),
                                s_name.as_ptr(), s_value.as_ptr(),
                                optionbits)
        }
    }
    pub fn set_property_date(&mut self, schema: &str, name: &str,
                             value: &XmpDateTime, optionbits: u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe {
            c::xmp_set_property_date(self.ptr, s_schema.as_ptr(),
                                     s_name.as_ptr(),
                                     value.as_ptr(),
                                     optionbits)
        }
    }
    pub fn set_property_float(&mut self, schema: &str, name: &str,
                              value: f64, optionbits: u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe {
            c::xmp_set_property_float(self.ptr, s_schema.as_ptr(),
                                      s_name.as_ptr(), value, optionbits)
        }
    }
    pub fn set_property_bool(&mut self, schema: &str, name: &str,
                              value: bool, optionbits: u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe {
            c::xmp_set_property_bool(self.ptr, s_schema.as_ptr(),
                                     s_name.as_ptr(), value, optionbits)
        }
    }
    pub fn set_property_int32(&mut self, schema: &str, name: &str,
                              value: i32, optionbits: u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe {
            c::xmp_set_property_int32(self.ptr, s_schema.as_ptr(),
                                      s_name.as_ptr(), value,
                                      optionbits)
        }
    }
    pub fn set_property_int64(&mut self, schema: &str, name: &str,
                              value: i64, optionbits: u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe {
            c::xmp_set_property_int64(self.ptr, s_schema.as_ptr(),
                                      s_name.as_ptr(), value,
                                      optionbits)
        }
    }

    pub fn append_array_item(&mut self, schema: &str, name: &str,
                             array_options: u32, value: &str,
                             optionsbits: u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let s_value = CString::new(value).unwrap();
        unsafe {
            c::xmp_append_array_item(self.ptr, s_schema.as_ptr(),
                                     s_name.as_ptr(), array_options,
                                     s_value.as_ptr(), optionsbits)
        }
    }
    pub fn delete_property(&mut self, schema: &str, name: &str) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe {
            c::xmp_delete_property(self.ptr, s_schema.as_ptr(), s_name.as_ptr())
        }
    }
    pub fn has_property(&self, schema: &str, name: &str) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        unsafe {
            c::xmp_has_property(self.ptr, s_schema.as_ptr(), s_name.as_ptr())
        }
    }
    // localized text
    pub fn get_localized_text(&self, schema: &str, name: &str, gen_lang: &str,
                              spec_lang: &str, actual_lang: &mut XmpString,
                              value: &mut XmpString,
                              propbits: &mut u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let s_gen_lang = CString::new(gen_lang).unwrap();
        let s_spec_lang = CString::new(spec_lang).unwrap();
        unsafe {
            c::xmp_get_localized_text(self.ptr, s_schema.as_ptr(),
                                      s_name.as_ptr(), s_gen_lang.as_ptr(),
                                      s_spec_lang.as_ptr(),
                                      actual_lang.as_mut_ptr(),
                                      value.as_mut_ptr(),
                                      propbits as *mut u32)
        }
    }
    pub fn set_localized_text(&mut self, schema: &str, name: &str,
                              gen_lang: &str, spec_lang: &str, value: &str,
                              propbits: u32) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let s_gen_lang = CString::new(gen_lang).unwrap();
        let s_spec_lang = CString::new(spec_lang).unwrap();
        let s_value = CString::new(value).unwrap();
        unsafe {
            c::xmp_set_localized_text(self.ptr, s_schema.as_ptr(),
                                      s_name.as_ptr(), s_gen_lang.as_ptr(),
                                      s_spec_lang.as_ptr(), s_value.as_ptr(),
                                      propbits)
        }
    }
    pub fn delete_localized_text(&mut self, schema: &str, name: &str,
                                 gen_lang: &str, spec_lang: &str) -> bool {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        let s_gen_lang = CString::new(gen_lang).unwrap();
        let s_spec_lang = CString::new(spec_lang).unwrap();
        unsafe {
            c::xmp_delete_localized_text(self.ptr, s_schema.as_ptr(),
                                         s_name.as_ptr(), s_gen_lang.as_ptr(),
                                         s_spec_lang.as_ptr())
        }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
    pub fn as_ptr(&self) -> *const c::Xmp {
        self.ptr
    }
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
    /// Drop the Xmp.
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { c::xmp_free(self.ptr) };
        }
    }
}



