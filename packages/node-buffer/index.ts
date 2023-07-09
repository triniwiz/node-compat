type StringEncoding = 'ascii' | 'utf8' | 'utf-8' | 'utf16le' | 'utf-16le' | 'ucs2' | 'ucs-2' | 'base64' | 'base64url' | 'latin1' | 'binary' | 'hex';

type ConcatType = number | Buffer;

export class Buffer {
  _native: NSCBuffer;

  [value: number]: number;

  private static indexedHandler: ProxyHandler<Buffer> = {
    get(target, property) {
      return target[property];
    },
    set(target, property, value): boolean {
      target[property] = value;
      return true;
    },
  };

  constructor() {
    return new Proxy(this, Buffer.indexedHandler);
  }

  static atob(value: string): string {
    return NSCBuffer.atob(value);
  }

  static btoa(value: string): string {
    return NSCBuffer.btoa(value);
  }

  static alloc(size: number): Buffer {
    const buffer = new Buffer();
    buffer._native = NSCBuffer.alloc(size);
    return buffer;
  }

  static allocUnsafe(size: number): Buffer {
    const buffer = new Buffer();
    buffer._native = NSCBuffer.allocUnsafe(size);
    return buffer;
  }

  static from(string: string, encoding?: string): Buffer;
  static from(array: number[]): Buffer;
  static from(value: string | number[], encoding?: string): Buffer {
    const buffer = new Buffer();
    if (Array.isArray(value)) {
      buffer._native = NSCBuffer.from(value);
    } else if (typeof value === 'string') {
      buffer._native = NSCBuffer.from(value, encoding);
    }
    return buffer;
  }

  static concat(buffers: ConcatType[], length: number = undefined): Buffer {
    const buffer = new Buffer();
    buffer._native = NSCBuffer.concat(
      buffers.map((buffer) => {
        if (buffer instanceof Buffer) {
          return buffer._native;
        }
        return buffer;
      }),
      length
    );
    return buffer;
  }

  get length() {
    return this._native.length;
  }

  toString(encoding: StringEncoding = undefined, start: number = 0, end: number = -1) {
    return this._native.toString(encoding, start, end);
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

  writeBigInt64LE(value: BigInt, offset: number) {
    return this._native.writeBigInt64LE(value, offset);
  }

  writeBigInt64BE(value: BigInt, offset: number) {
    return this._native.writeBigInt64BE(value, offset);
  }

  writeBigUInt64LE(value: BigInt, offset: number) {
    return this._native.writeBigUInt64LE(value, offset);
  }

  writeBigUInt64BE(value: BigInt, offset: number) {
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

  readBigInt64LE(offset: number): BigInt {
    return this._native.readBigInt64LE(offset);
  }

  readBigInt64BE(offset: number): BigInt {
    return this._native.readBigInt64BE(offset);
  }

  readBigUInt64LE(offset: number): BigInt {
    return this._native.readBigUInt64LE(offset);
  }

  readBigUInt64BE(offset: number): BigInt {
    return this._native.readBigUInt64BE(offset);
  }
}
