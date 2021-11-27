use crate::xmp::{PropFlags, Xmp};
use crate::xmpstring::XmpString;
use std::ffi::CString;

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

///
/// XmpIterator iterates over properties in the XMP Packet
///
/// ```no_run
/// use exempi2::{Xmp,XmpIterator};
/// use exempi2::{IterFlags, NS_EXIF};
///
/// let xmp = Xmp::new();
///
/// let iter = XmpIterator::new(&xmp, "http://ns.adobe.com/exif/1.0/",
///                             [], IterFlags::PROPERTIES);
///
/// iter.for_each(|value| {
///     println!("{}:{} = {} / {}", value.schema, value.name, value.value, value.option.bits());
/// });
/// ```
///
pub struct XmpIterator(*mut c::XmpIterator);

/// Value returned by th XmpIterator
#[derive(Debug, Default, PartialEq)]
pub struct IteratorValue {
    /// Schema of the property
    pub schema: XmpString,
    /// Name of the property
    pub name: XmpString,
    /// Value of the property
    pub value: XmpString,
    /// Property flags
    pub option: PropFlags,
}

impl IteratorValue {
    pub fn new() -> IteratorValue {
        IteratorValue::default()
    }
}

impl XmpIterator {
    /// Construct a new `XmpIterator` from a native pointer
    pub fn new<S, N>(xmp: &Xmp, schema: S, name: N, propsbits: IterFlags) -> XmpIterator
    where
        S: AsRef<[u8]>,
        N: AsRef<[u8]>,
    {
        let s_schema = CString::new(schema.as_ref()).unwrap();
        let s_name = CString::new(name.as_ref()).unwrap();
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

    /// Skip the poperties following the option bitset from `IterSkipBits`
    pub fn skip(&mut self, option: IterSkipFlags) -> bool {
        if self.is_null() {
            return false;
        }
        unsafe { c::xmp_iterator_skip(self.0, option.bits()) }
    }
}

impl Iterator for XmpIterator {
    type Item = IteratorValue;

    /// Iterate to the next element following the option set by the iterator
    ///
    fn next(&mut self) -> Option<Self::Item> {
        let mut value = IteratorValue::new();
        let mut raw_option: u32 = 0;
        if unsafe {
            c::xmp_iterator_next(
                self.0,
                value.schema.as_mut_ptr(),
                value.name.as_mut_ptr(),
                value.value.as_mut_ptr(),
                &mut raw_option,
            )
        } {
            value.option = PropFlags::from_bits(raw_option).unwrap_or_default();
            Some(value)
        } else {
            None
        }
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
    let mut iter = XmpIterator::new(
        &mut xmp,
        "http://ns.adobe.com/xap/1.0/",
        "keyword",
        IterFlags::from_bits(0).unwrap_or_default(),
    );

    assert_eq!(iter.next(), None);
}
