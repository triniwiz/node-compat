use std::borrow::Cow;
use std::ffi::{c_void, CStr};
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::sync::Arc;

use libc::size_t;

#[derive(Debug)]
pub enum FileDirentInner {
    Raw(String,*mut c_void),
    Regular(fs::DirEntry),
}

#[derive(Debug)]
pub struct FileDirent(pub(crate) Arc<FileDirentInner>);

impl FileDirent {
    pub fn new_regular(dir: fs::DirEntry) -> Self {
        Self(Arc::new(FileDirentInner::Regular(dir)))
    }

    pub fn new_raw(path: String, dir: *mut libc::dirent) -> Self {
        Self(Arc::new(FileDirentInner::Raw(path, dir as *mut c_void)))
    }

    pub fn name<'a>(&self) -> Cow<'a, str> {
        match self.0.as_ref() {
            FileDirentInner::Raw(_, raw) => unsafe {
                let raw = (*raw) as *mut libc::dirent;
                CStr::from_ptr((*raw).d_name.as_ptr()).to_string_lossy()
            },
            FileDirentInner::Regular(reg) => {
                Cow::from(reg.file_name().to_string_lossy().as_ref().to_string())
            }
        }
    }

    pub fn path<'a>(&self) -> Cow<'a, str> {
        match self.0.as_ref() {
            FileDirentInner::Raw(path,_) => unsafe {
                Cow::from(path)
            },
            FileDirentInner::Regular(reg) => {
                Cow::from(reg.path().to_string_lossy().as_ref().to_string())
            }
        }
    }

    pub fn is_block_device(&self) -> bool {
        match self.0.as_ref() {
            FileDirentInner::Raw(_, raw) => unsafe {
                let raw = (*raw) as *mut libc::dirent;
                (*raw).d_type == libc::DT_BLK
            },
            FileDirentInner::Regular(reg) => reg.file_type().map_or(false, |v| v.is_block_device()),
        }
    }

    pub fn is_character_device(&self) -> bool {
        match self.0.as_ref() {
            FileDirentInner::Raw(_, raw) => unsafe {
                let raw = (*raw) as *mut libc::dirent;
                (*raw).d_type == libc::DT_CHR
            },
            FileDirentInner::Regular(reg) => reg.file_type().map_or(false, |v| v.is_char_device()),
        }
    }

    pub fn is_directory(&self) -> bool {
        match self.0.as_ref() {
            FileDirentInner::Raw(_, raw) => unsafe {
                let raw = (*raw) as *mut libc::dirent;
                (*raw).d_type == libc::DT_DIR
            },
            FileDirentInner::Regular(reg) => reg.file_type().map_or(false, |v| v.is_dir()),
        }
    }

    pub fn is_fifo(&self) -> bool {
        match self.0.as_ref() {
            FileDirentInner::Raw(_, raw) => unsafe {
                let raw = (*raw) as *mut libc::dirent;
                (*raw).d_type == libc::DT_FIFO
            },
            FileDirentInner::Regular(reg) => reg.file_type().map_or(false, |v| v.is_fifo()),
        }
    }

    pub fn is_file(&self) -> bool {
        match self.0.as_ref() {
            FileDirentInner::Raw(_, raw) => unsafe {
                let raw = (*raw) as *mut libc::dirent;
                (*raw).d_type == libc::DT_REG
            },
            FileDirentInner::Regular(reg) => reg.file_type().map_or(false, |v| v.is_file()),
        }
    }

    pub fn is_socket(&self) -> bool {
        match self.0.as_ref() {
            FileDirentInner::Raw(_, raw) => unsafe {
                let raw = (*raw) as *mut libc::dirent;
                (*raw).d_type == libc::DT_SOCK
            },
            FileDirentInner::Regular(reg) => reg.file_type().map_or(false, |v| v.is_socket()),
        }
    }

    pub fn is_symbolic_link(&self) -> bool {
        match self.0.as_ref() {
            FileDirentInner::Raw(_, raw) => unsafe {
                let raw = (*raw) as *mut libc::dirent;
                (*raw).d_type == libc::DT_LNK
            },
            FileDirentInner::Regular(reg) => reg.file_type().map_or(false, |v| v.is_symlink()),
        }
    }
}

impl Drop for FileDirent {
    fn drop(&mut self) {
        if let FileDirentInner::Raw(_, value) = self.0.as_ref() {
            if value.is_null() {
                unsafe {
                    libc::free(*value);
                }
            }
        }
    }
}

impl Clone for FileDirent {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

#[repr(C)]
pub struct FileDirentBuf {
    data: *mut FileDirent,
    len: size_t,
}

impl From<Vec<FileDirent>> for FileDirentBuf {
    fn from(vec: Vec<FileDirent>) -> Self {
        let len = vec.len();
        let mut slice = vec.into_boxed_slice();
        let data = slice.as_mut_ptr();
        let _ = Box::into_raw(slice);
        Self { data, len }
    }
}

impl Drop for FileDirentBuf {
    fn drop(&mut self) {
        if !self.data.is_null() && self.len != 0 {
            unsafe {
                let _ = Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.len));
            }
        }
    }
}
