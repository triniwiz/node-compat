package org.nativescript.node_compat.fs

class FsWatcher internal constructor(var native: Long, var fileName: String) {

  fun ref() {
    nativeRef(fileName, native)
  }

  fun unref() {
    nativeUnref(fileName, native)
  }

  fun close() {
    nativeClose(fileName, native)
  }

  class Event internal constructor() {
    var fileName: String? = null
      internal set
    var eventType: String? = null
      internal set
  }

  companion object {
    @JvmStatic
    external fun nativeClose(fileName: String, nativeEvent: Long)

    @JvmStatic
    external fun nativeRef(fileName: String, nativeEvent: Long)

    @JvmStatic
    external fun nativeUnref(fileName: String, nativeEvent: Long)
  }
}
