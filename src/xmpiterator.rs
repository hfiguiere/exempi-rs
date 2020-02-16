use c;
use std::ffi::CString;
use xmp::{PropFlags, Xmp};
use xmpstring::XmpString;

bitflags! {
    #[derive(Default)]
    pub struct IterFlags: u32 {
        /// No iterator flag
        const NONE = 0;
        /// The low 8 bits are an enum of what data structure to iterate.
        const CLASS_MASK = 0x00FFu32;
        /// Iterate the property tree of a Xmp object.
        const PROPERTIES = 0x0000u32;
        /// Iterate the global alias table.
        const ALIASES = 0x0001u32;
        /// Iterate the global namespace table.
        const NAMESPACES = 0x0002u32;
        /// Just do the immediate children of the root, default is subtree.
        const JUST_CHILDREN = 0x0100u32;
        /// Just do the leaf nodes, default is all nodes in the subtree.
        const JUST_LEAF_NODES = 0x0200u32;
        /// Return just the leaf part of the path, default is the full path.
        const JUST_LEAF_NAME = 0x0400u32;
        /// Include aliases, default is justactual properties.
        const INCLUDE_ALIASES = 0x0800u32;
        /// Omit all qualifiers.
        const OMIT_QUALIFIERS = 0x1000u32;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct IterSkipFlags: u32 {
        /// Not flags.
        const NONE = 0;
        /// Skip the subtree below the current node.
        const SUBTREE = 0x0001u32;
        /// Skip the subtree below and remaining siblings
        /// of the current node.
        const SIBLINGS = 0x0002u32;
    }
}

pub struct XmpIterator(*mut c::XmpIterator);

impl XmpIterator {
    /// Construct a new `XmpIterator` from a native pointer
    pub fn new(xmp: &Xmp, schema: &str, name: &str, propsbits: IterFlags) -> XmpIterator {
        let s_schema = CString::new(schema).unwrap();
        let s_name = CString::new(name).unwrap();
        XmpIterator(unsafe {
            c::xmp_iterator_new(
                xmp.as_ptr(),
                s_schema.as_ptr(),
                s_name.as_ptr(),
                propsbits.bits(),
            )
        })
    }

    /// Whether native pointer is null
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Return native pointer.
    pub fn as_ptr(&self) -> *mut c::XmpIterator {
        self.0
    }

    /// Iterate to the next element following the option set by the iterator
    ///
    /// schema, name, value will be output with the respective info
    /// option will be output with property flags.
    /// return false when reaching the end
    ///
    pub fn next(
        &mut self,
        schema: &mut XmpString,
        name: &mut XmpString,
        value: &mut XmpString,
        option: &mut PropFlags,
    ) -> bool {
        let mut raw_option: u32 = 0;
        let result = unsafe {
            c::xmp_iterator_next(
                self.0,
                schema.as_mut_ptr(),
                name.as_mut_ptr(),
                value.as_mut_ptr(),
                &mut raw_option,
            )
        };
        *option = PropFlags::from_bits(raw_option).unwrap_or_default();
        result
    }

    /// Skip the poperties following the option bitset from `IterSkipBits`
    pub fn skip(&mut self, option: IterSkipFlags) -> bool {
        if self.is_null() {
            return false;
        }
        unsafe { c::xmp_iterator_skip(self.0, option.bits()) }
    }
}

/// `XmpIterator` implements the `Drop` trait to release the memory
/// from the native object.
impl Drop for XmpIterator {
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { c::xmp_iterator_free(self.0) };
        }
    }
}

#[cfg(test)]
#[test]
fn iterator_works() {
    let inited = super::init();
    assert!(inited);

    let mut xmp = Xmp::new();
    XmpIterator::new(
        &mut xmp,
        "http://ns.adobe.com/xap/1.0/",
        "keyword",
        IterFlags::from_bits(0).unwrap_or_default(),
    );
}
