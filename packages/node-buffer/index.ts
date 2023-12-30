type StringEncoding = 'ascii' | 'utf8' | 'utf-8' | 'utf16le' | 'utf-16le' | 'ucs2' | 'ucs-2' | 'base64' | 'base64url' | 'latin1' | 'binary' | 'hex';

type ConcatType = number | Buffer;

const K_MAX_LENGTH = 0x7fffffff;

type TypedArray = Uint8ClampedArray | Uint8Array | Uint16Array | Uint32Array | Int8Array | Int16Array | Int32Array | Float32Array | Float64Array | BigInt64Array | BigUint64Array;

export class Buffer {
  _native: NSCBuffer;
  _buffer: ArrayBuffer;
  poolSize = 8192;
  _length = 0;

  get buffer() {
    return this._buffer;
  }

  static atob(value: string): string {
    return NSCBuffer.atob(value);
  }

  static btoa(value: string): string {
    return NSCBuffer.btoa(value);
  }

  static alloc(size: number): Buffer {
    if (size > K_MAX_LENGTH) {
      throw new RangeError('The value "' + length + '" is invalid for option "size"');
    }

    const buffer = NSCBuffer.alloc(size);

    const buf = buffer.buffer as any;

    buf._length = buf.length;

    Object.setPrototypeOf(buf, Buffer.prototype);

    buf._native = buffer;

    return buf;
  }

  static allocUnsafe(size: number): Buffer {
    if (size > K_MAX_LENGTH) {
      throw new RangeError('The value "' + length + '" is invalid for option "size"');
    }

    const buffer = NSCBuffer.allocUnsafe(size);

    const buf = buffer.buffer as any;

    buf._length = buf.length;

    Object.setPrototypeOf(buf, Buffer.prototype);

    buf._native = buffer;

    return buf;
  }

  static copyBytesFrom(view: TypedArray, offset = 0, length = -1) {
    return NSCBuffer.copyBytesFrom(view, offset, length);
  }

  static from(string: string, encoding?: string): Buffer;
  static from(array: number[]): Buffer;
  static from(value: string | number[], encoding?: string): Buffer {
    if (value == null) {
      throw new TypeError('The first argument must be one of type string, Buffer, ArrayBuffer, Array, ' + 'or Array-like Object. Received type ' + typeof value);
    }

    let buffer: NSCBuffer = null;
    if (Array.isArray(value)) {
      buffer = NSCBuffer.from(value);
    } else if (typeof value === 'string') {
      buffer = NSCBuffer.from(value, encoding);
    }

    if (!buffer) {
      return null;
    }

    const buf = buffer.buffer as any;

    buf._length = buf.length;

    Object.setPrototypeOf(buf, Buffer.prototype);

    buf._native = buffer;

    return buf;
  }

  static concat(buffers: ConcatType[], length: number = undefined): Buffer {
    const buffer = NSCBuffer.concat(
      buffers.map((buffer) => {
        if (buffer instanceof Buffer) {
          return buffer._native;
        }
        return buffer;
      }),
      length
    );

    if (!buffer) {
      return null;
    }

    const buf = buffer.buffer as any;

    buf._length = buf.length;

    Object.setPrototypeOf(buf, Buffer.prototype);

    buf._native = buffer;

    return buf;
  }

  get length() {
    return this._length;
  }

  toString(encoding: StringEncoding = undefined, start = 0, end = -1) {
    if (arguments.length === 0) {
      return this._native.toString();
    }
    return this._native.toString(encoding ?? undefined, start ?? 0, end ?? -1);
  }

  writeInt8(value: number, offset: number) {
    return this._native.writeInt8(value, offset);
  }

  writeUInt8(value: number, offset: number) {
    return this._native.writeUInt8(value, offset);
  }

  writeInt16LE(value: number, offset: number) {
    return this._native.writeInt16LE(value, offset);
  }

  writeInt16BE(value: number, offset: number) {
    return this._native.writeInt16BE(value, offset);
  }

  writeUInt16LE(value: number, offset: number) {
    return this._native.writeUInt16LE(value, offset);
  }

  writeUInt16BE(value: number, offset: number) {
    return this._native.writeUInt16BE(value, offset);
  }

  writeInt32LE(value: number, offset: number) {
    return this._native.writeInt32LE(value, offset);
  }

  writeInt32BE(value: number, offset: number) {
    return this._native.writeInt32BE(value, offset);
  }

  writeUInt32LE(value: number, offset: number) {
    return this._native.writeUInt32LE(value, offset);
  }

  writeUInt32BE(value: number, offset: number) {
    return this._native.writeUInt32BE(value, offset);
  }

  writeFloatLE(value: number, offset: number) {
    return this._native.writeFloatLE(value, offset);
  }

  writeFloatBE(value: number, offset: number) {
    return this._native.writeFloatBE(value, offset);
  }

  writeDoubleLE(value: number, offset: number) {
    return this._native.writeDoubleLE(value, offset);
  }

  writeDoubleBE(value: number, offset: number) {
    return this._native.writeDoubleBE(value, offset);
  }

  writeBigInt64LE(value: bigint, offset: number) {
    return this._native.writeBigInt64LE(value, offset);
  }

  writeBigInt64BE(value: bigint, offset: number) {
    return this._native.writeBigInt64BE(value, offset);
  }

  writeBigUInt64LE(value: bigint, offset: number) {
    return this._native.writeBigUInt64LE(value, offset);
  }

  writeBigUInt64BE(value: bigint, offset: number) {
    return this._native.writeBigUInt64BE(value, offset);
  }

  readInt8(offset: number): number {
    return this._native.readInt8(offset);
  }

  readUInt8(offset: number): number {
    return this._native.readUInt8(offset);
  }

  readInt16LE(offset: number): number {
    return this._native.readInt16LE(offset);
  }

  readInt16BE(offset: number): number {
    return this._native.readInt16BE(offset);
  }

  readUInt16LE(offset: number): number {
    return this._native.readUInt16LE(offset);
  }

  readUInt16BE(offset: number): number {
    return this._native.readUInt16BE(offset);
  }

  readInt32LE(offset: number): number {
    return this._native.readInt32LE(offset);
  }

  readInt32BE(offset: number): number {
    return this._native.readInt32BE(offset);
  }

  readUInt32LE(offset: number): number {
    return this._native.readUInt32LE(offset);
  }

  readUInt32BE(offset: number): number {
    return this._native.readUInt32BE(offset);
  }

  readFloatLE(offset: number): number {
    return this._native.readFloatLE(offset);
  }

  readFloatBE(offset: number): number {
    return this._native.readFloatBE(offset);
  }

  readDoubleLE(offset: number): number {
    return this._native.readDoubleLE(offset);
  }

  readDoubleBE(offset: number): number {
    return this._native.readDoubleBE(offset);
  }

  readBigInt64LE(offset: number): bigint {
    return this._native.readBigInt64LE(offset);
  }

  readBigInt64BE(offset: number): bigint {
    return this._native.readBigInt64BE(offset);
  }

  readBigUInt64LE(offset: number): bigint {
    return this._native.readBigUInt64LE(offset);
  }

  readBigUInt64BE(offset: number): bigint {
    return this._native.readBigUInt64BE(offset);
  }
}
