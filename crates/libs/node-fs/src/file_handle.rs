use std::fs::File;
use std::os::unix::prelude::*;
use std::sync::Arc;

use libc::{c_int, c_long, c_uint, c_ushort};
use node_buffer::{Buffer, StringEncoding};

use crate::a_sync::{AsyncClosure};
use crate::file_stat::FileStat;
use crate::prelude::{FsEncoding};
use crate::sync::{AppendFileOptions, open_handle_with_path, ReadFileOptions, WriteFileOptions, WriteOptions};

pub struct FileHandle(File);

#[allow(non_snake_case)]
impl FileHandle {
    pub fn new(file: File) -> Self {
        Self(file)
    }

    pub fn new_async(
        path: &str,
        flags: c_int,
        mode: c_int,
        callback: Arc<AsyncClosure<FileHandle, std::io::Error>>,
    ) {
        let path = path.to_string();
        let _ = node_core::thread::spawn(
            move || match open_handle_with_path(&path, flags, mode) {
                Ok(handle) => {
                    callback.on_success(Some(handle));
                }
                Err(error) => {
                    callback.on_error(Some(error));
                }
            },
        );
    }

    pub fn append_file_with_str(
        &mut self,
        data: &str,
        options: AppendFileOptions,
        callback: Arc<AsyncClosure<(), std::io::Error>>,
    ) {
        let fd = self.fd();
        crate::a_sync::append_file_with_str(fd, data, options, callback);
    }

    pub fn append_file_with_bytes(
        &mut self,
        data: &Buffer,
        options: AppendFileOptions,
        callback: Arc<AsyncClosure<(), std::io::Error>>,
    ) {
        let fd = self.fd();
        crate::a_sync::append_file_with_bytes(fd, data, options, callback);
    }

    pub fn chmod(&self, mode: c_ushort, callback: Arc<AsyncClosure<(), std::io::Error>>) {
        let fd = self.fd();
        crate::a_sync::fchmod(fd, mode, callback);
    }

    pub fn chown(&self, uid: c_uint, gid: c_uint, callback: Arc<AsyncClosure<(), std::io::Error>>) {
        let fd = self.fd();
        crate::a_sync::fchown(fd, uid, gid, callback);
    }

    pub fn close(self, callback: Arc<AsyncClosure<(), std::io::Error>>) {
        {
            // drop the instance before to close
            let _ = self;
        }
        callback.on_success(None);
    }

    // TODO
    // pub fn createReadStream(){}

    // TODO
    // pub fn createWriteStream(){}

    pub fn datasync(&self, callback: Arc<AsyncClosure<(), std::io::Error>>) {
        let fd = self.fd();
        crate::a_sync::fdatasync(fd, callback);
    }

    pub fn fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }

    pub fn read(
        &mut self,
        buffer: &mut Buffer,
        offset: usize,
        length: usize,
        position: isize,
        callback: Arc<AsyncClosure<usize, std::io::Error>>,
    ) {
        let fd = self.fd();
        crate::a_sync::read(fd, buffer, offset, length, position, callback);
    }


    pub fn read_bytes(
        &mut self,
        buffer: &mut [u8],
        offset: usize,
        length: usize,
        position: isize,
        callback: Arc<AsyncClosure<usize, std::io::Error>>,
    ) {
        let fd = self.fd();
        crate::a_sync::read_bytes(fd, buffer, offset, length, position, callback);
    }


    pub fn read_file(
        &mut self,
        options: ReadFileOptions,
        callback: Arc<AsyncClosure<FsEncoding, std::io::Error>>,
    ) {
        let fd = self.fd();
        crate::a_sync::read_file_with_fd(fd, options, callback);
    }


    pub fn readv_slice(
        &mut self,
        buffers: Vec<Buffer>,
        position: c_long,
        callback: Arc<AsyncClosure<usize, std::io::Error>>,
    ) {
        let fd = self.fd();
        crate::a_sync::readv_slice(fd, buffers, position, callback);
    }

    pub fn readv(
        &mut self,
        buffers: Vec<Buffer>,
        position: c_long,
        callback: Arc<AsyncClosure<usize, std::io::Error>>,
    ) {
        let fd = self.fd();
        crate::a_sync::readv(fd, buffers, position, callback);
    }

    pub fn stat(&self, callback: Arc<AsyncClosure<FileStat, std::io::Error>>) {
        let fd = self.fd();
        crate::a_sync::fstat(fd, callback);
    }

    pub fn sync(&self, callback: Arc<AsyncClosure<(), std::io::Error>>) {
        let fd = self.fd();
        crate::a_sync::fsync(fd, callback);
    }

    pub fn truncate(&mut self, len: c_long, callback: Arc<AsyncClosure<(), std::io::Error>>) {
        let fd = self.fd();
        crate::a_sync::ftruncate(fd, len, callback);
    }

    pub fn utimes(
        &mut self,
        atime: c_long,
        mtime: c_long,
        callback: Arc<AsyncClosure<(), std::io::Error>>,
    ) {
        let fd = self.fd();
        crate::a_sync::futimes(fd, atime, mtime, callback);
    }

    pub fn write(
        &mut self,
        buffer: &Buffer,
        options: WriteOptions,
        callback: Arc<AsyncClosure<usize, std::io::Error>>,
    ) {
        let fd = self.fd();
        crate::a_sync::write(fd, buffer, options, callback);
    }

    pub fn write_string(
        &mut self,
        data: &str,
        encoding: StringEncoding,
        position: isize,
        callback: Arc<AsyncClosure<usize, std::io::Error>>,
    ) {
        let fd = self.fd();
        crate::a_sync::write_string(fd, data, encoding, position, callback);
    }

    pub fn write_file_with_str(
        &mut self,
        data: &str,
        options: WriteFileOptions,
        callback: Arc<AsyncClosure<(), std::io::Error>>,
    ) {
        let fd = self.fd();
        crate::a_sync::write_file_with_str(fd, data, options, callback);
    }

    pub fn write_file_with_bytes(
        &mut self,
        data: &Buffer,
        options: WriteFileOptions,
        callback: Arc<AsyncClosure<(), std::io::Error>>,
    ) {
        let fd = self.fd();
        crate::a_sync::write_file_with_bytes(fd, data, options, callback);
    }

    pub fn writev(
        &mut self,
        buffers: Vec<Buffer>,
        position: c_long,
        callback: Arc<AsyncClosure<usize, std::io::Error>>,
    ) {
        let fd = self.fd();
        crate::a_sync::writev(fd, buffers, position, callback);
    }

}
