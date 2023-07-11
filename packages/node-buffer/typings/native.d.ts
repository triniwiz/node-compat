declare class NSCBuffer {
  static alloc(size: number): NSCBuffer;

  static allocUnsafe(size: number): NSCBuffer;

  static from(array: number[]): NSCBuffer;

  static from(string: string, encoding?: string): NSCBuffer;

  static concat(buffers: any[], length: number): NSCBuffer;

  static atob(value: string): string;

  static btoa(value: string): string;

  readonly length: number;

  readonly buffer: Uint8Array;

  static copyBytesFrom(view: TypedArray, offset?: number, length?: number);

  toString(encoding?: string, start?: number, end?: number): string;

  writeInt8(value: number, offset: number);

  writeUInt8(value: number, offset: number);

  writeInt16LE(value: number, offset: number);

  writeInt16BE(value: number, offset: number);

  writeUInt16LE(value: number, offset: number);

  writeUInt16BE(value: number, offset: number);

  writeInt32LE(value: number, offset: number);

  writeInt32BE(value: number, offset: number);

  writeUInt32LE(value: number, offset: number);

  writeUInt32BE(value: number, offset: number);

  writeFloatLE(value: number, offset: number);

  writeFloatBE(value: number, offset: number);

  writeDoubleLE(value: number, offset: number);

  writeDoubleBE(value: number, offset: number);

  writeBigInt64LE(value: BigInt, offset: number);

  writeBigInt64BE(value: BigInt, offset: number);

  writeBigUInt64LE(value: BigInt, offset: number);

  writeBigUInt64BE(value: BigInt, offset: number);

  readInt8(offset: number): number;

  readUInt8(offset: number): number;

  readInt16LE(offset: number): number;

  readInt16BE(offset: number): number;

  readUInt16LE(offset: number): number;

  readUInt16BE(offset: number): number;

  readInt32LE(offset: number): number;

  readInt32BE(offset: number): number;

  readUInt32LE(offset: number): number;

  readUInt32BE(offset: number): number;

  readFloatLE(offset: number): number;

  readFloatBE(offset: number): number;

  readDoubleLE(offset: number): number;

  readDoubleBE(offset: number): number;

  readBigInt64LE(offset: number): BigInt;

  readBigInt64BE(offset: number): BigInt;

  readBigUInt64LE(offset: number): BigInt;

  readBigUInt64BE(offset: number): BigInt;
}
declare global {
  NSCBuffer;
}
