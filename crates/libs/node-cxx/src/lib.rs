use std::ffi::{c_long, c_ushort, CString};
use node_fs::prelude::handle_meta;

fn to_optional(value: isize) -> Option<usize> {
    if value < 0 {
        return None;
    }

    return Some(value as usize);
}

impl Default for ffi::FileStat {
    fn default() -> Self {
        Self {
            dev: 0,
            ino: 0,
            mode: 0,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            size: 0,
            blksize: 0,
            blocks: 0,
            atimeMs: 0.0,
            mtimeMs: 0.0,
            ctimeMs: 0.0,
            birthtimeMs: 0.0,
            birthtime: 0.0,
            atime: 0.0,
            mtime: 0.0,
            ctime: 0.0,
            isBlockDevice: false,
            isCharacterDevice: false,
            isDirectory: false,
            isFIFO: false,
            isFile: false,
            isSocket: false,
            isSymbolicLink: false,
        }
    }
}

#[derive(Clone)]
pub struct Buffer(node_buffer::Buffer);

impl Buffer {
    pub(crate) fn new(buffer: node_buffer::Buffer) -> Self {
        Self(buffer)
    }

    pub fn into_box(self) -> Box<Self> {
        Box::new(self)
    }
}

impl From<node_buffer::StringEncoding> for ffi::StringEncoding {
    fn from(value: node_buffer::StringEncoding) -> Self {
        match value {
            node_buffer::StringEncoding::Ascii => ffi::StringEncoding::Ascii,
            node_buffer::StringEncoding::Utf8 => ffi::StringEncoding::Utf8,
            node_buffer::StringEncoding::Utf16le => ffi::StringEncoding::Utf16le,
            node_buffer::StringEncoding::Ucs2 => ffi::StringEncoding::Ucs2,
            node_buffer::StringEncoding::Base64 => ffi::StringEncoding::Base64,
            node_buffer::StringEncoding::Latin1 => ffi::StringEncoding::Latin1,
            node_buffer::StringEncoding::Binary => ffi::StringEncoding::Binary,
            node_buffer::StringEncoding::Hex => ffi::StringEncoding::Hex,
        }
    }
}

fn buffer_alloc(size: usize) -> Box<Buffer> {
    Buffer(node_buffer::Buffer::builder().size(size).build()).into_box()
}

fn buffer_alloc_with_size_string_encoding(size: usize, string: &str, encoding: ffi::StringEncoding) -> Box<Buffer> {
    Box::new(
        Buffer(node_buffer::Buffer::builder()
            .size(size)
            .fill_text(CString::new(string).unwrap(), encoding.into())
            .build())
    )
}

fn buffer_from_string(string: &str, encoding: ffi::StringEncoding) -> Box<Buffer> {
    Buffer(
        node_buffer::Buffer::from_string(CString::new(string).unwrap(), encoding.into())
    ).into_box()
}

fn buffer_from_slice(slice: &[u8]) -> Box<Buffer> {
    Buffer(
        node_buffer::Buffer::from_slice(slice)
    ).into_box()
}

fn buffer_copy_bytes_from(buffer: &Buffer) -> Box<Buffer> {
    Buffer(
        node_buffer::Buffer::from_buffer(&buffer.0)
    ).into_box()
}

fn buffer_atob(string: &str) -> String {
    node_buffer::Buffer::atob(CString::new(string).unwrap())
}

fn buffer_btoa(string: &str) -> String {
    node_buffer::Buffer::btoa(CString::new(string).unwrap())
}

fn buffer_fill_string(buffer: &mut Buffer, string: &str, encoding: ffi::StringEncoding) {
    buffer.0.fill(CString::new(string).unwrap(), encoding.into());
}

fn buffer_to_string(buffer: &Buffer, encoding: ffi::StringEncoding, start: isize, end: isize) -> String {
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

    buffer.0.as_string(Some(encoding.into()), start, end)
}

fn buffer_to_print_string(buffer: &Buffer) -> String {
    format!("{}", &buffer.0)
}

fn buffer_length(buffer: &Buffer) -> usize {
    buffer.0.length()
}

fn buffer_buffer(buffer: &mut Buffer) -> &mut [u8] {
    buffer.0.buffer_mut()
}

fn buffer_write_int8(buffer: &mut Buffer, value: i8, offset: isize) {
    buffer.0.write_int8(value, to_optional(offset));
}

fn buffer_write_uint8(buffer: &mut Buffer, value: u8, offset: isize) {
    buffer.0.write_uint8(value, to_optional(offset));
}

fn buffer_write_uint16be(buffer: &mut Buffer, value: u16, offset: isize) {
    buffer.0.write_uint16be(value, to_optional(offset));
}

fn buffer_write_int16be(buffer: &mut Buffer, value: i16, offset: isize) {
    buffer.0.write_int16be(value, to_optional(offset));
}

fn buffer_write_uint16le(buffer: &mut Buffer, value: u16, offset: isize) {
    buffer.0.write_uint16le(value, to_optional(offset));
}

fn buffer_write_int16le(buffer: &mut Buffer, value: i16, offset: isize) {
    buffer.0.write_int16le(value, to_optional(offset));
}

fn buffer_write_uint32be(buffer: &mut Buffer, value: u32, offset: isize) {
    buffer.0.write_uint32be(value, to_optional(offset));
}

fn buffer_write_int32be(buffer: &mut Buffer, value: i32, offset: isize) {
    buffer.0.write_int32be(value, to_optional(offset));
}

fn buffer_write_uint32le(buffer: &mut Buffer, value: u32, offset: isize) {
    buffer.0.write_uint32le(value, to_optional(offset));
}

fn buffer_write_int32le(buffer: &mut Buffer, value: i32, offset: isize) {
    buffer.0.write_int32le(value, to_optional(offset));
}

fn buffer_write_big_uint64be(buffer: &mut Buffer, value: u64, offset: isize) {
    buffer.0.write_big_uint64be(value, to_optional(offset));
}

fn buffer_write_big_int64be(buffer: &mut Buffer, value: i64, offset: isize) {
    buffer.0.write_int64be(value, to_optional(offset));
}

fn buffer_write_big_uint64le(buffer: &mut Buffer, value: u64, offset: isize) {
    buffer.0.write_big_uint64le(value, to_optional(offset));
}

fn buffer_write_big_int64le(buffer: &mut Buffer, value: i64, offset: isize) {
    buffer.0.write_big_int64le(value, to_optional(offset));
}

fn buffer_write_float_be(buffer: &mut Buffer, value: f32, offset: isize) {
    buffer.0.write_float_be(value, to_optional(offset));
}

fn buffer_write_double_be(buffer: &mut Buffer, value: f64, offset: isize) {
    buffer.0.write_double_be(value, to_optional(offset));
}

fn buffer_write_float_le(buffer: &mut Buffer, value: f32, offset: isize) {
    buffer.0.write_float_le(value, to_optional(offset));
}

fn buffer_write_double_le(buffer: &mut Buffer, value: f64, offset: isize) {
    buffer.0.write_double_le(value, to_optional(offset));
}


fn buffer_read_int8(buffer: &mut Buffer, offset: isize) -> i8 {
    buffer.0.read_int8(to_optional(offset))
}

fn buffer_read_uint8(buffer: &mut Buffer, offset: isize) -> u8 {
    buffer.0.read_uint8(to_optional(offset))
}

fn buffer_read_uint16be(buffer: &mut Buffer, offset: isize) -> u16 {
    buffer.0.read_uint16be(to_optional(offset))
}

fn buffer_read_int16be(buffer: &mut Buffer, offset: isize) -> i16 {
    buffer.0.read_int16be(to_optional(offset))
}

fn buffer_read_uint16le(buffer: &mut Buffer, offset: isize) -> u16 {
    buffer.0.read_uint16le(to_optional(offset))
}

fn buffer_read_int16le(buffer: &mut Buffer, offset: isize) -> i16 {
    buffer.0.read_int16le(to_optional(offset))
}

fn buffer_read_uint32be(buffer: &mut Buffer, offset: isize) -> u32 {
    buffer.0.read_uint32be(to_optional(offset))
}

fn buffer_read_int32be(buffer: &mut Buffer, offset: isize) -> i32 {
    buffer.0.read_int32be(to_optional(offset))
}

fn buffer_read_uint32le(buffer: &mut Buffer, offset: isize) -> u32 {
    buffer.0.read_uint32le(to_optional(offset))
}

fn buffer_read_int32le(buffer: &mut Buffer, offset: isize) -> i32 {
    buffer.0.read_int32le(to_optional(offset))
}

fn buffer_read_big_uint64be(buffer: &mut Buffer, offset: isize) -> u64 {
    buffer.0.read_big_uint64be(to_optional(offset))
}

fn buffer_read_big_int64be(buffer: &mut Buffer, offset: isize) -> i64 {
    buffer.0.read_big_int64be(to_optional(offset))
}

fn buffer_read_big_uint64le(buffer: &mut Buffer, offset: isize) -> u64 {
    buffer.0.read_big_uint64le(to_optional(offset))
}

fn buffer_read_big_int64le(buffer: &mut Buffer, offset: isize) -> i64 {
    buffer.0.read_big_int64le(to_optional(offset))
}

fn buffer_read_float_be(buffer: &mut Buffer, offset: isize) -> f32 {
    buffer.0.read_float_be(to_optional(offset))
}

fn buffer_read_double_be(buffer: &mut Buffer, offset: isize) -> f64 {
    buffer.0.read_double_be(to_optional(offset))
}

fn buffer_read_float_le(buffer: &mut Buffer, offset: isize) -> f32 {
    buffer.0.read_float_le(to_optional(offset))
}

fn buffer_read_double_le(buffer: &mut Buffer, offset: isize) -> f64 {
    buffer.0.read_double_le(to_optional(offset))
}

fn fs_access_sync(path: &str, mode: i32) -> Result<(), String> {
    node_fs::sync::access(path, mode).map_err(|e| e.to_string())
}

fn fs_append_file_sync(fd: i32, buffer: &Buffer) -> Result<(), String> {
    node_fs::sync::append_file_with_buffer(fd, &buffer.0).map_err(|e| e.to_string())
}

fn fs_append_file_with_bytes_sync(fd: i32, bytes: &[u8]) -> Result<(), String> {
    node_fs::sync::append_file_with_bytes(fd, bytes).map_err(|e| e.to_string())
}

fn fs_append_file_with_string_encoding_sync(fd: i32, string: &str, encoding: ffi::StringEncoding) -> Result<(), String> {
    node_fs::sync::append_file_with_str_encoding(fd, string, encoding.into()).map_err(|e| e.to_string())
}

fn fs_append_file_with_path_sync(path: &str, buffer: &Buffer, mode: i32, flags: i32) -> Result<(), String> {
    node_fs::sync::append_file_with_path_buffer(path, &buffer.0, mode, flags).map_err(|e| e.to_string())
}

fn fs_append_file_with_path_bytes_sync(path: &str, bytes: &[u8], mode: i32, flags: i32) -> Result<(), String> {
    node_fs::sync::append_file_with_path_bytes(path, bytes, mode, flags).map_err(|e| e.to_string())
}

fn fs_append_file_with_path_string_encoding_sync(path: &str, string: &str, encoding: ffi::StringEncoding, mode: i32, flags: i32) -> Result<(), String> {
    node_fs::sync::append_file_with_path_str_encoding(path, string, encoding.into(), mode, flags).map_err(|e| e.to_string())
}

fn fs_chmod_sync(path: &str, mode: u32) -> Result<(), String> {
    node_fs::sync::chmod(path, mode).map_err(|e| e.to_string())
}

fn fs_chown_sync(path: &str, uid: u32, gid: u32) -> Result<(), String> {
    node_fs::sync::chown(path, uid, gid).map_err(|e| e.to_string())
}

fn fs_close_sync(fd: i32) -> Result<(), String> {
    node_fs::sync::close_fd(fd).map_err(|e| e.to_string())
}

fn fs_copy_file_sync(src: &str, dest: &str, flags: u32) -> Result<(), String> {
    node_fs::sync::copy_file(src, dest, flags).map_err(|e| e.to_string())
}

fn fs_cp_sync(src: &str, dest: &str, flags: u32) -> Result<(), String> {
    node_fs::sync::cp(src, dest, flags).map_err(|e| e.to_string())
}

fn fs_exists_sync(src: &str) -> bool {
    node_fs::sync::exists(src)
}

fn fs_fchmod_sync(fd: i32, mode: u32) -> Result<(), String> {
    node_fs::sync::fchmod(fd, mode as c_ushort).map_err(|e| e.to_string())
}

fn fs_fchown_sync(fd: i32, uid: u32, gid: u32) -> Result<(), String> {
    node_fs::sync::fchown(fd, uid, gid).map_err(|e| e.to_string())
}

fn fs_fdatasync_sync(fd: i32) -> Result<(), String> {
    node_fs::sync::fdatasync(fd).map_err(|e| e.to_string())
}

fn fs_fstat_sync(fd: i32) -> Result<ffi::FileStat, String> {
    node_fs::sync::fstat(fd).map(|metadata| {
        unsafe { std::mem::transmute(handle_meta(&metadata)) }
    })
        .map_err(|e| e.to_string())
}

fn fs_fsync_sync(fd: i32) -> Result<(), String> {
    node_fs::sync::fsync(fd).map_err(|e| e.to_string())
}

fn fs_ftruncate_sync(fd: i32, len: usize) -> Result<(), String> {
    let len: c_long = len.try_into().map_err(|e| e.to_string())?;
    node_fs::sync::ftruncate(fd, len).map_err(|e| e.to_string())
}

fn fs_futimes_sync(fd: i32, atime: usize, mtime: usize) -> Result<(), String> {
    let atime: c_long = atime.try_into().map_err(|e| e.to_string())?;
    let mtime: c_long = mtime.try_into().map_err(|e| e.to_string())?;
    node_fs::sync::futimes(fd, atime, mtime).map_err(|e| e.to_string())
}

#[cxx::bridge(namespace = "org::nativescript::nodecompat")]
pub mod ffi {
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    #[repr(C)]
    pub enum StringEncoding {
        Ascii,
        Utf8,
        Utf16le,
        Ucs2,
        Base64,
        Latin1,
        Binary,
        Hex,
    }

    #[allow(non_snake_case)]
    #[repr(C)]
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

    extern "Rust" {
        type Buffer;

        fn buffer_alloc(size: usize) -> Box<Buffer>;

        fn buffer_alloc_with_size_string_encoding(size: usize, string: &str, encoding: StringEncoding) -> Box<Buffer>;

        fn buffer_concat(buffers: &[&[u8]]) -> Box<Buffer>;

        fn buffer_from_string(string: &str, encoding: StringEncoding) -> Box<Buffer>;

        fn buffer_from_slice(slice: &[u8]) -> Box<Buffer>;

        fn buffer_copy_bytes_from(buffer: &Buffer) -> Box<Buffer>;

        fn buffer_atob(string: &str) -> String;

        fn buffer_btoa(string: &str) -> String;

        fn buffer_fill_string(buffer: &mut Buffer, string: &str, encoding: StringEncoding);

        fn buffer_to_string(buffer: &Buffer, encoding: StringEncoding, start: isize, end: isize) -> String;

        fn buffer_to_print_string(buffer: &Buffer) -> String;

        fn buffer_length(buffer: &Buffer) -> usize;

        fn buffer_buffer(buffer: &mut Buffer) -> &mut [u8];

        fn buffer_write_int8(buffer: &mut Buffer, value: i8, offset: isize);

        fn buffer_write_uint8(buffer: &mut Buffer, value: u8, offset: isize);

        fn buffer_write_uint16be(buffer: &mut Buffer, value: u16, offset: isize);

        fn buffer_write_int16be(buffer: &mut Buffer, value: i16, offset: isize);

        fn buffer_write_uint16le(buffer: &mut Buffer, value: u16, offset: isize);

        fn buffer_write_int16le(buffer: &mut Buffer, value: i16, offset: isize);

        fn buffer_write_uint32be(buffer: &mut Buffer, value: u32, offset: isize);

        fn buffer_write_int32be(buffer: &mut Buffer, value: i32, offset: isize);

        fn buffer_write_uint32le(buffer: &mut Buffer, value: u32, offset: isize);

        fn buffer_write_int32le(buffer: &mut Buffer, value: i32, offset: isize);

        fn buffer_write_big_uint64be(buffer: &mut Buffer, value: u64, offset: isize);

        fn buffer_write_big_int64be(buffer: &mut Buffer, value: i64, offset: isize);

        fn buffer_write_big_uint64le(buffer: &mut Buffer, value: u64, offset: isize);

        fn buffer_write_big_int64le(buffer: &mut Buffer, value: i64, offset: isize);

        fn buffer_write_float_be(buffer: &mut Buffer, value: f32, offset: isize);

        fn buffer_write_double_be(buffer: &mut Buffer, value: f64, offset: isize);

        fn buffer_write_float_le(buffer: &mut Buffer, value: f32, offset: isize);

        fn buffer_write_double_le(buffer: &mut Buffer, value: f64, offset: isize);

        fn buffer_read_int8(buffer: &mut Buffer, offset: isize) -> i8;

        fn buffer_read_uint8(buffer: &mut Buffer, offset: isize) -> u8;

        fn buffer_read_uint16be(buffer: &mut Buffer, offset: isize) -> u16;

        fn buffer_read_int16be(buffer: &mut Buffer, offset: isize) -> i16;

        fn buffer_read_uint16le(buffer: &mut Buffer, offset: isize) -> u16;

        fn buffer_read_int16le(buffer: &mut Buffer, offset: isize) -> i16;

        fn buffer_read_uint32be(buffer: &mut Buffer, offset: isize) -> u32;

        fn buffer_read_int32be(buffer: &mut Buffer, offset: isize) -> i32;

        fn buffer_read_uint32le(buffer: &mut Buffer, offset: isize) -> u32;

        fn buffer_read_int32le(buffer: &mut Buffer, offset: isize) -> i32;

        fn buffer_read_big_uint64be(buffer: &mut Buffer, offset: isize) -> u64;

        fn buffer_read_big_int64be(buffer: &mut Buffer, offset: isize) -> i64;

        fn buffer_read_big_uint64le(buffer: &mut Buffer, offset: isize) -> u64;

        fn buffer_read_big_int64le(buffer: &mut Buffer, offset: isize) -> i64;

        fn buffer_read_float_be(buffer: &mut Buffer, offset: isize) -> f32;

        fn buffer_read_double_be(buffer: &mut Buffer, offset: isize) -> f64;

        fn buffer_read_float_le(buffer: &mut Buffer, offset: isize) -> f32;

        fn buffer_read_double_le(buffer: &mut Buffer, offset: isize) -> f64;
    }

    extern "Rust" {
        // fs sync

        fn fs_access_sync(path: &str, mode: i32) -> Result<(), String>;

        fn fs_append_file_sync(fd: i32, buffer: &Buffer) -> Result<(), String>;

        fn fs_append_file_with_bytes_sync(fd: i32, bytes: &[u8]) -> Result<(), String>;

        fn fs_append_file_with_string_encoding_sync(fd: i32, string: &str, encoding: StringEncoding) -> Result<(), String>;

        fn fs_append_file_with_path_sync(path: &str, buffer: &Buffer, mode: i32, flags: i32) -> Result<(), String>;

        fn fs_append_file_with_path_bytes_sync(path: &str, bytes: &[u8], mode: i32, flags: i32) -> Result<(), String>;

        fn fs_append_file_with_path_string_encoding_sync(path: &str, string: &str, encoding: StringEncoding, mode: i32, flags: i32) -> Result<(), String>;

        fn fs_chmod_sync(path: &str, mode: i32) -> Result<(), String>;

        fn fs_chown_sync(path: &str, uid: u32, gid: u32) -> Result<(), String>;

        fn fs_close_sync(fd: i32) -> Result<(), String>;

        fn fs_copy_file_sync(src: &str, dest: &str, flags: u32) -> Result<(), String>;

        fn fs_cp_sync(src: &str, dest: &str, flags: u32) -> Result<(), String>;

        fn fs_exists_sync(src: &str) -> bool;

        fn fs_fchmod_sync(fd: i32, mode: u32) -> Result<(), String>;

        fn fs_fchown_sync(fd: i32, uid: u32, gid: u32) -> Result<(), String>;

        fn fs_fdatasync_sync(fd: i32) -> Result<(), String>;

        fn fs_fstat_sync(fd: i32) -> Result<FileStat, String>;

        fn fs_fsync_sync(fd: i32) -> Result<(), String>;

        fn fs_ftruncate_sync(fd: i32, len: usize) -> Result<(), String>;

        fn fs_futimes_sync(fd: i32, atime: usize, mtime: usize) -> Result<(), String>;
    }


    extern "Rust" {
        // fs async
    }
}