package org.nativescript.node_compat.fs

import android.os.ParcelFileDescriptor
import org.json.JSONObject
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
  executor.execute(() -> {
    try {
      closeSync(this);
      callback.onSuccess(null);
    } catch (IOException e) {
      callback.onError(e);
    }
  });
}

fun appendFile(data: FileHandle, options: String, FileSystem.Callback<Void> callback) {
  executors.execute(() -> {
    boolean error = false;
    try {
      appendFileSync(this, data, options);
    } catch (IOException e) {
      callback.onError(e);
      error = true;
    } finally {
      if (!error) {
        callback.onSuccess(null);
      }
    }
  });
}

fun appendFile(data: String, options: String, FileSystem.Callback<Void> callback) {
  executors.execute(() -> {
    boolean error = false;
    try {
      appendFileSync(this, data, options);
    } catch (IOException e) {
      callback.onError(e);
      error = true;
    } finally {
      if (!error) {
        callback.onSuccess(null);
      }
    }
  });
}

fun appendFile(data: ByteBuffer, options: String, FileSystem.Callback<Void> callback) {
  executors.execute(() -> {
    try {
      appendFileSync(this, data, options);
      callback.onSuccess(null);
    } catch (IOException e) {
      callback.onError(e);
    }
  });
}

fun appendFile(data: ByteArray, options: String, FileSystem.Callback<Void> callback) {
  executors.execute(() -> {
    try {
      appendFileSync(this, data, options);
      callback.onSuccess(null);
    } catch (IOException e) {
      callback.onError(e);
    }
  });
}

fun read(buffer: ByteArray, int offset, int length, int position, FileSystem.Callback<Long> callback) {
  executors.execute(() -> {
    try {
      long read = readSync(this, buffer, offset, length, position);
      callback.onSuccess(read);
    } catch (IOException e) {
      callback.onError(e);
    }
  });
}

fun read(buffer: ByteBuffer, offset: Int, length: Int, position: Int, FileSystem.Callback<Long> callback) {
  executors.execute(() -> {
    try {
      long read = readSync(this, buffer, offset, length, position);
      callback.onSuccess(read);
    } catch (IOException e) {
      callback.onError(e);
    }
  });
}

fun stat(FileSystem.Callback<String> callback) {
  executor.execute(() -> {
    try {
      JSONObject json = statSync(this);
      callback.onSuccess(json.toString());
    } catch (IOException e) {
      callback.onError(e);
    }
  });
}

fun datasync(FileSystem.Callback<Void> callback) {
  executor.execute(() -> {
    try {
      fdatasyncSync(this);
      callback.onSuccess(null);
    } catch (Exception e) {
      callback.onError(e);
    }
  });
}

fun sync(FileSystem.Callback<Void> callback) {
  executor.execute(() -> {
    try {
      fsyncSync(this);
      callback.onSuccess(null);
    } catch (Exception e) {
      callback.onError(e);
    }
  });
}

fun futimes(atime: Long, mtime: Long, FileSystem.Callback<Void> callback) {
  executor.execute(() -> {
    try {
      futimesSync(this, atime, mtime);
      callback.onSuccess(null);
    } catch (Exception e) {
      callback.onError(e);
    }
  });
}

fun copyFile(src: String, dest: String, FileSystem.Callback<Void> callback) {
  executors.execute(() -> {
    try {
      copyFileSync(src, dest);
      callback.onSuccess(null);
    } catch (Exception e) {
      callback.onError(e);
    }
  });
}

fun write(data: FileHandle, options: String, FileSystem.Callback<Void> callback) {
  executors.execute(() -> {
    boolean error = false;
    try {
      writeSync(this, data, options);
    } catch (IOException e) {
      callback.onError(e);
      error = true;
    } finally {
      if (!error) {
        callback.onSuccess(null);
      }
    }
  });
}

fun write(data: String, options: String, FileSystem.Callback<Void> callback) {
  executors.execute(() -> {
    boolean error = false;
    try {
      writeSync(this, data, options);
    } catch (IOException e) {
      callback.onError(e);
      error = true;
    } finally {
      if (!error) {
        callback.onSuccess(null);
      }
    }
  });
}

fun write(data: ByteBuffer, options: String, FileSystem.Callback<Void> callback) {
  executors.execute(() -> {
    try {
      writeSync(this, data, options);
      callback.onSuccess(null);
    } catch (IOException e) {
      callback.onError(e);
    }
  });
}

fun write(data: ByteArray, options: String, FileSystem.Callback<Void> callback) {
  executors.execute(() -> {
    try {
      writeSync(this, data, options);
      callback.onSuccess(null);
    } catch (IOException e) {
      callback.onError(e);
    }
  });
}


companion object {

  @JvmStatic
  fun open(path: String, flag: Int,mode: Int, callback: AsyncCallback<FileHandle>): FileHandle? {
    return nativeOpen(path, flag, mode, callback.native)
  }

  @JvmStatic
  external fun nativeInit(fd: Int): Long

  @JvmStatic
  external fun nativeOpen(path: String?, flags: Int, mode: Int, callback: Long): FileHandle?

  @JvmStatic
  external fun nativeDispose(fd: Long): Long

  @JvmStatic
  external fun nativeClose(fh: Long)

  @JvmStatic
  external fun nativeAppend(fo: Long, fi: Long)

  @JvmStatic
  external fun nativeAppendString(fh: Long, data: String?)

  @JvmStatic
  external fun nativeAppendBytes(fh: Long, data: ByteArray?)

  @JvmStatic
  external fun nativeAppendBuffer(fh: Long, data: ByteBuffer?)

  @JvmStatic
  external fun nativeRead(
    fh: Long,
    buffer: ByteArray?,
    offset: Long,
    length: Long,
    position: Long
  ): Long

  @JvmStatic
  external fun nativeReadBuffer(
    fh: Long,
    buffer: ByteBuffer?,
    offset: Long,
    length: Long,
    position: Long
  ): Long

  @JvmStatic
  external fun nativeWrite(fh: Long, data: Long): Long

  @JvmStatic
  external fun nativeWriteString(fh: Long, data: String?)

  @JvmStatic
  external fun nativeWriteBytes(fh: Long, data: ByteArray?): Long

  @JvmStatic
  external fun nativeWriteBuffer(fh: Long, buffer: ByteBuffer?): Long

  @JvmStatic
  external fun nativeStat(fh: Long): JSONObject?

  @JvmStatic
  external fun nativeDataSync(fh: Long)

  @JvmStatic
  external fun nativeSync(fh: Long)

  @JvmStatic
  external fun nativeFutimes(fh: Long, atime: Long, mtime: Long)

  @JvmStatic
  external fun nativeCopyFile(src: String?, dest: String?)

  @JvmStatic
  external fun nativeUnlink(path: String?)
}

}
