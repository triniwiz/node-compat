package org.nativescript.node_compat.fs

class FileDirent internal constructor(var native: Long) {
  fun getName(): String? {
    return nativeName(native)
  }

  fun isBlockDevice(): Boolean {
    return nativeIsBlockDevice(native)
  }

  fun isCharacterDevice(): Boolean {
    return nativeIsCharacterDevice(native)
  }

  fun isDirectory(): Boolean {
    return nativeIsDirectory(native)
  }

  fun isFifo(): Boolean {
    return nativeIsFifo(native)
  }

  fun isFile(): Boolean {
    return nativeIsFile(native)
  }

  fun isSocket(): Boolean {
    return nativeIsSocket(native)
  }

  fun isSymbolicLink(): Boolean {
    return nativeIsSymbolicLink(native)
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
    external fun nativeName(nativeFileDirent: Long): String?

    @JvmStatic
    external fun nativeDispose(nativeFileDirent: Long)

    @JvmStatic
    external fun nativeIsBlockDevice(nativeFileDirent: Long): Boolean

    @JvmStatic
    external fun nativeIsCharacterDevice(nativeFileDirent: Long): Boolean

    @JvmStatic
    external fun nativeIsDirectory(nativeFileDirent: Long): Boolean

    @JvmStatic
    external fun nativeIsFifo(nativeFileDirent: Long): Boolean

    @JvmStatic
    external fun nativeIsFile(nativeFileDirent: Long): Boolean

    @JvmStatic
    external fun nativeIsSocket(nativeFileDirent: Long): Boolean

    @JvmStatic
    external fun nativeIsSymbolicLink(nativeFileDirent: Long): Boolean
  }
}
