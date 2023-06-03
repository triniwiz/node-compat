use std::borrow::Cow;
use std::ffi::{c_void, CStr, CString};
use std::ops::Deref;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::ptr::NonNull;
use std::sync::Arc;
use libc::DIR;
use parking_lot::{RawRwLock, RwLock, RwLockReadGuard};
use parking_lot::lock_api::MappedRwLockReadGuard;

use crate::file_dirent::FileDirent;


#[derive(Clone)]
#[repr(transparent)]
struct FileDirInner(Arc<RwLock<(String,NonNull<DIR>)>>);

unsafe impl Send for FileDirInner {}

#[derive(Clone)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct FileDir(FileDirInner);

unsafe impl Send for FileDir {}

impl Drop for FileDir {
    fn drop(&mut self) {
        // todo drop dir
        unsafe {
         //   let _ = CString::from_raw(self.);
        }
    }
}

impl FileDir {
    pub(crate) fn new(path: String, dir: *mut DIR) -> FileDir {
        Self(FileDirInner(Arc::new(RwLock::new((path, NonNull::new(dir).unwrap())))))
    }

    pub fn close(&self) -> std::io::Result<()> {
        let dir = self.0.0.read();
        let ret = unsafe { libc::closedir(dir.1.as_ptr() as _) };

        if ret == -1 {
            let last_error = std::io::Error::last_os_error();
            return Err(last_error);
        }
        Ok(())
    }

    pub fn close_async(&self, callback: Box<dyn Fn(Option<std::io::Error>) + Send>) {
        let file_dir = self.clone();
        super::a_sync::runtime().spawn(async move {
            let file_dir = file_dir;
            match file_dir.close() {
                Ok(_) => {
                    (callback)(None);
                }
                Err(error) => (callback)(Some(error)),
            }
        });
    }

    pub fn path(&self) -> MappedRwLockReadGuard<'_, RawRwLock, str> {
        RwLockReadGuard::map(self.0.0.read(), |f| f.0.as_str())
    }

    pub(crate) fn dir(&self) -> MappedRwLockReadGuard<'_, RawRwLock, *mut DIR> {
        RwLockReadGuard::map(self.0.0.read(), |f| {
            unsafe {std::mem::transmute(f.1.as_ptr())}
        })
    }

    pub fn read(&self) -> std::io::Result<FileDirent> {
        let dir = unsafe { self.dir() };
        let ret = unsafe { libc::readdir(*dir) };

        if ret.is_null() {
            let last_error = std::io::Error::last_os_error();
            return Err(last_error);
        }

        Ok(FileDirent::new_raw(ret))
    }

    pub fn read_async(
        &self,
        callback: Box<dyn Fn(Option<FileDirent>, Option<std::io::Error>) + Send>,
    ) {

        let tmp_file_dir = FileDir(self.0.clone());

        super::a_sync::runtime().spawn(async move {

            match tmp_file_dir.read() {
                Ok(dir) => {
                    (callback)(Some(dir), None);
                }
                Err(error) => {
                    (callback)(None, Some(error));
                }
            }
        });
    }
}
