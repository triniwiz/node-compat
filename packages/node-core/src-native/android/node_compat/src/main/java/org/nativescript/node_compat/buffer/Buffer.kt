package org.nativescript.node_compat.buffer

import java.math.BigInteger
import java.nio.ByteBuffer

fun String.decodeHex(): ByteArray {
  check(length % 2 == 0) { "Must have an even length" }
  var index = 0
  if (startsWith("0x")) {
    index = 2
  }
  return substring(index)
    .chunked(2)
    .map { it.toInt(16).toByte() }
    .toByteArray()
}

class Buffer private constructor(var native: Long) {

  private var buffer: ByteBuffer? = null

  @JvmOverloads
  fun fill(value: String, encoding: Encoding = Encoding.Utf8) {
    nativeFillString(native, value, encoding.value)
  }

  fun buffer(): ByteBuffer? {
    if (buffer == null) {
      buffer = nativeBuffer(native)
    }
    return buffer
  }

  fun length(): Long {
    return nativeLength(native)
  }

  @JvmOverloads
  fun writeInt8(
    value: Byte,
    offset: Long = -1
  ) {
    nativeWriteInt8(native, value, offset)
  }

  @JvmOverloads
  fun writeUInt8(
    value: Byte,
    offset: Long = -1
  ) {
    nativeWriteUInt8(native, value, offset)
  }

  @JvmOverloads
  fun writeUInt16BE(
    value: Short,
    offset: Long = -1,
  ) {
    nativeWriteUInt16BE(native, value, offset)
  }

  @JvmOverloads
  fun writeUInt16LE(
    value: Short,
    offset: Long = -1,
  ) {
    nativeWriteUInt16LE(native, value, offset)
  }

  @JvmOverloads
  fun writeInt16BE(
    value: Short,
    offset: Long = -1,
  ) {
    nativeWriteInt16BE(native, value, offset)
  }

  @JvmOverloads
  fun writeInt16LE(
    value: Short,
    offset: Long = -1,
  ) {
    nativeWriteInt16LE(native, value, offset)
  }

  @JvmOverloads
  fun writeUInt32BE(
    value: Int,
    offset: Long = -1,
  ) {
    nativeWriteUInt32BE(native, value, offset)
  }

  @JvmOverloads
  fun writeUInt32LE(
    value: Int,
    offset: Long = -1,
  ) {
    nativeWriteUInt32LE(native, value, offset)
  }

  @JvmOverloads
  fun writeInt32BE(
    value: Int,
    offset: Long = -1,
  ) {
    nativeWriteInt32BE(native, value, offset)
  }

  @JvmOverloads
  fun writeInt32LE(
    value: Int,
    offset: Long = -1,
  ) {
    nativeWriteInt32LE(native, value, offset)
  }

  @JvmOverloads
  fun writeFloatBE(
    value: Float,
    offset: Long = -1,
  ) {
    nativeWriteFloatBE(native, value, offset)
  }

  @JvmOverloads
  fun writeFloatLE(
    value: Float,
    offset: Long = -1,
  ) {
    nativeWriteFloatLE(native, value, offset)
  }


  @JvmOverloads
  fun writeDoubleBE(
    value: Double,
    offset: Long = -1,
  ) {
    nativeWriteDoubleBE(native, value, offset)
  }

  @JvmOverloads
  fun writeDoubleLE(
    value: Double,
    offset: Long = -1,
  ) {
    nativeWriteDoubleLE(native, value, offset)
  }

  @JvmOverloads
  fun writeBigUInt64BE(
    value: BigInteger,
    offset: Long = -1,
  ) {
    nativeWriteUInt64BE(native, value.toByteArray(), offset)
  }

  @JvmOverloads
  fun writeBigUInt64LE(
    value: BigInteger,
    offset: Long = -1,
  ) {
    nativeWriteUInt64LE(native, value.toByteArray(), offset)
  }

  @JvmOverloads
  fun writeBigInt64BE(
    value: BigInteger,
    offset: Long = -1,
  ) {
    nativeWriteInt64BE(native, value.toByteArray(), offset)
  }

  @JvmOverloads
  fun writeBigInt64LE(
    value: BigInteger,
    offset: Long = -1,
  ) {
    nativeWriteInt64LE(native, value.toByteArray(), offset)
  }

  fun readInt8(
    offset: Long,
  ): Byte {
    return nativeReadInt8(native, offset)
  }

  fun readUInt8(
    offset: Long,
  ): Byte {
    return nativeReadUInt8(native, offset)
  }

  fun readUInt16BE(
    offset: Long,
  ): Short {
    return nativeReadUInt16BE(native, offset)
  }

  fun readUInt16LE(
    offset: Long,
  ): Short {
    return nativeReadUInt16LE(native, offset)
  }

  fun readInt16BE(
    offset: Long,
  ): Short {
    return nativeReadInt16BE(native, offset)
  }

  fun readInt16LE(
    offset: Long,
  ): Short {
    return nativeReadInt16LE(native, offset)
  }

  fun readUInt32BE(
    offset: Long,
  ): Int {
    return nativeReadUInt32BE(native, offset)
  }

  fun readUInt32LE(
    offset: Long,
  ): Int {
    return nativeReadUInt32LE(native, offset)
  }

  fun readInt32BE(
    offset: Long,
  ): Int {
    return nativeReadInt32BE(native, offset)
  }

  fun readInt32LE(
    offset: Long,
  ): Int {
    return nativeReadInt32LE(native, offset)
  }

  fun readFloatBE(
    offset: Long,
  ): Float {
    return nativeReadFloatBE(native, offset)
  }

  fun readFloatLE(
    offset: Long,
  ): Float {
    return nativeReadFloatLE(native, offset)
  }


  fun readDoubleBE(
    offset: Long,
  ): Double {
    return nativeReadDoubleBE(native, offset)
  }

  fun readDoubleLE(
    offset: Long,
  ): Double {
    return nativeReadDoubleLE(native, offset)
  }

  fun readUInt64BE(
    offset: Long,
  ): BigInteger {
    return BigInteger(nativeReadUInt64BE(native, offset))
  }

  fun readUInt64LE(
    offset: Long,
  ): BigInteger {
    return BigInteger(nativeReadUInt64LE(native, offset))
  }


  fun readInt64BE(
    offset: Long,
  ): BigInteger {
    return BigInteger(nativeReadInt64BE(native, offset))
  }

  fun readBigInt64LE(
    offset: Long,
  ): BigInteger {
    return BigInteger(nativeReadInt64LE(native, offset))
  }

  override fun toString(): String {
    return nativeToPrintString(native)
  }

  @JvmOverloads
  fun toString(encoding: Encoding, start: Long = -1, end: Long = -1): String {
    return nativeToString(native, encoding.value, start, end)
  }

  @Throws(Throwable::class)
  protected fun finalize() {
    if (native != 0L) {
      nativeDestroy(native)
      native = 0
    }
  }

  enum class Encoding(val value: Int) {
    Ascii(0),
    Utf8(1),
    Utf16le(2),
    Ucs2(3),
    Base64(4),
    Latin1(5),
    Binary(6),
    Hex(7)
  }

  companion object {

    init {
      System.loadLibrary("nodeandroid")
    }

    @JvmOverloads
    @JvmStatic
    fun alloc(size: Long, text: String? = null, encoding: Encoding = Encoding.Utf8): Buffer {
      return Buffer(nativeAlloc(size, text, encoding.value))
    }

    @JvmStatic
    fun from(value: String, encoding: Encoding = Encoding.Utf8): Buffer {
      return Buffer(nativeFromString(value, encoding.value))
    }

    @JvmStatic
    fun from(value: Buffer): Buffer {
      return Buffer(nativeFromBuffer(value.native))
    }

    @JvmStatic
    fun from(array: ByteArray): Buffer {
      return Buffer(nativeFromArray(array))
    }

    @JvmStatic
    fun copyBytesFrom(buffer: ByteBuffer, offset: Long = -1, length: Long = -1): Buffer {
      return Buffer(nativeCopyBytesFrom(buffer, offset, length))
    }

    @JvmOverloads
    @JvmStatic
    fun concat(value: Array<ByteBuffer>, length: Long = -1): Buffer {
      return Buffer(nativeConcat(value, length))
    }

    @JvmStatic
    fun atob(text: String): String {
      return nativeAtob(text)
    }

    @JvmStatic
    fun btoa(text: String): String {
      return nativeBtoa(text)
    }

    @JvmStatic
    external fun nativeAlloc(size: Long, text: String?, encoding: Int): Long

    @JvmStatic
    external fun nativeFromBuffer(buffer: Long): Long

    @JvmStatic
    external fun nativeFromString(text: String, encoding: Int): Long

    @JvmStatic
    external fun nativeFromArray(array: ByteArray): Long

    @JvmStatic
    external fun nativeCopyBytesFrom(buffer: ByteBuffer, offset: Long, length: Long): Long

    @JvmStatic
    external fun nativeConcat(value: Array<ByteBuffer>, length: Long): Long

    @JvmStatic
    external fun nativeDestroy(buffer: Long)

    @JvmStatic
    external fun nativeBuffer(buffer: Long): ByteBuffer

    @JvmStatic
    external fun nativeLength(buffer: Long): Long

    @JvmStatic
    external fun nativeFillString(buffer: Long, value: String, encoding: Int)

    @JvmStatic
    external fun nativeAtob(text: String): String

    @JvmStatic
    external fun nativeBtoa(text: String): String

    @JvmStatic
    external fun nativeToString(
      buffer: Long,
      encoding: Int,
      start: Long,
      end: Long
    ): String

    @JvmStatic
    external fun nativeToPrintString(
      buffer: Long
    ): String

    @JvmStatic
    external fun nativeWriteInt8(
      buffer: Long,
      value: Byte,
      offset: Long
    )

    @JvmStatic
    external fun nativeWriteUInt8(
      buffer: Long,
      value: Byte,
      offset: Long
    )

    @JvmStatic
    external fun nativeWriteUInt16BE(

      buffer: Long,
      value: Short,
      offset: Long,
    )

    @JvmStatic
    external fun nativeWriteUInt16LE(
      buffer: Long,
      value: Short,
      offset: Long,
    )

    @JvmStatic
    external fun nativeWriteInt16BE(
      buffer: Long,
      value: Short,
      offset: Long,
    )

    @JvmStatic
    external fun nativeWriteInt16LE(

      buffer: Long,
      value: Short,
      offset: Long,
    )

    @JvmStatic
    external fun nativeWriteUInt32BE(
      buffer: Long,
      value: Int,
      offset: Long,
    )

    @JvmStatic
    external fun nativeWriteUInt32LE(
      buffer: Long,
      value: Int,
      offset: Long,
    )

    @JvmStatic
    external fun nativeWriteInt32BE(
      buffer: Long,
      value: Int,
      offset: Long,
    )

    @JvmStatic
    external fun nativeWriteInt32LE(
      buffer: Long,
      value: Int,
      offset: Long,
    )

    @JvmStatic
    external fun nativeWriteFloatBE(

      buffer: Long,
      value: Float,
      offset: Long,
    )

    @JvmStatic
    external fun nativeWriteFloatLE(

      buffer: Long,
      value: Float,
      offset: Long,
    )


    @JvmStatic
    external fun nativeWriteDoubleBE(

      buffer: Long,
      value: Double,
      offset: Long,
    )

    @JvmStatic
    external fun nativeWriteDoubleLE(

      buffer: Long,
      value: Double,
      offset: Long,
    )


    @JvmStatic
    external fun nativeWriteUInt64BE(
      buffer: Long,
      value: ByteArray,
      offset: Long,
    )

    @JvmStatic
    external fun nativeWriteUInt64LE(
      buffer: Long,
      value: ByteArray,
      offset: Long,
    )

    @JvmStatic
    external fun nativeWriteInt64BE(
      buffer: Long,
      value: ByteArray,
      offset: Long,
    )

    @JvmStatic
    external fun nativeWriteInt64LE(
      buffer: Long,
      value: ByteArray,
      offset: Long,
    )


    @JvmStatic
    external fun nativeReadInt8(

      buffer: Long,
      offset: Long,
    ): Byte

    @JvmStatic
    external fun nativeReadUInt8(

      buffer: Long,
      offset: Long,
    ): Byte

    @JvmStatic
    external fun nativeReadUInt16BE(

      buffer: Long,
      offset: Long,
    ): Short

    @JvmStatic
    external fun nativeReadUInt16LE(

      buffer: Long,
      offset: Long,
    ): Short


    @JvmStatic
    external fun nativeReadInt16BE(

      buffer: Long,
      offset: Long,
    ): Short

    @JvmStatic
    external fun nativeReadInt16LE(

      buffer: Long,
      offset: Long,
    ): Short

    @JvmStatic
    external fun nativeReadUInt32BE(

      buffer: Long,
      offset: Long,
    ): Int

    @JvmStatic
    external fun nativeReadUInt32LE(

      buffer: Long,
      offset: Long,
    ): Int


    @JvmStatic
    external fun nativeReadInt32BE(
      buffer: Long,
      offset: Long,
    ): Int

    @JvmStatic
    external fun nativeReadInt32LE(
      buffer: Long,
      offset: Long,
    ): Int

    @JvmStatic
    external fun nativeReadFloatBE(

      buffer: Long,
      offset: Long,
    ): Float

    @JvmStatic
    external fun nativeReadFloatLE(
      buffer: Long,
      offset: Long,
    ): Float


    @JvmStatic
    external fun nativeReadDoubleBE(
      buffer: Long,
      offset: Long,
    ): Double

    @JvmStatic
    external fun nativeReadDoubleLE(
      buffer: Long,
      offset: Long,
    ): Double

    @JvmStatic
    external fun nativeReadUInt64BE(
      buffer: Long,
      offset: Long,
    ): ByteArray

    @JvmStatic
    external fun nativeReadUInt64LE(
      buffer: Long,
      offset: Long,
    ): ByteArray


    @JvmStatic
    external fun nativeReadInt64BE(
      buffer: Long,
      offset: Long,
    ): ByteArray

    @JvmStatic
    external fun nativeReadInt64LE(
      buffer: Long,
      offset: Long,
    ): ByteArray
  }
}
