use std::borrow::Cow;
use std::cmp::min;
use std::ffi::{CStr, CString, OsString};
use std::fs::{File, OpenOptions, Permissions};
use std::io::{IoSlice, IoSliceMut, Read, Seek, SeekFrom, Write};
use std::os::raw::c_ulonglong;


#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
#[cfg(unix)]
use std::os::unix::prelude::IntoRawFd;
#[cfg(unix)]
use std::os::unix::prelude::*;
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;

#[cfg(windows)]
use std::os::windows::prelude::*;


use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::Duration;
use std::{fs, io};

#[cfg(windows)]
use std::os::windows::io::RawHandle;
#[cfg(windows)]
use std::os::windows::prelude::FromRawHandle;

use backoff::Error;
use backoff::ExponentialBackoff;
use faccess::PathExt;
use libc::{c_char, c_int, c_long, c_uint, c_ushort, option};
use rand::{thread_rng, Rng};
use node_buffer::{Buffer, get_bytes, StringEncoding};

use crate::file_dir::FileDir;
use crate::file_dirent::FileDirent;
use crate::file_handle::FileHandle;
use crate::{
    FILE_ACCESS_OPTIONS_F_OK, FILE_ACCESS_OPTIONS_R_OK,
    FILE_ACCESS_OPTIONS_W_OK, FILE_ACCESS_OPTIONS_X_OK, FILE_OPEN_OPTIONS_O_APPEND,
    FILE_OPEN_OPTIONS_O_CREAT, FILE_OPEN_OPTIONS_O_EXCL, FILE_OPEN_OPTIONS_O_RDONLY,
    FILE_OPEN_OPTIONS_O_TRUNC, FILE_OPEN_OPTIONS_O_WRONLY,
};
use crate::prelude::{FsEncoding, FsEncodingType};

fn file_from_path(path: &str, flag: c_int, mode: c_int) -> std::io::Result<File> {
    let mut options = OpenOptions::new();

    if (flag & FILE_OPEN_OPTIONS_O_CREAT) == FILE_OPEN_OPTIONS_O_CREAT {
        options.create(true);
    }

    if (flag & FILE_OPEN_OPTIONS_O_RDONLY) == FILE_OPEN_OPTIONS_O_RDONLY {
        options.read(true);
    }

    if (flag & FILE_OPEN_OPTIONS_O_WRONLY) == FILE_OPEN_OPTIONS_O_WRONLY {
        options.write(true);
    }

    if (flag & FILE_OPEN_OPTIONS_O_APPEND) == FILE_OPEN_OPTIONS_O_APPEND {
        options.append(true);
    }

    if (flag & FILE_OPEN_OPTIONS_O_TRUNC) == FILE_OPEN_OPTIONS_O_TRUNC {
        options.truncate(true);
    }

    if (flag & FILE_OPEN_OPTIONS_O_EXCL) == FILE_OPEN_OPTIONS_O_EXCL {
        options.create_new(true);
    }

    if mode != 0 {
        options.mode(mode as u32);
    }
    options.open(path)
}

pub fn open_path(path: &str, flag: c_int, mode: c_int) -> std::io::Result<RawFd> {
    let file = file_from_path(path, flag, mode)?;
    Ok(file.into_raw_fd())
}

pub fn open_handle_with_fd(fd: i32) -> std::io::Result<FileHandle> {
    if fd == -1 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Bad file descriptor",
        ));
    }
    let file = unsafe { File::from_raw_fd(fd) };
    Ok(FileHandle::new(file))
}

pub fn open_handle_with_path(
    path: &str,
    flag: c_int,
    mode: c_int,
) -> std::io::Result<FileHandle> {
    file_from_path(path, flag, mode).map(|v| FileHandle::new(v))
}

pub fn access(path: &str, access: c_int) -> std::io::Result<()> {
    let path = Path::new(path);
    let mut mode = faccess::AccessMode::empty();
    if (access & FILE_ACCESS_OPTIONS_F_OK) == FILE_ACCESS_OPTIONS_F_OK {
        mode |= faccess::AccessMode::EXISTS;
    }

    if (access & FILE_ACCESS_OPTIONS_W_OK) == FILE_ACCESS_OPTIONS_W_OK {
        mode |= faccess::AccessMode::WRITE;
    }

    if (access & FILE_ACCESS_OPTIONS_R_OK) == FILE_ACCESS_OPTIONS_R_OK {
        mode |= faccess::AccessMode::READ;
    }

    if (access & FILE_ACCESS_OPTIONS_X_OK) == FILE_ACCESS_OPTIONS_X_OK {
        mode |= faccess::AccessMode::EXECUTE;
    }

    path.access(mode)
}


#[derive(Clone, Copy, Debug)]
pub struct AppendFileOptions {
    encoding: StringEncoding,
    mode: i32,
    flag: i32,
}

impl Default for AppendFileOptions {
    fn default() -> Self {
        Self {
            encoding: StringEncoding::Utf8,
            mode: 0o666,
            flag: FILE_OPEN_OPTIONS_O_APPEND,
        }
    }
}

pub fn append_file_with_str(fd: c_int, data: &str, options: AppendFileOptions) -> std::io::Result<()> {
    let mut file = unsafe { File::from_raw_fd(fd) };
    let bytes = Buffer::from_string(CString::new(data).unwrap(), options.encoding);
    let _ = file.write(bytes.buffer())?;
    let _ = file.into_raw_fd();
    Ok(())
}

pub fn append_file_with_bytes(fd: c_int, data: &[u8], options: AppendFileOptions) -> std::io::Result<()> {
    let mut file = unsafe { File::from_raw_fd(fd) };
    let _ = file.write(data)?;
    let _ = file.into_raw_fd();
    Ok(())
}

pub fn append_file_with_buffer(fd: c_int, data: &Buffer, options: AppendFileOptions) -> std::io::Result<()> {
    let data = data.buffer();
    append_file_with_bytes(fd, data, options)
}

pub fn append_file_with_path_str(
    path: &str,
    data: &str,
    options: AppendFileOptions,
) -> std::io::Result<()> {
    let fd = open(path, options.flag, options.mode)?;
    let mut file = unsafe { File::from_raw_fd(fd) };
    let buffer = Buffer::from_string(CString::new(data).unwrap(), options.encoding);
    let ret = file.write(buffer.buffer()).map(|_| ());
    let _ = file.into_raw_fd();
    ret
}

pub fn append_file_with_path_bytes(
    path: &str,
    data: &[u8],
    options: AppendFileOptions,
) -> std::io::Result<()> {
    let fd = open(path, options.flag, options.mode)?;
    let mut file = unsafe { File::from_raw_fd(fd) };
    let ret = file.write(data).map(|_| ());
    let _ = file.into_raw_fd();
    ret
}

pub fn append_file_with_path_buffer(
    path: &str,
    data: &Buffer,
    options: AppendFileOptions,
) -> std::io::Result<()> {
    let data = data.buffer();
    append_file_with_path_bytes(path, data, options)
}

pub fn chmod(path: &str, mode: c_uint) -> std::io::Result<()> {
    fs::set_permissions(path, Permissions::from_mode(mode))
}

pub fn chown(path: &str, uid: c_uint, gid: c_uint) -> std::io::Result<()> {
    std::os::unix::fs::chown(path, Some(uid), Some(gid))
}

pub fn close_fd(fd: c_int) {
    unsafe {
        File::from_raw_fd(fd);
    }
}

pub fn copy_file(src: &str, dest: &str, flag: c_uint) -> std::io::Result<()> {
    crate::copy_file::copy_file(Path::new(src), Path::new(dest), flag)
}

pub fn cp(_src: &str, _dest: &str, flag: u32) {
    todo!()
    // let src = Path::new(src);
    // let dest = Path::new(dest);
    // for entry in fs::read_dir(src)? {
    //     let entry = entry?;
    // }
}

pub fn create_read_stream(_path: &str) {
    todo!()
}

pub fn create_write_stream(_path: &str) {
    todo!()
}

pub fn exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn fchmod(fd: c_int, mode: c_ushort) -> io::Result<()> {
    let ret = unsafe { libc::fchmod(fd, mode.into()) };
    if ret == -1 {
        let last_error = std::io::Error::last_os_error();
        return Err(last_error);
    }
    Ok(())
}

pub fn fchown(fd: c_int, uid: c_uint, gid: c_uint) -> io::Result<()> {
    let ret = unsafe { libc::fchown(fd, uid, gid) };
    if ret == -1 {
        let last_error = std::io::Error::last_os_error();
        return Err(last_error);
    }
    Ok(())
}

pub fn fdatasync(fd: c_int) -> std::io::Result<()> {
    let file = unsafe { File::from_raw_fd(fd) };
    let ret = file.sync_data();
    let _ = file.into_raw_fd();
    ret
}

pub fn fstat(fd: c_int) -> io::Result<fs::Metadata> {
    let file = unsafe { File::from_raw_fd(fd) };
    let metadata = file.metadata();
    let _ = file.into_raw_fd();
    metadata
}

pub fn fsync(fd: c_int) -> std::io::Result<()> {
    let file = unsafe { File::from_raw_fd(fd) };
    let ret = file.sync_all();
    let _ = file.into_raw_fd();
    ret
}

pub fn ftruncate(fd: c_int, len: c_long) -> std::io::Result<()> {
    let ret = unsafe { libc::ftruncate(fd, len) };
    if ret == -1 {
        let last_error = std::io::Error::last_os_error();
        return Err(last_error);
    }
    Ok(())
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub fn futimes(fd: c_int, atime: c_long, mtime: c_long) -> std::io::Result<()> {
    let times = [
        libc::timespec {
            tv_sec: atime,
            tv_nsec: 0,
        },
        libc::timespec {
            tv_sec: mtime,
            tv_nsec: 0,
        },
    ];

    let ret = unsafe { libc::futimens(fd, times.as_ptr()) };

    if ret == -1 {
        let last_error = std::io::Error::last_os_error();
        return Err(last_error);
    }
    Ok(())
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn futimes(fd: c_int, atime: c_long, mtime: c_long) -> std::io::Result<()> {
    let times = [
        libc::timeval {
            tv_sec: atime,
            tv_usec: 0,
        },
        libc::timeval {
            tv_sec: mtime,
            tv_usec: 0,
        },
    ];

    let ret = unsafe { libc::futimes(fd, times.as_ptr()) };

    if ret == -1 {
        let last_error = std::io::Error::last_os_error();
        return Err(last_error);
    }
    Ok(())
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub fn lchmod(path: &str, mode: c_ushort) -> std::io::Result<()> {
    let mut options = OpenOptions::new();
    options.write(true);
    let file = options.open(path)?;
    let permissions = Permissions::from_mode(mode.into());
    file.set_permissions(permissions)
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn lchmod(path: &str, mode: c_ushort) -> std::io::Result<()> {
    mod internal {
        extern "C" {
            pub fn lchmod(__file: *const libc::c_char, __mode: libc::mode_t) -> libc::c_int;
        }
    }
    let path = CString::new(path)?;
    let ret = unsafe { internal::lchmod(path.as_ptr(), mode.into()) };

    if ret == -1 {
        let last_error = std::io::Error::last_os_error();
        return Err(last_error);
    }
    Ok(())
}

pub fn lchown(path: &str, uid: c_uint, gid: c_uint) -> std::io::Result<()> {
    let path = CString::new(path).unwrap();
    let ret = unsafe { libc::lchown(path.as_ptr(), uid, gid) };

    if ret == -1 {
        let last_error = std::io::Error::last_os_error();
        return Err(last_error);
    }
    Ok(())
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub fn lutimes(path: &str, atime: c_long, mtime: c_long) -> std::io::Result<()> {
    let file = File::open(path)?;
    let times = [
        libc::timespec {
            tv_sec: atime,
            tv_nsec: 0,
        },
        libc::timespec {
            tv_sec: mtime,
            tv_nsec: 0,
        },
    ];
    let ret = unsafe { libc::futimens(file.as_raw_fd(), times.as_ptr()) };

    if ret == -1 {
        let last_error = std::io::Error::last_os_error();
        return Err(last_error);
    }
    Ok(())
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn lutimes(path: &str, atime: c_long, mtime: c_long) -> std::io::Result<()> {
    let file = File::open(path)?;
    let times = [
        libc::timeval {
            tv_sec: atime,
            tv_usec: 0,
        },
        libc::timeval {
            tv_sec: mtime,
            tv_usec: 0,
        },
    ];
    let ret = unsafe { libc::futimes(file.as_raw_fd(), times.as_ptr()) };

    if ret == -1 {
        let last_error = std::io::Error::last_os_error();
        return Err(last_error);
    }
    Ok(())
}

pub fn link(existing_path: &str, new_path: &str) -> std::io::Result<()> {
    fs::hard_link(existing_path, new_path)
}

pub fn lstat(path: &str) -> std::io::Result<std::fs::Metadata> {
    fs::metadata(path)
}


#[derive(Copy, Clone, Debug)]
pub struct MkDirOptions {
    mode: u32,
    recursive: bool,
}

impl Default for MkDirOptions {
    fn default() -> Self {
        Self {
            mode: 0o777,
            recursive: false,
        }
    }
}

pub fn mkdir(path: &str, options: MkDirOptions) -> std::io::Result<()> {
    let path = Path::new(&path);

    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(options.recursive);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(options.mode);
    }
    builder.create(&path)
}

// https://github.com/denoland/deno/blob/5e845442fade02cd12d13e74222b26e217c5971d/runtime/ops/fs.rs#L1649
pub(crate) fn make_temp(
    dir: Option<&Path>,
    prefix: Option<&str>,
    suffix: Option<&str>,
    is_dir: bool,
    options: MkdTempOptions,
) -> std::io::Result<std::path::PathBuf> {
    let prefix_ = prefix
        .map(|data| {
            let buffer = Buffer::from_string(CString::new(data).unwrap(), options.encoding);
            buffer.as_string(None, None, None)
        })
        .unwrap_or_default();

    let suffix_ = suffix.unwrap_or("");
    let mut buf: PathBuf = match dir {
        Some(p) => p.to_path_buf(),
        None => std::env::temp_dir(),
    }
        .join("_");
    let mut rng = thread_rng();
    loop {
        let unique = rng.gen::<u32>();
        buf.set_file_name(format!("{}{:08x}{}", prefix_, unique, suffix_));
        let r = if is_dir {
            #[allow(unused_mut)]
                let mut builder = fs::DirBuilder::new();
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt;
                builder.mode(0o700);
            }
            builder.create(buf.as_path())
        } else {
            let mut open_options = std::fs::OpenOptions::new();
            open_options.write(true).create_new(true);
            #[cfg(unix)]
            {
                open_options.mode(0o600);
            }
            open_options.open(buf.as_path())?;
            Ok(())
        };
        match r {
            Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Ok(_) => return Ok(buf),
            Err(e) => return Err(e),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MkdTempOptions {
    encoding: StringEncoding,
}

impl Default for MkdTempOptions {
    fn default() -> Self {
        Self { encoding: StringEncoding::Utf8 }
    }
}

pub fn mkdtemp(prefix: &str, options: MkdTempOptions) -> std::io::Result<PathBuf> {
    make_temp(None, Some(prefix), None, true, options)
}

pub fn open(path: &str, flag: c_int, mode: c_int) -> std::io::Result<RawFd> {
    open_path(path, flag, mode)
}


#[derive(Copy, Clone, Debug)]
pub struct OpenDirOptions {
    encoding: StringEncoding,
    buffer_size: usize,
    recursive: bool,
}

impl Default for OpenDirOptions {
    fn default() -> Self {
        Self {
            encoding: StringEncoding::Utf8,
            buffer_size: 32,
            recursive: false,
        }
    }
}

pub fn opendir(path: &str, options: OpenDirOptions) -> io::Result<FileDir> {
    let c_path = CString::new(path)?;
    let dir = unsafe { libc::opendir(c_path.as_ptr()) };
    if dir.is_null() {
        let last_error = io::Error::last_os_error();
        return Err(last_error);
    }
    Ok(FileDir::new(path.to_string(), dir))
}

#[cfg(not(windows))]
pub fn read(
    fd: c_int,
    buffer: &mut [u8],
    offset: usize,
    length: usize,
    position: isize,
) -> std::io::Result<usize> {
    let mut file = unsafe { File::from_raw_fd(fd) };

    read_file_internal(&mut file, buffer, offset, length, position)
}

#[cfg(windows)]
pub fn read(
    fd: RawHandle,
    buffer: &mut [u8],
    offset: usize,
    length: usize,
    position: isize,
) -> std::io::Result<usize> {
    let mut file = unsafe { File::from_raw_handle(fd) };
    read_file_internal(&mut file, buffer, offset, length, position)
}

fn read_file_internal(
    file: &mut File,
    buffer: &mut [u8],
    offset: usize,
    length: usize,
    position: isize,
) -> std::io::Result<usize> {
    if position != -1 {
        match file.seek(SeekFrom::Start(position as u64)) {
            Ok(_) => {}
            Err(error) => return std::io::Result::Err(error),
        }
    }

    let new_position = file.stream_position().unwrap_or_default();
    let buffer_len = buffer.len();
    let tmp_buf = &mut buffer[offset..];
    let result = if length < buffer_len {
        let buf = &mut tmp_buf[..length];
        file.read(buf)
    } else {
        file.read(tmp_buf)
    };

    match result {
        Ok(read) => {
            if read == 0 {
                return Ok(min(new_position as usize, read));
            }
            Ok(read)
        }
        Err(error) => Err(error),
    }
}


#[derive(Clone, Debug)]
pub enum ReaddirResult {
    String(CString),
    Buffer(Buffer),
    Type(FileDirent),
}

impl ReaddirResult {
    pub fn get_string_value(&self) -> Option<CString> {
        match &self.0 {
            ReaddirResult::String(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_buffer_value(&self) -> Option<Buffer> {
        match &self.0 {
            ReaddirResult::Buffer(buffer) => { Some(buffer.clone()) }
            _ => { None }
        }
    }

    pub fn get_type_value(&self) -> Option<FileDirent> {
        match &self.0 {
            ReaddirResult::Type(dir) => { Some(dir.clone()) }
            _ => None
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ReaddirOptions {
    with_file_types: bool,
    encoding: FsEncodingType,
    recursive: bool,
}

impl Default for ReaddirOptions {
    fn default() -> Self {
        Self {
            with_file_types: false,
            encoding: FsEncodingType::Utf8,
            recursive: false,
        }
    }
}

pub fn readdir(path: &str, options: ReaddirOptions) -> io::Result<Vec<ReaddirResult>> {
    let read = fs::read_dir(path)?;
    read.map(|entry| {
        let dir = entry?;
        if options.with_file_types {
            ReaddirResult::Type(FileDirent::new_regular(dir))
        } else {
            let buffer = Buffer::from_string(
                CString::new(dir.file_name().to_string_lossy().to_string()).unwrap(), StringEncoding::Utf8,
            );
            match options.encoding {
                FsEncodingType::Ascii => {
                    CString::new(
                        buffer.as_string(Some(StringEncoding::Ascii), None, None)
                    ).unwrap()
                }
                FsEncodingType::Utf8 => {
                    CString::new(
                        buffer.as_string(Some(StringEncoding::Utf8), None, None)
                    ).unwrap()
                }
                FsEncodingType::Utf16le => {
                    CString::new(
                        buffer.as_string(Some(StringEncoding::Utf16le), None, None)
                    ).unwrap()
                }
                FsEncodingType::Ucs2 => {
                    CString::new(
                        buffer.as_string(Some(StringEncoding::Ucs2), None, None)
                    ).unwrap()
                }
                FsEncodingType::Latin1 => {
                    ReaddirResult::String(
                        CString::new(
                            buffer.as_string(Some(StringEncoding::Latin1), None, None)
                        ).unwrap()
                    )
                }
                FsEncodingType::Buffer => {
                    ReaddirResult::Buffer(buffer)
                }
            }
        }
    })
        .collect::<io::Result<Vec<ReaddirResult>>>()
}


#[derive(Copy, Clone, Debug)]
pub struct ReadFileOptions {
    flag: i32,
    encoding: FsEncodingType,
}

impl Default for ReadFileOptions {
    fn default() -> Self {
        Self {
            flag: FILE_OPEN_OPTIONS_O_RDONLY,
            encoding: FsEncodingType::Buffer,
        }
    }
}

impl ReadFileOptions {
    pub fn encoding(&self) -> FsEncodingType {
        self.encoding
    }

    pub fn set_encoding(&mut self, encoding: FsEncodingType) {
        self.encoding = encoding
    }
}

fn read_file_with_file(file: &mut File, options: ReadFileOptions) -> std::io::Result<FsEncoding> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let result = Buffer::from_vec(buf);
    let result = match options.encoding {
        FsEncodingType::Ascii => {
            FsEncoding::String(
                CString::new(
                    result.as_string(Some(StringEncoding::Ascii), None, None)
                ).unwrap()
            ).into()
        }
        FsEncodingType::Utf8 => {
            FsEncoding::String(
                CString::new(
                    result.as_string(Some(StringEncoding::Utf8), None, None)
                ).unwrap()
            ).into()
        }
        FsEncodingType::Utf16le => {
            FsEncoding::String(
                CString::new(
                    result.as_string(Some(StringEncoding::Utf16le), None, None)
                ).unwrap()
            ).into()
        }
        FsEncodingType::Ucs2 => {
            FsEncoding::String(
                CString::new(
                    result.as_string(Some(StringEncoding::Latin1), None, None)
                ).unwrap()
            ).into()
        }
        FsEncodingType::Latin1 => {
            FsEncoding::String(
                CString::new(
                    result.as_string(Some(StringEncoding::Latin1), None, None)
                ).unwrap()
            )
        }
        FsEncodingType::Buffer => {
            FsEncoding::Buffer(result).into()
        }
    };
    Ok(result)
}

pub fn read_file(path: &str, options: ReadFileOptions) -> std::io::Result<FsEncoding> {
    let mut file = file_from_path(path, options.flag, 0)?;
    read_file_with_file(&mut file, options)
}

pub fn read_file_with_fd(fd: c_int, options: ReadFileOptions) -> std::io::Result<FsEncoding> {
    let mut file = unsafe { File::from_raw_fd(fd) };
    read_file_with_file(&mut file, options)
}


#[derive(Copy, Clone, Debug)]
pub struct ReadLinkOptions {
    encoding: FsEncodingType,
}

impl Default for ReadLinkOptions {
    fn default() -> Self {
        Self { encoding: FsEncodingType::Utf8 }
    }
}

pub fn read_link(path: &str, options: ReadLinkOptions) -> std::io::Result<FsEncoding> {
    let result = fs::read_link(path)?;

    #[cfg(unix)]
        let result = result.into_os_string().into_vec();


    #[cfg(windows)]
        let result = result.into_os_string().encode_wide().chain(Some(0)).collect::<Vec<_>>();

    let result = CString::new(
        result
    )?;
    let buffer = Buffer::from_string(result, StringEncoding::Utf8);
    let result = match options.encoding {
        FsEncodingType::Ascii => {
            buffer.as_string(Some(StringEncoding::Ascii), None, None).into()
        }
        FsEncodingType::Utf8 => {
            buffer.as_string(Some(StringEncoding::Ascii), None, None).into()
        }
        FsEncodingType::Utf16le => {
            buffer.as_string(Some(StringEncoding::Ascii), None, None).into()
        }
        FsEncodingType::Ucs2 => {
            buffer.as_string(Some(StringEncoding::Ascii), None, None).into()
        }
        FsEncodingType::Latin1 => {
            buffer.as_string(Some(StringEncoding::Ascii), None, None).into()
        }
        FsEncodingType::Buffer => {
            buffer.into()
        }
    };

    Ok(result)
}

pub fn readv(fd: c_int, buffers: &mut [Buffer], position: c_long) -> std::io::Result<usize> {
    let mut file = unsafe { File::from_raw_fd(fd) };

    if position != -1 {
        match file.seek(SeekFrom::Start(position as u64)) {
            Ok(_) => {}
            Err(error) => return Err(error),
        }
    }

    let mut buffers: Vec<IoSliceMut> = buffers
        .iter_mut()
        .map(|mut b| {
            IoSliceMut::new(b.buffer_mut())
        })
        .collect();

    file.read_vectored(buffers.as_mut_slice())
}

pub fn readv_raw(
    fd: c_int,
    buffer: *const *mut Buffer,
    buffer_len: usize,
    position: c_long,
) -> io::Result<usize> {
    let buf = unsafe { std::slice::from_raw_parts(buffer, buffer_len) };

    let mut slice_buf = Vec::with_capacity(buffer_len);
    unsafe {
        for item in buf.iter() {
            let item = &*(*item);
            slice_buf.push(item.clone())
        }
    }

    readv(fd, slice_buf.as_mut_slice(), position)
}


#[derive(Copy, Clone, Debug)]
pub struct RealPathOptions {
    encoding: StringEncoding,
}

impl Default for RealPathOptions {
    fn default() -> Self {
        Self { encoding: StringEncoding::Utf8 }
    }
}

pub fn real_path(path: &str, options: RealPathOptions) -> std::io::Result<std::path::PathBuf> {
    match options.encoding {
        StringEncoding::Utf8 => {
            std::fs::canonicalize(path)
        }
        _ => {
            let buffer = Buffer::from_string(CString::new(path).unwrap(), options.encoding);
            let path = buffer.as_string(None, None, None);
            std::fs::canonicalize(path)
        }
    }
}

pub fn rename(old_path: &str, new_path: &str) -> std::io::Result<()> {
    fs::rename(Path::new(old_path), Path::new(new_path))
}


#[derive(Copy, Clone, Debug)]
pub struct RmDirOptions {
    max_retries: i32,
    recursive: bool,
    retry_delay: c_ulonglong,
}

impl Default for RmDirOptions {
    fn default() -> Self {
        Self {
            max_retries: 0,
            recursive: false,
            retry_delay: 100,
        }
    }
}

pub fn rmdir(
    path: &str,
    options: RmDirOptions,
) -> Result<(), node_core::error::AnyError> {
    if !options.recursive {
        fs::remove_dir(path).map_err(|err| node_core::error::custom_error("", err.to_string()))
    } else {
        let mut max_retries_count = AtomicI32::new(options.max_retries);
        let op = || {
            fs::remove_dir_all(path).map(|_| ()).map_err(|e| {
                if max_retries != 0 {
                    let current = max_retries_count.load(Ordering::SeqCst);
                    if current == 0 {
                        return Error::permanent(e);
                    }
                    *max_retries_count.get_mut() = current - 1;
                }

                let error_string = e.to_string();

                if error_string.contains("too many open files") {
                    return Error::Transient {
                        err: e,
                        retry_after: Some(Duration::from_millis(options.retry_delay)),
                    };
                }

                Error::permanent(e)
            })
        };
        let bf = ExponentialBackoff::default();
        backoff::retry(bf, op).map_err(|e| match e {
            Error::Permanent(ref err) => {
                node_core::error::custom_error("", err.to_string())
            }
            Error::Transient { ref err, .. } => {
                node_core::error::custom_error("", err.to_string())
            }
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RmOptions {
    force: bool,
    max_retries: i32,
    recursive: bool,
    retry_delay: c_ulonglong,
}

impl Default for RmOptions {
    fn default() -> Self {
        Self {
            force: false,
            max_retries: 0,
            recursive: false,
            retry_delay: 100,
        }
    }
}

pub fn rm(
    path: &str,
    options: RmOptions,
) -> Result<(), node_core::error::AnyError> {
    if !options.recursive {
        fs::remove_file(path).map_err(|err| node_core::error::custom_error("", err.to_string()))
    } else {
        let mut max_retries_count = AtomicI32::new(options.max_retries);
        let op = || {
            fs::remove_file(path).map_err(|e| {
                if max_retries != 0 {
                    let current = max_retries_count.load(Ordering::SeqCst);
                    if current == 0 {
                        return Error::permanent(e);
                    }
                    *max_retries_count.get_mut() = current - 1;
                }

                let error_string = e.to_string();

                if error_string.contains("too many open files") {
                    return Error::Transient {
                        err: e,
                        retry_after: Some(Duration::from_millis(options.retry_delay)),
                    };
                }

                Error::permanent(e)
            })
        };
        let bf = ExponentialBackoff::default();
        backoff::retry(bf, op).map_err(|e| match e {
            Error::Permanent(ref err) => {
                node_core::error::custom_error("", err.to_string())
            }
            Error::Transient { ref err, .. } => {
                node_core::error::custom_error("", err.to_string())
            }
        })
    }
}

pub fn stat(path: &str) -> std::io::Result<std::fs::Metadata> {
    fs::metadata(path)
}

pub fn symlink(target: &str, path: &str, _type_: &str) -> std::io::Result<()> {
    // todo handle type
    std::os::unix::fs::symlink(target, path)
}

pub fn truncate(path: &str, len: c_ulonglong) -> std::io::Result<()> {
    OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(path)?
        .set_len(len)
}

pub fn unlink(path: &str) -> std::io::Result<()> {
    fs::remove_file(path)
}

// pub fn unwatchFile(filename){}

pub fn utimes(path: &str, atime: c_long, mtime: c_long) -> std::io::Result<()> {
    let path = CString::new(path)?;
    let times = [
        libc::timeval {
            tv_sec: atime,
            tv_usec: 0,
        },
        libc::timeval {
            tv_sec: mtime,
            tv_usec: 0,
        },
    ];
    let ret = unsafe { libc::utimes(path.as_ptr(), times.as_ptr()) };

    if ret == -1 {
        let last_error = std::io::Error::last_os_error();
        return Err(last_error);
    }
    Ok(())
}

// pub fn watch(){}

// pub fn watchFile(){}


#[derive(Copy, Clone, Debug)]
pub struct WriteOptions {
    offset: usize,
    length: usize,
    position: isize,
}

pub fn write(
    fd: c_int,
    buffer: &[u8],
    options: WriteOptions,
) -> std::io::Result<usize> {
    let mut file = unsafe { File::from_raw_fd(fd) };
    let new_position = file.stream_position().unwrap_or_default();
    let buffer_len = buffer.len();
    let result = if options.length < buffer_len {
        let tmp_buf = &buffer[options.offset..];
        let buf = &tmp_buf[..options.length];
        if position == -1 {
            file.write(buf)
        } else {
            file.write_at(buffer, options.position as u64)
        }
    } else if position == -1 {
        file.write(buffer)
    } else {
        file.write_at(buffer, options.position as u64)
    };

    let ret = match result {
        Ok(wrote) => {
            if wrote == 0 {
                return Ok(std::cmp::min(new_position as usize, wrote));
            }
            std::io::Result::Ok(wrote)
        }
        Err(error) => std::io::Result::Err(error),
    };

    let _ = file.into_raw_fd();

    ret
}

pub fn write_string(
    fd: c_int,
    string: &str,
    encoding: StringEncoding,
    position: isize,
) -> std::io::Result<usize> {
    let mut file = unsafe { File::from_raw_fd(fd) };
    let new_position = file.stream_position().unwrap_or_default();
    let buffer = get_bytes(string, encoding);
    let result = if position == -1 {
        file.write(buffer.as_slice())
    } else {
        file.write_at(buffer.as_slice(), position as u64)
    };

    let ret = match result {
        Ok(wrote) => {
            if wrote == 0 {
                return Ok(min(new_position as usize, wrote));
            }
            Ok(wrote)
        }
        Err(error) => Err(error),
    };

    let _ = file.into_raw_fd();

    ret
}


#[derive(Copy, Clone, Debug)]
pub struct WriteFileOptions {
    encoding: StringEncoding,
    mode: i32,
    flag: i32,
}

impl Default for WriteFileOptions {
    fn default() -> Self {
        Self {
            encoding: StringEncoding::Utf8,
            mode: 0o666,
            flag: FILE_OPEN_OPTIONS_O_WRONLY,
        }
    }
}

pub fn write_file_with_str(fd: c_int, data: &str, options: WriteFileOptions) -> std::io::Result<()> {
    let mut file = unsafe { File::from_raw_fd(fd) };
    let data = get_bytes(data, options.encoding);
    let ret = file.write(data.as_slice());
    let _ = file.into_raw_fd();
    ret.map(|_| ())
}

pub fn write_file_with_bytes(fd: c_int, data: &[u8], options: WriteFileOptions) -> std::io::Result<()> {
    let mut file = unsafe { File::from_raw_fd(fd) };
    let ret = file.write(data);
    let _ = file.into_raw_fd();
    ret.map(|_| ())
}

pub fn write_file_with_str_from_path(
    path: &str,
    data: &str,
    options: WriteFileOptions,
) -> std::io::Result<()> {
    let mut opts = OpenOptions::new();
    if (options.flag & FILE_OPEN_OPTIONS_O_CREAT) == FILE_OPEN_OPTIONS_O_CREAT {
        opts.create(true);
    }

    if (options.flag & FILE_OPEN_OPTIONS_O_RDONLY) == FILE_OPEN_OPTIONS_O_RDONLY {
        opts.read(true);
    }

    if (options.flag & FILE_OPEN_OPTIONS_O_WRONLY) == FILE_OPEN_OPTIONS_O_WRONLY {
        opts.write(true);
    }

    if (options.flag & FILE_OPEN_OPTIONS_O_APPEND) == FILE_OPEN_OPTIONS_O_APPEND {
        opts.append(true);
    }

    if (options.flag & FILE_OPEN_OPTIONS_O_TRUNC) == FILE_OPEN_OPTIONS_O_TRUNC {
        opts.truncate(true);
    }

    if (options.flag & FILE_OPEN_OPTIONS_O_EXCL) == FILE_OPEN_OPTIONS_O_EXCL {
        opts.create_new(true);
    }

    if mode != 0 {
        opts.mode(options.mode as u32);
    }

    let mut file = opts.open(path)?;
    let data = get_bytes(data, encoding);
    file.write(data.as_slice()).map(|_| ())
}

pub fn write_file_with_bytes_from_path(
    path: &str,
    data: &[u8],
    options: WriteFileOptions,
) -> std::io::Result<()> {
    let mut opts = OpenOptions::new();
    if (options.flag & FILE_OPEN_OPTIONS_O_CREAT) == FILE_OPEN_OPTIONS_O_CREAT {
        opts.create(true);
    }

    if (options.flag & FILE_OPEN_OPTIONS_O_RDONLY) == FILE_OPEN_OPTIONS_O_RDONLY {
        opts.read(true);
    }

    if (options.flag & FILE_OPEN_OPTIONS_O_WRONLY) == FILE_OPEN_OPTIONS_O_WRONLY {
        opts.write(true);
    }

    if (options.flag & FILE_OPEN_OPTIONS_O_APPEND) == FILE_OPEN_OPTIONS_O_APPEND {
        opts.append(true);
    }

    if (options.flag & FILE_OPEN_OPTIONS_O_TRUNC) == FILE_OPEN_OPTIONS_O_TRUNC {
        opts.truncate(true);
    }

    if (options.flag & FILE_OPEN_OPTIONS_O_EXCL) == FILE_OPEN_OPTIONS_O_EXCL {
        opts.create_new(true);
    }

    if options.mode != 0 {
        opts.mode(mode as u32);
    }

    let mut file = opts.open(path)?;

    file.write(data).map(|_| ())
}

pub fn write_file_with_buffer_from_path(
    path: &str,
    data: &Buffer,
    options: WriteFileOptions,
) -> std::io::Result<()> {
    write_file_with_bytes_from_path(path, data.buffer(), options)
}

pub fn writev(fd: c_int, mut buffers: Vec<Buffer>, position: c_long) -> std::io::Result<usize> {
    let mut file = unsafe { File::from_raw_fd(fd) };

    if position != -1 {
        match file.seek(SeekFrom::Start(position as u64)) {
            Ok(_) => {}
            Err(error) => return Err(error),
        }
    }

    let buffers: Vec<IoSlice> = buffers
        .iter_mut()
        .map(|b| {
            IoSlice::new(b.buffer())
        })
        .collect();

    file.write_vectored(buffers.as_slice())
}

pub fn writev_raw(
    fd: c_int,
    buffer: *const *const Buffer,
    buffer_len: usize,
    position: c_long,
) -> std::io::Result<usize> {
    let buf = unsafe { std::slice::from_raw_parts(buffer, buffer_len) };
    let mut slice_buf = Vec::with_capacity(buffer_len);
    unsafe {
        for item in buf.iter() {
            let item = &*(*item);
            slice_buf.push(item.clone())
        }
    }

    writev(fd, slice_buf, position)
}
