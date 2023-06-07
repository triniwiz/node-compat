package org.nativescript.node_compat.fs

abstract class AsyncCallback<T> {
  internal var native: Long = createAsyncCallback(this)

  abstract fun onSuccess(result: T)

  abstract fun onError(error: Any)

  @Throws(Throwable::class)
  protected fun finalize() {
    if (native != 0L) {
      disposeAsyncCallback(native)
      native = 0
    }
  }

  companion object {
    init {
      System.loadLibrary("nodeandroid")
    }

    @JvmStatic
    external fun <T> createAsyncCallback(callback: AsyncCallback<T>): Long

    external fun disposeAsyncCallback(callback: Long)
  }
}
