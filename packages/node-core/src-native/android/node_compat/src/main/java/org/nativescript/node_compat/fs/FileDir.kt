package org.nativescript.node_compat.fs

class FileDir internal constructor(var native: Long) {
  fun getPath(): String? {
    return nativePath(native)
  }

  fun closeSync() {
    nativeCloseSync(native)
  }

  fun close(callback: AsyncCallback<Void?>) {
    nativeClose(native, callback.native)
  }

  fun readSync(): FileDirent? {
    return nativeReadSync(native)
  }

  fun read(callback: AsyncCallback<FileDirent?>) {
    nativeRead(native, callback.native)
  }

  @Throws(Throwable::class)
  protected fun finalize() {
    if (native != 0L) {
      nativeDispose(native)
      native = 0
    }
  }

  companion object {
    @JvmStatic
    private external fun nativePath(fileDir: Long): String?

    @JvmStatic
    private external fun nativeCloseSync(fileDir: Long): String?

    @JvmStatic
    private external fun nativeClose(fileDir: Long, callback: Long)

    @JvmStatic
    private external fun nativeReadSync(fileDir: Long): FileDirent?

    @JvmStatic
    private external fun nativeRead(fileDir: Long, callback: Long)

    @JvmStatic
    private external fun nativeDispose(fileDir: Long)
  }

}
