use std::borrow::Cow;
use std::ffi::{c_void, CString};
use std::sync::Arc;
use node_fs::file_dirent::FileDirent;
use node_fs::prelude::{FsEncodingType, handle_meta};
use node_core::error::Result;

pub struct Error(node_core::error::AnyError);

impl Error {
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
        node_core::error::get_custom_error_message(&self.0).unwrap_or_default()
    }
}

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

impl Into<node_buffer::StringEncoding> for ffi::StringEncoding {
    fn into(self) -> node_buffer::StringEncoding {
        if self == ffi::StringEncoding::Ascii  {
            return node_buffer::StringEncoding::Ascii;
        }else if self == ffi::StringEncoding::Utf8  {
            return node_buffer::StringEncoding::Utf8;
        } else if self == ffi::StringEncoding::Utf8  {
            return node_buffer::StringEncoding::Utf8;
        }
        else if self == ffi::StringEncoding::Utf16le  {
            return node_buffer::StringEncoding::Utf16le;
        }else if self == ffi::StringEncoding::Ucs2  {
            return node_buffer::StringEncoding::Ucs2;
        }else if self == ffi::StringEncoding::Base64  {
            return node_buffer::StringEncoding::Base64;
        }else if self == ffi::StringEncoding::Latin1  {
            return node_buffer::StringEncoding::Latin1;
        }else if self == ffi::StringEncoding::Binary  {
            return node_buffer::StringEncoding::Binary;
        }else {
            return node_buffer::StringEncoding::Hex;
        }

        // match self {
        //     ffi::StringEncoding::Ascii => node_buffer::StringEncoding::Ascii,
        //     ffi::StringEncoding::Utf8 => node_buffer::StringEncoding::Utf8,
        //     ffi::StringEncoding::Utf16le => node_buffer::StringEncoding::Utf16le,
        //     ffi::StringEncoding::Ucs2 => node_buffer::StringEncoding::Ucs2,
        //     ffi::StringEncoding::Base64 => node_buffer::StringEncoding::Base64,
        //     ffi::StringEncoding::Latin1 => node_buffer::StringEncoding::Latin1,
        //     ffi::StringEncoding::Binary => node_buffer::StringEncoding::Binary,
        //     ffi::StringEncoding::Hex => node_buffer::StringEncoding::Hex,
        // }
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

fn buffer_concat(buffers: &[&[u8]]) -> Box<Buffer> {
    Buffer(
        node_buffer::Buffer::concat(buffers, None)
    ).into_box()
}

fn buffer_concat_length(buffers: &[&[u8]], length: usize) -> Box<Buffer> {
    Buffer(
        node_buffer::Buffer::concat(buffers, Some(length))
    ).into_box()
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
    buffer.0.fill(CString::new(string).unwrap(), Some(encoding.into()));
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
    buffer.0.write_big_int64be(value, to_optional(offset));
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


#[derive(Clone)]
pub struct Metadata(std::fs::Metadata);


#[derive(Clone, Debug)]
pub struct ReaddirResult(node_fs::sync::ReaddirResult);

impl ReaddirResult {
    pub fn get_type(&self) -> ffi::ReaddirResultType {
        match &self.0 {
            node_fs::sync::ReaddirResult::String(_) => ffi::ReaddirResultType::String,
            node_fs::sync::ReaddirResult::Buffer(_) => ffi::ReaddirResultType::Buffer,
            node_fs::sync::ReaddirResult::Type(_) => ffi::ReaddirResultType::Type
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

    pub fn get_buffer_value(&self) -> Result<node_buffer::Buffer> {
        match self.0.get_buffer_value() {
            None => {
                Err(node_core::error::generic_error("Invalid Type".to_string()))
            }
            Some(buffer) => {
                Ok(buffer)
            }
        }
    }

    pub fn get_type_value(&self) -> Result<FileDirent> {
        match self.0.get_type_value() {
            None => {
                Err(node_core::error::generic_error("Invalid Type".to_string()))
            }
            Some(dirent) => {
                Ok(dirent)
            }
        }
    }
}

fn fs_access_sync(path: &str, mode: i32) -> Result<()> {
    node_fs::sync::access(path, mode)
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_append_file_sync(fd: i32, buffer: &Buffer, options: ffi::AppendFileOptions) -> Result<()> {
    node_fs::sync::append_file_with_buffer(fd, &buffer.0, options.into())
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_append_file_with_bytes_sync(fd: i32, bytes: &[u8], options: ffi::AppendFileOptions) -> Result<()> {
    node_fs::sync::append_file_with_bytes(fd, bytes, options.into()).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_append_file_with_string_sync(fd: i32, string: &str, options: ffi::AppendFileOptions) -> Result<()> {
    node_fs::sync::append_file_with_str(fd, string, options.into()).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_append_file_with_path_sync(path: &str, buffer: &Buffer, options: ffi::AppendFileOptions) -> Result<()> {
    node_fs::sync::append_file_with_path_buffer(path, &buffer.0, options.into()).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_append_file_with_path_bytes_sync(path: &str, bytes: &[u8], options: ffi::AppendFileOptions) -> Result<()> {
    node_fs::sync::append_file_with_path_bytes(path, bytes, options.into()).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_append_file_with_path_string_sync(path: &str, string: &str, options: ffi::AppendFileOptions) -> Result<()> {
    node_fs::sync::append_file_with_path_str(path, string, options.into()).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_chmod_sync(path: &str, mode: u32) -> Result<()> {
    node_fs::sync::chmod(path, mode).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_chown_sync(path: &str, uid: u32, gid: u32) -> Result<()> {
    node_fs::sync::chown(path, uid, gid).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_close_sync(fd: i32) -> Result<()> {
    node_fs::sync::close_fd(fd);
    Ok(())
}

fn fs_copy_file_sync(src: &str, dest: &str, flags: u32) -> Result<()> {
    node_fs::sync::copy_file(src, dest, flags).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_cp_sync(src: &str, dest: &str, flags: u32) -> Result<()> {
    node_fs::sync::cp(src, dest, flags);
    Ok(())
}

fn fs_exists_sync(src: &str) -> bool {
    node_fs::sync::exists(src)
}

fn fs_fchmod_sync(fd: i32, mode: u32) -> Result<()> {
    node_fs::sync::fchmod(fd, mode as u16).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_fchown_sync(fd: i32, uid: u32, gid: u32) -> Result<()> {
    node_fs::sync::fchown(fd, uid, gid).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_fdatasync_sync(fd: i32) -> Result<()> {
    node_fs::sync::fdatasync(fd).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_fstat_sync(fd: i32) -> Result<ffi::FileStat> {
    node_fs::sync::fstat(fd).map(|metadata| {
        unsafe { std::mem::transmute(handle_meta(&metadata)) }
    })
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_fsync_sync(fd: i32) -> Result<()> {
    node_fs::sync::fsync(fd).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_ftruncate_sync(fd: i32, len: usize) -> Result<()> {
    node_fs::sync::ftruncate(fd, len.try_into().unwrap()).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_futimes_sync(fd: i32, atime: usize, mtime: usize) -> Result<()> {
    node_fs::sync::futimes(fd, atime.try_into().unwrap(), mtime.try_into().unwrap()).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_lchmod_sync(path: &str, mode: u32) -> Result<()> {
    node_fs::sync::chmod(path, mode).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_lchown_sync(path: &str, uid: u32, gid: u32) -> Result<()> {
    node_fs::sync::chown(path, uid, gid).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_lutimes_sync(path: &str, atime: i64, mtime: i64) -> Result<()> {
    node_fs::sync::lutimes(path, atime.try_into().unwrap(), mtime.try_into().unwrap()).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_link_sync(existing_path: &str, new_path: &str) -> Result<()> {
    node_fs::sync::link(existing_path, new_path).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_lstat_sync(path: &str) -> Result<Box<Metadata>> {
    node_fs::sync::lstat(path).map(|metadata| Box::new(Metadata(metadata))).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_mkdir_sync(path: &str, options: ffi::MkDirOptions) -> Result<()> {
    node_fs::sync::mkdir(path, options.into()).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_mkdtemp_sync(prefix: &str, options: ffi::MkdTempOptions) -> Result<String> {
    node_fs::sync::mkdtemp(prefix, options.into())
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_open_sync(path: &str, flag: i32, mode: i32) -> Result<i32> {
    node_fs::sync::open(path, flag, mode)
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_opendir_sync(path: &str, options: ffi::OpenDirOptions) -> Result<Box<FileDir>> {
    node_fs::sync::opendir(path, options.into())
        .map(|dir| {
            Box::new(FileDir(dir))
        })
        .map_err(|e| node_core::error::error_from_io_error(e))
}


#[cfg(not(windows))]
fn fs_read_sync(
    fd: i32,
    buffer: &mut [u8],
    offset: usize,
    length: usize,
    position: isize,
) -> Result<usize> {
    node_fs::sync::read(fd, buffer, offset, length, position).map_err(|e| node_core::error::error_from_io_error(e))
}

#[cfg(windows)]
fn fs_read_sync(
    fd: i64,
    buffer: &mut [u8],
    offset: usize,
    length: usize,
    position: isize,
) -> Result<usize, String> {
    unsafe {
        let fd = fd as *mut c_void;
        node_fs::sync::read(fd, buffer, offset, length, position).map_err(|e| node_core::error::error_from_io_error(e))
    }
}

impl Into<node_fs::FsEncodingType> for ffi::FsEncodingType {
    fn into(self) -> node_fs::FsEncodingType {
        match self {
            ffi::FsEncodingType::Ascii => node_fs::FsEncodingType::Ascii,
            ffi::FsEncodingType::Utf8 => node_fs::FsEncodingType::Utf8,
            ffi::FsEncodingType::Utf16le => node_fs::FsEncodingType::Utf16le,
            ffi::FsEncodingType::Ucs2 => node_fs::FsEncodingType::Ucs2,
            ffi::FsEncodingType::Latin1 => node_fs::FsEncodingType::Latin1,
            ffi::FsEncodingType::Buffer => node_fs::FsEncodingType::Buffer,
            _ => { todo!()}
        }
    }
}

impl From<FsEncodingType> for ffi::FsEncodingType {
    fn from(value: FsEncodingType) -> Self {
        match value {
            FsEncodingType::Ascii => ffi::FsEncodingType::Ascii,
            FsEncodingType::Utf8 => ffi::FsEncodingType::Utf8,
            FsEncodingType::Utf16le => ffi::FsEncodingType::Utf16le,
            FsEncodingType::Ucs2 => ffi::FsEncodingType::Ucs2,
            FsEncodingType::Latin1 => ffi::FsEncodingType::Latin1,
            FsEncodingType::Buffer => ffi::FsEncodingType::Buffer
        }
    }
}

fn fs_readdir_sync(path: &str, options: ffi::ReaddirOptions) -> Result<Vec<ReaddirResult>> {
    node_fs::sync::readdir(path, options.into())
        .map(|mut value| {
            value.into_iter()
                .map(|value| ReaddirResult(value))
                .collect()
        })
        .map_err(|e| node_core::error::error_from_io_error(e))
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FsEncoding(node_fs::FsEncoding);

impl FsEncoding {
    pub fn get_string_value(&self) -> Result<String> {
        match self.0.get_string_value() {
            Some(value) => { Ok(value.to_string_lossy().to_string()) }
            None => {
                Err(node_core::error::generic_error("Invalid Type".to_string()))
            }
        }
    }

    pub fn get_buffer_value(&self) -> Result<Buffer> {
        match self.0.get_buffer_value() {
            None => {
                Err(node_core::error::generic_error("Invalid Type".to_string()))
            }
            Some(buffer) => Ok(Buffer(buffer))
        }
    }
}

fn fs_read_file_sync(path: &str, options: ffi::ReadFileOptions) -> Result<Box<FsEncoding>> {
    node_fs::sync::read_file(path, options.into())
        .map(|f| Box::new(FsEncoding(f)))
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_read_file_with_fd_sync(fd: i32, options: ffi::ReadFileOptions) -> Result<Box<FsEncoding>> {
    node_fs::sync::read_file_with_fd(fd, options.into())
        .map(|f| Box::new(FsEncoding(f)))
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_read_link_sync(path: &str, options: ffi::ReadLinkOptions) -> Result<Box<FsEncoding>> {
    node_fs::sync::read_link(path, options.into())
        .map(|f| Box::new(FsEncoding(f)))
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_readv_sync(fd: i32, buffers: &mut [Buffer], position: i64) -> Result<usize> {
    let mut buffers = buffers.iter().map(|buffer| buffer.0.clone())
        .collect::<Vec<node_buffer::Buffer>>();
    node_fs::sync::readv(fd, buffers.as_mut_slice(), position.try_into().unwrap())
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_real_path_sync(path: &str, options: ffi::RealPathOptions) -> Result<String> {
    node_fs::sync::real_path(path, options.into())
        .map(|v| v.to_string_lossy().to_string())
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_rename_sync(old_path: &str, new_path: &str) -> Result<()> {
    node_fs::sync::rename(old_path, new_path)
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_rmdir_sync(
    path: &str,
    options: ffi::RmDirOptions,
) -> Result<()> {
    node_fs::sync::rmdir(path, options.into())
}

fn fs_rm_sync(
    path: &str,
    options: ffi::RmOptions,
) -> Result<()> {
    node_fs::sync::rm(path, options.into())
}

fn fs_stat_sync(path: &str) -> Result<Box<Metadata>> {
    node_fs::sync::stat(path)
        .map(|meta| Box::new(Metadata(meta)))
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_symlink_sync(target: &str, path: &str, _type_: &str) -> Result<()> {
    node_fs::sync::symlink(target, path, _type_)
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_truncate_sync(path: &str, len: u64) -> Result<()> {
    node_fs::sync::truncate(path, len.try_into().unwrap())
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_unlink_sync(path: &str) -> Result<()> {
    node_fs::sync::unlink(path)
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_utimes_sync(path: &str, atime: i64, mtime: i64) -> Result<()> {
    node_fs::sync::utimes(path, atime.try_into().unwrap(), mtime.try_into().unwrap())
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_write_sync(
    fd: i32,
    buffer: &[u8],
    options: ffi::WriteOptions,
) -> Result<usize> {
    node_fs::sync::write(fd, buffer, options.into())
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_write_string_sync(
    fd: i32,
    string: &str,
    encoding: ffi::StringEncoding,
    position: isize,
) -> Result<usize> {
    node_fs::sync::write_string(
        fd, string, encoding.into(), position,
    ).map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_write_file_with_str_sync(fd: i32, data: &str, options: ffi::WriteFileOptions) -> Result<()> {
    node_fs::sync::write_file_with_str(fd, data, options.into())
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_write_file_with_bytes_sync(fd: i32, data: &[u8], options: ffi::WriteFileOptions) -> Result<()> {
    node_fs::sync::write_file_with_bytes(fd, data, options.into())
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_write_file_with_str_from_path_sync(
    path: &str,
    data: &str,
    options: ffi::WriteFileOptions,
) -> Result<()> {
    node_fs::sync::write_file_with_str_from_path(
        path, data, options.into(),
    )
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_write_file_with_bytes_from_path_sync(
    path: &str,
    data: &[u8],
    options: ffi::WriteFileOptions,
) -> Result<()> {
    node_fs::sync::write_file_with_bytes_from_path(
        path, data, options.into(),
    )
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_write_file_with_buffer_from_path_sync(
    path: &str,
    data: &Buffer,
    options: ffi::WriteFileOptions,
) -> Result<()> {
    node_fs::sync::write_file_with_buffer_from_path(
        path, &data.0, options.into(),
    )
        .map_err(|e| node_core::error::error_from_io_error(e))
}

fn fs_writev_sync(fd: i32, mut buffers: Vec<Buffer>, position: i64) -> Result<usize> {
    let buffers = buffers.iter().map(|buffer| buffer.0.clone()).collect::<Vec<node_buffer::Buffer>>();
    node_fs::sync::writev(
        fd, buffers, position.try_into().unwrap(),
    )
        .map_err(|e| node_core::error::error_from_io_error(e))
}


// async

pub fn fs_async_access(path: &str, access: i32, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::access(path, access, cb)
}

pub fn fs_async_append_file_with_str(fd: i32, data: &str, options: ffi::AppendFileOptions, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::append_file_with_str(fd, data, options.into(), cb)
}

pub fn fs_async_append_file_with_bytes(fd: i32, data: &Buffer, options: ffi::AppendFileOptions, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::append_file_with_bytes(fd, &data.0, options.into(), cb)
}

pub fn fs_async_append_file_with_path_str(
    path: &str,
    data: &str,
    options: ffi::AppendFileOptions,
    callback: &AsyncClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::append_file_with_path_str(path, data, options.into(), cb)
}

pub fn fs_async_append_file_with_path_bytes(
    path: &str,
    data: &Buffer,
    options: ffi::AppendFileOptions,
    callback: &AsyncClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::append_file_with_path_bytes(path, &data.0, options.into(), cb)
}

pub fn fs_async_chmod(path: &str, mode: u32, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::chmod(path, mode, cb)
}

pub fn fs_async_chown(path: &str, uid: u32, gid: u32, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    node_fs::a_sync::chown(path, uid, gid, cb)
}

pub fn fs_async_close(fd: i32, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::close(fd, cb)
}

pub fn fs_async_copy_file(src: &str, dest: &str, flag: u32, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::copy_file(src, dest, flag, cb)
}

pub fn fs_async_cp(_src: &str, _dest: &str) {}

pub fn fs_async_exists(path: &str, callback: &AsyncBoolClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result)
            }
        }))
    );
    node_fs::a_sync::exists(path, cb)
}

pub fn fs_async_fchmod(fd: i32, mode: u16, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::fchmod(fd, mode, cb)
}

pub fn fs_async_fchown(fd: i32, uid: u32, gid: u32, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::fchown(fd, uid, gid, cb)
}

pub fn fs_async_fdatasync(fd: i32, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::fdatasync(fd, cb)
}

pub fn fs_async_fstat(fd: i32, callback: &AsyncFileStatClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result.map(|stat: node_fs::file_stat::FileStat| stat.into()))
            }
        }))
    );
    node_fs::a_sync::fstat(fd, cb);
}

pub fn fs_async_fsync(fd: i32, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::fdatasync(fd, cb)
}

pub fn fs_async_ftruncate(fd: i32, len: i64, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::ftruncate(fd, len.try_into().unwrap(), cb)
}

pub fn fs_async_futimes(fd: i32, atime: i64, mtime: i64, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::futimes(fd, atime.try_into().unwrap(), mtime.try_into().unwrap(), cb)
}

pub fn fs_async_lchmod(path: &str, mode: u16, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::lchmod(path, mode, cb)
}

pub fn fs_async_lchown(path: &str, uid: u32, gid: u32, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::lchown(path, uid, gid, cb)
}

pub fn fs_async_lutimes(path: &str, atime: i64, mtime: i64, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::lutimes(path, atime.try_into().unwrap(), mtime.try_into().unwrap(), cb)
}

pub fn fs_async_link(existing_path: &str, new_path: &str, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::link(existing_path, new_path, cb)
}

pub fn fs_async_lstat(path: &str, callback: &AsyncFileStatClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result.map(|stat: node_fs::file_stat::FileStat| stat.into()))
            }
        }))
    );

    node_fs::a_sync::lstat(path, cb)
}

pub fn fs_async_mkdir(path: &str, options: ffi::MkDirOptions, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::mkdir(path, options.into(), cb)
}

pub fn fs_async_mkdtemp(prefix: &str, options: ffi::MkdTempOptions, callback: &AsyncStringClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |path, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(path.map(|path: std::path::PathBuf| path.to_string_lossy().to_string()))
            }
        }))
    );
    node_fs::a_sync::mkdtemp(prefix, options.into(), cb);
}

pub fn fs_async_open(path: &str, flag: i32, mode: i32, callback: &AsyncI32Closure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result)
            }
        }))
    );
    node_fs::a_sync::open(path, flag, mode, cb);
}

pub fn fs_async_opendir(path: &str, options: ffi::OpenDirOptions, callback: &AsyncFileDirClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result.map(FileDir))
            }
        }))
    );
    node_fs::a_sync::opendir(path, options.into(), cb);
}

pub fn fs_async_read(
    fd: i32,
    buffer: &mut Buffer,
    offset: usize,
    length: usize,
    position: isize,
    callback: &AsyncUsizeClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result)
            }
        }))
    );

    node_fs::a_sync::read(fd, &mut buffer.0, offset, length, position, cb)
}

pub fn fs_async_readdir(
    path: &str,
    options: ffi::ReaddirOptions,
    callback: &AsyncReaddirClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result: Option<Vec<node_fs::sync::ReaddirResult>>, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result.map(|result| result.into_iter()
                    .map(|value| ReaddirResult(value))
                    .collect::<Vec<ReaddirResult>>()))
            }
        }))
    );

    node_fs::a_sync::readdir(path, options.into(), cb)
}

pub fn fs_async_read_file(path: &str, options: ffi::ReadFileOptions, callback: &AsyncFsEncodingClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result.map(FsEncoding))
            }
        }))
    );

    node_fs::a_sync::read_file(path, options.into(), cb)
}

pub fn fs_async_read_file_with_fd(fd: i32, options: ffi::ReadFileOptions, callback: &AsyncFsEncodingClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result.map(FsEncoding))
            }
        }))
    );

    node_fs::a_sync::read_file_with_fd(fd, options.into(), cb)
}

pub fn fs_async_read_link(path: &str, options: ffi::ReadLinkOptions, callback: &AsyncFsEncodingClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result.map(FsEncoding))
            }
        }))
    );

    node_fs::a_sync::read_link(path, options.into(), cb)
}

pub fn fs_async_readv(
    fd: i32,
    buffers: Vec<Buffer>,
    position: usize,
    callback: &AsyncUsizeClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result)
            }
        }))
    );

    let buffers = buffers.into_iter().map(|buffer| buffer.0).collect::<Vec<node_buffer::Buffer>>();

    node_fs::a_sync::readv(fd, buffers, position.try_into().unwrap(), cb)
}

pub fn fs_async_real_path(path: &str, options: ffi::RealPathOptions, callback: &AsyncStringClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result: Option<std::path::PathBuf>, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result.map(|result| result.to_string_lossy().to_string()))
            }
        }))
    );

    node_fs::a_sync::real_path(path, options.into(), cb)
}

pub fn fs_async_rename(old_path: &str, new_path: &str, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::rename(old_path, new_path, cb)
}

pub fn fs_async_rmdir(
    path: &str,
    options: ffi::RmDirOptions,
    callback: &AsyncClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::rmdir(path, options.into(), cb)
}

pub fn fs_async_rm(
    path: &str,
    options: ffi::RmOptions,
    callback: &AsyncClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::rm(path, options.into(), cb)
}

pub fn fs_async_stat(path: &str, throw_if_no_entry: bool, callback: &AsyncFileStatClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result: Option<node_fs::file_stat::FileStat>, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result.map(|result| result.into()))
            }
        }))
    );
    node_fs::a_sync::stat(path, throw_if_no_entry, cb)
}

pub fn fs_async_symlink(target: &str, path: &str, type_: &str, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::symlink(target, path, type_, cb)
}

pub fn fs_async_truncate(path: &str, len: u64, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::truncate(path, len, cb)
}

pub fn fs_async_unlink(path: &str, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::unlink(path, cb)
}

pub fn fs_async_unwatch_file(filename: &str) {
    node_fs::a_sync::unwatch_file(filename, None)
}

pub fn fs_async_unwatch_file_with_callback(filename: &str, callback: &AsyncFileWatchClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(event.map(FileWatchEvent))
            }
        }))
    );
    node_fs::a_sync::unwatch_file(filename, Some(cb))
}

pub fn fs_async_utimes(path: &str, atime: i64, mtime: i64, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::utimes(path, atime.try_into().unwrap(), mtime.try_into().unwrap(), cb)
}

pub fn fs_async_file_watcher_unref(filename: &str, callback: &AsyncFileWatchClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(event.map(FileWatchEvent))
            }
        }))
    );
    node_fs::a_sync::file_watcher_unref(filename, cb);
}

pub fn fs_async_file_watcher_ref(filename: &str, callback: &AsyncFileWatchClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(event.map(FileWatchEvent))
            }
        }))
    );
    node_fs::a_sync::file_watcher_ref(filename, cb)
}

pub fn fs_async_watch(
    filename: &str,
    persistent: bool,
    recursive: bool,
    encoding: ffi::FsEncodingType,
    callback: &AsyncWatchClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(event.map(WatchEvent))
            }
        }))
    );

    node_fs::a_sync::watch(
        filename, persistent, recursive, encoding.into(), cb,
    )
}

pub fn fs_async_watcher_unref(filename: &str, callback: &AsyncWatchClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(event.map(WatchEvent))
            }
        }))
    );

    node_fs::a_sync::watcher_unref(filename, cb)
}

pub fn fs_async_watcher_ref(filename: &str, callback: &AsyncWatchClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(event.map(WatchEvent))
            }
        }))
    );
    node_fs::a_sync::watcher_ref(filename, cb)
}

pub fn fs_async_watcher_close(
    filename: &str,
    callback: &AsyncWatchClosure,
    on_close: &AsyncClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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
                    .map(Error)
                )
            } else {
                on_close.on_success(None)
            }
        }))
    );


    node_fs::a_sync::watcher_close(filename, cb, on_close_cb)
}

pub fn fs_async_watch_file(
    filename: &str,
    bigint: bool,
    persistent: bool,
    interval: u64,
    encoding: ffi::FsEncodingType,
    callback: &AsyncFileWatchClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |event, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(event.map(FileWatchEvent))
            }
        }))
    );

    node_fs::a_sync::watch_file(filename, bigint, persistent, interval.try_into().unwrap(), encoding.into(), cb)
}

pub fn fs_async_write(
    fd: i32,
    buffer: &Buffer,
    options: ffi::WriteOptions,
    callback: &AsyncUsizeClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |wrote, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(wrote)
            }
        }))
    );

    node_fs::a_sync::write(fd, &buffer.0, options.into(), cb)
}

pub fn fs_async_write_string(
    fd: i32,
    string: &str,
    encoding: ffi::StringEncoding,
    position: isize,
    callback: &AsyncUsizeClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |wrote, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(wrote)
            }
        }))
    );
    node_fs::a_sync::write_string(fd, string, encoding.into(), position, cb)
}

pub fn fs_async_write_file_with_str(
    fd: i32,
    data: &str,
    options: ffi::WriteFileOptions,
    callback: &AsyncClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::write_file_with_str(fd, data, options.into(), cb)
}

pub fn fs_async_write_file_with_bytes(fd: i32, data: &Buffer, options: ffi::WriteFileOptions, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::write_file_with_bytes(fd, &data.0, options.into(), cb)
}

pub fn fs_async_write_file_with_str_from_path(
    path: &str,
    data: &str,
    options: ffi::WriteFileOptions,
    callback: &AsyncClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::write_file_with_str_from_path(
        path, data, options.into(), cb,
    )
}

pub fn fs_async_write_file_with_bytes_from_path(
    path: &str,
    data: &Buffer,
    options: ffi::WriteFileOptions,
    callback: &AsyncClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );
    node_fs::a_sync::write_file_with_bytes_from_path(path, &data.0, options.into(), cb)
}

pub fn fs_async_writev(
    fd: i32,
    buffers: Vec<Buffer>,
    position: usize,
    callback: &AsyncUsizeClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |wrote, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

pub fn fs_handle_new_async(
    path: &str,
    flags: i32,
    mode: i32,
    callback: &AsyncFileHandleClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |handle, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(handle
                    .map(FileHandle)
                    .map(|handle| Box::new(handle)))
            }
        }))
    );

    node_fs::file_handle::FileHandle::new_async(
        path, flags, mode, cb,
    )
}

pub fn fs_handle_append_file_with_str(
    handle: &mut FileHandle,
    data: &str,
    options: ffi::AppendFileOptions,
    callback: &AsyncClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    handle.0.append_file_with_str(
        data, options.into(), cb,
    )
}

pub fn fs_handle_append_file_with_bytes(
    handle: &mut FileHandle,
    data: &Buffer,
    options: ffi::AppendFileOptions,
    callback: &AsyncClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

pub fn fs_handle_chmod(handle: &mut FileHandle, mode: u16, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

pub fn fs_handle_chown(handle: &mut FileHandle, uid: u32, gid: u32, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

pub fn fs_handle_close(handle: Box<FileHandle>, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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
// pub fn fs_handle_createReadStream(){}

// TODO
// pub fn fs_handle_createWriteStream(){}

pub fn fs_handle_datasync(handle: &mut FileHandle, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

pub fn fs_handle_fd(handle: &mut FileHandle) -> i32 {
    handle.0.fd()
}

pub fn fs_handle_read(
    handle: &mut FileHandle,
    buffer: &mut Buffer,
    offset: usize,
    length: usize,
    position: isize,
    callback: &AsyncUsizeClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |read, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

pub fn fs_handle_read_file(
    handle: &mut FileHandle,
    options: ffi::ReadFileOptions,
    callback: &AsyncFsEncodingClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

pub fn fs_handle_readv(
    handle: &mut FileHandle,
    buffers: Vec<Buffer>,
    position: usize,
    callback: &AsyncUsizeClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |read, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

pub fn fs_handle_stat(handle: &mut FileHandle, callback: &AsyncFileStatClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |stat: Option<node_fs::file_stat::FileStat>, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

pub fn fs_handle_sync(handle: &mut FileHandle, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

pub fn fs_handle_truncate(handle: &mut FileHandle, len: usize, callback: &AsyncClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

pub fn fs_handle_utimes(
    handle: &mut FileHandle,
    atime: usize,
    mtime: usize,
    callback: &AsyncClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

pub fn fs_handle_write(
    handle: &mut FileHandle,
    buffer: &Buffer,
    options: ffi::WriteOptions,
    callback: &AsyncUsizeClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |wrote, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

pub fn fs_handle_write_string(
    handle: &mut FileHandle,
    data: &str,
    encoding: ffi::StringEncoding,
    position: isize,
    callback: &AsyncUsizeClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |wrote, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(wrote)
            }
        }))
    );

    handle.0.write_string(
        data, encoding.into(), position, cb,
    )
}

pub fn fs_handle_write_file_with_str(
    handle: &mut FileHandle,
    data: &str,
    options: ffi::WriteFileOptions,
    callback: &AsyncClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(None)
            }
        }))
    );

    handle.0.write_file_with_str(
        data, options.into(), cb,
    )
}

pub fn fs_handle_write_file_with_bytes(
    handle: &mut FileHandle,
    data: &Buffer,
    options: ffi::WriteFileOptions,
    callback: &AsyncClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

pub fn fs_handle_writev(
    handle: &mut FileHandle,
    buffers: Vec<Buffer>,
    position: usize,
    callback: &AsyncUsizeClosure,
) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |wrote, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
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

#[derive(Copy, Clone, Debug)]
pub struct FileWatchEvent(node_fs::a_sync::FileWatchEvent);

#[derive(Clone, Debug)]
pub struct WatchEvent(node_fs::a_sync::WatchEvent);

#[derive(Clone)]
pub struct FileDir(node_fs::file_dir::FileDir);

pub struct FileHandle(node_fs::file_handle::FileHandle);

#[derive(Clone)]
pub struct AsyncClosure(Arc<node_fs::a_sync::AsyncClosure<(), Error>>);

#[derive(Clone)]
pub struct AsyncBoolClosure(Arc<node_fs::a_sync::AsyncClosure<bool, Error>>);

#[derive(Clone)]
pub struct AsyncFileStatClosure(Arc<node_fs::a_sync::AsyncClosure<ffi::FileStat, Error>>);

#[derive(Clone)]
pub struct AsyncStringClosure(Arc<node_fs::a_sync::AsyncClosure<String, Error>>);

#[derive(Clone)]
pub struct AsyncUsizeClosure(Arc<node_fs::a_sync::AsyncClosure<usize, Error>>);

#[derive(Clone)]
pub struct AsyncI32Closure(Arc<node_fs::a_sync::AsyncClosure<i32, Error>>);

#[derive(Clone)]
pub struct AsyncFileWatchClosure(Arc<node_fs::a_sync::AsyncClosure<FileWatchEvent, Error>>);

#[derive(Clone)]
pub struct AsyncFsEncodingClosure(Arc<node_fs::a_sync::AsyncClosure<FsEncoding, Error>>);

#[derive(Clone)]
pub struct AsyncWatchClosure(Arc<node_fs::a_sync::AsyncClosure<WatchEvent, Error>>);

#[derive(Clone)]
pub struct AsyncReaddirClosure(Arc<node_fs::a_sync::AsyncClosure<Vec<ReaddirResult>, Error>>);

#[derive(Clone)]
pub struct AsyncFileDirClosure(Arc<node_fs::a_sync::AsyncClosure<FileDir, Error>>);

#[derive(Clone)]
pub struct AsyncFileHandleClosure(Arc<node_fs::a_sync::AsyncClosure<Box<FileHandle>, Error>>);

impl Into<node_fs::sync::AppendFileOptions> for ffi::AppendFileOptions {
    fn into(self) -> node_fs::sync::AppendFileOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::MkDirOptions> for ffi::MkDirOptions {
    fn into(self) -> node_fs::sync::MkDirOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::MkdTempOptions> for ffi::MkdTempOptions {
    fn into(self) -> node_fs::sync::MkdTempOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::OpenDirOptions> for ffi::OpenDirOptions {
    fn into(self) -> node_fs::sync::OpenDirOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::ReaddirOptions> for ffi::ReaddirOptions {
    fn into(self) -> node_fs::sync::ReaddirOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::ReadFileOptions> for ffi::ReadFileOptions {
    fn into(self) -> node_fs::sync::ReadFileOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::ReadLinkOptions> for ffi::ReadLinkOptions {
    fn into(self) -> node_fs::sync::ReadLinkOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::RealPathOptions> for ffi::RealPathOptions {
    fn into(self) -> node_fs::sync::RealPathOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::RmDirOptions> for ffi::RmDirOptions {
    fn into(self) -> node_fs::sync::RmDirOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::RmOptions> for ffi::RmOptions {
    fn into(self) -> node_fs::sync::RmOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::WriteOptions> for ffi::WriteOptions {
    fn into(self) -> node_fs::sync::WriteOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}

impl Into<node_fs::sync::WriteFileOptions> for ffi::WriteFileOptions {
    fn into(self) -> node_fs::sync::WriteFileOptions {
        unsafe {
            std::mem::transmute_copy(&self)
        }
    }
}


impl From<node_fs::sync::AppendFileOptions> for ffi::AppendFileOptions {
    fn from(value: node_fs::sync::AppendFileOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::MkDirOptions> for ffi::MkDirOptions {
    fn from(value: node_fs::sync::MkDirOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::MkdTempOptions> for ffi::MkdTempOptions {
    fn from(value: node_fs::sync::MkdTempOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::OpenDirOptions> for ffi::OpenDirOptions {
    fn from(value: node_fs::sync::OpenDirOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::ReaddirOptions> for ffi::ReaddirOptions {
    fn from(value: node_fs::sync::ReaddirOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::ReadFileOptions> for ffi::ReadFileOptions {
    fn from(value: node_fs::sync::ReadFileOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::ReadLinkOptions> for ffi::ReadLinkOptions {
    fn from(value: node_fs::sync::ReadLinkOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::RealPathOptions> for ffi::RealPathOptions {
    fn from(value: node_fs::sync::RealPathOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::RmDirOptions> for ffi::RmDirOptions {
    fn from(value: node_fs::sync::RmDirOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::RmOptions> for ffi::RmOptions {
    fn from(value: node_fs::sync::RmOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::WriteOptions> for ffi::WriteOptions {
    fn from(value: node_fs::sync::WriteOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

impl From<node_fs::sync::WriteFileOptions> for ffi::WriteFileOptions {
    fn from(value: node_fs::sync::WriteFileOptions) -> Self {
        unsafe {
            std::mem::transmute_copy(&value)
        }
    }
}

fn fs_async_create_async_closure(on_success: *mut ffi::c_void, on_error: *mut ffi::c_void) -> Box<AsyncClosure> {
    Box::new(
        AsyncClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |_, error|{
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(Box<Error>)>(on_error) };
                        (on_error)(Box::new(error.unwrap()));
                    }else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn()>(on_success) };
                        (on_success)();
                    }
                }))
            )
        )
    )
}

fn fs_async_create_async_bool_closure(on_success: *mut ffi::c_void, on_error: *mut ffi::c_void) -> Box<AsyncBoolClosure> {
    Box::new(
        AsyncBoolClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error|{
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(Box<Error>)>(on_error) };
                        (on_error)(Box::new(error.unwrap()));
                    }else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(bool)>(on_success) };
                        (on_success)(value.unwrap());
                    }
                }))
            )
        )
    )
}

fn fs_async_create_async_file_stat_closure(on_success: *mut ffi::c_void, on_error: *mut ffi::c_void) -> Box<AsyncFileStatClosure> {
    Box::new(
        AsyncFileStatClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error|{
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(Box<Error>)>(on_error) };
                        (on_error)(Box::new(error.unwrap()));
                    }else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(Box<ffi::FileStat>)>(on_success) };
                        (on_success)(Box::new(value.unwrap()));
                    }
                }))
            )
        )
    )
}

fn fs_async_create_async_string_closure(on_success: *mut ffi::c_void, on_error: *mut ffi::c_void) -> Box<AsyncStringClosure> {
    Box::new(
        AsyncStringClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error|{
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(Box<Error>)>(on_error) };
                        (on_error)(Box::new(error.unwrap()));
                    }else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(String)>(on_success) };
                        (on_success)(value.unwrap());
                    }
                }))
            )
        )
    )
}

fn fs_async_create_async_usize_closure(on_success: *mut ffi::c_void, on_error: *mut ffi::c_void) -> Box<AsyncUsizeClosure> {
    Box::new(
        AsyncUsizeClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error|{
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(Box<Error>)>(on_error) };
                        (on_error)(Box::new(error.unwrap()));
                    }else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(usize)>(on_success) };
                        (on_success)(value.unwrap());
                    }
                }))
            )
        )
    )
}

fn fs_async_create_async_i32_closure(on_success: *mut ffi::c_void, on_error: *mut ffi::c_void) -> Box<AsyncI32Closure> {
    Box::new(
        AsyncI32Closure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error|{
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(Box<Error>)>(on_error) };
                        (on_error)(Box::new(error.unwrap()));
                    }else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(i32)>(on_success) };
                        (on_success)(value.unwrap());
                    }
                }))
            )
        )
    )
}

fn fs_async_create_async_file_watch_closure(on_success: *mut ffi::c_void, on_error: *mut ffi::c_void) -> Box<AsyncFileWatchClosure> {
    Box::new(
        AsyncFileWatchClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error|{
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(Box<Error>)>(on_error) };
                        (on_error)(Box::new(error.unwrap()));
                    }else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(Box<FileWatchEvent>)>(on_success) };
                        (on_success)(Box::new(value.unwrap()));
                    }
                }))
            )
        )
    )
}

fn fs_async_create_async_fs_encoding_closure(on_success: *mut ffi::c_void, on_error: *mut ffi::c_void) -> Box<AsyncFsEncodingClosure> {
    Box::new(
        AsyncFsEncodingClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error|{
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(Box<Error>)>(on_error) };
                        (on_error)(Box::new(error.unwrap()));
                    }else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(Box<FsEncoding>)>(on_success) };
                        (on_success)(Box::new(value.unwrap()));
                    }
                }))
            )
        )
    )
}

fn fs_async_create_async_fs_watch_closure(on_success: *mut ffi::c_void, on_error: *mut ffi::c_void) -> Box<AsyncWatchClosure> {
    Box::new(
        AsyncWatchClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error|{
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(Box<Error>)>(on_error) };
                        (on_error)(Box::new(error.unwrap()));
                    }else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(Box<WatchEvent>)>(on_success) };
                        (on_success)(Box::new(value.unwrap()));
                    }
                }))
            )
        )
    )
}

fn fs_async_create_async_fs_readdir_closure(on_success: *mut ffi::c_void, on_error: *mut ffi::c_void) -> Box<AsyncReaddirClosure> {
    Box::new(
        AsyncReaddirClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error|{
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(Box<Error>)>(on_error) };
                        (on_error)(Box::new(error.unwrap()));
                    }else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(Vec<ReaddirResult>)>(on_success) };
                        (on_success)(value.unwrap());
                    }
                }))
            )
        )
    )
}

fn fs_async_create_async_fs_file_dir_closure(on_success: *mut ffi::c_void, on_error: *mut ffi::c_void) -> Box<AsyncFileDirClosure> {
    Box::new(
        AsyncFileDirClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error|{
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(Box<Error>)>(on_error) };
                        (on_error)(Box::new(error.unwrap()));
                    }else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(Box<FileDir>)>(on_success) };
                        (on_success)(Box::new(value.unwrap()));
                    }
                }))
            )
        )
    )
}

fn fs_async_create_async_fs_file_handle_closure(on_success: *mut ffi::c_void, on_error: *mut ffi::c_void) -> Box<AsyncFileHandleClosure> {
    Box::new(
        AsyncFileHandleClosure(
            Arc::new(
                node_fs::a_sync::AsyncClosure::new(Box::new(move |value, error|{
                    if error.is_some() {
                        let on_error = on_error as *const ();
                        let on_error = unsafe { std::mem::transmute::<*const (), fn(Box<Error>)>(on_error) };
                        (on_error)(Box::new(error.unwrap()));
                    }else {
                        let on_success = on_success as *const ();
                        let on_success = unsafe { std::mem::transmute::<*const (), fn(Box<FileHandle>)>(on_success) };
                        (on_success)(value.unwrap());
                    }
                }))
            )
        )
    )
}


#[cxx::bridge(namespace = "org::nativescript::nodecompat")]
pub mod ffi {
    #[derive(Copy, Clone, Debug)]
    pub struct WriteFileOptions {
        encoding: StringEncoding,
        mode: i32,
        flag: i32,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct WriteOptions {
        offset: usize,
        length: usize,
        position: isize,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct RmOptions {
        force: bool,
        max_retries: i32,
        recursive: bool,
        retry_delay: u64,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct RmDirOptions {
        max_retries: i32,
        recursive: bool,
        retry_delay: u64,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct RealPathOptions {
        encoding: StringEncoding,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct ReadLinkOptions {
        encoding: FsEncodingType,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct ReadFileOptions {
        flag: i32,
        encoding: FsEncodingType,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct ReaddirOptions {
        with_file_types: bool,
        encoding: FsEncodingType,
        recursive: bool,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct OpenDirOptions {
        encoding: StringEncoding,
        buffer_size: usize,
        recursive: bool,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct MkdTempOptions {
        encoding: StringEncoding,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct MkDirOptions {
        mode: u32,
        recursive: bool,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct AppendFileOptions {
        encoding: StringEncoding,
        mode: i32,
        flag: i32,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum FsEncodingType {
        Ascii,
        Utf8,
        Utf16le,
        Ucs2,
        Latin1,
        Buffer,
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub(crate) enum ReaddirResultType {
        String,
        Buffer,
        Type,
    }

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

    extern "C++" {
        include!("include/helpers.hpp");

        #[namespace = ""]
        type c_void;
    }


    extern "Rust" {
        type Error;
        type Buffer;

        fn buffer_alloc(size: usize) -> Box<Buffer>;

        fn buffer_alloc_with_size_string_encoding(size: usize, string: &str, encoding: StringEncoding) -> Box<Buffer>;

        fn buffer_concat(buffers: &[&[u8]]) -> Box<Buffer>;

        fn buffer_concat_length(buffers: &[&[u8]], length: usize) -> Box<Buffer>;

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

        type Metadata;

        type ReaddirResult;

        type FsEncoding;

        fn fs_access_sync(path: &str, mode: i32) -> Result<()>;

        fn fs_append_file_sync(fd: i32, buffer: &Buffer, options: AppendFileOptions) -> Result<()>;

        fn fs_append_file_with_bytes_sync(fd: i32, bytes: &[u8], options: AppendFileOptions) -> Result<()>;

        fn fs_append_file_with_string_sync(fd: i32, string: &str, options: AppendFileOptions) -> Result<()>;

        fn fs_append_file_with_path_sync(path: &str, buffer: &Buffer, options: AppendFileOptions) -> Result<()>;

        fn fs_append_file_with_path_bytes_sync(path: &str, bytes: &[u8], options: AppendFileOptions) -> Result<()>;

        fn fs_append_file_with_path_string_sync(path: &str, string: &str, options: AppendFileOptions) -> Result<()>;

        fn fs_chmod_sync(path: &str, mode: u32) -> Result<()>;

        fn fs_chown_sync(path: &str, uid: u32, gid: u32) -> Result<()>;

        fn fs_close_sync(fd: i32) -> Result<()>;

        fn fs_copy_file_sync(src: &str, dest: &str, flags: u32) -> Result<()>;

        fn fs_cp_sync(src: &str, dest: &str, flags: u32) -> Result<()>;

        fn fs_exists_sync(src: &str) -> bool;

        fn fs_fchmod_sync(fd: i32, mode: u32) -> Result<()>;

        fn fs_fchown_sync(fd: i32, uid: u32, gid: u32) -> Result<()>;

        fn fs_fdatasync_sync(fd: i32) -> Result<()>;

        fn fs_fstat_sync(fd: i32) -> Result<FileStat>;

        fn fs_fsync_sync(fd: i32) -> Result<()>;

        fn fs_ftruncate_sync(fd: i32, len: usize) -> Result<()>;

        fn fs_futimes_sync(fd: i32, atime: usize, mtime: usize) -> Result<()>;

        fn fs_lchmod_sync(path: &str, mode: u32) -> Result<()>;

        fn fs_lchown_sync(path: &str, uid: u32, gid: u32) -> Result<()>;

        fn fs_lutimes_sync(path: &str, atime: i64, mtime: i64) -> Result<()>;

        fn fs_link_sync(existing_path: &str, new_path: &str) -> Result<()>;

        fn fs_lstat_sync(path: &str) -> Result<Box<Metadata>>;

        fn fs_mkdir_sync(path: &str, options: MkDirOptions) -> Result<()>;

        fn fs_mkdtemp_sync(prefix: &str, options: MkdTempOptions) -> Result<String>;

        fn fs_open_sync(path: &str, flag: i32, mode: i32) -> Result<i32>;

        fn fs_opendir_sync(path: &str, options: OpenDirOptions) -> Result<Box<FileDir>>;

        fn fs_read_sync(
            fd: i32,
            buffer: &mut [u8],
            offset: usize,
            length: usize,
            position: isize,
        ) -> Result<usize>;

        fn fs_readdir_sync(path: &str, options: ReaddirOptions) -> Result<Vec<ReaddirResult>>;

        fn fs_read_file_sync(path: &str, options: ReadFileOptions) -> Result<Box<FsEncoding>>;

        fn fs_read_file_with_fd_sync(fd: i32, options: ReadFileOptions) -> Result<Box<FsEncoding>>;

        fn fs_read_link_sync(path: &str, options: ReadLinkOptions) -> Result<Box<FsEncoding>>;

        fn fs_readv_sync(fd: i32, buffers: &mut [Buffer], position: i64) -> Result<usize>;

        fn fs_real_path_sync(path: &str, options: RealPathOptions) -> Result<String>;

        fn fs_rename_sync(old_path: &str, new_path: &str) -> Result<()>;

        fn fs_rmdir_sync(
            path: &str,
            options: RmDirOptions,
        ) -> Result<()>;

        fn fs_rm_sync(
            path: &str,
            options: RmOptions,
        ) -> Result<()>;

        fn fs_stat_sync(path: &str) -> Result<Box<Metadata>>;

        fn fs_symlink_sync(target: &str, path: &str, _type_: &str) -> Result<()>;

        fn fs_truncate_sync(path: &str, len: u64) -> Result<()>;

        fn fs_unlink_sync(path: &str) -> Result<()>;

        fn fs_utimes_sync(path: &str, atime: i64, mtime: i64) -> Result<()>;

        fn fs_write_sync(
            fd: i32,
            buffer: &[u8],
            options: WriteOptions,
        ) -> Result<usize>;

        fn fs_write_string_sync(
            fd: i32,
            string: &str,
            encoding: StringEncoding,
            position: isize,
        ) -> Result<usize>;

        fn fs_write_file_with_str_sync(fd: i32, data: &str, options: WriteFileOptions) -> Result<()>;

        fn fs_write_file_with_bytes_sync(fd: i32, data: &[u8], options: WriteFileOptions) -> Result<()>;

        fn fs_write_file_with_str_from_path_sync(
            path: &str,
            data: &str,
            options: WriteFileOptions,
        ) -> Result<()>;

        fn fs_write_file_with_bytes_from_path_sync(
            path: &str,
            data: &[u8],
            options: WriteFileOptions,
        ) -> Result<()>;

        fn fs_write_file_with_buffer_from_path_sync(
            path: &str,
            data: &Buffer,
            options: WriteFileOptions,
        ) -> Result<()>;

        fn fs_writev_sync(fd: i32, buffers: Vec<Buffer>, position: i64) -> Result<usize>;
    }

    extern "Rust" {
        // fs async

        type AsyncClosure;

        type AsyncBoolClosure;

        type AsyncFileStatClosure;

        type AsyncStringClosure;

        type AsyncUsizeClosure;

        type AsyncFsEncodingClosure;

        type AsyncFileWatchClosure;

        type AsyncWatchClosure;

        type AsyncReaddirClosure;

        type FileDir;

        type AsyncFileDirClosure;

        type FileWatchEvent;

        type WatchEvent;

        type AsyncI32Closure;


        pub unsafe fn fs_async_create_async_closure(on_success: *mut c_void, on_error: *mut c_void) -> Box<AsyncClosure>;

        unsafe fn fs_async_create_async_bool_closure(on_success: *mut c_void, on_error: *mut c_void) -> Box<AsyncBoolClosure>;

        unsafe fn fs_async_create_async_file_stat_closure(on_success: *mut c_void, on_error: *mut c_void) -> Box<AsyncFileStatClosure>;

        unsafe fn fs_async_create_async_string_closure(on_success: *mut c_void, on_error: *mut c_void) -> Box<AsyncStringClosure>;

        unsafe fn fs_async_create_async_usize_closure(on_success: *mut c_void, on_error: *mut c_void) -> Box<AsyncUsizeClosure>;

        unsafe fn fs_async_create_async_i32_closure(on_success: *mut c_void, on_error: *mut c_void) -> Box<AsyncI32Closure>;

        unsafe fn fs_async_create_async_file_watch_closure(on_success: *mut c_void, on_error: *mut c_void) -> Box<AsyncFileWatchClosure>;

        unsafe fn fs_async_create_async_fs_encoding_closure(on_success: *mut c_void, on_error: *mut c_void) -> Box<AsyncFsEncodingClosure>;

        unsafe fn fs_async_create_async_fs_watch_closure(on_success: *mut c_void, on_error: *mut c_void) -> Box<AsyncWatchClosure>;

        unsafe fn fs_async_create_async_fs_readdir_closure(on_success: *mut c_void, on_error: *mut c_void) -> Box<AsyncReaddirClosure>;

        unsafe fn fs_async_create_async_fs_file_dir_closure(on_success: *mut c_void, on_error: *mut c_void) -> Box<AsyncFileDirClosure>;

        unsafe fn fs_async_create_async_fs_file_handle_closure(on_success: *mut c_void, on_error: *mut c_void) -> Box<AsyncFileHandleClosure>;


        pub fn fs_async_access(path: &str, access: i32, callback: &AsyncClosure);

        pub fn fs_async_append_file_with_str(fd: i32, data: &str, options: AppendFileOptions, callback: &AsyncClosure);

        pub fn fs_async_append_file_with_bytes(fd: i32, data: &Buffer, options: AppendFileOptions, callback: &AsyncClosure);

        pub fn fs_async_append_file_with_path_str(
            path: &str,
            data: &str,
            options: AppendFileOptions,
            callback: &AsyncClosure,
        );

        pub fn fs_async_append_file_with_path_bytes(
            path: &str,
            data: &Buffer,
            options: AppendFileOptions,
            callback: &AsyncClosure,
        );

        pub fn fs_async_chmod(path: &str, mode: u32, callback: &AsyncClosure);

        pub fn fs_async_chown(path: &str, uid: u32, gid: u32, callback: &AsyncClosure);

        pub fn fs_async_close(fd: i32, callback: &AsyncClosure);

        pub fn fs_async_copy_file(src: &str, dest: &str, flags: u32, callback: &AsyncClosure);

        pub fn fs_async_cp(src: &str, dest: &str);

        pub fn fs_async_exists(path: &str, callback: &AsyncBoolClosure);

        pub fn fs_async_fchmod(fd: i32, mode: u16, callback: &AsyncClosure);

        pub fn fs_async_fchown(fd: i32, uid: u32, gid: u32, callback: &AsyncClosure);

        pub fn fs_async_fdatasync(fd: i32, callback: &AsyncClosure);

        pub fn fs_async_fstat(fd: i32, callback: &AsyncFileStatClosure);

        pub fn fs_async_fsync(fd: i32, callback: &AsyncClosure);

        pub fn fs_async_ftruncate(fd: i32, len: i64, callback: &AsyncClosure);

        pub fn fs_async_futimes(fd: i32, atime: i64, mtime: i64, callback: &AsyncClosure);

        pub fn fs_async_lchmod(path: &str, mode: u16, callback: &AsyncClosure);

        pub fn fs_async_lchown(path: &str, uid: u32, gid: u32, callback: &AsyncClosure);

        pub fn fs_async_lutimes(path: &str, atime: i64, mtime: i64, callback: &AsyncClosure);

        pub fn fs_async_link(existing_path: &str, new_path: &str, callback: &AsyncClosure);

        pub fn fs_async_lstat(path: &str, callback: &AsyncFileStatClosure);

        pub fn fs_async_mkdir(path: &str, options: MkDirOptions, callback: &AsyncClosure);

        pub fn fs_async_mkdtemp(prefix: &str, options: MkdTempOptions, callback: &AsyncStringClosure);

        pub fn fs_async_open(path: &str, flags: i32, mode: i32, callback: &AsyncI32Closure);

        pub fn fs_async_opendir(path: &str, options: OpenDirOptions, callback: &AsyncFileDirClosure);

        pub fn fs_async_read(
            fd: i32,
            buffer: &mut Buffer,
            offset: usize,
            length: usize,
            position: isize,
            callback: &AsyncUsizeClosure,
        );

        pub fn fs_async_readdir(
            path: &str,
            options: ReaddirOptions,
            callback: &AsyncReaddirClosure,
        );

        pub fn fs_async_read_file(path: &str, options: ReadFileOptions, callback: &AsyncFsEncodingClosure);

        pub fn fs_async_read_file_with_fd(fd: i32, options: ReadFileOptions, callback: &AsyncFsEncodingClosure);

        pub fn fs_async_read_link(path: &str, options: ReadLinkOptions, callback: &AsyncFsEncodingClosure);

        pub fn fs_async_readv(
            fd: i32,
            buffers: Vec<Buffer>,
            position: usize,
            callback: &AsyncUsizeClosure,
        );

        pub fn fs_async_real_path(path: &str, options: RealPathOptions, callback: &AsyncStringClosure);

        pub fn fs_async_rename(old_path: &str, new_path: &str, callback: &AsyncClosure);

        pub fn fs_async_rmdir(
            path: &str,
            options: RmDirOptions,
            callback: &AsyncClosure,
        );

        pub fn fs_async_rm(
            path: &str,
            options: RmOptions,
            callback: &AsyncClosure,
        );

        pub fn fs_async_stat(path: &str, throw_if_no_entry: bool, callback: &AsyncFileStatClosure);

        pub fn fs_async_symlink(target: &str, path: &str, type_: &str, callback: &AsyncClosure);

        pub fn fs_async_truncate(path: &str, len: u64, callback: &AsyncClosure);

        pub fn fs_async_unlink(path: &str, callback: &AsyncClosure);

        pub fn fs_async_unwatch_file(filename: &str);

        pub fn fs_async_unwatch_file_with_callback(filename: &str, callback: &AsyncFileWatchClosure);

        pub fn fs_async_utimes(path: &str, atime: i64, mtime: i64, callback: &AsyncClosure);

        pub fn fs_async_file_watcher_unref(filename: &str, callback: &AsyncFileWatchClosure);

        pub fn fs_async_file_watcher_ref(filename: &str, callback: &AsyncFileWatchClosure);

        pub fn fs_async_watch(
            filename: &str,
            persistent: bool,
            recursive: bool,
            encoding: FsEncodingType,
            callback: &AsyncWatchClosure,
        );

        pub fn fs_async_watcher_unref(filename: &str, callback: &AsyncWatchClosure);

        pub fn fs_async_watcher_ref(filename: &str, callback: &AsyncWatchClosure);

        pub fn fs_async_watcher_close(
            filename: &str,
            callback: &AsyncWatchClosure,
            on_close: &AsyncClosure,
        );

        pub fn fs_async_watch_file(
            filename: &str,
            _bigint: bool,
            persistent: bool,
            interval: u64,
            encoding: FsEncodingType,
            callback: &AsyncFileWatchClosure,
        );

        pub fn fs_async_write(
            fd: i32,
            buffer: &Buffer,
            options: WriteOptions,
            callback: &AsyncUsizeClosure,
        );

        pub fn fs_async_write_string(
            fd: i32,
            string: &str,
            encoding: StringEncoding,
            position: isize,
            callback: &AsyncUsizeClosure,
        );

        pub fn fs_async_write_file_with_str(
            fd: i32,
            data: &str,
            options: WriteFileOptions,
            callback: &AsyncClosure,
        );

        pub fn fs_async_write_file_with_bytes(fd: i32, data: &Buffer, options: WriteFileOptions, callback: &AsyncClosure);

        pub fn fs_async_write_file_with_str_from_path(
            path: &str,
            data: &str,
            options: WriteFileOptions,
            callback: &AsyncClosure,
        );

        pub fn fs_async_write_file_with_bytes_from_path(
            path: &str,
            data: &Buffer,
            options: WriteFileOptions,
            callback: &AsyncClosure,
        );

        pub fn fs_async_writev(
            fd: i32,
            buffers: Vec<Buffer>,
            position: usize,
            callback: &AsyncUsizeClosure,
        );
    }

    extern "Rust" {
        // filehandle
        type FileHandle;

        type AsyncFileHandleClosure;

        pub fn fs_handle_new_async(
            path: &str,
            flags: i32,
            mode: i32,
            callback: &AsyncFileHandleClosure,
        );

        pub fn fs_handle_append_file_with_str(
            handle: &mut FileHandle,
            data: &str,
            options: AppendFileOptions,
            callback: &AsyncClosure,
        );

        pub fn fs_handle_append_file_with_bytes(
            handle: &mut FileHandle,
            data: &Buffer,
            options: AppendFileOptions,
            callback: &AsyncClosure,
        );

        pub fn fs_handle_chmod(handle: &mut FileHandle, mode: u16, callback: &AsyncClosure);

        pub fn fs_handle_chown(handle: &mut FileHandle, uid: u32, gid: u32, callback: &AsyncClosure);

        pub fn fs_handle_close(handle: Box<FileHandle>, callback: &AsyncClosure);

        // TODO
        // pub fn fs_handle_createReadStream(){}

        // TODO
        // pub fn fs_handle_createWriteStream(){}

        pub fn fs_handle_datasync(handle: &mut FileHandle, callback: &AsyncClosure);

        pub fn fs_handle_fd(handle: &mut FileHandle) -> i32;

        pub fn fs_handle_read(
            handle: &mut FileHandle,
            buffer: &mut Buffer,
            offset: usize,
            length: usize,
            position: isize,
            callback: &AsyncUsizeClosure,
        );

        pub fn fs_handle_read_file(
            handle: &mut FileHandle,
            options: ReadFileOptions,
            callback: &AsyncFsEncodingClosure,
        );

        pub fn fs_handle_readv(
            handle: &mut FileHandle,
            buffers: Vec<Buffer>,
            position: usize,
            callback: &AsyncUsizeClosure,
        );

        pub fn fs_handle_stat(handle: &mut FileHandle, callback: &AsyncFileStatClosure);

        pub fn fs_handle_sync(handle: &mut FileHandle, callback: &AsyncClosure);

        pub fn fs_handle_truncate(handle: &mut FileHandle, len: usize, callback: &AsyncClosure);

        pub fn fs_handle_utimes(
            handle: &mut FileHandle,
            atime: usize,
            mtime: usize,
            callback: &AsyncClosure,
        );

        pub fn fs_handle_write(
            handle: &mut FileHandle,
            buffer: &Buffer,
            options: WriteOptions,
            callback: &AsyncUsizeClosure,
        );

        pub fn fs_handle_write_string(
            handle: &mut FileHandle,
            data: &str,
            encoding: StringEncoding,
            position: isize,
            callback: &AsyncUsizeClosure,
        );

        pub fn fs_handle_write_file_with_str(
            handle: &mut FileHandle,
            data: &str,
            options: WriteFileOptions,
            callback: &AsyncClosure,
        );

        pub fn fs_handle_write_file_with_bytes(
            handle: &mut FileHandle,
            data: &Buffer,
            options: WriteFileOptions,
            callback: &AsyncClosure,
        );

        pub fn fs_handle_writev(
            handle: &mut FileHandle,
            buffers: Vec<Buffer>,
            position: usize,
            callback: &AsyncUsizeClosure,
        );
    }
}


impl From<node_fs::file_stat::FileStat> for ffi::FileStat {
    fn from(value: node_fs::file_stat::FileStat) -> Self {
        ffi::FileStat {
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