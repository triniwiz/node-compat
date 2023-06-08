package org.nativescript.node_compat.fs

import android.os.ParcelFileDescriptor
import org.json.JSONObject
import org.nativescript.node_compat.buffer.Buffer
import java.io.IOException
import java.nio.ByteBuffer


class FileHandle {
  var closed: Boolean = true
    internal set

  var fd: Int = 0
    internal set

  internal var native: Long = 0

  constructor(fd: Int) {
    native = nativeInit(fd)
    this.fd = fd
  }

  constructor(pfd: ParcelFileDescriptor) {
    // Gets around the warning
    /*
		 * Peer expected signal when closed unable to deliver after detach
		 *  */
    // Gets around the warning
    /*
		 * Peer expected signal when closed unable to deliver after detach
		 *  */
    try {
      val descriptor = pfd.dup()
      fd = descriptor.detachFd()
      native = nativeInit(
        fd
      )
      // close orig
      pfd.close()
    } catch (ignored: IOException) {
    }
  }


  fun close(callback: AsyncCallback<Void>) {
    nativeClose(native, callback.native)
  }

  fun appendFile(data: FileHandle, options: String, callback: AsyncCallback<Void>) {
    //nativeAppend(native, data.native, callback.native)
  }

  fun appendFile(data: String, options: String, callback: AsyncCallback<Void>) {
    nativeAppendFileWithString(native, data, callback.native)
  }

  fun appendFile(data: ByteBuffer, options: String, callback: AsyncCallback<Void>) {
    //nativeAppendFileWithBuffer(native, data, callback.native)
  }

  fun appendFile(data: ByteArray, options: String, callback: AsyncCallback<Void>) {
    nativeAppendFileWithBytes(native, data, callback.native)
  }

  fun read(
    buffer: ByteArray,
    offset: Long,
    length: Long,
    position: Long,
    callback: AsyncCallback<Long>
  ) {
    nativeReadWithBytes(native, buffer, offset, length, position, callback.native)
  }

  fun read(
    buffer: ByteBuffer,
    offset: Long,
    length: Long,
    position: Long,
    callback: AsyncCallback<Long>
  ) {
    nativeRead(native, buffer, offset, length, position, callback.native)
  }

  fun stat(callback: AsyncCallback<FileStat>) {
    nativeStat(native, callback.native)
  }

  fun datasync(callback: AsyncCallback<Void>) {
    nativeDatasync(native, callback.native)
  }

  fun sync(callback: AsyncCallback<Void>) {
    nativeSync(native, callback.native)
  }

  fun futimes(atime: Long, mtime: Long, callback: AsyncCallback<Void>) {
  }

  fun copyFile(src: String, dest: String, callback: AsyncCallback<Void>) {

  }

  fun write(data: FileHandle, options: String, callback: AsyncCallback<Void>) {

  }

  fun write(data: String, options: String, callback: AsyncCallback<Void>) {
    //  nativeWriteString(native, data, )
  }

  fun write(data: ByteBuffer, options: String, callback: AsyncCallback<Void>) {
    // nativeWrite(native, data, )
  }

  fun write(data: ByteArray, options: String, callback: AsyncCallback<Void>) {
    // nativeWriteBytes(native,)
  }

  fun writev(
    buffers: Array<Buffer>,
    position: Long,
    callback: AsyncCallback<Long>
  ) {
    nativeWritev(native, buffers.map { it.native }.toLongArray(), position, callback.native)
  }


  companion object {

    @JvmStatic
    fun open(path: String, flag: Int, mode: Int, callback: AsyncCallback<FileHandle>) {
      nativeOpen(path, flag, mode, callback.native)
    }

    @JvmStatic
    external fun nativeInit(fd: Int): Long

    @JvmStatic
    external fun nativeOpenSync(

      fd: Int,
    ): Long

    @JvmStatic
    external fun nativeOpen(
      path: String,
      flags: Int,
      mode: Int,
      callback: Long,
    )

    @JvmStatic
    external fun nativeAppendFileWithBytes(

      handle: Long,
      bytes: ByteArray,
      callback: Long,
    )

    @JvmStatic
    external fun nativeAppendFileWithString(

      handle: Long,
      data: String,
      callback: Long,
    )

    @JvmStatic
    external fun nativeChmod(

      handle: Long,
      mode: Int,
      callback: Long,
    )

    @JvmStatic
    external fun nativeChown(

      handle: Long,
      uid: Int,
      gid: Int,
      callback: Long,
    )

    @JvmStatic
    external fun nativeClose(

      handle: Long,
      callback: Long,
    )

    @JvmStatic
    external fun nativeDatasync(

      handle: Long,
      callback: Long,
    )

    @JvmStatic
    external fun nativeGetFd(

      handle: Int,
    ): Int

    @JvmStatic
    external fun nativeRead(

      handle: Long,
      buffer: ByteBuffer,
      offset: Long,
      length: Long,
      position: Long,
      callback: Long,
    )

    @JvmStatic
    external fun nativeReadWithBytes(

      handle: Long,
      buffer: ByteArray,
      offset: Long,
      length: Long,
      position: Long,
      callback: Long,
    )

    @JvmStatic
    external fun nativeReadv(

      handle: Long,
      buffers: LongArray,
      position: Long,
      callback: Long,
    )

    @JvmStatic
    external fun nativeStat(

      handle: Long,
      callback: Long,
    )

    @JvmStatic
    external fun nativeSync(

      handle: Long,
      callback: Long,
    )

    @JvmStatic
    external fun nativeTruncate(

      handle: Long,
      len: Long,
      callback: Long,
    )

    @JvmStatic
    external fun nativeUtimes(

      handle: Long,
      atime: Long,
      mtime: Long,
      callback: Long,
    )

    @JvmStatic
    external fun nativeWrite(

      handle: Long,
      buffer: ByteBuffer,
      offset: Long,
      length: Long,
      position: Long,
      callback: Long,
    )

    @JvmStatic
    external fun nativeWriteBytes(

      handle: Long,
      buffer: ByteArray,
      offset: Long,
      length: Long,
      position: Long,
      callback: Long,
    )

    @JvmStatic
    external fun nativeWriteString(

      handle: Long,
      string: String,
      encoding: Int,
      position: Long,
      callback: Long,
    )

    @JvmStatic
    external fun nativeWriteFileWithString(

      handle: Int,
      data: String,
      encoding: Int,
      callback: Long,
    )

    @JvmStatic
    external fun nativeWriteFileWithBytes(

      handle: Long,
      data: ByteArray,
      callback: Long,
    )

    @JvmStatic
    external fun nativeWriteFileWithBuffer(

      handle: Long,
      data: ByteBuffer,
      callback: Long,
    )

    @JvmStatic
    external fun nativeWritev(
      handle: Long,
      buffers: LongArray,
      position: Long,
      callback: Long,
    )
  }

}
