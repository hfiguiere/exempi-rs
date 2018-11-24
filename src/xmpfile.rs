extern crate libc;

use c;
use xmp::Xmp;
use xmpstring::XmpString;
use std::ffi::{CString};
use c::FileType;
use c::XmpPacketInfo as PacketInfo;
use super::Result;

pub mod flags {
    bitflags! {
        /// Flag options for opening files.
        pub flags OpenFlags: u32 {
            /// No open option
            const OPEN_NONE = 0x0000_0000,
            /// Open for read-only access.
            const OPEN_READ = 0x0000_0001,
            /// Open for reading and writing.
            const OPEN_FOR_UPDATE = 0x0000_0002,
            /// Only the XMP is wanted, allows space/time optimizations.
            const OPEN_ONLY_XMP = 0x0000_0004,
            /// Cache thumbnail if possible, GetThumbnail will be called.
            const OPEN_CACHE_TNAIL = 0x0000_0008,
            /// Be strict about locating XMP and reconciling with other forms.
            const OPEN_STRICTLY = 0x0000_0010,
            /// Require the use of a smart handler.
            const OPEN_USE_SMART_HANDLER = 0x0000_0020,
            /// Force packet scanning, don't use a smart handler.
            const OPEN_USE_PACKET_SCANNING = 0x0000_0040,
            /// Only packet scan files "known" to need scanning.
            const OPEN_LIMITED_SCANNING = 0x0000_0080,
            /// Attempt to repair a file opened for update, default is to not open (throw an exception).
            const OPEN_REPAIR_FILE = 0x0000_0100,
            /// Optimize MPEG4 to support stream when updating This can take some time
            const OPEN_OPTIMIZE_FILE_LAYOUT = 0x0000_0200,
            /// Set if calling from background thread.
            const OPEN_IN_BACKGROUND = 0x1000_0000,
        }
    }

    bitflags! {
        /// Flag options to close files.
        pub flags CloseFlags: u32 {
            /// No close option
            const CLOSE_NONE = 0x0000,
            /// Write into a temporary file and swap for crash safety.
            const CLOSE_SAFE_UPDATE = 0x0001,
        }
    }

    bitflags! {
        /// Result flage for file / format infos.
        pub flags FormatOptionFlags: u32 {
            const FORMAT_NONE = 0,
            /// Can inject first-time XMP into an existing file.
	    const FORMAT_CAN_INJECT_XMP = 0x0000_0001,
            /// Can expand XMP or other metadata in an existing file.
	    const FORMAT_CAN_EXPAND = 0x0000_0002,
            /// Can copy one file to another, writing new metadata.
	    const FORMAT_CAN_REWRITE = 0x0000_0004,
            /// Can expand, but prefers in-place update.
	    const FORMAT_PREFERS_IN_PLACE = 0x0000_0008,
            /// Supports reconciliation between XMP and other forms.
	    const FORMAT_CAN_RECONCILE = 0x0000_0010,
            /// Allows access to just the XMP, ignoring other forms.
	    const FORMAT_ALLOWS_ONLY_XMP = 0x0000_0020,
            /// File handler returns raw XMP packet information.
	    const FORMAT_RETURNS_RAW_PACKET = 0x0000_0040,
            /// The file handler does the file open and close.
	    const FORMAT_HANDLER_OWNS_FILE = 0x0000_0100,
            /// The file handler allows crash-safe file updates.
	    const FORMAT_ALLOW_SAFE_UPDATE = 0x0000_0200,
            /// The file format needs the XMP packet to be read-only.
	    const FORMAT_NEEDS_READONLY_PACKET = 0x0000_0400,
            /// The file handler uses a "sidecar" file for the XMP.
	    const FORMAT_USE_SIDECAR_XMP = 0x0000_0800,
            /// The format is folder oriented, for example the P2 video format.
	    const FORMAT_FOLDER_BASED_FORMAT = 0x0000_1000,
        }
    }
}

use self::flags::{ FormatOptionFlags, FORMAT_NONE,
                   OpenFlags, OPEN_NONE,
                   CloseFlags };

pub struct XmpFile {
    ptr: *mut c::XmpFile,
}

impl Default for XmpFile {
    fn default() -> XmpFile {
        XmpFile { ptr: unsafe { c::xmp_files_new() } }
    }
}

impl XmpFile {
    /// Create new XmpFile
    pub fn new() -> XmpFile {
        XmpFile::default()
    }

    /// Create and open a new XmpFile
    /// Equivalent to calling new then open.
    /// Return Err in case of failure
    pub fn open_new(p: &str, options: OpenFlags) -> Result<XmpFile> {
        let pp = CString::new(p).unwrap();
        let ptr = unsafe {
            c::xmp_files_open_new(pp.as_ptr(), options.bits())
        };
        if ptr.is_null() {
            return Err(super::get_error());
        }
        Ok(XmpFile { ptr })
    }

    /// Open an XmpFile. Usually called after new.
    pub fn open(&mut self, path: &str, options: OpenFlags) -> Result<()> {
        if self.is_null() {
            return Err(super::Error::BadObject);
        }
        let pp = CString::new(path).unwrap();
        if unsafe {
            c::xmp_files_open(self.ptr, pp.as_ptr(), options.bits())
        } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Close the XmpFile
    pub fn close(&mut self, options: CloseFlags) -> Result<()> {
        if self.is_null() {
            return Err(super::Error::BadObject);
        }
        if unsafe {
            c::xmp_files_close(self.ptr, options.bits())
        } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Return true if native pointer is null
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    /// Get a new Xmp fronm the currently open file
    pub fn get_new_xmp(&self) -> Result<Xmp> {
        let ptr = unsafe {
            c::xmp_files_get_new_xmp(self.ptr)
        };
        if ptr.is_null() {
            return Err(super::get_error());
        }
        Ok(Xmp::from(ptr))
    }

    /// Get the xmp data an Xmp.
    pub fn get_xmp(&self, xmp: &mut Xmp) -> Result<()> {
        if self.is_null() || xmp.is_null() {
            return Err(super::Error::BadObject);
        }
        if unsafe { c::xmp_files_get_xmp(self.ptr, xmp.as_mut_ptr()) } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Get the xmp packet as a string.
    pub fn get_xmp_xmpstring(&self, packet: &mut XmpString,
                             info: &mut PacketInfo) -> Result<()> {
        if self.is_null() || packet.is_null() {
            return Err(super::Error::BadObject);
        }
        if unsafe { c::xmp_files_get_xmp_xmpstring(self.ptr,
                                                packet.as_mut_ptr(),
                                                   info as *mut PacketInfo)
        } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Return true if it can put the Xmp into the XmpFile.
    pub fn can_put_xmp(&self, xmp: &Xmp) -> bool {
        if self.is_null() || xmp.is_null() {
            return false;
        }
        unsafe { c::xmp_files_can_put_xmp(self.ptr, xmp.as_ptr()) }
    }

    /// Return true if it can put the XmpString packet into the XmpFile.
    pub fn can_put_xmp_xmpstring(&self, xmp_packet: &XmpString) -> bool {
        if self.is_null() || xmp_packet.is_null() {
            return false;
        }
        unsafe { c::xmp_files_can_put_xmp_xmpstring(self.ptr,
                                                    xmp_packet.as_ptr()) }
    }

    /// Return true if it can put the XmpString packet into the XmpFile.
    pub fn can_put_xmp_str(&self, xmp_packet: &str) -> bool {
        if self.is_null() {
            return false;
        }
        let pp = CString::new(xmp_packet).unwrap();
        unsafe { c::xmp_files_can_put_xmp_cstr(self.ptr,
                                               pp.as_ptr(), xmp_packet.len()) }
    }

    /// Put the Xmp into the XmpFile
    pub fn put_xmp(&mut self, xmp: &Xmp) -> Result<()> {
        if self.is_null() || xmp.is_null() {
            return Err(super::Error::BadObject);
        }
        if unsafe { c::xmp_files_put_xmp(self.ptr, xmp.as_ptr()) } {
            Ok(())
        } else {
            Err(super::get_error())
        }
    }

    /// Get info from the XmpFile.
    pub fn get_file_info(&self, file_path: &mut String,
                         options: &mut OpenFlags, format: &mut FileType,
                         handler_flags: &mut FormatOptionFlags) -> bool {
        if self.is_null() {
            return false;
        }
        let mut s = XmpString::new();

        let mut raw_options : u32 = 0;
        let mut raw_handler_flags : u32 = 0;
        let result = unsafe {
            c::xmp_files_get_file_info(self.ptr, s.as_mut_ptr(),
                                       &mut raw_options, format,
                                       &mut raw_handler_flags)
        };
        *options = OpenFlags::from_bits(raw_options).unwrap_or(OPEN_NONE);
        *handler_flags = FormatOptionFlags::from_bits(raw_handler_flags)
            .unwrap_or(FORMAT_NONE);

        file_path.push_str(s.to_str());
        result
    }

    /// Check the file format for the specified path
    pub fn check_file_format(path: &str) -> FileType {
        let pp = CString::new(path).unwrap();
        unsafe { c::xmp_files_check_file_format(pp.as_ptr()) }
    }

    /// Get FormatOptions for the FileType
    pub fn get_format_info(format: FileType) -> Result<FormatOptionFlags> {
        let mut raw_options: u32 = 0;
        if unsafe { c::xmp_files_get_format_info(format, &mut raw_options) } {
            Ok(FormatOptionFlags::from_bits(raw_options).unwrap_or(FORMAT_NONE))
        } else {
            Err(super::get_error())
        }
    }

}

impl Drop for XmpFile {
    /// Drop the XmpFile.
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { c::xmp_files_free(self.ptr) };
        }
    }
}


#[test]
fn it_works() {
    use c::XmpError;

    let inited = ::init();

    assert!(inited);

    let xf = XmpFile::new();
    assert!(!xf.is_null());

    assert!(::get_error() == XmpError::Unknown);
}
