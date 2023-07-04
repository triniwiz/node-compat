use std::os::raw::c_uint;

use libc::{c_int, mode_t};

#[no_mangle]
pub static FILE_COPY_OPTIONS_COPYFILE_EXCL: c_uint = 1;

#[no_mangle]
pub static FILE_COPY_OPTIONS_COPYFILE_FICLONE: c_uint = 2;

#[no_mangle]
pub static FILE_COPY_OPTIONS_COPYFILE_FICLONE_FORCE: c_uint = 3;

#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_RDONLY: c_int = libc::O_RDONLY;

#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_WRONLY: c_int = libc::O_WRONLY;

#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_RDWR: c_int = libc::O_RDWR;

#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_CREAT: c_int = libc::O_CREAT;

#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_EXCL: c_int = libc::O_EXCL;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_NOCTTY: c_int = libc::O_NOCTTY;

#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_TRUNC: c_int = libc::O_TRUNC;

#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_APPEND: c_int = libc::O_APPEND;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_DIRECTORY: c_int = libc::O_DIRECTORY;

#[cfg(any(target_os = "android"))]
#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_NOATIME: c_int = libc::MS_NOATIME as c_int;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_NOATIME: c_int = libc::MNT_NOATIME;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_NOFOLLOW: c_int = libc::O_NOFOLLOW;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_SYNC: c_int = libc::O_SYNC;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_DSYNC: c_int = libc::O_DSYNC;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_SYMLINK: c_int = libc::O_SYMLINK;

#[cfg(any(target_os = "android"))]
#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_SYMLINK: c_int = -1;

#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_DIRECT: c_int = 0x4000;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_OPEN_OPTIONS_O_NONBLOCK: c_int = libc::O_NONBLOCK;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_ACCESS_OPTIONS_F_OK: c_int = libc::F_OK;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_ACCESS_OPTIONS_R_OK: c_int = libc::R_OK;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_ACCESS_OPTIONS_W_OK: c_int = libc::W_OK;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_ACCESS_OPTIONS_X_OK: c_int = libc::X_OK;

#[no_mangle]
pub static FILE_TYPE_OPTIONS_S_IFMT: mode_t = libc::S_IFMT;

#[no_mangle]
pub static FILE_TYPE_OPTIONS_S_IFREG: mode_t = libc::S_IFREG;

#[no_mangle]
pub static FILE_TYPE_OPTIONS_S_IFDIR: mode_t = libc::S_IFDIR;

#[no_mangle]
pub static FILE_TYPE_OPTIONS_S_IFCHR: mode_t = libc::S_IFCHR;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_TYPE_OPTIONS_S_IFBLK: mode_t = libc::S_IFBLK;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_TYPE_OPTIONS_S_IFIFO: mode_t = libc::S_IFIFO;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_TYPE_OPTIONS_S_IFLNK: mode_t = libc::S_IFLNK;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_TYPE_OPTIONS_S_IFSOCK: mode_t = libc::S_IFSOCK;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_MODE_OPTIONS_S_IRWXU: mode_t = libc::S_IRWXU;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_MODE_OPTIONS_S_IRUSR: mode_t = libc::S_IRUSR;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_MODE_OPTIONS_S_IWUSR: mode_t = libc::S_IWUSR;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_MODE_OPTIONS_S_IXUSR: mode_t = libc::S_IXUSR;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_MODE_OPTIONS_S_IRWXG: mode_t = libc::S_IRWXG;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_MODE_OPTIONS_S_IRGRP: mode_t = libc::S_IRGRP;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_MODE_OPTIONS_S_IWGRP: mode_t = libc::S_IWGRP;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_MODE_OPTIONS_S_IXGRP: mode_t = libc::S_IXGRP;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_MODE_OPTIONS_S_IRWXO: mode_t = libc::S_IRWXO;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_MODE_OPTIONS_S_IROTH: mode_t = libc::S_IROTH;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_MODE_OPTIONS_S_IWOTH: mode_t = libc::S_IWOTH;

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub static FILE_MODE_OPTIONS_S_IXOTH: mode_t = libc::S_IXOTH;
