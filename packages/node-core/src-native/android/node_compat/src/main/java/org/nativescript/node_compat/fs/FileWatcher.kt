package org.nativescript.node_compat.fs

class FileWatcher internal constructor(internal var native: Long, var fileName: String) {

  fun ref() {
    nativeRef(fileName, native)
  }

  fun unref() {
    nativeUnref(fileName, native)
  }

  @Throws(Throwable::class)
  protected fun finalize() {
    if (native != 0L) {
      nativeUnref(fileName, native);
      native = 0
    }
  }

  class Event internal constructor() {
    var previous: FileStat? = null
    var current: FileStat? = null
  }

  companion object {
    @JvmStatic
    private external fun nativeRef(fileName: String, nativeEvent: Long)

    @JvmStatic
    private external fun nativeUnref(fileName: String, nativeEvent: Long)
  }
}
