use std::ffi::CString;
use node_buffer::Buffer;
use crate::file_stat::FileStat;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FsEncoding {
    String(CString),
    Buffer(Buffer),
}

impl FsEncoding {
    pub fn get_string_value(&self) -> Option<CString> {
        match self {
            FsEncoding::String(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_buffer_value(&self) -> Option<Buffer> {
        match self {
            FsEncoding::Buffer(buffer) => { Some(buffer.clone()) }
            _ => { None }
        }
    }
}

impl From<String> for FsEncoding {
    fn from(value: String) -> Self {
        Self::String(CString::new(value).unwrap())
    }
}

impl From<CString> for FsEncoding {
    fn from(value: CString) -> Self {
        Self::String(value)
    }
}

impl From<Buffer> for FsEncoding {
    fn from(value: Buffer) -> Self {
        Self::Buffer(value)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub enum FsEncodingType {
    Ascii,
    Utf8,
    Utf16le,
    Ucs2,
    Latin1,
    Buffer,
}

impl TryFrom<i32> for FsEncodingType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FsEncodingType::Ascii),
            1 => Ok(FsEncodingType::Utf8),
            2 => Ok(FsEncodingType::Utf16le),
            3 => Ok(FsEncodingType::Ucs2),
            4 => Ok(FsEncodingType::Latin1),
            5 => Ok(FsEncodingType::Buffer),
            _ => {
                Err("Invalid Encoding")
            }
        }
    }
}

pub fn handle_meta(metadata: &std::fs::Metadata) -> FileStat {
    use std::os::unix::prelude::*;

    let mut stat = FileStat {
        dev: metadata.dev() as i64,
        ino: metadata.ino() as i64,
        mode: metadata.mode() as i32,
        nlink: metadata.nlink() as i64,
        uid: metadata.uid() as i32,
        gid: metadata.gid() as i32,
        rdev: metadata.rdev() as i64,
        size: metadata.size() as i64,
        blksize: metadata.blksize() as i64,
        blocks: metadata.blocks() as i64,
        atimeMs: (metadata.atime_nsec() / 1000000) as f64,
        mtimeMs: (metadata.mtime_nsec() / 1000000) as f64,
        ctimeMs: (metadata.ctime_nsec() / 1000000) as f64,
        ..Default::default()
    };

    if let Ok(time) = metadata.created() {
        if let Ok(duration) = time.duration_since(std::time::SystemTime::UNIX_EPOCH) {
            stat.birthtimeMs = duration.as_millis() as f64;
            stat.birthtime = duration.as_secs() as f64;
        }
    }

    stat.atime = metadata.atime() as f64;
    stat.mtime = metadata.mtime() as f64;
    stat.ctime = metadata.ctime() as f64;

    let ft = metadata.file_type();
    stat.isBlockDevice = ft.is_block_device();
    stat.isCharacterDevice = ft.is_char_device();
    stat.isDirectory = ft.is_dir();
    stat.isFIFO = ft.is_fifo();
    stat.isFile = ft.is_file();
    stat.isSocket = ft.is_socket();
    stat.isSymbolicLink = ft.is_symlink();
    stat
}
