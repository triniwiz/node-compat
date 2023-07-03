use std::borrow::Cow;
use std::ffi::{i64, c_uint, c_ushort, c_void, CString, c_ulonglong, c_ulong};
use std::os::raw::{c_int, c_long};
use std::sync::Arc;
use node_fs::file_dirent::FileDirent;
use node_fs::prelude::{FsEncodingType, handle_meta};
use crate::ffi::{ReaddirResultType};

#[derive(Clone)]
pub struct Error(node_core::error::AnyError);

impl Error {
    pub fn custom_error(clazz: &str, message: &str) -> Self {
        Self(
            node_core::error::custom_error(clazz, message)
        )
    }

    pub fn generic_error(message: &str) -> Self {
        Self(
            node_core::error::generic_error(message)
        )
    }

    pub fn type_error(message: &str) -> Self {
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


#[derive(Clone)]
pub struct Metadata(std::fs::Metadata);


#[derive(Clone, Debug)]
pub struct ReaddirResult(node_fs::sync::ReaddirResult);

impl ReaddirResult {
    pub fn get_type(&self) -> ReaddirResultType {
        match &self.0 {
            ReaddirResult::String(_) => ReaddirResultType::String,
            ReaddirResult::Buffer(_) => ReaddirResultType::Buffer,
            ReaddirResult::Type(_) => ffi: ReaddirResultType::Type
        }
    }

    pub fn get_string_value(&self) -> Result<String, String> {
        self.0.get_string_value()
            .map(|v| v.to_string_lossy().to_string())
            .ok_or("Invalid Type".to_string())
    }

    pub fn get_buffer_value(&self) -> Result<node_buffer::Buffer, String> {
        self.0.get_buffer_value().ok_or("Invalid Type".to_string())
    }

    pub fn get_type_value(&self) -> Result<FileDirent, String> {
        self.0.get_type_value().ok_or("Invalid Type".to_string())
    }
}

fn fs_access_sync(path: &str, mode: i32) -> Result<(), String> {
    node_fs::sync::access(path, mode).map_err(|e| e.to_string())
}

fn fs_append_file_sync(fd: i32, buffer: &Buffer, options: ffi::AppendFileOptions) -> Result<(), String> {
    node_fs::sync::append_file_with_buffer(fd, &buffer.0, options.into()).map_err(|e| e.to_string())
}

fn fs_append_file_with_bytes_sync(fd: i32, bytes: &[u8], options: ffi::AppendFileOptions) -> Result<(), String> {
    node_fs::sync::append_file_with_bytes(fd, bytes, options.into()).map_err(|e| e.to_string())
}

fn fs_append_file_with_string_sync(fd: i32, string: &str, options: ffi::AppendFileOptions) -> Result<(), String> {
    node_fs::sync::append_file_with_str(fd, string, options.into()).map_err(|e| e.to_string())
}

fn fs_append_file_with_path_sync(path: &str, buffer: &Buffer, options: ffi::AppendFileOptions) -> Result<(), String> {
    node_fs::sync::append_file_with_path_buffer(path, &buffer.0, options.into()).map_err(|e| e.to_string())
}

fn fs_append_file_with_path_bytes_sync(path: &str, bytes: &[u8], options: ffi::AppendFileOptions) -> Result<(), String> {
    node_fs::sync::append_file_with_path_bytes(path, bytes, options.into()).map_err(|e| e.to_string())
}

fn fs_append_file_with_path_string_sync(path: &str, string: &str, options: ffi::AppendFileOptions) -> Result<(), String> {
    node_fs::sync::append_file_with_path_str(path, string, options.into()).map_err(|e| e.to_string())
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
    let len: i64 = len.try_into().map_err(|e| e.to_string())?;
    node_fs::sync::ftruncate(fd, len as c_long).map_err(|e| e.to_string())
}

fn fs_futimes_sync(fd: i32, atime: usize, mtime: usize) -> Result<(), String> {
    let atime: i64 = atime.try_into().map_err(|e| e.to_string())?;
    let mtime: i64 = mtime.try_into().map_err(|e| e.to_string())?;
    node_fs::sync::futimes(fd, atime, mtime).map_err(|e| e.to_string())
}

fn fs_lchmod_sync(path: &str, mode: c_uint) -> Result<(), String> {
    node_fs::sync::chmod(path, mode).map_err(|e| e.to_string())
}

fn fs_lchown_sync(path: &str, uid: c_uint, gid: c_uint) -> Result<(), String> {
    node_fs::sync::chown(path, uid, gid).map_err(|e| e.to_string())
}

fn fs_lutimes_sync(path: &str, atime: i64, mtime: i64) -> Result<(), String> {
    node_fs::sync::lutimes(path, atime, mtime).map_err(|e| e.to_string())
}

fn fs_link_sync(existing_path: &str, new_path: &str) -> Result<(), String> {
    node_fs::sync::link(existing_path, new_path).map_err(|e| e.to_string())
}

fn fs_lstat_sync(path: &str) -> Result<Box<Metadata>, String> {
    node_fs::sync::lstat(path).map(|metadata| Box::new(Metadata(metadata))).map_err(|e| e.to_string())
}

fn fs_mkdir_sync(path: &str, options: ffi::MkDirOptions) -> Result<(), String> {
    node_fs::sync::mkdir(path, options.into()).map_err(|e| e.to_string())
}

fn fs_mkdtemp_sync(prefix: &str, options: ffi::MkdTempOptions) -> Result<String, String> {
    node_fs::sync::mkdtemp(prefix, options.into())
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

fn fs_open_sync(path: &str, flag: c_int, mode: c_int) -> Result<i32, String> {
    node_fs::sync::open(path, flag, mode)
        .map_err(|e| e.to_string())
}

fn fs_opendir_sync(path: &str, options: ffi::OpenDirOptions) -> Result<Box<FileDir>, String> {
    node_fs::sync::opendir(path, options.into())
        .map(|dir| {
            Box::new(FileDir(dir))
        })
        .map_err(|e| e.to_string())
}


#[cfg(not(windows))]
fn fs_read_sync(
    fd: c_int,
    buffer: &mut [u8],
    offset: usize,
    length: usize,
    position: isize,
) -> Result<usize, String> {
    node_fs::sync::read(fd, buffer, offset, length, position).map_err(|e| e.to_string())
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
        node_fs::sync::read(fd, buffer, offset, length, position).map_err(|e| e.to_string())
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

fn fs_readdir_sync(path: &str, options: ffi::ReaddirOptions) -> Result<Vec<ReaddirResult>, String> {
    node_fs::sync::readdir(path, options.into())
        .map(|mut value| {
            value.into_iter()
                .map(|value| ReaddirResult(value))
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FsEncoding(node_fs::FsEncoding);

impl FsEncoding {
    pub fn get_string_value(&self) -> Result<String, String> {
        self.0.get_string_value()
            .map(|v| v.to_string_lossy().to_string())
            .ok_or("Invalid Type".to_string())
    }

    pub fn get_buffer_value(&self) -> Result<Buffer, String> {
        self.get_buffer_value()
            .ok_or("Invalid Type".to_string())
    }
}

fn fs_read_file_sync(path: &str, options: ffi::ReadFileOptions) -> Result<FsEncoding, String> {
    node_fs::sync::read_file(path, options.into())
        .map(|f| FsEncoding(f))
        .map_err(|e| e.to_string())
}

fn fs_read_file_with_fd_sync(fd: c_int, options: ffi::ReadFileOptions) -> Result<FsEncoding, String> {
    node_fs::sync::read_file_with_fd(fd, options.into())
        .map(|f| FsEncoding(f))
        .map_err(|e| e.to_string())
}

fn fs_read_link_sync(path: &str, options: ffi::ReadLinkOptions) -> Result<FsEncoding, String> {
    node_fs::sync::read_link(path, options.into())
        .map(|f| FsEncoding(f))
        .map_err(|e| e.to_string())
}

fn fs_readv_sync(fd: c_int, buffers: &mut [Buffer], position: i64) -> Result<usize, String> {
    let mut buffers = buffers.iter().map(|buffer| buffer.0.clone())
        .collect::<Vec<node_buffer::Buffer>>();
    node_fs::sync::readv(fd, buffers.as_mut_slice(), position.try_into().unwrap())
        .map_err(|e| e.to_string())
}

fn fs_real_path_sync(path: &str, options: ffi::RealPathOptions) -> Result<String, String> {
    node_fs::sync::real_path(path, options.into())
        .map(|v| v.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

fn fs_rename_sync(old_path: &str, new_path: &str) -> Result<(), String> {
    node_fs::sync::rename(old_path, new_path)
        .map_err(|e| e.to_string())
}

fn fs_rmdir_sync(
    path: &str,
    options: ffi::RmDirOptions,
) -> Result<(), String> {
    node_fs::sync::rmdir(path, options.into())
        .map_err(|e| e.to_string())
}

fn fs_rm_sync(
    path: &str,
    options: ffi::RmOptions,
) -> Result<(), String> {
    node_fs::sync::rm(path, options.into())
        .map_err(|e| e.to_string())
}

fn fs_stat_sync(path: &str) -> Result<Box<Metadata>, String> {
    node_fs::sync::stat(path)
        .map(|meta| Box::new(Metadata(meta)))
        .map_err(|e| e.to_string())
}

fn fs_symlink_sync(target: &str, path: &str, _type_: &str) -> Result<(), String> {
    node_fs::sync::symlink(target, path, _type_)
        .map_err(|e| e.to_string())
}

fn fs_truncate_sync(path: &str, len: i64) -> Result<(), String> {
    node_fs::sync::truncate(path, len.try_into().unwrap())
        .map_err(|e| e.to_string())
}

fn fs_unlink_sync(path: &str) -> Result<(), String> {
    node_fs::sync::unlink(path)
        .map_err(|e| e.to_string())
}

fn fs_utimes_sync(path: &str, atime: i64, mtime: i64) -> Result<(), String> {
    node_fs::sync::utimes(path, atime.try_into().unwrap(), mtime.try_into().unwrap())
        .map_err(|e| e.to_string())
}

fn fs_write_sync(
    fd: c_int,
    buffer: &[u8],
    options: ffi::WriteOptions,
) -> Result<usize, String> {
    node_fs::sync::write(fd, buffer, options.into())
        .map_err(|e| e.to_string())
}

fn fs_write_string_sync(
    fd: c_int,
    string: &str,
    encoding: ffi::StringEncoding,
    position: isize,
) -> Result<usize, String> {
    node_fs::sync::write_string(
        fd, string, encoding.into(), position,
    ).map_err(|e| e.to_string())
}

fn fs_write_file_with_str_sync(fd: c_int, data: &str, options: ffi::WriteFileOptions) -> Result<(), String> {
    node_fs::sync::write_file_with_str(fd, data, options.into())
        .map_err(|e| e.to_string())
}

fn fs_write_file_with_bytes_sync(fd: c_int, data: &[u8], options: ffi::WriteFileOptions) -> Result<(), String> {
    node_fs::sync::write_file_with_bytes(fd, data, options.into())
        .map_err(|e| e.to_string())
}

fn fs_write_file_with_str_from_path_sync(
    path: &str,
    data: &str,
    options: ffi::WriteFileOptions,
) -> Result<(), String> {
    node_fs::sync::write_file_with_str_from_path(
        path, data, options.into(),
    )
        .map_err(|e| e.to_string())
}

fn fs_write_file_with_bytes_from_path_sync(
    path: &str,
    data: &[u8],
    options: ffi::WriteFileOptions,
) -> Result<(), String> {
    node_fs::sync::write_file_with_bytes_from_path(
        path, data, options.into(),
    )
        .map_err(|e| e.to_string())
}

fn fs_write_file_with_buffer_from_path_sync(
    path: &str,
    data: &Buffer,
    options: ffi::WriteFileOptions,
) -> Result<(), String> {
    node_fs::sync::write_file_with_buffer_from_path(
        path, &data.0, options.into(),
    )
        .map_err(|e| e.to_string())
}

fn fs_writev_sync(fd: c_int, mut buffers: Vec<Buffer>, position: i64) -> Result<usize, String> {
    let buffers = buffers.iter().map(|buffer| buffer.clone()).collect::<Vec<node_buffer::Buffer>>();
    node_fs::sync::writev(
        fd, buffers, position.try_into().unwrap(),
    )
        .map_err(|e| e.to_string())
}


// async

pub fn access(path: &str, access: i32, callback: AsyncClosure) {
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

pub fn append_file_with_str(fd: i32, data: &str, options: ffi::AppendFileOptions, callback: AsyncClosure) {
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

pub fn append_file_with_bytes(fd: i32, data: &Buffer, options: ffi::AppendFileOptions, callback: AsyncClosure) {
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
    node_fs::a_sync::append_file_with_bytes(fd, &data.0, options.0, cb)
}

pub fn append_file_with_path_str(
    path: &str,
    data: &str,
    options: ffi::AppendFileOptions,
    callback: AsyncClosure,
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

pub fn append_file_with_path_bytes(
    path: &str,
    data: &Buffer,
    options: ffi::AppendFileOptions,
    callback: AsyncClosure,
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

pub fn chmod(path: &str, mode: u32, callback: AsyncClosure) {
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

pub fn chown(path: &str, uid: u32, gid: u32, callback: AsyncClosure) {
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

pub fn close(fd: i32, callback: AsyncClosure) {
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

pub fn copy_file(src: &str, dest: &str, flag: u32, callback: AsyncClosure) {
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

pub fn cp(_src: &str, _dest: &str) {}

pub fn exists(path: &str, callback: AsyncBoolClosure) {
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

pub fn fchmod(fd: i32, mode: u16, callback: AsyncClosure) {
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

pub fn fchown(fd: i32, uid: u32, gid: u32, callback: AsyncClosure) {
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

pub fn fdatasync(fd: i32, callback: AsyncClosure) {
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

pub fn fstat(fd: i32, callback: AsyncFileStatClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result.map(|stat| stat.into()))
            }
        }))
    );
    node_fs::a_sync::fstat(fd, cb);
}

pub fn fsync(fd: i32, callback: AsyncClosure) {
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

pub fn ftruncate(fd: i32, len: i64, callback: AsyncClosure) {
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
    node_fs::a_sync::ftruncate(fd, len, cb)
}

pub fn futimes(fd: c_int, atime: i64, mtime: i64, callback: AsyncClosure) {
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
    node_fs::a_sync::futimes(fd, atime, mtime, cb)
}

pub fn lchmod(path: &str, mode: c_ushort, callback: AsyncClosure) {
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

pub fn lchown(path: &str, uid: c_uint, gid: c_uint, callback: AsyncClosure) {
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

pub fn lutimes(path: &str, atime: i64, mtime: i64, callback: AsyncClosure) {
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
    node_fs::a_sync::lutimes(path, atime, mtime, cb)
}

pub fn link(existing_path: &str, new_path: &str, callback: AsyncClosure) {
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

pub fn lstat(path: &str, callback: AsyncFileStatClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(result.map(|stat| stat.into()))
            }
        }))
    );

    node_fs::a_sync::lstat(path, cb)
}

pub fn mkdir(path: &str, options: ffi::MkDirOptions, callback: AsyncClosure) {
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

pub fn mkdtemp(prefix: &str, callback: AsyncStringClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |path, error| {
            if error.is_some() {
                callback.on_error(error
                    .map(node_core::error::error_from_io_error)
                    .map(Error)
                )
            } else {
                callback.on_success(path.map(|path| path.to_string_lossy().to_string()))
            }
        }))
    );
    node_fs::a_sync::mkdtemp(prefix, options.into(), cb);
}

pub fn open(path: &str, flag: c_int, mode: c_int, callback: AsyncI32Closure) {
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

pub fn opendir(path: &str, options: ffi::OpenDirOptions, callback: AsyncFileDirClosure) {
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

pub fn read(
    fd: c_int,
    buffer: &mut Buffer,
    offset: usize,
    length: usize,
    position: isize,
    callback: AsyncUsizeClosure,
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

pub fn readdir(
    path: &str,
    options: ffi::ReaddirOptions,
    callback: AsyncReaddirClosure,
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
                callback.on_success( result.map(|result|result.into_iter()
                    .map(|value| ReaddirResult(value))
                    .collect::<Vec<ReaddirResult>>()))
            }
        }))
    );

    node_fs::a_sync::readdir(path, options.into(), cb)
}

pub fn read_file(path: &str, options: ffi::ReadFileOptions, callback: AsyncFsEncodingClosure) {
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

pub fn read_file_with_fd(fd: c_int, options: ffi::ReadFileOptions, callback: AsyncFsEncodingClosure) {
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

pub fn read_link(path: &str, options: ffi::ReadLinkOptions, callback: AsyncFsEncodingClosure) {
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

pub fn readv(
    fd: c_int,
    buffers: Vec<Buffer>,
    position: i64,
    callback: AsyncUsizeClosure,
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

    node_fs::a_sync::readv(fd, buffers, position, cb)
}

pub fn real_path(path: &str, options: ffi::RealPathOptions, callback: AsyncStringClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
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

pub fn rename(old_path: &str, new_path: &str, callback: AsyncClosure) {
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

pub fn rmdir(
    path: &str,
    options: ffi::RmDirOptions,
    callback: AsyncClosure,
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

pub fn rm(
    path: &str,
    options: ffi::RmOptions,
    callback: AsyncClosure,
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

pub fn stat(path: &str, throw_if_no_entry: bool, callback: AsyncFileStatClosure) {
    let callback = Arc::clone(&callback.0);
    let cb = Arc::new(
        node_fs::a_sync::AsyncClosure::new(Box::new(move |result, error| {
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

pub fn symlink(target: &str, path: &str, type_: &str, callback: AsyncClosure) {
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

pub fn truncate(path: &str, len: c_ulonglong, callback: AsyncClosure) {
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

pub fn unlink(path: &str, callback: AsyncClosure) {
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

pub fn unwatch_file(filename: &str) {
    node_fs::a_sync::unwatch_file(filename, None)
}

pub fn unwatch_file_with_callback(filename: &str, callback: AsyncFileWatchClosure) {
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

pub fn utimes(path: &str, atime: i64, mtime: i64, callback: AsyncClosure) {
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
    node_fs::a_sync::utimes(path, atime, mtime, cb)
}

pub fn file_watcher_unref(filename: &str, callback: AsyncFileWatchClosure) {
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

pub fn file_watcher_ref(filename: &str, callback: AsyncFileWatchClosure) {
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

pub fn watch(
    filename: &str,
    persistent: bool,
    recursive: bool,
    encoding: FsEncodingType,
    callback: AsyncWatchClosure,
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
        filename, persistent, recursive, encoding, cb
    )
}

pub fn watcher_unref(filename: &str, callback: AsyncWatchClosure) {
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

pub fn watcher_ref(filename: &str, callback: AsyncWatchClosure) {
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

pub fn watcher_close(
    filename: &str,
    callback: AsyncWatchClosure,
    on_close: AsyncClosure,
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

pub fn watch_file(
    filename: &str,
    bigint: bool,
    persistent: bool,
    interval: c_ulong,
    encoding: FsEncodingType,
    callback: AsyncFileWatchClosure,
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

    node_fs::a_sync::watch_file(filename, bigint, persistent, interval, encoding, cb)
}

pub fn write(
    fd: c_int,
    buffer: &Buffer,
    options: ffi::WriteOptions,
    callback: AsyncUsizeClosure,
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

pub fn write_string(
    fd: c_int,
    string: &str,
    encoding: ffi::StringEncoding,
    position: isize,
    callback: AsyncUsizeClosure,
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

pub fn write_file_with_str(
    fd: c_int,
    data: &str,
    encoding: ffi::StringEncoding,
    callback: AsyncClosure,
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
    node_fs::a_sync::write_file_with_str(fd, data, encoding.into(), cb)
}

pub fn write_file_with_bytes(fd: c_int, data: &Buffer, options: ffi::WriteFileOptions,callback: AsyncClosure) {
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

pub fn write_file_with_str_from_path(
    path: &str,
    data: &str,
    options: ffi::WriteFileOptions,
    callback: AsyncClosure,
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
        path, data, options.into(), cb
    )
}

pub fn write_file_with_bytes_from_path(
    path: &str,
    data: &Buffer,
    options: ffi::WriteFileOptions,
    callback: AsyncClosure,
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

pub fn writev(
    fd: c_int,
    buffers: Vec<Buffer>,
    position: i64,
    callback: AsyncUsizeClosure,
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

    node_fs::a_sync::writev(fd, buffers, position, cb)
}


#[derive(Clone, Debug)]
pub struct AsyncClosure(Arc<node_fs::a_sync::AsyncClosure<(), Error>>);

#[derive(Clone, Debug)]
pub struct AsyncBoolClosure(Arc<node_fs::a_sync::AsyncClosure<bool, Error>>);

#[derive(Clone, Debug)]
pub struct AsyncFileStatClosure(Arc<node_fs::a_sync::AsyncClosure<ffi::FileStat, Error>>);

#[derive(Clone, Debug)]
pub struct AsyncStringClosure(Arc<node_fs::a_sync::AsyncClosure<String, Error>>);

#[derive(Clone, Debug)]
pub struct AsyncUsizeClosure(Arc<node_fs::a_sync::AsyncClosure<usize, Error>>);

#[derive(Clone, Debug)]
pub struct AsyncI32Closure(Arc<node_fs::a_sync::AsyncClosure<i32, Error>>);

#[derive(Copy, Clone, Debug)]
pub struct FileWatchEvent(node_fs::a_sync::FileWatchEvent);

#[derive(Clone, Debug)]
pub struct AsyncFileWatchClosure(Arc<node_fs::a_sync::AsyncClosure<ffi::FileWatchEvent, Error>>);

#[derive(Clone, Debug)]
pub struct AsyncFsEncodingClosure(Arc<node_fs::a_sync::AsyncClosure<FsEncoding, Error>>);

#[derive(Copy, Clone, Debug)]
pub struct WatchEvent(node_fs::a_sync::WatchEvent);

#[derive(Clone, Debug)]
pub struct AsyncWatchClosure(Arc<node_fs::a_sync::AsyncClosure<WatchEvent, Error>>);

#[derive(Clone, Debug)]
pub struct AsyncReaddirClosure(Arc<node_fs::a_sync::AsyncClosure<Vec<ReaddirResult>, Error>>);

#[derive(Clone, Debug)]
pub struct FileDir(node_fs::file_dir::FileDir);

#[derive(Clone, Debug)]
pub struct AsyncFileDirClosure(Arc<node_fs::a_sync::AsyncClosure<FileDir, Error>>);

#[cxx::bridge(namespace = "org::nativescript::nodecompat")]
pub mod ffi {
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

    #[derive(Debug, Clone, Copy)]
    pub struct AppendFileOptions(pub(crate) node_fs::sync::AppendFileOptions);

    #[derive(Copy, Clone, Debug)]
    pub struct MkDirOptions(pub(crate) node_fs::sync::MkDirOptions);

    #[derive(Copy, Clone, Debug)]
    pub struct MkdTempOptions(pub(crate) node_fs::sync::MkdTempOptions);

    #[derive(Copy, Clone, Debug)]
    pub struct OpenDirOptions(pub(crate) node_fs::sync::OpenDirOptions);

    #[derive(Copy, Clone, Debug)]
    pub struct ReaddirOptions(pub(crate) node_fs::sync::ReaddirOptions);

    #[derive(Copy, Clone, Debug)]
    pub struct ReadFileOptions(pub(crate) node_fs::sync::ReadFileOptions);

    #[derive(Copy, Clone, Debug)]
    pub struct ReadLinkOptions(pub(crate) node_fs::sync::ReadLinkOptions);

    #[derive(Copy, Clone, Debug)]
    pub struct RealPathOptions(pub(crate) node_fs::sync::RealPathOptions);

    #[derive(Copy, Clone, Debug)]
    pub struct RmDirOptions(pub(crate) node_fs::sync::RmDirOptions);

    #[derive(Copy, Clone, Debug)]
    pub struct RmOptions(pub(crate) node_fs::sync::RmOptions);

    #[derive(Copy, Clone, Debug)]
    pub struct WriteOptions(pub(crate) node_fs::sync::WriteOptions);

    #[derive(Copy, Clone, Debug)]
    pub struct WriteFileOptions(pub(crate) node_fs::sync::WriteFileOptions);


    extern "Rust" {
        type Error;
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

        type Metadata;

        type ReaddirResult;

        type FsEncoding;

        fn fs_access_sync(path: &str, mode: i32) -> Result<(), String>;

        fn fs_append_file_sync(fd: i32, buffer: &Buffer, options: AppendFileOptions) -> Result<(), String>;

        fn fs_append_file_with_bytes_sync(fd: i32, bytes: &[u8], options: AppendFileOptions) -> Result<(), String>;

        fn fs_append_file_with_string_sync(fd: i32, string: &str, options: AppendFileOptions) -> Result<(), String>;

        fn fs_append_file_with_path_sync(path: &str, buffer: &Buffer, options: AppendFileOptions) -> Result<(), String>;

        fn fs_append_file_with_path_bytes_sync(path: &str, bytes: &[u8], options: AppendFileOptions) -> Result<(), String>;

        fn fs_append_file_with_path_string_sync(path: &str, string: &str, options: AppendFileOptions) -> Result<(), String>;

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

        fn fs_lchmod_sync(path: &str, mode: c_uint) -> Result<(), String>;

        fn fs_lchown_sync(path: &str, uid: c_uint, gid: c_uint) -> Result<(), String>;

        fn fs_lutimes_sync(path: &str, atime: i64, mtime: i64) -> Result<(), String>;

        fn fs_link_sync(existing_path: &str, new_path: &str) -> Result<(), String>;

        fn fs_lstat_sync(path: &str) -> Result<Box<Metadata>, String>;

        fn fs_mkdir_sync(path: &str, options: MkDirOptions) -> Result<(), String>;

        fn fs_mkdtemp_sync(prefix: &str, options: MkdTempOptions) -> Result<String, String>;

        fn fs_open_sync(path: &str, flag: c_int, mode: c_int) -> Result<i32, String>;

        fn fs_opendir_sync(path: &str, options: OpenDirOptions) -> Result<Box<FileDir>, String>;

        #[cfg(not(windows))]
        fn fs_read_sync(
            fd: c_int,
            buffer: &mut [u8],
            offset: usize,
            length: usize,
            position: isize,
        ) -> Result<usize, String>;

        #[cfg(windows)]
        fn fs_read_sync(
            fd: i64,
            buffer: &mut [u8],
            offset: usize,
            length: usize,
            position: isize,
        ) -> Result<usize, String>;

        fn fs_readdir_sync(path: &str, options: ReaddirOptions) -> Result<Vec<ReaddirResult>, String>;

        fn fs_read_file_sync(path: &str, options: ReadFileOptions) -> Result<FsEncoding, String>;

        fn fs_read_file_with_fd_sync(fd: c_int, options: ReadFileOptions) -> Result<FsEncoding, String>;

        fn fs_read_link_sync(path: &str, options: ReadLinkOptions) -> Result<FsEncoding, String>;

        fn fs_readv_sync(fd: c_int, buffers: &mut [Buffer], position: i64) -> Result<usize, String>;

        fn fs_real_path_sync(path: &str, options: RealPathOptions) -> Result<String, String>;

        fn fs_rename_sync(old_path: &str, new_path: &str) -> Result<(), String>;

        fn fs_rmdir_sync(
            path: &str,
            options: RmDirOptions,
        ) -> Result<(), String>;

        fn fs_rm_sync(
            path: &str,
            options: RmOptions,
        ) -> Result<(), String>;

        fn fs_stat_sync(path: &str) -> Result<Box<Metadata>, String>;

        fn fs_symlink_sync(target: &str, path: &str, _type_: &str) -> Result<(), String>;

        fn fs_truncate_sync(path: &str, len: c_ulonglong) -> Result<(), String>;

        fn fs_unlink_sync(path: &str) -> Result<(), String>;

        fn fs_utimes_sync(path: &str, atime: i64, mtime: i64) -> Result<(), String>;

        fn fs_write_sync(
            fd: c_int,
            buffer: &[u8],
            options: WriteOptions,
        ) -> Result<usize, String>;

        fn fs_write_string_sync(
            fd: c_int,
            string: &str,
            options: WriteOptions,
        ) -> Result<usize, String>;

        fn fs_write_file_with_str_sync(fd: c_int, data: &str, options: WriteFileOptions) -> Result<(), String>;

        fn fs_write_file_with_bytes_sync(fd: c_int, data: &[u8], options: WriteFileOptions) -> Result<(), String>;

        fn fs_write_file_with_str_from_path_sync(
            path: &str,
            data: &str,
            options: WriteFileOptions,
        ) -> Result<(), String>;

        fn fs_write_file_with_bytes_from_path_sync(
            path: &str,
            data: &[u8],
            options: WriteFileOptions,
        ) -> Result<(), String>;

        fn fs_write_file_with_buffer_from_path_sync(
            path: &str,
            data: &Buffer,
            options: WriteFileOptions,
        ) -> Result<(), String>;

        fn fs_writev_sync(fd: c_int, buffers: Vec<Buffer>, position: i64) -> Result<usize, String>;
    }

    extern "Rust" {
        // fs async

        type AsyncClosure;

        type AsyncBoolClosure

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

        pub fn access(path: &str, access: i32, callback: AsyncClosure);

        pub fn append_file_with_str(fd: i32, data: &str, options: AppendFileOptions, callback: AsyncClosure);

        pub fn append_file_with_bytes(fd: i32, data: &Buffer, options: AppendFileOptions, callback: AsyncClosure);

        pub fn append_file_with_path_str(
            path: &str,
            data: &str,
            options: AppendFileOptions,
            callback: AsyncClosure,
        );

        pub fn append_file_with_path_bytes(
            path: &str,
            data: &Buffer,
            options: AppendFileOptions,
            callback: AsyncClosure,
        );

        pub fn chmod(path: &str, mode: u32, callback: AsyncClosure);

        pub fn chown(path: &str, uid: u32, gid: u32, callback: AsyncClosure);

        pub fn close(fd: i32, callback: AsyncClosure);

        pub fn copy_file(src: &str, dest: &str, flags: u32, callback: AsyncClosure);

        pub fn cp(_src: &str, _dest: &str);

        pub fn exists(path: &str, callback: AsyncBoolClosure);

        pub fn fchmod(fd: i32, mode: u16, callback: AsyncClosure);

        pub fn fchown(fd: i32, uid: u32, gid: u32, callback: AsyncClosure);

        pub fn fdatasync(fd: i32, callback: AsyncClosure);

        pub fn fstat(fd: i32, callback: AsyncFileStatClosure);

        pub fn fsync(fd: i32, callback: AsyncClosure);

        pub fn ftruncate(fd: i32, len: i64, callback: AsyncClosure);

        pub fn futimes(fd: c_int, atime: i64, mtime: i64, callback: AsyncClosure);

        pub fn lchmod(path: &str, mode: c_ushort, callback: AsyncClosure);

        pub fn lchown(path: &str, uid: c_uint, gid: c_uint, callback: AsyncClosure);

        pub fn lutimes(path: &str, atime: i64, mtime: i64, callback: AsyncClosure);

        pub fn link(existing_path: &str, new_path: &str, callback: AsyncClosure);

        pub fn lstat(path: &str, callback: AsyncFileStatClosure);

        pub fn mkdir(path: &str, mode: c_uint, recursive: bool, callback: AsyncClosure);

        pub fn mkdtemp(prefix: &str, callback: AsyncStringClosure);

        pub fn open(path: &str, flags: c_int, mode: c_int, callback: AsyncUsizeClosure);

        pub fn opendir(path: &str, callback: AsyncFileDirClosure);

        pub fn read(
            fd: c_int,
            buffer: &mut Buffer,
            offset: usize,
            length: usize,
            position: isize,
            callback: AsyncUsizeClosure,
        );

        pub fn readdir(
            path: &str,
            with_file_types: bool,
            encoding: FsEncodingType,
            callback: AsyncReaddirClosure,
        );

        pub fn read_file(path: &str, encoding: FsEncodingType, flags: c_int, callback: AsyncFsEncodingClosure);

        pub fn read_file_with_fd(fd: c_int, encoding: FsEncodingType, flags: c_int, callback: AsyncFsEncodingClosure);

        pub fn read_link(path: &str, encoding: FsEncodingType, callback: AsyncFsEncodingClosure);

        pub fn readv(
            fd: c_int,
            buffers: Vec<Buffer>,
            position: i64,
            callback: AsyncUsizeClosure,
        );

        pub fn real_path(path: &str, callback: AsyncStringClosure);

        pub fn rename(old_path: &str, new_path: &str, callback: AsyncClosure);

        pub fn rmdir(
            path: &str,
            max_retries: c_int,
            recursive: bool,
            retry_delay: c_ulonglong,
            callback: AsyncClosure,
        );

        pub fn rm(
            path: &str,
            max_retries: c_int,
            recursive: bool,
            retry_delay: c_ulonglong,
            callback: AsyncClosure,
        );

        pub fn stat(path: &str, throw_if_no_entry: bool, callback: AsyncFileStatClosure);

        pub fn symlink(target: &str, path: &str, type_: &str, callback: AsyncClosure);

        pub fn truncate(path: &str, len: c_ulonglong, callback: AsyncClosure);

        pub fn unlink(path: &str, callback: AsyncClosure);

        pub fn unwatch_file(filename: &str);

        pub fn unwatch_file_with_callback(filename: &str, callback: AsyncFileWatchClosure);

        pub fn utimes(path: &str, atime: i64, mtime: i64, callback: AsyncClosure);

        pub fn file_watcher_unref(filename: &str, callback: AsyncFileWatchClosure);

        pub fn file_watcher_ref(filename: &str, callback: AsyncFileWatchClosure);

        pub fn watch(
            filename: &str,
            persistent: bool,
            recursive: bool,
            encoding: FsEncodingType,
            callback: AsyncWatchClosure,
        );

        pub fn watcher_unref(filename: &str, callback: AsyncWatchClosure);

        pub fn watcher_ref(filename: &str, callback: AsyncWatchClosure);

        pub fn watcher_close(
            filename: &str,
            callback: AsyncWatchClosure,
            on_close: AsyncClosure,
        );

        pub fn watch_file(
            filename: &str,
            _bigint: bool,
            persistent: bool,
            interval: c_ulong,
            encoding: FsEncodingType,
            callback: AsyncFileWatchClosure,
        );

        pub fn write(
            fd: c_int,
            buffer: &Buffer,
            offset: usize,
            length: usize,
            position: isize,
            callback: AsyncUsizeClosure,
        );

        pub fn write_string(
            fd: c_int,
            string: &str,
            encoding: StringEncoding,
            position: isize,
            callback: AsyncUsizeClosure,
        );

        pub fn write_file_with_str(
            fd: c_int,
            data: &str,
            encoding: StringEncoding,
            callback: AsyncClosure,
        );

        pub fn write_file_with_bytes(fd: c_int, data: &Buffer, callback: AsyncClosure);

        pub fn write_file_with_str_from_path(
            path: &str,
            data: &str,
            encoding: StringEncoding,
            mode: c_int,
            flag: c_int,
            callback: AsyncClosure,
        );

        pub fn write_file_with_bytes_from_path(
            path: &str,
            data: &Buffer,
            mode: c_int,
            flag: c_int,
            callback: AsyncClosure,
        );

        pub fn writev(
            fd: c_int,
            buffers: Vec<Buffer>,
            position: i64,
            callback: AsyncUsizeClosure,
        );
    }
}


impl Into<node_fs::sync::AppendFileOptions> for ffi::AppendFileOptions {
    fn into(self) -> node_fs::sync::AppendFileOptions {
        self.0
    }
}

impl Into<node_fs::sync::MkDirOptions> for ffi::MkDirOptions {
    fn into(self) -> node_fs::sync::MkDirOptions {
        self.0
    }
}


impl Into<node_fs::sync::MkdTempOptions> for ffi::MkdTempOptions {
    fn into(self) -> node_fs::sync::MkdTempOptions {
        self.0
    }
}

impl Into<node_fs::sync::OpenDirOptions> for ffi::OpenDirOptions {
    fn into(self) -> node_fs::sync::OpenDirOptions {
        self.0
    }
}


impl Into<node_fs::sync::ReaddirOptions> for ffi::ReaddirOptions {
    fn into(self) -> node_fs::sync::ReaddirOptions {
        self.0
    }
}

impl Into<node_fs::sync::ReadFileOptions> for ffi::ReadFileOptions {
    fn into(self) -> node_fs::sync::ReadFileOptions {
        self.0
    }
}

impl Into<node_fs::sync::ReadLinkOptions> for ffi::ReadLinkOptions {
    fn into(self) -> node_fs::sync::ReadLinkOptions {
        self.0
    }
}

impl Into<node_fs::sync::RealPathOptions> for ffi::RealPathOptions {
    fn into(self) -> node_fs::sync::RealPathOptions {
        self.0
    }
}

impl Into<node_fs::sync::RmDirOptions> for ffi::RmDirOptions {
    fn into(self) -> node_fs::sync::RmDirOptions {
        self.0
    }
}

impl Into<node_fs::sync::RmOptions> for ffi::RmOptions {
    fn into(self) -> node_fs::sync::RmOptions {
        self.0
    }
}

impl Into<node_fs::sync::WriteOptions> for ffi::WriteOptions {
    fn into(self) -> node_fs::sync::WriteOptions {
        self.0
    }
}

impl Into<node_fs::sync::WriteFileOptions> for ffi::WriteFileOptions {
    fn into(self) -> node_fs::sync::WriteFileOptions {
        self.0
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