package org.nativescript.node_compat.fs

import java.nio.ByteBuffer


class FileSystem {
  companion object {

    @JvmStatic
    fun access(path: String, mode: Int, callback: AsyncCallback<Void?>) {
      nativeAccess(path, mode, callback.native)
    }

    @JvmStatic
    fun appendFile(fd: Int, bytes: ByteArray, callback: AsyncCallback<Void?>) {
      nativeAppendFileWithBytes(fd, bytes, callback.native)
    }

    @JvmStatic
    fun appendFile(fd: Int, string: String, callback: AsyncCallback<Void?>) {
      nativeAppendFileWithString(fd, string, callback.native)
    }

    @JvmStatic
    fun appendFile(
      path: String,
      data: ByteArray,
      flags: Int,
      mode: Int,
      callback: AsyncCallback<Void?>
    ) {
      nativeAppendFileWithPathBytes(path, data, flags, mode, callback.native)
    }

    @JvmStatic
    fun appendFile(
      path: String,
      data: String,
      flags: Int,
      mode: Int,
      callback: AsyncCallback<Void?>
    ) {
      nativeAppendFileWithPathString(path, data, flags, mode, callback.native)
    }

    @JvmStatic
    fun chmod(path: String, mode: Int, callback: AsyncCallback<Void?>) {
      nativeChmod(path, mode, callback.native)
    }

    @JvmStatic
    fun chown(path: String, uid: Int, gid: Int, callback: AsyncCallback<Void?>) {
      nativeChown(path, uid, gid, callback.native)
    }

    @JvmStatic
    fun copyFile(src: String, dest: String, flags: Int, callback: AsyncCallback<Void?>) {
      nativeCopyFile(src, dest, flags, callback.native)
    }

    @JvmStatic
    fun exists(path: String, callback: AsyncCallback<Boolean?>) {
      nativeExists(path, callback.native)
    }

    @JvmStatic
    fun fchmod(fd: Int, mode: Int, callback: AsyncCallback<Void?>) {
      nativeFchmod(fd, mode, callback.native)
    }

    @JvmStatic
    fun fchown(fd: Int, uid: Int, gid: Int, callback: AsyncCallback<Void?>) {
      nativeFchown(fd, uid, gid, callback.native)
    }

    @JvmStatic
    fun fdatasync(fd: Int, callback: AsyncCallback<Void?>) {
      nativeFdatasync(fd, callback.native)
    }

    @JvmStatic
    fun fstat(fd: Int, callback: AsyncCallback<FileStat?>) {
      nativeFstat(fd, callback.native)
    }

    @JvmStatic
    fun fsync(fd: Int, callback: AsyncCallback<Void?>) {
      nativeFsync(fd, callback.native)
    }

    @JvmStatic
    fun ftruncate(fd: Int, len: Long, callback: AsyncCallback<Void?>) {
      nativeFtruncate(fd, len, callback.native)
    }

    @JvmStatic
    fun futimes(fd: Int, atime: Long, mtime: Long, callback: AsyncCallback<Void?>) {
      nativeFutimes(fd, atime, mtime, callback.native)
    }

    @JvmStatic
    fun lchmod(path: String, mode: Int, callback: AsyncCallback<Void?>) {
      nativeLchmod(path, mode, callback.native)
    }

    @JvmStatic
    fun lchown(path: String, uid: Int, gid: Int, callback: AsyncCallback<Void?>) {
      nativeLchown(path, uid, gid, callback.native)
    }

    @JvmStatic
    fun lutimes(path: String, atime: Long, mtime: Long, callback: AsyncCallback<Void?>) {
      nativeLutimes(path, atime, mtime, callback.native)
    }

    @JvmStatic
    fun link(oldPath: String, newPath: String, callback: AsyncCallback<Void?>) {
      nativeLink(oldPath, newPath, callback.native)
    }

    fun lstat(path: String, callback: AsyncCallback<FileStat?>) {
      nativeLstat(path, callback.native)
    }

    fun mkdir(path: String, mode: Int, recursive: Boolean, callback: AsyncCallback<Void?>) {
      nativeMkdir(path, mode, recursive, callback.native)
    }

    fun mkdtemp(prefix: String, callback: AsyncCallback<String?>) {
      nativeMkdtemp(prefix, callback.native)
    }

    fun open(path: String, flag: Int, mode: Int, callback: AsyncCallback<Int?>) {
      nativeOpen(path, flag, mode, callback.native)
    }

    fun openDir(path: String, callback: AsyncCallback<FileDir?>) {
      nativeOpenDir(path, callback.native)
    }

    fun read(
      fd: Int,
      buffer: ByteBuffer,
      offset: Long,
      length: Long,
      position: Long,
      callback: AsyncCallback<Long?>
    ) {
      nativeRead(fd, buffer, offset, length, position, callback.native)
    }

    fun read(
      fd: Int,
      buffer: ByteArray,
      offset: Long,
      length: Long,
      position: Long,
      callback: AsyncCallback<Long?>
    ) {
      nativeReadWithBytes(fd, buffer, offset, length, position, callback.native)
    }

    fun readdir(
      path: String,
      encoding: String,
      withTypes: Boolean,
      callback: AsyncCallback<Array<Any?>?>
    ) {
      if (withTypes) {
        nativeReaddirWithFileTypes(path, encoding, callback.native)
      } else {
        nativeReaddirWithFile(path, encoding, callback.native)
      }
    }

    fun readFile(path: String, flags: Int, callback: AsyncCallback<ByteBuffer?>) {
      nativeReadFile(path, flags, callback.native)
    }

    fun readFile(fd: Int, flags: Int, callback: AsyncCallback<ByteBuffer?>) {
      nativeReadFileWithFd(fd, flags, callback.native)
    }

    fun readFileBytes(path: String, flags: Int, callback: AsyncCallback<ByteArray?>) {
      nativeReadFileBytes(path, flags, callback.native)
    }

    fun readFileBytes(fd: Int, flags: Int, callback: AsyncCallback<ByteArray?>) {
      nativeReadFileBytesWithFd(fd, flags, callback.native)
    }

    fun readLink(path: String, encoding: String, callback: AsyncCallback<String?>) {
      nativeReadLink(path, encoding, callback.native)
    }

    fun readv(fd: Int, buffers: Array<ByteBuffer>, position: Long, callback: AsyncCallback<Long?>) {
      nativeReadv(fd, buffers, position, callback.native)
    }

    fun realPath(path: String, callback: AsyncCallback<String?>) {
      nativeRealPath(path, callback.native)
    }

    fun rename(oldPath: String, newPath: String, callback: AsyncCallback<Void?>) {
      nativeRename(oldPath, newPath, callback.native)
    }

    fun rmdir(
      path: String,
      maxRetries: Int,
      recursive: Boolean,
      retryDelay: Long,
      callback: AsyncCallback<Void?>
    ) {
      nativeRmdir(path, maxRetries, recursive, retryDelay, callback.native)
    }

    fun rm(
      path: String,
      maxRetries: Int,
      recursive: Boolean,
      retryDelay: Long,
      callback: AsyncCallback<Void?>
    ) {
      nativeRm(path, maxRetries, recursive, retryDelay, callback.native)
    }

    fun stat(path: String, throwIfNoEntry: Boolean, callback: AsyncCallback<FileStat?>) {
      nativeStat(path, throwIfNoEntry, callback.native)
    }

    fun symlink(target: String, path: String, callback: AsyncCallback<Void?>) {
      nativeSymlink(target, path, callback.native)
    }

    fun truncate(path: String, len: Long, callback: AsyncCallback<Void?>) {
      nativeTruncate(path, len, callback.native)
    }

    fun unlink(path: String, callback: AsyncCallback<Void?>) {
      nativeUnlink(path, callback.native)
    }

    fun unwatchFile(path: String, callback: AsyncCallback<FileWatcher.Event?>) {
      nativeUnwatchFile(path, callback.native)
    }

    fun utimes(path: String, atime: Long, mtime: Long, callback: AsyncCallback<Void?>) {
      nativeUtimes(path, atime, mtime, callback.native)
    }

    fun watch(
      path: String,
      persistent: Boolean,
      recursive: Boolean,
      encoding: String,
      callback: AsyncCallback<FsWatcher.Event?>
    ): FsWatcher? {
      return nativeWatch(path, persistent, recursive, encoding, callback.native)
    }

    fun watchFile(
      path: String,
      bigint: Boolean,
      persistent: Boolean,
      interval: Long,
      callback: AsyncCallback<FileWatcher.Event?>
    ): FileWatcher? {
      return nativeWatchFile(path, bigint, persistent, interval, callback.native)
    }

    fun write(
      fd: Int,
      buffer: ByteBuffer,
      offset: Long,
      length: Long,
      position: Long,
      callback: AsyncCallback<Long?>
    ) {
      nativeWrite(fd, buffer, offset, length, position, callback.native)
    }

    fun write(
      fd: Int,
      buffer: ByteArray,
      offset: Long,
      length: Long,
      position: Long,
      callback: AsyncCallback<Long?>
    ) {
      nativeWriteBytes(fd, buffer, offset, length, position, callback.native)
    }

    fun write(
      fd: Int,
      string: String,
      encoding: String,
      position: Long,
      callback: AsyncCallback<Long?>
    ) {
      nativeWriteString(fd, string, encoding, position, callback.native)
    }

    fun writeFile(fd: Int, data: String, callback: AsyncCallback<Void?>) {
      nativeWriteFileWithString(fd, data, callback.native)
    }

    fun writeFile(fd: Int, data: ByteBuffer, callback: AsyncCallback<Void?>) {
      nativeWriteFileWithBuffer(fd, data, callback.native)
    }

    fun writeFile(fd: Int, data: ByteArray, callback: AsyncCallback<Void?>) {
      nativeWriteFileWithBytes(fd, data, callback.native)
    }

    fun writeFile(
      path: String,
      data: String,
      encoding: String,
      mode: Int,
      flag: Int,
      callback: AsyncCallback<Void?>
    ) {
      nativeWriteFileWithStringFromPath(
        path,
        data,
        encoding,
        mode,
        flag,
        callback.native
      )
    }

    fun writeFile(
      path: String,
      data: ByteArray,
      encoding: String,
      mode: Int,
      flag: Int,
      callback: AsyncCallback<Void?>
    ) {
      nativeWriteFileWithBytesFromPath(
        path,
        data,
        encoding,
        mode,
        flag,
        callback.native
      )
    }

    fun writeFile(
      path: String,
      data: ByteBuffer,
      encoding: String,
      mode: Int,
      flag: Int,
      callback: AsyncCallback<Void?>
    ) {
      nativeWriteFileWithBufferFromPath(
        path,
        data,
        encoding,
        mode,
        flag,
        callback.native
      )
    }

    fun writev(
      fd: Int,
      buffers: Array<ByteBuffer>,
      position: Long,
      callback: AsyncCallback<Long?>
    ) {
      nativeWritev(fd, buffers, position, callback.native)
    }


    @Throws(java.lang.Exception::class)
    fun accessSync(path: String, mode: Int) {
      nativeAccessSync(path, mode)
    }

    @Throws(java.lang.Exception::class)
    fun appendFileSync(fd: Int, bytes: ByteArray) {
      nativeAppendFileWithBytesSync(fd, bytes)
    }

    @Throws(java.lang.Exception::class)
    fun appendFilesYNC(fd: Int, string: String?) {
      nativeAppendFileWithStringSync(fd, string!!)
    }

    @Throws(java.lang.Exception::class)
    fun appendFileSync(path: String?, data: ByteArray?, flags: Int, mode: Int) {
      nativeAppendFileWithPathBytesSync(path!!, data!!, flags, mode)
    }

    @Throws(java.lang.Exception::class)
    fun appendFileSync(path: String?, data: String?, flags: Int, mode: Int) {
      nativeAppendFileWithPathStringSync(path!!, data!!, flags, mode)
    }

    @Throws(java.lang.Exception::class)
    fun chmodSync(path: String?, mode: Int) {
      nativeChmodSync(path!!, mode)
    }

    @Throws(java.lang.Exception::class)
    fun chownSync(path: String?, uid: Int, gid: Int) {
      nativeChownSync(path!!, uid, gid)
    }

    @Throws(java.lang.Exception::class)
    fun copyFileSync(src: String?, dest: String?, flags: Int) {
      nativeCopyFileSync(src!!, dest!!, flags)
    }

    fun existsSync(path: String?): Boolean {
      return nativeExistsSync(path!!)
    }

    @Throws(java.lang.Exception::class)
    fun fchmodSync(fd: Int, mode: Int) {
      nativeFchmodSync(fd, mode)
    }

    @Throws(java.lang.Exception::class)
    fun fchownSync(fd: Int, uid: Int, gid: Int) {
      nativeFchownSync(fd, uid, gid)
    }

    @Throws(java.lang.Exception::class)
    fun fdatasyncSync(fd: Int) {
      nativeFdatasyncSync(fd)
    }

    @Throws(java.lang.Exception::class)
    fun fstatSync(fd: Int): FileStat? {
      return nativeFstatSync(fd)
    }

    @Throws(java.lang.Exception::class)
    fun fsyncSync(fd: Int) {
      nativeFsyncSync(fd)
    }

    @Throws(java.lang.Exception::class)
    fun ftruncateSync(fd: Int, len: Long) {
      nativeFtruncateSync(fd, len)
    }

    @Throws(java.lang.Exception::class)
    fun futimesSync(fd: Int, atime: Long, mtime: Long) {
      nativeFutimesSync(fd, atime, mtime)
    }

    @Throws(java.lang.Exception::class)
    fun lchmodSync(path: String?, mode: Int) {
      nativeLchmodSync(path!!, mode)
    }

    @Throws(java.lang.Exception::class)
    fun lchownSync(path: String?, uid: Int, gid: Int) {
      nativeLchownSync(path!!, uid, gid)
    }

    @Throws(java.lang.Exception::class)
    fun lutimesSync(path: String?, atime: Long, mtime: Long) {
      nativeLutimesSync(path!!, atime, mtime)
    }

    @Throws(java.lang.Exception::class)
    fun linkSync(oldPath: String?, newPath: String?) {
      nativeLinkSync(oldPath!!, newPath!!)
    }

    @Throws(java.lang.Exception::class)
    fun lstatSync(path: String?): FileStat? {
      return nativeLstatSync(path!!)
    }

    @Throws(java.lang.Exception::class)
    fun mkdirSync(path: String?, mode: Int, recursive: Boolean) {
      nativeMkdirSync(path!!, mode, recursive)
    }

    @Throws(java.lang.Exception::class)
    fun mkdtempSync(prefix: String?): String? {
      return nativeMkdtempSync(prefix!!)
    }

    @Throws(java.lang.Exception::class)
    fun openSync(path: String?, flag: Int, mode: Int): Int {
      return nativeOpenSync(path!!, flag, mode)
    }

    @Throws(java.lang.Exception::class)
    fun openDirSync(path: String?): FileDir? {
      return nativeOpenDirSync(path!!)
    }

    @Throws(java.lang.Exception::class)
    fun readSync(fd: Int, buffer: ByteBuffer?, offset: Long, length: Long, position: Long): Long {
      return nativeReadSync(fd, buffer!!, offset, length, position)
    }

    @Throws(java.lang.Exception::class)
    fun readSync(fd: Int, buffer: ByteArray?, offset: Long, length: Long, position: Long): Long {
      return nativeReadWithBytesSync(fd, buffer!!, offset, length, position)
    }

    @Throws(java.lang.Exception::class)
    fun readdirSync(path: String?, encoding: String?, withTypes: Boolean): Array<Any> {
      return if (withTypes) {
        nativeReaddirWithFileTypesSync(path!!, encoding!!)
      } else {
        nativeReaddirWithFileSync(path!!, encoding!!)
      }
    }

    @Throws(java.lang.Exception::class)
    fun readFileSync(path: String?, flags: Int): ByteBuffer? {
      return nativeReadFileSync(path!!, flags)
    }

    @Throws(java.lang.Exception::class)
    fun readFileSync(fd: Int, flags: Int): ByteBuffer? {
      return nativeReadFileWithFdSync(fd, flags)
    }

    @Throws(java.lang.Exception::class)
    fun readFileBytesSync(path: String?, flags: Int): ByteArray? {
      return nativeReadFileBytesSync(path!!, flags)
    }

    @Throws(java.lang.Exception::class)
    fun readFileBytesSync(fd: Int, flags: Int): ByteArray? {
      return nativeReadFileBytesWithFdSync(fd, flags)
    }

    @Throws(java.lang.Exception::class)
    fun readLinkSync(path: String?, encoding: String?): String? {
      return nativeReadLinkSync(path!!, encoding!!)
    }

    @Throws(java.lang.Exception::class)
    fun readvSync(fd: Int, buffers: Array<ByteBuffer?>?, position: Long): Long {
      return nativeReadvSync(fd, buffers, position)
    }

    @Throws(java.lang.Exception::class)
    fun realPathSync(path: String?): String? {
      return nativeRealPathSync(path!!)
    }

    @Throws(java.lang.Exception::class)
    fun renameSync(oldPath: String?, newPath: String?) {
      nativeRenameSync(oldPath!!, newPath!!)
    }

    @Throws(java.lang.Exception::class)
    fun rmdirSync(path: String?, maxRetries: Int, recursive: Boolean, retryDelay: Long) {
      nativeRmdirSync(path!!, maxRetries, recursive, retryDelay)
    }

    @Throws(java.lang.Exception::class)
    fun rmSync(path: String?, maxRetries: Int, recursive: Boolean, retryDelay: Long) {
      nativeRmSync(path!!, maxRetries, recursive, retryDelay)
    }

    @Throws(java.lang.Exception::class)
    fun statSync(path: String?, throwIfNoEntry: Boolean): FileStat? {
      return nativeStatSync(path!!, throwIfNoEntry)
    }

    @Throws(java.lang.Exception::class)
    fun symlinkSync(target: String?, path: String?) {
      nativeSymlinkSync(target!!, path!!)
    }

    @Throws(java.lang.Exception::class)
    fun truncateSync(path: String?, len: Long) {
      nativeTruncateSync(path!!, len)
    }

    @Throws(java.lang.Exception::class)
    fun unlinkSync(path: String?) {
      nativeUnlinkSync(path!!)
    }

    @Throws(java.lang.Exception::class)
    fun utimesSync(path: String?, atime: Long, mtime: Long) {
      nativeUtimesSync(path!!, atime, mtime)
    }

    @Throws(java.lang.Exception::class)
    fun writeSync(fd: Int, buffer: ByteBuffer?, offset: Long, length: Long, position: Long): Long {
      return nativeWriteSync(fd, buffer!!, offset, length, position)
    }

    @Throws(java.lang.Exception::class)
    fun writeSync(fd: Int, buffer: ByteArray?, offset: Long, length: Long, position: Long): Long {
      return nativeWriteBytesSync(fd, buffer!!, offset, length, position)
    }

    @Throws(java.lang.Exception::class)
    fun writeSync(fd: Int, string: String?, encoding: String?, position: Long): Long {
      return nativeWriteStringSync(fd, string!!, encoding!!, position)
    }

    @Throws(java.lang.Exception::class)
    fun writeFileSync(fd: Int, data: String?) {
      nativeWriteFileWithStringSync(fd, data!!)
    }

    @Throws(java.lang.Exception::class)
    fun writeFileSync(fd: Int, data: ByteBuffer?) {
      nativeWriteFileWithBufferSync(fd, data!!)
    }

    @Throws(java.lang.Exception::class)
    fun writeFileSync(fd: Int, data: ByteArray?) {
      nativeWriteFileWithBytesSync(fd, data!!)
    }

    @Throws(java.lang.Exception::class)
    fun writeFileSync(path: String?, data: String?, encoding: String?, mode: Int, flag: Int) {
      nativeWriteFileWithStringFromPathSync(path!!, data!!, encoding!!, mode, flag)
    }

    @Throws(java.lang.Exception::class)
    fun writeFileSync(path: String?, data: ByteArray?, encoding: String?, mode: Int, flag: Int) {
      nativeWriteFileWithBytesFromPathSync(path!!, data!!, encoding!!, mode, flag)
    }

    @Throws(java.lang.Exception::class)
    fun writeFileSync(path: String?, data: ByteBuffer?, encoding: String?, mode: Int, flag: Int) {
      nativeWriteFileWithBufferFromPathSync(path!!, data!!, encoding!!, mode, flag)
    }

    @Throws(java.lang.Exception::class)
    fun writevSync(fd: Int, buffers: Array<ByteBuffer?>?, position: Long): Long {
      return nativeWritevSync(fd, buffers, position)
    }




    @JvmStatic
    external fun nativeAccess(path: String, mode: Int, callback: Long)

    @JvmStatic
    external fun nativeAppendFileWithBytes(fd: Int, bytes: ByteArray, callback: Long)

    @JvmStatic
    external fun nativeAppendFileWithString(fd: Int, data: String, callback: Long)

    @JvmStatic
    external fun nativeAppendFileWithPathBytes(
      path: String,
      bytes: ByteArray,
      mode: Int,
      flags: Int,
      callback: Long
    )

    @JvmStatic
    external fun nativeAppendFileWithPathString(
      path: String,
      data: String,
      mode: Int,
      flags: Int,
      callback: Long
    )

    @JvmStatic
    external fun nativeChmod(path: String, mode: Int, callback: Long)

    @JvmStatic
    external fun nativeChown(path: String, uid: Int, gid: Int, callback: Long)

    @JvmStatic
    external fun nativeCopyFile(src: String, dest: String, flags: Int, callback: Long)

    @JvmStatic
    external fun nativeExists(path: String, callback: Long)

    @JvmStatic
    external fun nativeFchmod(fd: Int, mode: Int, callback: Long)

    @JvmStatic
    external fun nativeFchown(fd: Int, uid: Int, gid: Int, callback: Long)

    @JvmStatic
    external fun nativeFdatasync(fd: Int, callback: Long)

    @JvmStatic
    external fun nativeFstat(fd: Int, callback: Long)

    @JvmStatic
    external fun nativeFsync(fd: Int, callback: Long)

    @JvmStatic
    external fun nativeFtruncate(fd: Int, len: Long, callback: Long)

    @JvmStatic
    external fun nativeFutimes(fd: Int, atime: Long, mtime: Long, callback: Long)

    @JvmStatic
    external fun nativeLchmod(path: String, mode: Int, callback: Long)

    @JvmStatic
    external fun nativeLchown(path: String, uid: Int, gid: Int, callback: Long)

    @JvmStatic
    external fun nativeLutimes(path: String, atime: Long, mtime: Long, callback: Long)

    @JvmStatic
    external fun nativeLink(oldPath: String, newPath: String, callback: Long)

    @JvmStatic
    external fun nativeLstat(path: String, callback: Long)

    @JvmStatic
    external fun nativeMkdir(path: String, mode: Int, recursive: Boolean, callback: Long)

    @JvmStatic
    external fun nativeMkdtemp(prefix: String, callback: Long)

    @JvmStatic
    external fun nativeOpen(path: String, flags: Int, mode: Int, callback: Long)

    @JvmStatic
    external fun nativeOpenDir(path: String, callback: Long)

    @JvmStatic
    external fun nativeRead(
      fd: Int,
      buffer: ByteBuffer,
      offset: Long,
      length: Long,
      position: Long,
      callback: Long
    )

    @JvmStatic
    external fun nativeReadWithBytes(
      fd: Int,
      buffer: ByteArray,
      offset: Long,
      length: Long,
      position: Long,
      callback: Long
    )

    @JvmStatic
    external fun nativeReaddirWithFileTypes(path: String, encoding: String, callback: Long)

    @JvmStatic
    external fun nativeReaddirWithFile(path: String, encoding: String, callback: Long)

    @JvmStatic
    external fun nativeReadFile(path: String, flags: Int, callback: Long)

    @JvmStatic
    external fun nativeReadFileBytes(path: String, flags: Int, callback: Long)

    @JvmStatic
    external fun nativeReadFileWithFd(fd: Int, flags: Int, callback: Long)

    @JvmStatic
    external fun nativeReadFileBytesWithFd(fd: Int, flags: Int, callback: Long)

    @JvmStatic
    external fun nativeReadLink(path: String, encoding: String, callback: Long)

    @JvmStatic
    external fun nativeReadv(
      fd: Int,
      buffers: Array<ByteBuffer>,
      position: Long,
      callback: Long
    )

    @JvmStatic
    external fun nativeRealPath(path: String, callback: Long)

    @JvmStatic
    external fun nativeRename(oldPath: String, newPath: String, callback: Long)

    @JvmStatic
    external fun nativeRmdir(
      path: String,
      maxRetries: Int,
      recursive: Boolean,
      retryDelay: Long,
      callback: Long
    )

    @JvmStatic
    external fun nativeRm(
      path: String,
      maxRetries: Int,
      recursive: Boolean,
      retryDelay: Long,
      callback: Long
    )

    @JvmStatic
    external fun nativeStat(path: String, throwIfNoEntry: Boolean, callback: Long)

    @JvmStatic
    external fun nativeSymlink(target: String, path: String, callback: Long)

    @JvmStatic
    external fun nativeTruncate(path: String, len: Long, callback: Long)

    @JvmStatic
    external fun nativeUnlink(path: String, callback: Long)

    @JvmStatic
    external fun nativeUnwatchFile(path: String, callback: Long)

    @JvmStatic
    external fun nativeUtimes(path: String, atime: Long, mtime: Long, callback: Long)

    @JvmStatic
    external fun nativeWatch(
      path: String,
      persistent: Boolean,
      recursive: Boolean,
      encoding: String,
      callback: Long
    ): FsWatcher?

    @JvmStatic
    external fun nativeWatchFile(
      path: String,
      bigint: Boolean,
      persistent: Boolean,
      interval: Long,
      callback: Long
    ): FileWatcher?

    @JvmStatic
    external fun nativeWrite(
      fd: Int,
      buffer: ByteBuffer,
      offset: Long,
      length: Long,
      position: Long,
      callback: Long
    )

    @JvmStatic
    external fun nativeWriteBytes(
      fd: Int,
      buffer: ByteArray,
      offset: Long,
      length: Long,
      position: Long,
      callback: Long
    )

    @JvmStatic
    external fun nativeWriteString(
      fd: Int,
      string: String,
      encoding: String,
      position: Long,
      callback: Long
    )

    @JvmStatic
    external fun nativeWriteFileWithString(fd: Int, data: String, callback: Long)

    @JvmStatic
    external fun nativeWriteFileWithBuffer(fd: Int, data: ByteBuffer, callback: Long)

    @JvmStatic
    external fun nativeWriteFileWithBytes(fd: Int, data: ByteArray, callback: Long)

    @JvmStatic
    external fun nativeWriteFileWithStringFromPath(
      path: String,
      data: String,
      encoding: String,
      mode: Int,
      flag: Int,
      callback: Long
    )

    @JvmStatic
    external fun nativeWriteFileWithBytesFromPath(
      path: String,
      data: ByteArray,
      encoding: String,
      mode: Int,
      flag: Int,
      callback: Long
    )

    @JvmStatic
    external fun nativeWriteFileWithBufferFromPath(
      path: String,
      data: ByteBuffer,
      encoding: String,
      mode: Int,
      flag: Int,
      callback: Long
    )

    @JvmStatic
    external fun nativeWritev(
      fd: Int,
      buffers: Array<ByteBuffer>,
      position: Long,
      callback: Long
    )

    @JvmStatic
    external fun nativeAccessSync(path: String, mode: Int)

    @JvmStatic
    external fun nativeAppendFileWithBytesSync(fd: Int, bytes: ByteArray)

    @JvmStatic
    external fun nativeAppendFileWithStringSync(fd: Int, data: String)

    @JvmStatic
    external fun nativeAppendFileWithPathBytesSync(
      path: String,
      bytes: ByteArray,
      mode: Int,
      flags: Int
    )

    @JvmStatic
    external fun nativeAppendFileWithPathStringSync(
      path: String,
      data: String,
      mode: Int,
      flags: Int
    )

    @JvmStatic
    external fun nativeChmodSync(path: String, mode: Int)

    @JvmStatic
    external fun nativeChownSync(path: String, uid: Int, gid: Int)

    @JvmStatic
    external fun nativeCopyFileSync(src: String, dest: String, flags: Int)

    @JvmStatic
    external fun nativeExistsSync(path: String): Boolean

    @JvmStatic
    external fun nativeFchmodSync(fd: Int, mode: Int)

    @JvmStatic
    external fun nativeFchownSync(fd: Int, uid: Int, gid: Int)

    @JvmStatic
    external fun nativeFdatasyncSync(fd: Int)

    @JvmStatic
    external fun nativeFstatSync(fd: Int): FileStat?

    @JvmStatic
    external fun nativeFsyncSync(fd: Int)

    @JvmStatic
    external fun nativeFtruncateSync(fd: Int, len: Long)

    @JvmStatic
    external fun nativeFutimesSync(fd: Int, atime: Long, mtime: Long)

    @JvmStatic
    external fun nativeLchmodSync(path: String, mode: Int)

    @JvmStatic
    external fun nativeLchownSync(path: String, uid: Int, gid: Int)

    @JvmStatic
    external fun nativeLutimesSync(path: String, atime: Long, mtime: Long)

    @JvmStatic
    external fun nativeLinkSync(oldPath: String, newPath: String)

    @JvmStatic
    external fun nativeLstatSync(path: String): FileStat?

    @JvmStatic
    external fun nativeMkdirSync(path: String, mode: Int, recursive: Boolean)

    @JvmStatic
    external fun nativeMkdtempSync(prefix: String): String?

    @JvmStatic
    external fun nativeOpenSync(path: String, flags: Int, mode: Int): Int

    @JvmStatic
    external fun nativeOpenDirSync(path: String): FileDir?

    @JvmStatic
    external fun nativeReadSync(
      fd: Int,
      buffer: ByteBuffer,
      offset: Long,
      length: Long,
      position: Long
    ): Long

    @JvmStatic
    external fun nativeReadWithBytesSync(
      fd: Int,
      buffer: ByteArray,
      offset: Long,
      length: Long,
      position: Long
    ): Long

    @JvmStatic
    external fun nativeReaddirWithFileTypesSync(
      path: String,
      encoding: String
    ): Array<FileDirent?>?

    @JvmStatic
    external fun nativeReaddirWithFileSync(path: String, encoding: String): Array<String?>?

    @JvmStatic
    external fun nativeReadFileSync(path: String, flags: Int): ByteBuffer?

    @JvmStatic
    external fun nativeReadFileBytesSync(path: String, flags: Int): ByteArray?

    @JvmStatic
    external fun nativeReadFileWithFdSync(fd: Int, flags: Int): ByteBuffer?

    @JvmStatic
    external fun nativeReadFileBytesWithFdSync(fd: Int, flags: Int): ByteArray?

    @JvmStatic
    external fun nativeReadLinkSync(path: String, encoding: String): String?

    @JvmStatic
    external fun nativeReadvSync(fd: Int, buffers: Array<ByteBuffer>, position: Long): Long

    @JvmStatic
    external fun nativeRealPathSync(path: String): String?

    @JvmStatic
    external fun nativeRenameSync(oldPath: String, newPath: String)

    @JvmStatic
    external fun nativeRmdirSync(
      path: String,
      maxRetries: Int,
      recursive: Boolean,
      retryDelay: Long
    )

    @JvmStatic
    external fun nativeRmSync(
      path: String,
      maxRetries: Int,
      recursive: Boolean,
      retryDelay: Long
    )

    @JvmStatic
    external fun nativeStatSync(path: String, throwIfNoEntry: Boolean): FileStat?

    @JvmStatic
    external fun nativeSymlinkSync(target: String, path: String)

    @JvmStatic
    external fun nativeTruncateSync(path: String, len: Long)

    @JvmStatic
    external fun nativeUnlinkSync(path: String)

    @JvmStatic
    external fun nativeUtimesSync(path: String, atime: Long, mtime: Long)

    @JvmStatic
    external fun nativeWriteSync(
      fd: Int,
      buffer: ByteBuffer,
      offset: Long,
      length: Long,
      position: Long
    ): Long

    @JvmStatic
    external fun nativeWriteBytesSync(
      fd: Int,
      buffer: ByteArray,
      offset: Long,
      length: Long,
      position: Long
    ): Long

    @JvmStatic
    external fun nativeWriteStringSync(
      fd: Int,
      string: String,
      encoding: String,
      position: Long
    ): Long

    @JvmStatic
    external fun nativeWriteFileWithStringSync(fd: Int, data: String)

    @JvmStatic
    external fun nativeWriteFileWithBufferSync(fd: Int, data: ByteBuffer)

    @JvmStatic
    external fun nativeWriteFileWithBytesSync(fd: Int, data: ByteArray)

    @JvmStatic
    external fun nativeWriteFileWithStringFromPathSync(
      path: String,
      data: String,
      encoding: String,
      mode: Int,
      flag: Int
    )

    @JvmStatic
    external fun nativeWriteFileWithBytesFromPathSync(
      path: String,
      data: ByteArray,
      encoding: String,
      mode: Int,
      flag: Int
    )

    @JvmStatic
    external fun nativeWriteFileWithBufferFromPathSync(
      path: String,
      data: ByteBuffer,
      encoding: String,
      mode: Int,
      flag: Int
    )

    @JvmStatic
    external fun nativeWritevSync(fd: Int, buffers: Array<ByteBuffer>, position: Long): Long

    @JvmStatic
    var isLoaded = false

    internal fun loadNative() {
      if (isLoaded) {
        return
      }
      try {
        System.loadLibrary("nodeandroid")
      } catch (ignored: Exception) {
      }
    }
  }
}
