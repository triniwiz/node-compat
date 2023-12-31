use std::borrow::Cow;
use std::ffi::{c_char, CStr, CString};
use std::fmt::Display;
use std::os::fd::RawFd;
use std::os::raw::{c_int, c_void};
use std::sync::Arc;
use node_fs::prelude::{handle_meta};
use node_core::error::{AnyError, get_custom_error_message, Result};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WriteFileOptions {
    encoding: StringEncoding,
    mode: i32,
    flag: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WriteOptions {
    offset: usize,
    length: usize,
    position: isize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RmOptions {
    force: bool,
    max_retries: i32,
    recursive: bool,
    retry_delay: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RmDirOptions {
    max_retries: i32,
    recursive: bool,
    retry_delay: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RealPathOptions {
    encoding: StringEncoding,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ReadLinkOptions {
    encoding: FsEncodingType,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ReadFileOptions {
    flag: i32,
    encoding: FsEncodingType,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ReaddirOptions {
    with_file_types: bool,
    encoding: FsEncodingType,
    recursive: bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct OpenDirOptions {
    encoding: StringEncoding,
    buffer_size: usize,
    recursive: bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MkdTempOptions {
    encoding: StringEncoding,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MkDirOptions {
    mode: u32,
    recursive: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppendFileOptions {
    encoding: StringEncoding,
    mode: i32,
    flag: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StringEncoding {
    StringEncodingAscii,
    StringEncodingUtf8,
    StringEncodingUtf16le,
    StringEncodingUcs2,
    StringEncodingBase64,
    StringEncodingBase64Url,
    StringEncodingLatin1,
    StringEncodingBinary,
    StringEncodingHex,
}

impl From<node_buffer::StringEncoding> for StringEncoding {
    fn from(value: node_buffer::StringEncoding) -> Self {
        match value {
            node_buffer::StringEncoding::Ascii => StringEncoding::StringEncodingAscii,
            node_buffer::StringEncoding::Utf8 => StringEncoding::StringEncodingUtf8,
            node_buffer::StringEncoding::Utf16le => StringEncoding::StringEncodingUtf16le,
            node_buffer::StringEncoding::Ucs2 => StringEncoding::StringEncodingUcs2,
            node_buffer::StringEncoding::Base64 => StringEncoding::StringEncodingBase64,
            node_buffer::StringEncoding::Base64Url => StringEncoding::StringEncodingBase64Url,
            node_buffer::StringEncoding::Latin1 => StringEncoding::StringEncodingLatin1,
            node_buffer::StringEncoding::Binary => StringEncoding::StringEncodingBinary,
            node_buffer::StringEncoding::Hex => StringEncoding::StringEncodingHex,
        }
    }
}


impl Into<node_buffer::StringEncoding> for StringEncoding {
    fn into(self) -> node_buffer::StringEncoding {
        match self {
            StringEncoding::StringEncodingAscii => node_buffer::StringEncoding::Ascii,
            StringEncoding::StringEncodingUtf8 => node_buffer::StringEncoding::Utf8,
            StringEncoding::StringEncodingUtf16le => node_buffer::StringEncoding::Utf16le,
            StringEncoding::StringEncodingUcs2 => node_buffer::StringEncoding::Ucs2,
            StringEncoding::StringEncodingBase64 => node_buffer::StringEncoding::Base64,
            StringEncoding::StringEncodingBase64Url => node_buffer::StringEncoding::Base64Url,
            StringEncoding::StringEncodingLatin1 => node_buffer::StringEncoding::Latin1,
            StringEncoding::StringEncodingBinary => node_buffer::StringEncoding::Binary,
            StringEncoding::StringEncodingHex => node_buffer::StringEncoding::Hex
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FsEncodingType {
    FsEncodingTypeAscii,
    FsEncodingTypeUtf8,
    FsEncodingTypeUtf16le,
    FsEncodingTypeUcs2,
    FsEncodingTypeLatin1,
    FsEncodingTypeBuffer,
}


impl Into<node_fs::FsEncodingType> for FsEncodingType {
    fn into(self) -> node_fs::FsEncodingType {
        match self {
            FsEncodingType::FsEncodingTypeAscii => node_fs::FsEncodingType::Ascii,
            FsEncodingType::FsEncodingTypeUtf8 => node_fs::FsEncodingType::Utf8,
            FsEncodingType::FsEncodingTypeUtf16le => node_fs::FsEncodingType::Utf16le,
            FsEncodingType::FsEncodingTypeUcs2 => node_fs::FsEncodingType::Ucs2,
            FsEncodingType::FsEncodingTypeLatin1 => node_fs::FsEncodingType::Latin1,
            FsEncodingType::FsEncodingTypeBuffer => node_fs::FsEncodingType::Buffer,
        }
    }
}

impl From<node_fs::prelude::FsEncodingType> for FsEncodingType {
    fn from(value: node_fs::prelude::FsEncodingType) -> Self {
        match value {
            node_fs::prelude::FsEncodingType::Ascii => FsEncodingType::FsEncodingTypeAscii,
            node_fs::prelude::FsEncodingType::Utf8 => FsEncodingType::FsEncodingTypeUtf8,
            node_fs::prelude::FsEncodingType::Utf16le => FsEncodingType::FsEncodingTypeUtf16le,
            node_fs::prelude::FsEncodingType::Ucs2 => FsEncodingType::FsEncodingTypeUcs2,
            node_fs::prelude::FsEncodingType::Latin1 => FsEncodingType::FsEncodingTypeLatin1,
            node_fs::prelude::FsEncodingType::Buffer => FsEncodingType::FsEncodingTypeBuffer
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReaddirResultType {
    ReaddirResultTypeString,
    ReaddirResultTypeBuffer,
    ReaddirResultTypeType,
}


#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
pub struct FileStat {
    pub dev: i64,
    pub ino: i64,
    pub mode: i32,
    pub nlink: i64,
    pub uid: i32,
    pub gid: i32,
    pub rdev: i64,
    pub size: i64,
    pub blksize: i64,
    pub blocks: i64,
    pub atimeMs: f64,
    pub mtimeMs: f64,
    pub ctimeMs: f64,
    pub birthtimeMs: f64,
    pub birthtime: f64,
    pub atime: f64,
    pub mtime: f64,
    pub ctime: f64,
    pub isBlockDevice: bool,
    pub isCharacterDevice: bool,
    pub isDirectory: bool,
    pub isFIFO: bool,
    pub isFile: bool,
    pub isSocket: bool,
    pub isSymbolicLink: bool,
}


#[no_mangle]
pub unsafe extern "C" fn node_string_destroy(string: *mut c_char) {
    if string.is_null() {
        return;
    }
    let _ = unsafe { CString::from_raw(string) };
}


#[no_mangle]
pub unsafe extern "C" fn filestat_destroy(file_stat: *mut FileStat) {
    if file_stat.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(file_stat) };
}

#[derive(Debug)]
pub struct NodeError(node_core::error::AnyError);


#[no_mangle]
pub unsafe extern "C" fn node_error_get_clazz(error: *const NodeError) -> *const c_char {
    if error.is_null() {
        return std::ptr::null();
    }
    let error = unsafe { &*error };
    let message = error.clazz();
    CString::new(message.to_string()).unwrap().into_raw()
}


#[no_mangle]
pub unsafe extern "C" fn node_error_get_message(error: *const NodeError) -> *const c_char {
    if error.is_null() {
        return std::ptr::null();
    }
    let error = unsafe { &*error };
    let message = error.message();
    CString::new(message.to_string()).unwrap().into_raw()
}


#[no_mangle]
pub unsafe extern "C" fn node_error_destroy(error: *mut NodeError) {
    if error.is_null() {
        return;
    }

    let _ = unsafe { Box::from_raw(error) };
}


impl From<node_core::error::AnyError> for Box<NodeError> {
    fn from(value: AnyError) -> Self {
        Box::new(NodeError(value))
    }
}

impl NodeError {
    pub fn custom_error(clazz: &'static str, message: &'static str) -> Self {
        Self(
            node_core::error::custom_error(clazz, message)
        )
    }

    pub fn generic_error(message: &'static str) -> Self {
        Self(
            node_core::error::generic_error(message)
        )
    }

    pub fn type_error(message: &'static str) -> Self {
        Self(
            node_core::error::type_error(message)
        )
    }

    pub fn clazz(&self) -> &'static str {
        node_core::error::get_custom_error_class(&self.0).unwrap_or("Error")
    }

    pub fn message(&self) -> Cow<str> {
        get_custom_error_message(&self.0).unwrap_or_default()
    }
}

fn to_optional(value: isize) -> Option<usize> {
    if value < 0 {
        return None;
    }

    return Some(value as usize);
}



thread_local!(
    static LAST_ERROR: std::cell::RefCell<Option<Box<NodeError >>> = std::cell::RefCell::new(None);
);

/// Set the thread-local `LAST_ERROR` variable.
pub fn update_last_error<E: Into<Box<NodeError>> + 'static>(e: E) {
    let boxed = e.into();

    LAST_ERROR.with(|last| {
        *last.borrow_mut() = Some(boxed);
    });
}

/// Get the last error, clearing the variable in the process.
pub fn get_last_error() -> Option<Box<NodeError>> {
    LAST_ERROR.with(|last| last.borrow_mut().take())
}

/// Write the latest error message to a buffer.
///
/// # Returns
///
/// This returns the number of bytes written to the buffer. If no bytes were
/// written (i.e. there is no last error) then it returns `0`. If the buffer
/// isn't big enough or a `null` pointer was passed in, you'll get a `-1`.
#[no_mangle]
pub unsafe extern "C" fn node_error_message(buffer: *mut c_char, length: libc::c_int) -> libc::c_int {
    if buffer.is_null() {
        return -1;
    }

    let buffer = std::slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    // Take the last error, if there isn't one then there's no error message to
    // display.
    let err = match get_last_error() {
        Some(e) => e,
        None => return 0,
    };

    if let Some(error_message) = get_custom_error_message(&err.0) {
        let bytes_required = error_message.len() + 1;

        if buffer.len() < bytes_required {
            // We don't have enough room. Make sure to return the error so it
            // isn't accidentally consumed
            update_last_error(err);
            return -1;
        }

        let data = error_message.as_bytes();
        std::ptr::copy_nonoverlapping(data.as_ptr(), buffer.as_mut_ptr(), data.len());

        // zero out the rest of the buffer just in case
        let rest = &mut buffer[data.len()..];
        std::ptr::write_bytes(rest.as_mut_ptr(), 0, rest.len());

        return data.len() as libc::c_int;
    }
    0
}

#[derive(Clone)]
pub struct Buffer(node_buffer::Buffer);

#[no_mangle]
pub extern "C" fn buffer_destroy(buffer: *mut Buffer) {
    if buffer.is_null() {
        return;
    }

    let _ = unsafe { Box::from_raw(buffer) };
}

impl Buffer {
    pub(crate) fn new(buffer: node_buffer::Buffer) -> Self {
        Self(buffer)
    }

    pub fn into_box(self) -> Box<Self> {
        Box::new(self)
    }
}

#[no_mangle]
pub extern "C" fn buffer_clone(buffer: *const Buffer) -> *mut Buffer {
    if buffer.is_null() {
        return std::ptr::null_mut();
    }
    let buffer = unsafe { &*buffer };

    Box::into_raw(Box::new(buffer.clone()))
}

#[no_mangle]
pub extern "C" fn buffer_alloc(size: usize) -> *mut Buffer {
    Box::into_raw(
        Buffer(node_buffer::Buffer::builder().size(size).build()).into_box()
    )
}

#[no_mangle]
pub extern "C" fn buffer_alloc_with_size_string_encoding(size: usize, string: *const c_char, encoding: StringEncoding) -> *mut Buffer {
    let string = unsafe { CStr::from_ptr(string) };
    Box::into_raw(
        Box::new(
            Buffer(node_buffer::Buffer::builder()
                .size(size)
                .fill_text(CString::from(string), encoding.into())
                .build())
        )
    )
}

#[no_mangle]
pub extern "C" fn buffer_concat(buffers: *const *const u8, buffers_length: *const usize, count: usize) -> *mut Buffer {
    Box::into_raw(
        Buffer(
            unsafe { node_buffer::Buffer::concat_raw(buffers, buffers_length, count, None) }
        ).into_box()
    )
}

#[no_mangle]
pub extern "C" fn buffer_concat_length(buffers: *const *const u8, buffers_length: *const usize, count: usize, length: usize) -> *mut Buffer {
    Box::into_raw(
        Buffer(
            unsafe { node_buffer::Buffer::concat_raw(buffers, buffers_length, count, Some(length)) }
        ).into_box()
    )
}

#[no_mangle]
pub extern "C" fn buffer_from_string(string: *const c_char, encoding: StringEncoding) -> *mut Buffer {
    let string = unsafe { CStr::from_ptr(string) };
    Box::into_raw(
        Buffer(
            node_buffer::Buffer::from_str(string, encoding.into())
        ).into_box()
    )
}

#[no_mangle]
pub extern "C" fn buffer_from_slice(slice: *const u8, length: usize) -> *mut Buffer {
    Box::into_raw(
        Buffer(
            unsafe { node_buffer::Buffer::from_slice(std::slice::from_raw_parts(slice, length)) }
        ).into_box()
    )
}

#[no_mangle]
pub extern "C" fn buffer_copy_bytes_from(buffer: *const Buffer) -> *mut Buffer {
    if buffer.is_null() {
        return std::ptr::null_mut();
    }
    let buffer = unsafe { &*buffer };
    Box::into_raw(
        Buffer(
            node_buffer::Buffer::from_buffer(&buffer.0)
        ).into_box()
    )
}

#[no_mangle]
pub extern "C" fn buffer_from_reference(data: *mut u8, size: usize) -> *mut Buffer {
    unsafe {
        Box::into_raw(
            Buffer(
                node_buffer::Buffer::from_reference(data, size)
            ).into_box()
        )
    }
}

#[no_mangle]
pub extern "C" fn buffer_atob(string: *const c_char) -> *const c_char {
    unsafe { node_buffer::Buffer::atob_raw(string) }
}

#[no_mangle]
pub extern "C" fn buffer_btoa(string: *const c_char) -> *const c_char {
    unsafe { node_buffer::Buffer::btoa_raw(string) }
}

#[no_mangle]
pub extern "C" fn buffer_fill_string(buffer: *mut Buffer, string: *const c_char, encoding: StringEncoding) {
    if buffer.is_null() || string.is_null() {
        return;
    }
    let buffer = unsafe { &mut *buffer };
    unsafe { buffer.0.fill_str(CStr::from_ptr(string), Some(encoding.into())); }
}

#[no_mangle]
pub extern "C" fn buffer_to_string(buffer: *const Buffer, encoding: StringEncoding, start: isize, end: isize) -> *const c_char {
    if buffer.is_null() {
        return std::ptr::null();
    }
    let buffer = unsafe { &*buffer };

    let start = if start < 0 {
        None
    } else {
        Some(start as usize)
    };

    let end = if end == -1 {
        None
    } else {
        Some(end as usize)
    };

    match CString::new(
        buffer.0.as_string(Some(encoding.into()), start, end)
    ) {
        Ok(cstring) => cstring.into_raw(),
        Err(_) => std::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn buffer_to_print_string(buffer: *const Buffer) -> *const c_char {
    if buffer.is_null() {
        return std::ptr::null();
    }

    let buffer = unsafe { &*buffer };

    match CString::new(format!("{}", &buffer.0)) {
        Ok(cstring) => cstring.into_raw(),
        Err(_) => std::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn buffer_length(buffer: *const Buffer) -> usize {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };

    buffer.0.length()
}

#[no_mangle]
pub extern "C" fn buffer_buffer(buffer: *mut Buffer) -> *mut u8 {
    if buffer.is_null() {
        return std::ptr::null_mut();
    }
    let buffer = unsafe { &mut *buffer };

    buffer.0.buffer_mut().as_mut_ptr()
}


#[no_mangle]
pub extern "C" fn buffer_write_int8(buffer: *mut Buffer, value: i8, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };

    buffer.0.write_int8(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_uint8(buffer: *mut Buffer, value: u8, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };

    buffer.0.write_uint8(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_uint16be(buffer: *mut Buffer, value: u16, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };
    buffer.0.write_uint16be(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_int16be(buffer: *mut Buffer, value: i16, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };

    buffer.0.write_int16be(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_uint16le(buffer: *mut Buffer, value: u16, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };

    buffer.0.write_uint16le(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_int16le(buffer: *mut Buffer, value: i16, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };

    buffer.0.write_int16le(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_uint32be(buffer: *mut Buffer, value: u32, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };
    buffer.0.write_uint32be(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_int32be(buffer: *mut Buffer, value: i32, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };
    buffer.0.write_int32be(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_uint32le(buffer: *mut Buffer, value: u32, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };
    buffer.0.write_uint32le(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_int32le(buffer: *mut Buffer, value: i32, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };
    buffer.0.write_int32le(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_big_uint64be(buffer: *mut Buffer, value: u64, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };
    buffer.0.write_big_uint64be(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_big_int64be(buffer: *mut Buffer, value: i64, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };
    buffer.0.write_big_int64be(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_big_uint64le(buffer: *mut Buffer, value: u64, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };
    buffer.0.write_big_uint64le(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_big_int64le(buffer: *mut Buffer, value: i64, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };
    buffer.0.write_big_int64le(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_float_be(buffer: *mut Buffer, value: f32, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };
    buffer.0.write_float_be(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_double_be(buffer: *mut Buffer, value: f64, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };

    buffer.0.write_double_be(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_float_le(buffer: *mut Buffer, value: f32, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };
    buffer.0.write_float_le(value, to_optional(offset));
}

#[no_mangle]
pub extern "C" fn buffer_write_double_le(buffer: *mut Buffer, value: f64, offset: isize) {
    if buffer.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };
    buffer.0.write_double_le(value, to_optional(offset));
}


#[no_mangle]
pub extern "C" fn buffer_read_int8(buffer: *const Buffer, offset: isize) -> i8 {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_int8(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_uint8(buffer: *const Buffer, offset: isize) -> u8 {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };

    buffer.0.read_uint8(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_uint16be(buffer: *const Buffer, offset: isize) -> u16 {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_uint16be(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_int16be(buffer: *const Buffer, offset: isize) -> i16 {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_int16be(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_uint16le(buffer: *const Buffer, offset: isize) -> u16 {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_uint16le(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_int16le(buffer: *const Buffer, offset: isize) -> i16 {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_int16le(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_uint32be(buffer: *const Buffer, offset: isize) -> u32 {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_uint32be(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_int32be(buffer: *const Buffer, offset: isize) -> i32 {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_int32be(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_uint32le(buffer: *const Buffer, offset: isize) -> u32 {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_uint32le(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_int32le(buffer: *const Buffer, offset: isize) -> i32 {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_int32le(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_big_uint64be(buffer: *const Buffer, offset: isize) -> u64 {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_big_uint64be(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_big_int64be(buffer: *const Buffer, offset: isize) -> i64 {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_big_int64be(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_big_uint64le(buffer: *const Buffer, offset: isize) -> u64 {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_big_uint64le(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_big_int64le(buffer: *const Buffer, offset: isize) -> i64 {
    if buffer.is_null() {
        return 0;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_big_int64le(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_float_be(buffer: *const Buffer, offset: isize) -> f32 {
    if buffer.is_null() {
        return 0.;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_float_be(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_double_be(buffer: *const Buffer, offset: isize) -> f64 {
    if buffer.is_null() {
        return 0.;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_double_be(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_float_le(buffer: *const Buffer, offset: isize) -> f32 {
    if buffer.is_null() {
        return 0.;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_float_le(to_optional(offset))
}

#[no_mangle]
pub extern "C" fn buffer_read_double_le(buffer: *const Buffer, offset: isize) -> f64 {
    if buffer.is_null() {
        return 0.;
    }

    let buffer = unsafe { &*buffer };
    buffer.0.read_double_le(to_optional(offset))
}


#[no_mangle]
pub extern "C" fn fs_encoding_get_string_value(encoding: *const FsEncoding) -> *const c_char {
    if encoding.is_null() {
        return std::ptr::null();
    }

    let encoding = unsafe { &*encoding };

    match encoding.get_string_value() {
        Ok(encoding) => {
            CString::new(encoding).unwrap().into_raw()
        }
        Err(err) => {
            update_last_error(err);
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_encoding_get_buffer_value(encoding: *const FsEncoding) -> *mut Buffer {
    if encoding.is_null() {
        return std::ptr::null_mut();
    }

    let encoding = unsafe { &*encoding };
    match encoding.get_buffer_value() {
        Ok(buffer) => {
            Box::into_raw(buffer)
        }
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_encoding_is_buffer(encoding: *const FsEncoding) -> bool {
    if encoding.is_null() {
        return false;
    }

    let encoding = unsafe { &*encoding };
    encoding.is_buffer()
}

#[derive(Debug)]
pub struct FileDirent(node_fs::file_dirent::FileDirent);

#[no_mangle]
pub extern "C" fn fs_dirent_is_block_device(dirent: *const FileDirent) -> bool {
    if dirent.is_null() {
        return false;
    }

    let dirent = unsafe { &*dirent };
    dirent.0.is_block_device()
}

#[no_mangle]
pub extern "C" fn fs_dirent_path(dirent: *const FileDirent) -> *const c_char {
    if dirent.is_null() {
        return std::ptr::null();
    }

    let dirent = unsafe { &*dirent };
    CString::new(dirent.0.path().to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn fs_dirent_name(dirent: *const FileDirent) -> *const c_char {
    if dirent.is_null() {
        return std::ptr::null();
    }

    let dirent = unsafe { &*dirent };

    CString::new(dirent.0.name().to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn fs_dirent_is_character_device(dirent: *const FileDirent) -> bool {
    if dirent.is_null() {
        return false;
    }
    let dirent = unsafe { &*dirent };
    dirent.0.is_character_device()
}

#[no_mangle]
pub extern "C" fn fs_dirent_is_directory(dirent: *const FileDirent) -> bool {
    if dirent.is_null() {
        return false;
    }
    let dirent = unsafe { &*dirent };
    dirent.0.is_directory()
}

#[no_mangle]
pub extern "C" fn fs_dirent_is_fifo(dirent: *const FileDirent) -> bool {
    if dirent.is_null() {
        return false;
    }
    let dirent = unsafe { &*dirent };
    dirent.0.is_fifo()
}

#[no_mangle]
pub extern "C" fn fs_dirent_is_file(dirent: *const FileDirent) -> bool {
    if dirent.is_null() {
        return false;
    }
    let dirent = unsafe { &*dirent };
    dirent.0.is_file()
}

#[no_mangle]
pub extern "C" fn fs_dirent_is_socket(dirent: *const FileDirent) -> bool {
    if dirent.is_null() {
        return false;
    }
    let dirent = unsafe { &*dirent };
    dirent.0.is_socket()
}

#[no_mangle]
pub extern "C" fn fs_dirent_is_symbolic_link(dirent: *const FileDirent) -> bool {
    if dirent.is_null() {
        return false;
    }
    let dirent = unsafe { &*dirent };
    dirent.0.is_symbolic_link()
}

#[no_mangle]
pub extern "C" fn fs_dir_close_sync(dir: *const FileDir) {
    if dir.is_null() {
        return;
    }
    let dir = unsafe { &*dir };


    if let Err(err) = dir.0.close()
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_dir_path(dir: *const FileDir) -> *const c_char {
    if dir.is_null() {
        return std::ptr::null();
    }
    let dir = unsafe { &*dir };
    CString::new(
        dir.0.path().to_string()
    ).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn fs_dir_read_sync(dir: *const FileDir) -> *mut FileDirent {
    if dir.is_null() {
        return std::ptr::null_mut();
    }
    let dir = unsafe { &*dir };

    match dir.0.read()
        .map(|dirent| Box::new(FileDirent(dirent)))
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        Ok(dirent) => {
            Box::into_raw(dirent)
        }
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}


#[no_mangle]
pub extern "C" fn fs_readdir_get_type(value: *const ReaddirResult) -> ReaddirResultType {
    let value = unsafe { &*value };
    return value.get_type();
}

#[no_mangle]
pub extern "C" fn fs_readdir_get_string_value(value: *const ReaddirResult) -> *const c_char {
    let value = unsafe { &*value };
    match value.get_string_value() {
        Ok(value) => {
            CString::new(value).unwrap().into_raw()
        }
        Err(err) => {
            update_last_error(err);
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_readdir_get_buffer_value(value: *const ReaddirResult) -> *mut Buffer {
    if value.is_null() {
        return std::ptr::null_mut();
    }
    let value = unsafe { &*value };

    match value.get_buffer_value() {
        Ok(value) => { Box::into_raw(value) }
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_readdir_get_type_value(value: *const ReaddirResult) -> *mut FileDirent {
    if value.is_null() {
        return std::ptr::null_mut();
    }
    let value = unsafe { &*value };

    match value.get_type_value() {
        Ok(value) => {
            Box::into_raw(value)
        }
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}


#[derive(Clone)]
pub struct Metadata(std::fs::Metadata);


#[derive(Clone, Debug)]
pub struct ReaddirResult(node_fs::sync::ReaddirResult);

impl ReaddirResult {
    pub fn get_type(&self) -> ReaddirResultType {
        match &self.0 {
            node_fs::sync::ReaddirResult::String(_) => ReaddirResultType::ReaddirResultTypeString,
            node_fs::sync::ReaddirResult::Buffer(_) => ReaddirResultType::ReaddirResultTypeBuffer,
            node_fs::sync::ReaddirResult::Type(_) => ReaddirResultType::ReaddirResultTypeType
        }
    }

    pub fn get_string_value(&self) -> Result<String> {
        match self.0.get_string_value() {
            None => {
                Err(node_core::error::generic_error("Invalid Type".to_string()))
            }
            Some(value) => {
                Ok(value.to_string_lossy().to_string())
            }
        }
    }

    pub fn get_buffer_value(&self) -> Result<Box<Buffer>> {
        match self.0.get_buffer_value() {
            None => {
                Err(node_core::error::generic_error("Invalid Type".to_string()))
            }
            Some(buffer) => {
                Ok(Box::new(Buffer(buffer)))
            }
        }
    }

    pub fn get_type_value(&self) -> Result<Box<FileDirent>> {
        match self.0.get_type_value() {
            None => {
                Err(node_core::error::generic_error("Invalid Type".to_string()))
            }
            Some(dirent) => {
                Ok(Box::new(FileDirent(dirent)))
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_parse_flag_sync(value: *const c_char) -> c_int {
    if value.is_null() {
        return 0;
    }
    let value = unsafe { CStr::from_ptr(value) };
    let value = value.to_string_lossy();
    node_fs::prelude::parse_flag(value.as_ref())
}

#[no_mangle]
pub extern "C" fn fs_access_sync(path: *const c_char, mode: i32) {
    if path.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();
    if let Err(err) = node_fs::sync::access(path.as_ref(), mode)
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_append_file_sync(fd: i32, buffer: *const Buffer, options: AppendFileOptions) {
    if buffer.is_null() {
        return;
    }
    let buffer = unsafe { &*buffer };
    if let Err(err) = node_fs::sync::append_file_with_buffer(fd, &buffer.0, options.into())
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_append_file_with_bytes_sync(fd: i32, bytes: *const u8, length: usize, options: AppendFileOptions) {
    if bytes.is_null() || length == 0 {
        return;
    }
    let bytes = unsafe { std::slice::from_raw_parts(bytes, length) };
    if let Err(err) = node_fs::sync::append_file_with_bytes(fd, bytes, options.into()).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_append_file_with_string_sync(fd: i32, string: *const c_char, options: AppendFileOptions) {
    if string.is_null() {
        return;
    }

    let string = unsafe { CStr::from_ptr(string) };
    let string = string.to_string_lossy();
    if let Err(err) = node_fs::sync::append_file_with_str(fd, string.as_ref(), options.into()).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_append_file_with_path_sync(path: *const c_char, buffer: *const Buffer, options: AppendFileOptions) {
    if buffer.is_null() || path.is_null() {
        return;
    }
    let buffer = unsafe { &*buffer };
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    if let Err(err) = node_fs::sync::append_file_with_path_buffer(path.as_ref(), &buffer.0, options.into()).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_append_file_with_path_bytes_sync(path: *const c_char, bytes: *const u8, length: usize, options: AppendFileOptions) {
    if bytes.is_null() || length == 0 || path.is_null() {
        return;
    }
    let bytes = unsafe { std::slice::from_raw_parts(bytes, length) };
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    if let Err(err) = node_fs::sync::append_file_with_path_bytes(path.as_ref(), bytes, options.into()).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_append_file_with_path_buffer_sync(path: *const c_char, buffer: *const Buffer, options: AppendFileOptions) {
    if buffer.is_null() || path.is_null() {
        return;
    }
    let buffer = unsafe { &*buffer };
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();
    if let Err(err) = node_fs::sync::append_file_with_path_buffer(path.as_ref(), &buffer.0, options.into()).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}


#[no_mangle]
pub extern "C" fn fs_append_file_with_path_string_sync(path: *const c_char, string: *const c_char, options: AppendFileOptions) {
    if path.is_null() || string.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let string = unsafe { CStr::from_ptr(string) };
    let string = string.to_string_lossy();
    if let Err(err) = node_fs::sync::append_file_with_path_str(path.as_ref(), string.as_ref(), options.into()).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_append_file_with_buffer_buffer_sync(
    dest: *mut Buffer,
    data: *const Buffer,
    options: AppendFileOptions,
) {
    if dest.is_null() || data.is_null() {
        return;
    }
    let dest = unsafe { &mut *dest };
    let data = unsafe { &*data };

    if let Err(err) = node_fs::sync::append_file_with_buffer_buffer(&mut dest.0, &data.0, options.into()).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}


#[no_mangle]
pub extern "C" fn fs_append_file_with_buffer_string_sync(
    dest: *mut Buffer,
    data: *const c_char,
    options: AppendFileOptions,
) {
    if dest.is_null() || data.is_null() {
        return;
    }
    let dest = unsafe { &mut *dest };
    let data = unsafe { CStr::from_ptr(data) };
    let data = data.to_string_lossy();

    if let Err(err) = node_fs::sync::append_file_with_buffer_string(&mut dest.0, data.as_ref(), options.into()).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}


#[no_mangle]
pub extern "C" fn fs_chmod_sync(path: *const c_char, mode: u32) {
    if path.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();
    if let Err(err) = node_fs::sync::chmod(path.as_ref(), mode).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_chown_sync(path: *const c_char, uid: u32, gid: u32) {
    if path.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    if let Err(err) = node_fs::sync::chown(path.as_ref(), uid, gid).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_close_sync(fd: i32) {
    node_fs::sync::close_fd(fd);
}

#[no_mangle]
pub extern "C" fn fs_copy_file_sync(src: *const c_char, dest: *const c_char, flags: u32) {
    if src.is_null() || dest.is_null() {
        return;
    }
    let src = unsafe { CStr::from_ptr(src) };
    let src = src.to_string_lossy();


    let dest = unsafe { CStr::from_ptr(dest) };
    let dest = dest.to_string_lossy();


    if let Err(err) = node_fs::sync::copy_file(src.as_ref(), dest.as_ref(), flags).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_cp_sync(src: *const c_char, dest: *const c_char, flags: u32) {
    if src.is_null() || dest.is_null() {
        return;
    }
    let src = unsafe { CStr::from_ptr(src) };
    let src = src.to_string_lossy();


    let dest = unsafe { CStr::from_ptr(dest) };
    let dest = dest.to_string_lossy();

    node_fs::sync::cp(src.as_ref(), dest.as_ref(), flags);
}

#[no_mangle]
pub extern "C" fn fs_exists_sync(src: *const c_char) -> bool {
    if src.is_null() {
        return false;
    }
    let src = unsafe { CStr::from_ptr(src) };
    let src = src.to_string_lossy();

    node_fs::sync::exists(src.as_ref())
}

#[no_mangle]
pub extern "C" fn fs_fchmod_sync(fd: i32, mode: u32) {
    if let Err(err) = node_fs::sync::fchmod(fd, mode as u16).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_fchown_sync(fd: i32, uid: u32, gid: u32) {
    if let Err(err) = node_fs::sync::fchown(fd, uid, gid).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_fdatasync_sync(fd: i32) {
    if let Err(err) = node_fs::sync::fdatasync(fd).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_fstat_sync(fd: i32) -> *mut FileStat {
    match node_fs::sync::fstat(fd).map(|metadata| {
        handle_meta(&metadata)
    })
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        Ok(stat) => {
            Box::into_raw(
                Box::new(FileStat::from(stat))
            )
        }
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_fsync_sync(fd: i32) {
    if let Err(err) = node_fs::sync::fsync(fd).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_ftruncate_sync(fd: i32, len: usize) {
    if let Err(err) = node_fs::sync::ftruncate(fd, len.try_into().unwrap()).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_futimes_sync(fd: i32, atime: usize, mtime: usize) {
    if let Err(err) = node_fs::sync::futimes(fd, atime.try_into().unwrap(), mtime.try_into().unwrap()).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_lchmod_sync(path: *const c_char, mode: u32) {
    if path.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    if let Err(err) = node_fs::sync::chmod(path.as_ref(), mode).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_lchown_sync(path: *const c_char, uid: u32, gid: u32) {
    if path.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    if let Err(err) = node_fs::sync::chown(path.as_ref(), uid, gid).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_lutimes_sync(path: *const c_char, atime: i64, mtime: i64) {
    if path.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    if let Err(err) = node_fs::sync::lutimes(path.as_ref(), atime.try_into().unwrap(), mtime.try_into().unwrap()).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_link_sync(existing_path: *const c_char, new_path: *const c_char) {
    if existing_path.is_null() || new_path.is_null() {
        return;
    }
    let existing_path = unsafe { CStr::from_ptr(existing_path) };
    let existing_path = existing_path.to_string_lossy();

    let new_path = unsafe { CStr::from_ptr(new_path) };
    let new_path = new_path.to_string_lossy();

    if let Err(err) = node_fs::sync::link(existing_path.as_ref(), new_path.as_ref()).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_lstat_sync(path: *const c_char) -> *mut FileStat {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    match node_fs::sync::lstat(path.as_ref())
        .map(|metadata| {
            handle_meta(&metadata)
        })
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        Ok(stat) => {
            Box::into_raw(
                Box::new(FileStat::from(stat))
            )
        }
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_mkdir_sync(path: *const c_char, options: MkDirOptions) {
    if path.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    if let Err(err) = node_fs::sync::mkdir(path.as_ref(), options.into()).map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_mkdtemp_sync(prefix: *const c_char, options: MkdTempOptions) -> *const c_char {
    if prefix.is_null() {
        return std::ptr::null();
    }
    let prefix = unsafe { CStr::from_ptr(prefix) };
    let prefix = prefix.to_string_lossy();

    match node_fs::sync::mkdtemp(prefix.as_ref(), options.into())
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        Ok(path) => {
            CString::new(path).unwrap().into_raw()
        }
        Err(err) => {
            update_last_error(err);
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_open_sync(path: *const c_char, flag: i32, mode: i32) -> c_int {
    if path.is_null() {
        return 0;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    match node_fs::sync::open(path.as_ref(), flag, mode)
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        Ok(fd) => fd,
        Err(err) => {
            update_last_error(err);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_opendir_sync(path: *const c_char, options: OpenDirOptions) -> *mut FileDir {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();
    match node_fs::sync::opendir(path.as_ref(), options.into())
        .map(|dir| {
            Box::new(FileDir(dir))
        })
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        Ok(dir) => { Box::into_raw(dir) }
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}


#[cfg(not(windows))]
#[no_mangle]
pub extern "C" fn fs_read_sync(
    fd: i32,
    buffer: *mut u8,
    buffer_length: usize,
    offset: usize,
    length: usize,
    position: isize,
) -> usize {
    let buffer = unsafe { std::slice::from_raw_parts_mut(buffer, buffer_length) };
    node_fs::sync::read(fd, buffer, offset, length, position).map_err(|e| node_core::error::error_from_io_error(e)).unwrap_or_else(|err| {
        update_last_error(err);
        0
    })
}

#[cfg(windows)]
#[no_mangle]
pub extern "C" fn fs_read_sync(
    fd: i64,
    buffer: *mut u8,
    buffer_length: usize,
    offset: usize,
    length: usize,
    position: isize,
) -> usize {
    unsafe {
        let buffer = std::slice::from_raw_parts_mut(buffer, buffer_length);
        let fd = fd as *mut std::ffi::c_void;
        node_fs::sync::read(fd, buffer, offset, length, position).map_err(|e| node_core::error::error_from_io_error(e)).unwrap_or_else(|err| {
            update_last_error(err);
            0
        })
    }
}


#[repr(C)]
pub struct ReaddirResultArray {
    data: *mut ReaddirResult,
    length: usize,
}

impl Drop for ReaddirResultArray {
    fn drop(&mut self) {
        let _ = unsafe { Vec::from_raw_parts(self.data, self.length, self.length) };
    }
}


#[no_mangle]
pub extern "C" fn fs_readdir_result_array_destroy(value: *mut ReaddirResultArray) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[no_mangle]
pub extern "C" fn fs_readdir_sync(path: *const c_char, options: ReaddirOptions) -> *mut ReaddirResultArray {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    match node_fs::sync::readdir(path.as_ref(), options.into())
        .map(|mut value| {
            value.into_iter()
                .map(|value| ReaddirResult(value))
                .collect::<Vec<ReaddirResult>>()
        })
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        Ok(mut result) => {
            let ptr = result.as_mut_ptr();
            let len = result.len();
            result.shrink_to_fit();
            std::mem::forget(result);
            let ret = ReaddirResultArray {
                data: ptr,
                length: len,
            };
            Box::into_raw(
                Box::new(ret)
            )
        }
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FsEncoding(node_fs::FsEncoding);


#[no_mangle]
pub extern "C" fn fs_encoding_destroy(value: *mut FsEncoding) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

impl FsEncoding {
    pub fn is_buffer(&self) -> bool {
        match &self.0 {
            node_fs::FsEncoding::String(_) => false,
            node_fs::FsEncoding::Buffer(_) => true
        }
    }
    pub fn get_string_value(&self) -> Result<String> {
        match self.0.get_string_value() {
            Some(value) => { Ok(value.to_string_lossy().to_string()) }
            None => {
                Err(node_core::error::generic_error("Invalid Type".to_string()))
            }
        }
    }

    pub fn get_buffer_value(&self) -> Result<Box<Buffer>> {
        match self.0.get_buffer_value() {
            None => {
                Err(node_core::error::generic_error("Invalid Type".to_string()))
            }
            Some(buffer) => Ok(Box::new(Buffer(buffer)))
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_read_file_sync(path: *const c_char, options: ReadFileOptions) -> *mut FsEncoding {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    match node_fs::sync::read_file(path.as_ref(), options.into())
        .map(|f| Box::new(FsEncoding(f)))
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        Ok(encoding) => Box::into_raw(encoding),
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_read_file_with_fd_sync(fd: i32, options: ReadFileOptions) -> *mut FsEncoding {
    match node_fs::sync::read_file_with_fd(fd, options.into())
        .map(|f| Box::new(FsEncoding(f)))
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        Ok(encoding) => Box::into_raw(encoding),
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_read_link_sync(path: *const c_char, options: ReadLinkOptions) -> *mut FsEncoding {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    match node_fs::sync::read_link(path.as_ref(), options.into())
        .map(|f| Box::new(FsEncoding(f)))
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        Ok(encoding) => Box::into_raw(encoding),
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_readv_sync(fd: i32, buffers: *mut *mut Buffer, length: usize, position: i64) -> usize {
    let buffers = unsafe { std::slice::from_raw_parts_mut(buffers, length) };

    let mut buffers = buffers.iter().map(|buffer| {
        let buffer = unsafe { &mut **buffer };
        buffer.0.clone()
    })
        .collect::<Vec<node_buffer::Buffer>>();

    node_fs::sync::readv(fd, buffers.as_mut_slice(), position.try_into().unwrap())
        .map_err(|e| node_core::error::error_from_io_error(e)).unwrap_or_else(|err| {
        update_last_error(err);
        0
    })
}

#[no_mangle]
pub extern "C" fn fs_readv_sync_slice(fd: i32, buffers: *const *mut u8, buffers_buffers: *const usize, length: usize, position: i64) -> usize {
    let buffer_length = unsafe { std::slice::from_raw_parts(buffers_buffers, length) };
    let buffers = unsafe { std::slice::from_raw_parts(buffers, length) };

    let mut buffers = buffers.iter().zip(buffer_length.iter())
        .map(|(buffer, len)| {
            unsafe { node_buffer::Buffer::from_reference(*buffer, *len) }
        }).collect::<Vec<node_buffer::Buffer>>();

    node_fs::sync::readv(fd, buffers.as_mut_slice(), position.try_into().unwrap())
        .map_err(|e| node_core::error::error_from_io_error(e)).unwrap_or_else(|err| {
        update_last_error(err);
        0
    })
}

#[no_mangle]
pub extern "C" fn fs_real_path_sync(path: *const c_char, options: RealPathOptions) -> *const c_char {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    match node_fs::sync::real_path(path.as_ref(), options.into())
        .map(|v| v.to_string_lossy().to_string())
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        Ok(path) => CString::new(path).unwrap().into_raw(),
        Err(err) => {
            update_last_error(err);
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_rename_sync(old_path: *const c_char, new_path: *const c_char) {
    if old_path.is_null() || new_path.is_null() {
        return;
    }
    let old_path = unsafe { CStr::from_ptr(old_path) };
    let old_path = old_path.to_string_lossy();

    let new_path = unsafe { CStr::from_ptr(new_path) };
    let new_path = new_path.to_string_lossy();

    if let Err(err) = node_fs::sync::rename(old_path.as_ref(), new_path.as_ref())
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_rmdir_sync(
    path: *const c_char,
    options: RmDirOptions,
) {
    if path.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    if let Err(err) = node_fs::sync::rmdir(path.as_ref(), options.into()) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_rm_sync(
    path: *const c_char,
    options: RmOptions,
) {
    if path.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    if let Err(err) = node_fs::sync::rm(path.as_ref(), options.into()) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_stat_sync(path: *const c_char) -> *mut FileStat {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    match node_fs::sync::stat(path.as_ref())
        .map(|metadata| {
            handle_meta(&metadata)
        })
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        Ok(stat) => Box::into_raw(Box::new(FileStat::from(stat))),
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_symlink_sync(target: *const c_char, path: *const c_char, _type_: *const c_char) {
    if target.is_null() || path.is_null() || _type_.is_null() {
        return;
    }

    let target = unsafe { CStr::from_ptr(target) };
    let target = target.to_string_lossy();

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let _type_ = unsafe { CStr::from_ptr(_type_) };
    let _type_ = _type_.to_string_lossy();

    if let Err(err) = node_fs::sync::symlink(target.as_ref(), path.as_ref(), _type_.as_ref())
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_truncate_sync(path: *const c_char, len: u64) {
    if path.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    if let Err(err) = node_fs::sync::truncate(path.as_ref(), len.try_into().unwrap())
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_unlink_sync(path: *const c_char) {
    if path.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    if let Err(err) = node_fs::sync::unlink(path.as_ref())
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_utimes_sync(path: *const c_char, atime: i64, mtime: i64) {
    if path.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    if let Err(err) = node_fs::sync::utimes(path.as_ref(), atime.try_into().unwrap(), mtime.try_into().unwrap())
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_write_sync(
    fd: i32,
    buffer: *const u8,
    length: usize,
    options: WriteOptions,
) -> usize {
    let buffer = unsafe { std::slice::from_raw_parts(buffer, length) };
    node_fs::sync::write(fd, buffer, options.into())
        .map_err(|e| node_core::error::error_from_io_error(e)).unwrap_or_else(|err| {
        update_last_error(err);
        0
    })
}

#[no_mangle]
pub extern "C" fn fs_write_string_sync(
    fd: i32,
    string: *const c_char,
    encoding: StringEncoding,
    position: isize,
) -> usize {
    if string.is_null() {
        return 0;
    }
    let string = unsafe { CStr::from_ptr(string) };
    let string = string.to_string_lossy();

    node_fs::sync::write_string(
        fd, string.as_ref(), encoding.into(), position,
    ).map_err(|e| node_core::error::error_from_io_error(e)).unwrap_or_else(|err| {
        update_last_error(err);
        0
    })
}

#[no_mangle]
pub extern "C" fn fs_write_file_with_str_sync(fd: i32, data: *const c_char, options: WriteFileOptions) {
    if data.is_null() {
        return;
    }
    let data = unsafe { CStr::from_ptr(data) };
    let data = data.to_string_lossy();

    if let Err(err) = node_fs::sync::write_file_with_str(fd, data.as_ref(), options.into())
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_write_file_with_bytes_sync(fd: i32, data: *const u8, length: usize, options: WriteFileOptions) {
    if data.is_null() || length == 0 {
        return;
    }
    let data = unsafe { std::slice::from_raw_parts(data, length) };
    if let Err(err) = node_fs::sync::write_file_with_bytes(fd, data, options.into())
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_write_file_with_str_from_path_sync(
    path: *const c_char,
    data: *const c_char,
    options: WriteFileOptions,
) {
    if path.is_null() || data.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let data = unsafe { CStr::from_ptr(data) };
    let data = data.to_string_lossy();

    if let Err(err) = node_fs::sync::write_file_with_str_from_path(
        path.as_ref(), data.as_ref(), options.into(),
    )
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_write_file_with_bytes_from_path_sync(
    path: *const c_char,
    data: *const u8,
    length: usize,
    options: WriteFileOptions,
) {
    if path.is_null() || data.is_null() || length == 0 {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let data = unsafe { std::slice::from_raw_parts(data, length) };

    if let Err(err) = node_fs::sync::write_file_with_bytes_from_path(
        path.as_ref(), data, options.into(),
    )
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_write_file_with_buffer_from_path_sync(
    path: *const c_char,
    data: *const Buffer,
    options: WriteFileOptions,
) {
    if path.is_null() || data.is_null() {
        return;
    }
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let data = unsafe { &*data };

    if let Err(err) = node_fs::sync::write_file_with_buffer_from_path(
        path.as_ref(), &data.0, options.into(),
    )
        .map_err(|e| node_core::error::error_from_io_error(e)) {
        update_last_error(err);
    }
}

#[no_mangle]
pub extern "C" fn fs_writev_sync(fd: i32, buffers: *mut *mut Buffer, length: usize, position: i64) -> usize {
    let buffers = unsafe { std::slice::from_raw_parts_mut(buffers, length) };
    let buffers = buffers.iter().map(|buffer| {
        let buffer = unsafe { &mut **buffer };
        buffer.0.clone()
    }).collect::<Vec<node_buffer::Buffer>>();
    node_fs::sync::writev(
        fd, buffers, position.try_into().unwrap(),
    )
        .map_err(|e| node_core::error::error_from_io_error(e)).unwrap_or_else(|err| {
        update_last_error(err);
        0
    })
}


#[no_mangle]
pub extern "C" fn fs_writev_sync_slice(fd: i32, buffers: *const *const u8, buffers_buffers: *const usize, length: usize, position: i64) -> usize {
    let buffer_length = unsafe { std::slice::from_raw_parts(buffers_buffers, length) };
    let buffers = unsafe { std::slice::from_raw_parts(buffers, length) };

    let buffers = buffers.iter().zip(buffer_length.iter())
        .map(|(buffer, len)| {
            unsafe { std::slice::from_raw_parts(*buffer, *len) }
        }).collect::<Vec<_>>();


    node_fs::sync::writev_slice(
        fd, buffers.as_slice(), position.try_into().unwrap(),
    )
        .map_err(|e| node_core::error::error_from_io_error(e)).unwrap_or_else(|err| {
        update_last_error(err);
        0
    })
}

// async

#[no_mangle]
pub extern "C" fn fs_async_access(path: *const c_char, access: i32, callback: *const AsyncClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::access(path.as_ref(), access, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_append_file_with_str(fd: i32, data: *const c_char, options: AppendFileOptions, callback: *const AsyncClosure) {
    if data.is_null() || callback.is_null() {
        return;
    }

    let data = unsafe { CStr::from_ptr(data) };
    let data = data.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::append_file_with_str(fd, data.as_ref(), options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_append_file_with_bytes(fd: i32, data: *const Buffer, options: AppendFileOptions, callback: *const AsyncClosure) {
    if data.is_null() || callback.is_null() {
        return;
    }

    let data = unsafe { &*data };

    let callback = unsafe { &*callback };
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::append_file_with_bytes(fd, &data.0, options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_append_file_with_path_str(
    path: *const c_char,
    data: *const c_char,
    options: AppendFileOptions,
    callback: *const AsyncClosure,
) {
    if path.is_null() || data.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let data = unsafe { CStr::from_ptr(data) };
    let data = data.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::append_file_with_path_str(path.as_ref(), data.as_ref(), options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_append_file_with_path_bytes(
    path: *const c_char,
    data: *const Buffer,
    options: AppendFileOptions,
    callback: *const AsyncClosure,
) {
    if path.is_null() || data.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let data = unsafe { &*data };

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::append_file_with_path_bytes(path.as_ref(), &data.0, options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_chmod(path: *const c_char, mode: u32, callback: *const AsyncClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::chmod(path.as_ref(), mode, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_chown(path: *const c_char, uid: u32, gid: u32, callback: *const AsyncClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    node_fs::a_sync::chown(path.as_ref(), uid, gid, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_close(fd: i32, callback: *const AsyncClosure) {
    if callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::close(fd, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_copy_file(src: *const c_char, dest: *const c_char, flag: u32, callback: *const AsyncClosure) {
    if src.is_null() || dest.is_null() || callback.is_null() {
        return;
    }

    let src = unsafe { CStr::from_ptr(src) };
    let src = src.to_string_lossy();

    let dest = unsafe { CStr::from_ptr(dest) };
    let dest = dest.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::copy_file(src.as_ref(), dest.as_ref(), flag, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_cp(_src: *const c_char, _dest: *const c_char) {}

#[no_mangle]
pub extern "C" fn fs_async_exists(path: *const c_char, callback: *const AsyncBoolClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(result)
            }
        }))
    );
    node_fs::a_sync::exists(path.as_ref(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_fchmod(fd: i32, mode: u16, callback: *const AsyncClosure) {
    if callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::fchmod(fd, mode, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_fchown(fd: i32, uid: u32, gid: u32, callback: *const AsyncClosure) {
    if callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::fchown(fd, uid, gid, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_fdatasync(fd: i32, callback: *const AsyncClosure) {
    if callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::fdatasync(fd, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_fstat(fd: i32, callback: *const AsyncFileStatClosure) {
    if callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(result.map(|stat: node_fs::file_stat::FileStat| stat.into()))
            }
        }))
    );
    node_fs::a_sync::fstat(fd, cb);
}

#[no_mangle]
pub extern "C" fn fs_async_fsync(fd: i32, callback: *const AsyncClosure) {
    if callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::fdatasync(fd, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_ftruncate(fd: i32, len: i64, callback: *const AsyncClosure) {
    if callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::ftruncate(fd, len.try_into().unwrap(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_futimes(fd: i32, atime: i64, mtime: i64, callback: *const AsyncClosure) {
    if callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::futimes(fd, atime.try_into().unwrap(), mtime.try_into().unwrap(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_lchmod(path: *const c_char, mode: u16, callback: *const AsyncClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::lchmod(path.as_ref(), mode, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_lchown(path: *const c_char, uid: u32, gid: u32, callback: *const AsyncClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::lchown(path.as_ref(), uid, gid, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_lutimes(path: *const c_char, atime: i64, mtime: i64, callback: *const AsyncClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::lutimes(path.as_ref(), atime.try_into().unwrap(), mtime.try_into().unwrap(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_link(existing_path: *const c_char, new_path: *const c_char, callback: *const AsyncClosure) {
    if existing_path.is_null() || new_path.is_null() || callback.is_null() {
        return;
    }

    let existing_path = unsafe { CStr::from_ptr(existing_path) };
    let existing_path = existing_path.to_string_lossy();

    let new_path = unsafe { CStr::from_ptr(new_path) };
    let new_path = new_path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::link(existing_path.as_ref(), new_path.as_ref(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_lstat(path: *const c_char, callback: *const AsyncFileStatClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(result.map(|stat: node_fs::file_stat::FileStat| stat.into()))
            }
        }))
    );

    node_fs::a_sync::lstat(path.as_ref(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_mkdir(path: *const c_char, options: MkDirOptions, callback: *const AsyncClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::mkdir(path.as_ref(), options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_mkdtemp(prefix: *const c_char, options: MkdTempOptions, callback: *const AsyncStringClosure) {
    if prefix.is_null() || callback.is_null() {
        return;
    }

    let prefix = unsafe { CStr::from_ptr(prefix) };
    let prefix = prefix.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |path, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(path.map(|path: std::path::PathBuf| path.to_string_lossy().to_string()))
            }
        }))
    );
    node_fs::a_sync::mkdtemp(prefix.as_ref(), options.into(), cb);
}

#[no_mangle]
pub extern "C" fn fs_async_open(path: *const c_char, flag: i32, mode: i32, callback: *const AsyncI32Closure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error: Option<std::io::Error>| {
            if error.is_some() {
                let error = error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError);

                callback.on_error(error)
            } else {
                callback.on_success(result)
            }
        }))
    );
    node_fs::a_sync::open(path.as_ref(), flag, mode, cb);
}


#[no_mangle]
pub extern "C" fn fs_async_open_handle(path: *const c_char, flag: i32, mode: i32, callback: *const AsyncFileHandleClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error: Option<std::io::Error>| {
            if error.is_some() {
                let error = error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError);

                callback.on_error(error)
            } else {
                callback.on_success(result.map(FileHandle))
            }
        }))
    );

    node_fs::file_handle::FileHandle::new_async(path.as_ref(), flag, mode, cb);
}

#[no_mangle]
pub extern "C" fn fs_async_opendir(path: *const c_char, options: OpenDirOptions, callback: *const AsyncFileDirClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(result.map(FileDir))
            }
        }))
    );
    node_fs::a_sync::opendir(path.as_ref(), options.into(), cb);
}

#[no_mangle]
pub extern "C" fn fs_async_read(
    fd: i32,
    buffer: *mut Buffer,
    offset: usize,
    length: usize,
    position: isize,
    callback: *const AsyncUsizeClosure,
) {
    if buffer.is_null() || callback.is_null() {
        return;
    }

    let buffer = unsafe { &mut *buffer };

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(result)
            }
        }))
    );

    node_fs::a_sync::read(fd, &mut buffer.0, offset, length, position, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_readdir(
    path: *const c_char,
    options: ReaddirOptions,
    callback: *const AsyncReaddirClosure,
) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result: Option<Vec<node_fs::sync::ReaddirResult>>, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(result.map(|result| result.into_iter()
                    .map(|value| ReaddirResult(value))
                    .collect::<Vec<ReaddirResult>>()))
            }
        }))
    );

    node_fs::a_sync::readdir(path.as_ref(), options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_read_file(path: *const c_char, options: ReadFileOptions, callback: *const AsyncFsEncodingClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(result.map(FsEncoding))
            }
        }))
    );

    node_fs::a_sync::read_file(path.as_ref(), options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_read_file_with_fd(fd: i32, options: ReadFileOptions, callback: *const AsyncFsEncodingClosure) {
    if callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(result.map(FsEncoding))
            }
        }))
    );

    node_fs::a_sync::read_file_with_fd(fd, options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_read_link(path: *const c_char, options: ReadLinkOptions, callback: *const AsyncFsEncodingClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(result.map(FsEncoding))
            }
        }))
    );

    node_fs::a_sync::read_link(path.as_ref(), options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_readv(
    fd: i32,
    buffers: *const Buffer,
    length: usize,
    position: usize,
    callback: *const AsyncUsizeClosure,
) {
    if buffers.is_null() || length == 0 || callback.is_null() {
        return;
    }

    let buffers = unsafe { std::slice::from_raw_parts(buffers, length) };

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(result)
            }
        }))
    );

    let buffers = buffers.into_iter().map(|buffer| buffer.0.clone()).collect::<Vec<node_buffer::Buffer>>();

    node_fs::a_sync::readv(fd, buffers, position.try_into().unwrap(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_real_path(path: *const c_char, options: RealPathOptions, callback: *const AsyncStringClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result: Option<std::path::PathBuf>, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(result.map(|result| result.to_string_lossy().to_string()))
            }
        }))
    );

    node_fs::a_sync::real_path(path.as_ref(), options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_rename(old_path: *const c_char, new_path: *const c_char, callback: *const AsyncClosure) {
    if old_path.is_null() || new_path.is_null() || callback.is_null() {
        return;
    }

    let old_path = unsafe { CStr::from_ptr(old_path) };
    let old_path = old_path.to_string_lossy();

    let new_path = unsafe { CStr::from_ptr(new_path) };
    let new_path = new_path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::rename(old_path.as_ref(), new_path.as_ref(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_rmdir(
    path: *const c_char,
    options: RmDirOptions,
    callback: *const AsyncClosure,
) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::rmdir(path.as_ref(), options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_rm(
    path: *const c_char,
    options: RmOptions,
    callback: *const AsyncClosure,
) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::rm(path.as_ref(), options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_stat(path: *const c_char, throw_if_no_entry: bool, callback: *const AsyncFileStatClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result: Option<node_fs::file_stat::FileStat>, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(result.map(|result| result.into()))
            }
        }))
    );
    node_fs::a_sync::stat(path.as_ref(), throw_if_no_entry, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_symlink(target: *const c_char, path: *const c_char, type_: *const c_char, callback: *const AsyncClosure) {
    if target.is_null() || path.is_null() || type_.is_null() || callback.is_null() {
        return;
    }

    let target = unsafe { CStr::from_ptr(target) };
    let target = target.to_string_lossy();

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();


    let type_ = unsafe { CStr::from_ptr(type_) };
    let type_ = type_.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::symlink(target.as_ref(), path.as_ref(), type_.as_ref(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_truncate(path: *const c_char, len: u64, callback: *const AsyncClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::truncate(path.as_ref(), len, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_unlink(path: *const c_char, callback: *const AsyncClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::unlink(path.as_ref(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_unwatch_file(filename: *const c_char) {
    if filename.is_null() {
        return;
    }

    let filename = unsafe { CStr::from_ptr(filename) };
    let filename = filename.to_string_lossy();

    node_fs::a_sync::unwatch_file(filename.as_ref(), None)
}

#[no_mangle]
pub extern "C" fn fs_async_unwatch_file_with_callback(filename: *const c_char, callback: *const AsyncFileWatchClosure) {
    if filename.is_null() || callback.is_null() {
        return;
    }

    let filename = unsafe { CStr::from_ptr(filename) };
    let filename = filename.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(event.map(FileWatchEvent))
            }
        }))
    );
    node_fs::a_sync::unwatch_file(filename.as_ref(), Some(cb))
}

#[no_mangle]
pub extern "C" fn fs_async_utimes(path: *const c_char, atime: i64, mtime: i64, callback: *const AsyncClosure) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::utimes(path.as_ref(), atime.try_into().unwrap(), mtime.try_into().unwrap(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_file_watcher_unref(filename: *const c_char, callback: *const AsyncFileWatchClosure) {
    if filename.is_null() || callback.is_null() {
        return;
    }

    let filename = unsafe { CStr::from_ptr(filename) };
    let filename = filename.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(event.map(FileWatchEvent))
            }
        }))
    );
    node_fs::a_sync::file_watcher_unref(filename.as_ref(), cb);
}

#[no_mangle]
pub extern "C" fn fs_async_file_watcher_ref(filename: *const c_char, callback: *const AsyncFileWatchClosure) {
    if filename.is_null() || callback.is_null() {
        return;
    }

    let filename = unsafe { CStr::from_ptr(filename) };
    let filename = filename.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(event.map(FileWatchEvent))
            }
        }))
    );
    node_fs::a_sync::file_watcher_ref(filename.as_ref(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_watch(
    filename: *const c_char,
    persistent: bool,
    recursive: bool,
    encoding: FsEncodingType,
    callback: *const AsyncWatchClosure,
) {
    if filename.is_null() || callback.is_null() {
        return;
    }

    let filename = unsafe { CStr::from_ptr(filename) };
    let filename = filename.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(event.map(WatchEvent))
            }
        }))
    );

    node_fs::a_sync::watch(
        filename.as_ref(), persistent, recursive, encoding.into(), cb,
    )
}

#[no_mangle]
pub extern "C" fn fs_async_watcher_unref(filename: *const c_char, callback: *const AsyncWatchClosure) {
    if filename.is_null() || callback.is_null() {
        return;
    }

    let filename = unsafe { CStr::from_ptr(filename) };
    let filename = filename.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(event.map(WatchEvent))
            }
        }))
    );

    node_fs::a_sync::watcher_unref(filename.as_ref(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_watcher_ref(filename: *const c_char, callback: *const AsyncWatchClosure) {
    if filename.is_null() || callback.is_null() {
        return;
    }

    let filename = unsafe { CStr::from_ptr(filename) };
    let filename = filename.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(event.map(WatchEvent))
            }
        }))
    );
    node_fs::a_sync::watcher_ref(filename.as_ref(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_watcher_close(
    filename: *const c_char,
    callback: *const AsyncWatchClosure,
    on_close: *const AsyncClosure,
) {
    if filename.is_null() || callback.is_null() || on_close.is_null() {
        return;
    }

    let filename = unsafe { CStr::from_ptr(filename) };
    let filename = filename.to_string_lossy();

    let callback = unsafe { &*callback };

    let on_close = unsafe { &*on_close };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(event.map(WatchEvent))
            }
        }))
    );


    let on_close = Arc::clone(&on_close.0);
    let on_close_cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                on_close.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                on_close.on_success(None)
            }
        }))
    );


    node_fs::a_sync::watcher_close(filename.as_ref(), cb, on_close_cb)
}

#[no_mangle]
pub extern "C" fn fs_async_watch_file(
    filename: *const c_char,
    bigint: bool,
    persistent: bool,
    interval: u64,
    encoding: FsEncodingType,
    callback: *const AsyncFileWatchClosure,
) {
    if filename.is_null() || callback.is_null() {
        return;
    }

    let filename = unsafe { CStr::from_ptr(filename) };
    let filename = filename.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(event.map(FileWatchEvent))
            }
        }))
    );

    node_fs::a_sync::watch_file(filename.as_ref(), bigint, persistent, interval.try_into().unwrap(), encoding.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_write(
    fd: i32,
    buffer: *const Buffer,
    options: WriteOptions,
    callback: *const AsyncUsizeClosure,
) {
    if buffer.is_null() || callback.is_null() {
        return;
    }

    let buffer = unsafe { &*buffer };

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |wrote, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(wrote)
            }
        }))
    );

    node_fs::a_sync::write(fd, &buffer.0, options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_write_string(
    fd: i32,
    string: *const c_char,
    encoding: StringEncoding,
    position: isize,
    callback: *const AsyncUsizeClosure,
) {
    if string.is_null() || callback.is_null() {
        return;
    }

    let string = unsafe { CStr::from_ptr(string) };
    let string = string.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |wrote, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(wrote)
            }
        }))
    );
    node_fs::a_sync::write_string(fd, string.as_ref(), encoding.into(), position, cb)
}

#[no_mangle]
pub extern "C" fn fs_async_write_file_with_str(
    fd: i32,
    data: *const c_char,
    options: WriteFileOptions,
    callback: *const AsyncClosure,
) {
    if data.is_null() || callback.is_null() {
        return;
    }

    let data = unsafe { CStr::from_ptr(data) };
    let data = data.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::write_file_with_str(fd, data.as_ref(), options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_write_file_with_bytes(fd: i32, data: *const Buffer, options: WriteFileOptions, callback: *const AsyncClosure) {
    if data.is_null() || callback.is_null() {
        return;
    }

    let data = unsafe { &*data };

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::write_file_with_bytes(fd, &data.0, options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_write_file_with_str_from_path(
    path: *const c_char,
    data: *const c_char,
    options: WriteFileOptions,
    callback: *const AsyncClosure,
) {
    if path.is_null() || data.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let data = unsafe { CStr::from_ptr(data) };
    let data = data.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::write_file_with_str_from_path(
        path.as_ref(), data.as_ref(), options.into(), cb,
    )
}

#[no_mangle]
pub extern "C" fn fs_async_write_file_with_bytes_from_path(
    path: *const c_char,
    data: *const Buffer,
    options: WriteFileOptions,
    callback: *const AsyncClosure,
) {
    if path.is_null() || data.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let data = unsafe { &*data };

    let callback = unsafe { &*callback };


    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::write_file_with_bytes_from_path(path.as_ref(), &data.0, options.into(), cb)
}

#[no_mangle]
pub extern "C" fn fs_async_writev(
    fd: i32,
    buffers: *const Buffer,
    length: usize,
    position: usize,
    callback: *const AsyncUsizeClosure,
) {
    if buffers.is_null() || length == 0 || callback.is_null() {
        return;
    }

    let buffers = unsafe { std::slice::from_raw_parts(buffers, length) };

    let callback = unsafe { &*callback };


    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |wrote, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(wrote)
            }
        }))
    );

    let buffers = buffers.iter().map(|buffer| buffer.0.clone())
        .collect::<Vec<node_buffer::Buffer>>();

    node_fs::a_sync::writev(fd, buffers, position.try_into().unwrap(), cb)
}


// fs filehandle

#[no_mangle]
pub extern "C" fn fs_handle_new_async(
    path: *const c_char,
    flags: i32,
    mode: i32,
    callback: *const AsyncFileHandleClosure,
) {
    if path.is_null() || callback.is_null() {
        return;
    }

    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();

    let callback = unsafe { &*callback };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |handle, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(handle
                    .map(FileHandle))
            }
        }))
    );

    node_fs::file_handle::FileHandle::new_async(
        path.as_ref(), flags, mode, cb,
    )
}

#[no_mangle]
pub extern "C" fn fs_handle_append_file_with_str(
    handle: *mut FileHandle,
    data: *const c_char,
    options: AppendFileOptions,
    callback: *const AsyncClosure,
) {
    if handle.is_null() || data.is_null() || callback.is_null() {
        return;
    }

    let data = unsafe { CStr::from_ptr(data) };
    let data = data.to_string_lossy();

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    handle.0.append_file_with_str(
        data.as_ref(), options.into(), cb,
    )
}

#[no_mangle]
pub extern "C" fn fs_handle_append_file_with_bytes(
    handle: *mut FileHandle,
    data: *const Buffer,
    options: AppendFileOptions,
    callback: *const AsyncClosure,
) {
    if handle.is_null() || data.is_null() || callback.is_null() {
        return;
    }

    let data = unsafe { &*data };

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    handle.0.append_file_with_bytes(
        &data.0, options.into(), cb,
    )
}

#[no_mangle]
pub extern "C" fn fs_handle_chmod(handle: *mut FileHandle, mode: u16, callback: *const AsyncClosure) {
    if handle.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    handle.0.chmod(
        mode, cb,
    )
}

#[no_mangle]
pub extern "C" fn fs_handle_chown(handle: *mut FileHandle, uid: u32, gid: u32, callback: *const AsyncClosure) {
    if handle.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    handle.0.chown(
        uid, gid, cb,
    )
}

// consumes the handle, do not use after closing
#[no_mangle]
pub extern "C" fn fs_handle_close(handle: *mut FileHandle, callback: *const AsyncClosure) {
    if handle.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { Box::from_raw(handle) };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );


    handle.0.close(
        cb
    )
}

// TODO
// #[no_mangle]
// pub extern "C" fn fs_handle_create_read_stream() {}

// TODO
// #[no_mangle]
// pub extern "C" fn fs_handle_create_write_stream() {}

#[no_mangle]
pub extern "C" fn fs_handle_datasync(handle: *mut FileHandle, callback: *const AsyncClosure) {
    if handle.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    handle.0.datasync(
        cb
    )
}

#[no_mangle]
pub extern "C" fn fs_handle_fd(handle: *mut FileHandle) -> i32 {
    if handle.is_null() {
        return 0;
    }

    let handle = unsafe { &mut *handle };

    handle.0.fd()
}

#[no_mangle]
pub extern "C" fn fs_handle_read(
    handle: *mut FileHandle,
    buffer: *mut Buffer,
    offset: usize,
    length: usize,
    position: isize,
    callback: *const AsyncUsizeClosure,
) {
    if handle.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let buffer = unsafe { &mut *buffer };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |read, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(read)
            }
        }))
    );

    handle.0.read(
        &mut buffer.0, offset, length, position, cb,
    )
}


#[no_mangle]
pub extern "C" fn fs_handle_read_bytes(
    handle: *mut FileHandle,
    buffer: *mut u8,
    buffer_length: usize,
    offset: usize,
    length: usize,
    position: isize,
    callback: *const AsyncUsizeClosure,
) {
    if handle.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let buffer = unsafe { std::slice::from_raw_parts_mut(buffer, buffer_length) };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |read, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(read)
            }
        }))
    );

    handle.0.read_bytes(
        buffer, offset, length, position, cb,
    )
}

#[no_mangle]
pub extern "C" fn fs_handle_read_file(
    handle: *mut FileHandle,
    options: ReadFileOptions,
    callback: *const AsyncFsEncodingClosure,
) {
    if handle.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(result.map(FsEncoding))
            }
        }))
    );

    handle.0.read_file(
        options.into(), cb,
    )
}


#[no_mangle]
pub extern "C" fn fs_handle_readv_slice(handle: *mut FileHandle, buffers: *const *mut u8, buffers_buffers: *const usize, length: usize, position: i64, callback: *const AsyncUsizeClosure) {
    if handle.is_null() || buffers.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };


    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |read, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(read)
            }
        }))
    );

    let buffer_length = unsafe { std::slice::from_raw_parts(buffers_buffers, length) };
    let buffers = unsafe { std::slice::from_raw_parts(buffers, length) };

    let buffers = buffers.iter().zip(buffer_length.iter())
        .map(|(buffer, len)| {
            unsafe { node_buffer::Buffer::from_reference(*buffer, *len) }
        }).collect::<Vec<node_buffer::Buffer>>();

    handle.0.readv(
        buffers, position.try_into().unwrap(), cb,
    )
}


#[no_mangle]
pub extern "C" fn fs_handle_readv(
    handle: *mut FileHandle,
    buffers: *const Buffer,
    length: usize,
    position: usize,
    callback: *const AsyncUsizeClosure,
) {
    if handle.is_null() || buffers.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let buffers = unsafe { std::slice::from_raw_parts(buffers, length) };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |read, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(read)
            }
        }))
    );

    let buffers = buffers.iter().map(|buffer| buffer.0.clone())
        .collect::<Vec<node_buffer::Buffer>>();

    handle.0.readv(
        buffers, position.try_into().unwrap(), cb,
    )
}

#[no_mangle]
pub extern "C" fn fs_handle_stat(handle: *mut FileHandle, callback: *const AsyncFileStatClosure) {
    if handle.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |stat: Option<node_fs::file_stat::FileStat>, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(stat.map(|stat| stat.into()))
            }
        }))
    );

    handle.0.stat(
        cb
    )
}

#[no_mangle]
pub extern "C" fn fs_handle_sync(handle: *mut FileHandle, callback: *const AsyncClosure) {
    if handle.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    handle.0.sync(
        cb
    )
}

#[no_mangle]
pub extern "C" fn fs_handle_truncate(handle: *mut FileHandle, len: usize, callback: *const AsyncClosure) {
    if handle.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    handle.0.truncate(
        len.try_into().unwrap(), cb,
    )
}

#[no_mangle]
pub extern "C" fn fs_handle_utimes(
    handle: *mut FileHandle,
    atime: usize,
    mtime: usize,
    callback: *const AsyncClosure,
) {
    if handle.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    handle.0.utimes(
        atime.try_into().unwrap(), mtime.try_into().unwrap(), cb,
    )
}

#[no_mangle]
pub extern "C" fn fs_handle_write(
    handle: *mut FileHandle,
    buffer: *const Buffer,
    options: WriteOptions,
    callback: *const AsyncUsizeClosure,
) {
    if handle.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let buffer = unsafe { &*buffer };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |wrote, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(wrote)
            }
        }))
    );

    handle.0.write(
        &buffer.0, options.into(), cb,
    )
}


#[no_mangle]
pub extern "C" fn fs_handle_write_bytes(
    handle: *mut FileHandle,
    buffer: *const u8,
    length: usize,
    options: WriteOptions,
    callback: *const AsyncUsizeClosure,
) {
    if handle.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };


    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |wrote, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(wrote)
            }
        }))
    );

    let buffer = unsafe { node_buffer::Buffer::from_reference(buffer as _, length) };

    handle.0.write(
        &buffer, options.into(), cb,
    )
}

#[no_mangle]
pub extern "C" fn fs_handle_write_string(
    handle: *mut FileHandle,
    data: *const c_char,
    encoding: StringEncoding,
    position: isize,
    callback: *const AsyncUsizeClosure,
) {
    if handle.is_null() || data.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let data = unsafe { CStr::from_ptr(data) };
    let data = data.to_string_lossy();

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |wrote, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(wrote)
            }
        }))
    );

    handle.0.write_string(
        data.as_ref(), encoding.into(), position, cb,
    )
}

#[no_mangle]
pub extern "C" fn fs_handle_write_file_with_str(
    handle: *mut FileHandle,
    data: *const c_char,
    options: WriteFileOptions,
    callback: *const AsyncClosure,
) {
    if handle.is_null() || data.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let data = unsafe { CStr::from_ptr(data) };
    let data = data.to_string_lossy();

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    handle.0.write_file_with_str(
        data.as_ref(), options.into(), cb,
    )
}

#[no_mangle]
pub extern "C" fn fs_handle_write_file_with_bytes(
    handle: *mut FileHandle,
    data: *const Buffer,
    options: WriteFileOptions,
    callback: *const AsyncClosure,
) {
    if handle.is_null() || data.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let data = unsafe { &*data };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    handle.0.write_file_with_bytes(
        &data.0, options.into(), cb,
    )
}


#[no_mangle]
pub extern "C" fn fs_handle_write_file_with_bytes_slice(
    handle: *mut FileHandle,
    data: *const u8,
    length: usize,
    options: WriteFileOptions,
    callback: *const AsyncClosure,
) {
    if handle.is_null() || data.is_null() || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };


    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    let buffer = unsafe { node_buffer::Buffer::from_reference(data as _, length) };
    handle.0.write_file_with_bytes(
        &buffer, options.into(), cb,
    )
}


#[no_mangle]
pub extern "C" fn fs_handle_writev(
    handle: *mut FileHandle,
    buffers: *const Buffer,
    length: usize,
    position: usize,
    callback: *const AsyncUsizeClosure,
) {
    if handle.is_null() || buffers.is_null() || length == 0 || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };

    let buffers = unsafe { std::slice::from_raw_parts(buffers, length) };

    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |wrote, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(wrote)
            }
        }))
    );

    let buffers = buffers.iter().map(|buffer| buffer.0.clone())
        .collect::<Vec<node_buffer::Buffer>>();

    handle.0.writev(
        buffers, position.try_into().unwrap(), cb,
    )
}


#[no_mangle]
pub extern "C" fn fs_handle_writev_slice(
    handle: *mut FileHandle,
    buffers: *const *const u8,
    buffers_buffers: *const usize,
    length: usize,
    position: usize,
    callback: *const AsyncUsizeClosure,
) {
    if handle.is_null() || buffers.is_null() || length == 0 || callback.is_null() {
        return;
    }

    let callback = unsafe { &*callback };

    let handle = unsafe { &mut *handle };


    let buffer_length = unsafe { std::slice::from_raw_parts(buffers_buffers, length) };
    let buffers = unsafe { std::slice::from_raw_parts(buffers, length) };


    let buffers = buffers.iter().zip(buffer_length.iter())
        .map(|(buffer, len)| {
            unsafe { node_buffer::Buffer::from_reference((*buffer) as *mut _, length) }
        }).collect::<Vec<_>>();


    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |wrote, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(NodeError)
                )
            } else {
                callback.on_success(wrote)
            }
        }))
    );

    handle.0.writev(
        buffers, position.try_into().unwrap(), cb,
    )
}

#[derive(Copy, Clone, Debug)]
pub struct FileWatchEvent(node_fs::a_sync::FileWatchEvent);


#[no_mangle]
pub extern "C" fn fs_filewatch_event_destroy(
    event: *mut FileWatchEvent,
) {
    if event.is_null() {
        return;
    }

    let _ = unsafe { Box::from_raw(event) };
}

#[no_mangle]
pub extern "C" fn fs_filewatch_event_current(
    event: *const FileWatchEvent,
) -> *mut FileStat {
    if event.is_null() {
        return std::ptr::null_mut();
    }

    let event = unsafe { &*event };
    let current = event.0.current();
    current
        .map(|stat| {
            Box::into_raw(
                Box::new(
                    FileStat::from(stat)
                )
            )
        })
        .unwrap_or_else(|| {
            std::ptr::null_mut()
        })
}


#[no_mangle]
pub extern "C" fn fs_filewatch_event_previous(
    event: *const FileWatchEvent,
) -> *mut FileStat {
    if event.is_null() {
        return std::ptr::null_mut();
    }

    let event = unsafe { &*event };
    let previous = event.0.previous();
    previous
        .map(|stat| {
            Box::into_raw(
                Box::new(
                    FileStat::from(stat)
                )
            )
        })
        .unwrap_or_else(|| {
            std::ptr::null_mut()
        })
}


#[derive(Clone, Debug)]
pub struct WatchEvent(node_fs::a_sync::WatchEvent);


#[no_mangle]
pub extern "C" fn fs_watch_event_destroy(
    event: *mut WatchEvent,
) {
    if event.is_null() {
        return;
    }

    let _ = unsafe { Box::from_raw(event) };
}

#[no_mangle]
pub extern "C" fn fs_watch_event_event_type(
    event: *const WatchEvent,
) -> *const c_char {
    if event.is_null() {
        return std::ptr::null_mut();
    }

    let event = unsafe { &*event };
    let event_type = event.0.event_type();
    event_type
        .map(|value| {
            CString::new(value.to_string()).unwrap().into_raw()
        })
        .unwrap_or_else(|| {
            std::ptr::null_mut()
        })
}


#[no_mangle]
pub extern "C" fn fs_watch_event_filename(
    event: *const WatchEvent,
) -> *const c_char {
    if event.is_null() {
        return std::ptr::null_mut();
    }

    let event = unsafe { &*event };
    let filename = event.0.filename();
    filename
        .map(|value| {
            CString::new(value.to_string()).unwrap().into_raw()
        })
        .unwrap_or_else(|| {
            std::ptr::null_mut()
        })
}

#[derive(Clone)]
pub struct FileDir(node_fs::file_dir::FileDir);


#[no_mangle]
pub extern "C" fn fs_file_dir_destroy(value: *mut FileDir) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

pub struct FileHandle(node_fs::file_handle::FileHandle);

#[no_mangle]
pub extern "C" fn fs_filehandle_destroy(value: *mut FileHandle) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[derive(Clone)]
pub struct AsyncClosure(Arc<node_fs::a_sync::AsyncClosure<(), NodeError>>);

#[derive(Clone)]
pub struct AsyncBoolClosure(Arc<node_fs::a_sync::AsyncClosure<bool, NodeError>>);

#[derive(Clone)]
pub struct AsyncFileStatClosure(Arc<node_fs::a_sync::AsyncClosure<FileStat, NodeError>>);

#[derive(Clone)]
pub struct AsyncStringClosure(Arc<node_fs::a_sync::AsyncClosure<String, NodeError>>);

#[derive(Clone)]
pub struct AsyncUsizeClosure(Arc<node_fs::a_sync::AsyncClosure<usize, NodeError>>);

#[derive(Clone)]
pub struct AsyncI32Closure(Arc<node_fs::a_sync::AsyncClosure<i32, NodeError>>);

#[derive(Clone)]
pub struct AsyncFileWatchClosure(Arc<node_fs::a_sync::AsyncClosure<FileWatchEvent, NodeError>>);

#[derive(Clone)]
pub struct AsyncFsEncodingClosure(Arc<node_fs::a_sync::AsyncClosure<FsEncoding, NodeError>>);

#[derive(Clone)]
pub struct AsyncWatchClosure(Arc<node_fs::a_sync::AsyncClosure<WatchEvent, NodeError>>);

#[derive(Clone)]
pub struct AsyncReaddirClosure(Arc<node_fs::a_sync::AsyncClosure<Vec<ReaddirResult>, NodeError>>);

#[derive(Clone)]
pub struct AsyncFileDirClosure(Arc<node_fs::a_sync::AsyncClosure<FileDir, NodeError>>);

#[derive(Clone)]
pub struct AsyncFileHandleClosure(Arc<node_fs::a_sync::AsyncClosure<FileHandle, NodeError>>);

impl Into<node_fs::sync::AppendFileOptions> for AppendFileOptions {
    fn into(self) -> node_fs::sync::AppendFileOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::MkDirOptions> for MkDirOptions {
    fn into(self) -> node_fs::sync::MkDirOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::MkdTempOptions> for MkdTempOptions {
    fn into(self) -> node_fs::sync::MkdTempOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::OpenDirOptions> for OpenDirOptions {
    fn into(self) -> node_fs::sync::OpenDirOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::ReaddirOptions> for ReaddirOptions {
    fn into(self) -> node_fs::sync::ReaddirOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::ReadFileOptions> for ReadFileOptions {
    fn into(self) -> node_fs::sync::ReadFileOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::ReadLinkOptions> for ReadLinkOptions {
    fn into(self) -> node_fs::sync::ReadLinkOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::RealPathOptions> for RealPathOptions {
    fn into(self) -> node_fs::sync::RealPathOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::RmDirOptions> for RmDirOptions {
    fn into(self) -> node_fs::sync::RmDirOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::RmOptions> for RmOptions {
    fn into(self) -> node_fs::sync::RmOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::WriteOptions> for WriteOptions {
    fn into(self) -> node_fs::sync::WriteOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::WriteFileOptions> for WriteFileOptions {
    fn into(self) -> node_fs::sync::WriteFileOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}


impl From<node_fs::sync::AppendFileOptions> for AppendFileOptions {
    fn from(value: node_fs::sync::AppendFileOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::MkDirOptions> for MkDirOptions {
    fn from(value: node_fs::sync::MkDirOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::MkdTempOptions> for MkdTempOptions {
    fn from(value: node_fs::sync::MkdTempOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::OpenDirOptions> for OpenDirOptions {
    fn from(value: node_fs::sync::OpenDirOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::ReaddirOptions> for ReaddirOptions {
    fn from(value: node_fs::sync::ReaddirOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::ReadFileOptions> for ReadFileOptions {
    fn from(value: node_fs::sync::ReadFileOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::ReadLinkOptions> for ReadLinkOptions {
    fn from(value: node_fs::sync::ReadLinkOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::RealPathOptions> for RealPathOptions {
    fn from(value: node_fs::sync::RealPathOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::RmDirOptions> for RmDirOptions {
    fn from(value: node_fs::sync::RmDirOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::RmOptions> for RmOptions {
    fn from(value: node_fs::sync::RmOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::WriteOptions> for WriteOptions {
    fn from(value: node_fs::sync::WriteOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::WriteFileOptions> for WriteFileOptions {
    fn from(value: node_fs::sync::WriteFileOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

#[no_mangle]
pub extern "C" fn fs_async_create_async_closure(on_success: *mut c_void, on_error: *mut c_void, data: *mut c_void) -> *mut AsyncClosure {
    Box::into_raw(Box::new(
        AsyncClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(*mut NodeError, *mut c_void)>(on_error) };
                        on_error(Box::into_raw(Box::new(error.unwrap())), data);
                    } else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(*mut c_void)>(on_success) };
                        on_success(data);
                    }
                }))
            )
        )
    ))
}

#[no_mangle]
pub extern "C" fn fs_async_create_async_bool_closure(on_success: *mut c_void, on_error: *mut c_void, data: *mut c_void) -> *mut AsyncBoolClosure {
    Box::into_raw(Box::new(
        AsyncBoolClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error| {
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(*mut NodeError, *mut c_void)>(on_error) };
                        on_error(Box::into_raw(Box::new(error.unwrap())), data);
                    } else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(bool, *mut c_void)>(on_success) };
                        on_success(value.unwrap(), data);
                    }
                }))
            )
        )
    ))
}

#[no_mangle]
pub extern "C" fn fs_async_create_async_file_stat_closure(on_success: *mut c_void, on_error: *mut c_void, data: *mut c_void) -> *mut AsyncFileStatClosure {
    Box::into_raw(Box::new(
        AsyncFileStatClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error| {
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(*mut NodeError, *mut c_void)>(on_error) };
                        on_error(Box::into_raw(Box::new(error.unwrap())), data);
                    } else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(*mut FileStat, *mut c_void)>(on_success) };
                        on_success(Box::into_raw(Box::new(value.unwrap())), data);
                    }
                }))
            )
        )
    ))
}

#[no_mangle]
pub extern "C" fn fs_async_create_async_string_closure(on_success: *mut c_void, on_error: *mut c_void, data: *mut c_void) -> *mut AsyncStringClosure {
    Box::into_raw(Box::new(
        AsyncStringClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error| {
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(*mut NodeError, *mut c_void)>(on_error) };
                        on_error(Box::into_raw(Box::new(error.unwrap())), data);
                    } else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(*const c_char, *mut c_void)>(on_success) };
                        on_success(CString::new(value.unwrap()).unwrap().into_raw(), data);
                    }
                }))
            )
        )
    ))
}

#[no_mangle]
pub extern "C" fn fs_async_create_async_usize_closure(on_success: *mut c_void, on_error: *mut c_void, data: *mut c_void) -> *mut AsyncUsizeClosure {
    Box::into_raw(Box::new(
        AsyncUsizeClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error| {
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(*mut NodeError, *mut c_void)>(on_error) };
                        on_error(Box::into_raw(Box::new(error.unwrap())), data);
                    } else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(usize, *mut c_void)>(on_success) };
                        on_success(value.unwrap(), data);
                    }
                }))
            )
        )
    ))
}

#[no_mangle]
pub extern "C" fn fs_async_create_async_i32_closure(on_success: *mut c_void, on_error: *mut c_void, data: *mut c_void) -> *mut AsyncI32Closure {
    Box::into_raw(Box::new(
        AsyncI32Closure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error| {
                    if error.is_some() {
                        let on_error: fn(*mut NodeError, *mut c_void) = unsafe { std::mem::transmute(on_error) };
                        let err = Box::into_raw(Box::new(error.unwrap()));
                        on_error(err, data);
                    } else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(i32, *mut c_void)>(on_success) };
                        on_success(value.unwrap(), data);
                    }
                }))
            )
        )
    ))
}

#[no_mangle]
pub extern "C" fn fs_async_create_async_file_watch_closure(on_success: *mut c_void, on_error: *mut c_void, data: *mut c_void) -> *mut AsyncFileWatchClosure {
    Box::into_raw(Box::new(
        AsyncFileWatchClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error| {
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(*mut NodeError, *mut c_void)>(on_error) };
                        on_error(Box::into_raw(Box::new(error.unwrap())), data);
                    } else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(*mut FileWatchEvent, *mut c_void)>(on_success) };
                        on_success(Box::into_raw(Box::new(value.unwrap())), data);
                    }
                }))
            )
        )
    ))
}

#[no_mangle]
pub extern "C" fn fs_async_create_async_fs_encoding_closure(on_success: *mut c_void, on_error: *mut c_void, data: *mut c_void) -> *mut AsyncFsEncodingClosure {
    Box::into_raw(Box::new(
        AsyncFsEncodingClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error| {
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(*mut NodeError, *mut c_void)>(on_error) };
                        on_error(Box::into_raw(Box::new(error.unwrap())), data);
                    } else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(*mut FsEncoding, *mut c_void)>(on_success) };
                        on_success(Box::into_raw(Box::new(value.unwrap())), data);
                    }
                }))
            )
        )
    ))
}

#[no_mangle]
pub extern "C" fn fs_async_create_async_fs_watch_closure(on_success: *mut c_void, on_error: *mut c_void, data: *mut c_void) -> *mut AsyncWatchClosure {
    Box::into_raw(Box::new(
        AsyncWatchClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error| {
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(*mut NodeError, *mut c_void)>(on_error) };
                        on_error(Box::into_raw(Box::new(error.unwrap())), data);
                    } else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(*mut WatchEvent, *mut c_void)>(on_success) };
                        on_success(Box::into_raw(Box::new(value.unwrap())), data);
                    }
                }))
            )
        )
    ))
}

#[no_mangle]
pub extern "C" fn fs_async_create_async_fs_readdir_closure(on_success: *mut c_void, on_error: *mut c_void, data: *mut c_void) -> *mut AsyncReaddirClosure {
    Box::into_raw(Box::new(
        AsyncReaddirClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error| {
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(*mut NodeError, *mut c_void)>(on_error) };
                        on_error(Box::into_raw(Box::new(error.unwrap())), data);
                    } else {
                        let on_success = on_success as *const ();


                        let mut result = value.unwrap();

                        let ptr = result.as_mut_ptr();
                        let len = result.len();
                        result.shrink_to_fit();
                        std::mem::forget(result);
                        let ret = ReaddirResultArray {
                            data: ptr,
                            length: len,
                        };


                        let on_success = unsafe { std::mem::transmute::<*const (), fn(*mut ReaddirResultArray, *mut c_void)>(on_success) };
                        on_success(Box::into_raw(
                            Box::new(ret)
                        ), data);
                    }
                }))
            )
        )
    ))
}

#[no_mangle]
pub extern "C" fn fs_async_create_async_fs_file_dir_closure(on_success: *mut c_void, on_error: *mut c_void, data: *mut c_void) -> *mut AsyncFileDirClosure {
    Box::into_raw(Box::new(
        AsyncFileDirClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error| {
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(*mut NodeError, *mut c_void)>(on_error) };
                        on_error(Box::into_raw(Box::new(error.unwrap())), data);
                    } else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(*mut FileDir, *mut c_void)>(on_success) };
                        on_success(Box::into_raw(Box::new(value.unwrap())), data);
                    }
                }))
            )
        )
    ))
}

#[no_mangle]
pub extern "C" fn fs_async_create_async_fs_file_handle_closure(on_success: *mut c_void, on_error: *mut c_void, data: *mut c_void) -> *mut AsyncFileHandleClosure {
    Box::into_raw(Box::new(
        AsyncFileHandleClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error| {
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(*mut NodeError, *mut c_void)>(on_error) };
                        on_error(Box::into_raw(Box::new(error.unwrap())), data);
                    } else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(*mut FileHandle, *mut c_void)>(on_success) };
                        on_success(Box::into_raw(Box::new(value.unwrap())), data);
                    }
                }))
            )
        )
    ))
}


impl From<node_fs::file_stat::FileStat> for FileStat {
    fn from(value: node_fs::file_stat::FileStat) -> Self {
        FileStat {
            dev: value.dev,
            ino: value.ino,
            mode: value.mode,
            nlink: value.nlink,
            uid: value.uid,
            gid: value.gid,
            rdev: value.rdev,
            size: value.size,
            blksize: value.blksize,
            blocks: value.blocks,
            atimeMs: value.atimeMs,
            mtimeMs: value.mtimeMs,
            ctimeMs: value.ctimeMs,
            birthtimeMs: value.birthtimeMs,
            birthtime: value.birthtime,
            atime: value.atime,
            mtime: value.mtime,
            ctime: value.ctime,
            isBlockDevice: value.isBlockDevice,
            isCharacterDevice: value.isCharacterDevice,
            isDirectory: value.isDirectory,
            isFIFO: value.isFIFO,
            isFile: value.isFile,
            isSocket: value.isSocket,
            isSymbolicLink: value.isSymbolicLink,
        }
    }
}