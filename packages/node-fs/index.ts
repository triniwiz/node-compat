require('@nativescript/node-core');

import { Buffer } from '@nativescript/node-buffer';

export interface IFileStat {
  dev: number | bigint;
  ino: number | bigint;
  mode: number;
  nlink: number | bigint;
  uid: number;
  gid: number;
  rdev: number | bigint;
  size: number | bigint;
  blksize: number | bigint;
  blocks: number | bigint;
  atimeMs: number;
  mtimeMs: number;
  ctimeMs: number;
  birthtimeMs: number;
  birthtime: number;
  atime: number;
  mtime: number;
  ctime: number;
  isBlockDevice: boolean;
  isCharacterDevice: boolean;
  isDirectory: boolean;
  isFIFO: boolean;
  isFile: boolean;
  isSocket: boolean;
  isSymbolicLink: boolean;
}

const O_APPEND = 8;
const O_CREAT = 512;
const O_EXCL = 2048;
const O_RDONLY = 0;
const O_WRONLY = 1;
const O_DSYNC = 0x00400000;
const O_TRUNC = 1024;

function parseFlags(flags: string) {
  let ret = 0;
  switch (flags) {
    case 'a':
      ret = O_APPEND | O_CREAT;
      break;
    case 'ax':
      ret = O_APPEND | O_CREAT | O_EXCL;
      break;
    case 'a+':
      ret = O_APPEND | O_CREAT | O_RDONLY;
      break;
    case 'ax+':
      ret = O_APPEND | O_CREAT | O_RDONLY | O_EXCL;
      break;
    case 'as':
      ret = O_APPEND | O_CREAT | O_DSYNC;
      break;
    case 'as+':
      ret = O_APPEND | O_CREAT | O_DSYNC | O_RDONLY;
      break;
    case 'r':
      ret = O_RDONLY;
      break;
    case 'r+':
      ret = O_RDONLY | O_WRONLY;
      break;
    case 'rs+':
      ret = O_RDONLY | O_WRONLY | O_DSYNC;
      break;
    case 'w':
      ret = O_WRONLY;
      break;
    case 'wx':
      ret = O_WRONLY | O_EXCL;
      break;
    case 'w+':
      ret = O_WRONLY | O_RDONLY | O_CREAT | O_TRUNC;
      break;
    case 'wx+':
      ret = O_WRONLY | O_RDONLY | O_CREAT | O_TRUNC | O_EXCL;
      break;
  }
  return ret;
}

class FSWatcher {
  _native: any;
  close() {
    this._native.close();
  }

  ref() {
    this._native.ref();
  }

  unref() {
    this._native.unref();
  }
}

class StatWatcher {
  _native: any;

  ref() {
    this._native.ref();
  }

  unref() {
    this._native.unref();
  }
}

class Fs {
  static accessSync(path: string | Buffer, mode = 0) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }
    return NSCFS.accessSync(path, mode);
  }

  static appendFileSync(
    path: string | Buffer | URL | number,
    data: string | Buffer,
    options?: {
      encoding?: string | null;
      mode?: number;
      flag?: string;
    }
  ) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else if (typeof path !== 'number') {
      path = path.toString();
    }

    if (data instanceof Buffer) {
      data = data.toString('utf-8');
    } else {
      data = data.toString();
    }
    return NSCFS.appendFileSync(path, data, options);
  }

  static chmodSync(path: string | Buffer | URL, mode: string | number = 0) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }
    return NSCFS.chmodSync(path, mode);
  }

  static chownSync(path: string | Buffer | URL, uid: number, gid: number) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }
    return NSCFS.chownSync(path, uid, gid);
  }

  static closeSync(fd: number) {
    return NSCFS.closeSync(fd);
  }

  static copyFileSync(src: string | Buffer | URL, dest: string | Buffer | URL, mode = 0) {
    if (src instanceof Buffer) {
      src = src.toString('utf-8');
    } else {
      src = src.toString();
    }

    if (dest instanceof Buffer) {
      dest = dest.toString('utf-8');
    } else {
      dest = dest.toString();
    }
    return NSCFS.copyFileSync(src, dest, mode);
  }

  static cpSync(
    src: string | URL,
    dest: string | URL,
    options?: {
      dereference?: boolean;
      errorOnExist?: boolean;
      filter?: (src: string, dest: string) => boolean;
      force?: boolean;
      mode?: number;
      preserveTimestamps: boolean;
      recursive: boolean;
      verbatimSymlinks?: boolean;
    }
  ) {}

  static existsSync(path: string | Buffer | URL): boolean {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.existsSync(path);
  }

  static fchmodSync(fd: number, mode: string | number = 0) {
    return NSCFS.fchmodSync(fd, mode);
  }

  static fchownSync(fd: number, uid: number, gid: number) {
    return NSCFS.fchownSync(fd, uid, gid);
  }

  static fdatasyncSync(fd: number) {
    return NSCFS.fdatasyncSync(fd);
  }

  static fstatSync(fd: number, options?: { bigint?: boolean }) {
    return NSCFS.fstatSync(fd);
  }

  static lchmodSync(path: string | Buffer | URL, mode = 0) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.lchmodSync(path, mode);
  }

  static lchownSync(path: string | Buffer | URL, uid: number, gid: number) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.lchownSync(path, uid, gid);
  }

  static lutimesSync(path: string | Buffer | URL, atime: number | string | Date, mtime: number | string | Date) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.lutimesSync(path, atime, mtime);
  }

  static linkSync(existingPath: string | Buffer | URL, newPath: string | Buffer | URL) {
    if (existingPath instanceof Buffer) {
      existingPath = existingPath.toString('utf-8');
    } else {
      existingPath = existingPath.toString();
    }

    if (newPath instanceof Buffer) {
      newPath = newPath.toString('utf-8');
    } else {
      newPath = newPath.toString();
    }

    return NSCFS.linkSync(existingPath, newPath);
  }

  static mkdirSync(path: string | Buffer | URL, options?: { recursive?: boolean; mode: string | number }) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.mkdirSync(path, { recursive: false, mode: 0o777, ...options });
  }

  static mkdtempSync(prefix: string, options?: { encoding?: string }) {
    return NSCFS.mkdtempSync(prefix, options);
  }

  static opendirSync(path: string | Buffer | URL, options?: { encoding?: string | null; bufferSize?: number; recursive?: boolean }) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }
    return NSCFS.opendirSync(path, options);
  }

  static openSync(path: string | Buffer | URL, flags: string | number, mode: number): number {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }
    return NSCFS.openSync(path, flags, mode);
  }

  static readdirSync(
    path: string | Buffer | URL,
    options?: {
      encoding?: string;
      withFileTypes?: boolean;
      recursive?: boolean;
    }
  ): string[] | Buffer[] | Dirent[] {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.readdirSync(path, { encoding: 'utf8', withFileTypes: false, recursive: false, ...options });
  }

  static readFileSync(
    path: string | Buffer | URL,
    options?: {
      encoding?: string | null;
      flag?: string;
    }
  ): string | Buffer {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.readFileSync(path, options);
  }

  static readlinkSync(path: string | Buffer | URL, options?: { encoding?: string }): string | Buffer {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.readlinkSync(path, options);
  }

  static readSync(fd: number, buffer: Buffer | ArrayBufferView | DataView, offset?: number, length?: number, position?: number | bigint): number {
    return NSCFS.readSync(fd, buffer, offset ?? 0, length ?? (buffer as any)?.length, position ?? -1);
  }

  static readvSync(fd: number, buffers: ArrayBufferView[], position?: number): number {
    return NSCFS.readvSync(fd, buffers, position ?? -1);
  }

  static realpathSync(path: string | Buffer | URL, options?: { encoding?: string }): string | Buffer {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.realpathSync(path, options);
  }

  // static realpathSync.native(path[, options])

  static renameSync(oldPath: string | Buffer | URL, newPath: string | Buffer | URL) {
    if (oldPath instanceof Buffer) {
      oldPath = oldPath.toString('utf-8');
    } else {
      oldPath = oldPath.toString();
    }

    if (newPath instanceof Buffer) {
      newPath = newPath.toString('utf-8');
    } else {
      newPath = newPath.toString();
    }

    return NSCFS.renameSync(oldPath, newPath);
  }

  static rmdirSync(
    path: string | Buffer | URL,
    options?: {
      maxRetries?: number;
      recursive?: boolean;
      retryDelay?: number;
    }
  ) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.rmdirSync(path, options);
  }

  static rmSync(
    path: string | Buffer | URL,
    options?: {
      force?: boolean;
      maxRetries?: number;
      recursive?: boolean;
      retryDelay?: number;
    }
  ) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.rmSync(path, options);
  }

  static statSync(
    path: string | Buffer | URL,
    options?: {
      bigint?: boolean;
      throwIfNoEntry?: boolean;
    }
  ): Stats {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.statSync(path, options);
  }

  static statfsSync(path: string | Buffer | URL, options?: { bigint?: boolean }) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }
    return NSCFS.statSync(path, options);
  }

  static symlinkSync(target: string | Buffer | URL, path: string | Buffer | URL, type: string | null) {
    if (target instanceof Buffer) {
      target = target.toString('utf-8');
    } else {
      target = target.toString();
    }

    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.symlinkSync(target, path, type);
  }

  static truncateSync(path: string | Buffer | URL, len: number) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }
    return NSCFS.truncateSync(path, len ?? 0);
  }

  static unlinkSync(path: string | Buffer | URL) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }
    return NSCFS.unlinkSync(path);
  }

  static utimesSync(path: string | Buffer | URL, atime: number | string | Date, mtime: number | string | Date) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }
    return NSCFS.utimesSync(path, atime, mtime);
  }

  static writeFileSync(
    file: string | Buffer | URL | number,
    data: string | Buffer | ArrayBufferView | DataView,
    options?: {
      encoding?: string | null;
      mode?: number;
      flag?: string;
    }
  ) {
    if (file instanceof Buffer) {
      file = file.toString('utf-8');
    } else if (typeof file !== 'number') {
      file = file.toString();
    }
    return NSCFS.writeFileSync(file, data, options);
  }

  static writeSync(fd: number, buffer: Buffer | ArrayBufferView | DataView, offset: number, length: number, position: number): number {
    return NSCFS.writeSync(fd, buffer, offset ?? 0, length ?? -1, position ?? 0);
  }

  static writevSync(fd: number, buffers: ArrayBufferView[], position?: number): number {
    return NSCFS.writevSync(fd, buffers, position ?? 0);
  }

  static open(path: string | Buffer | URL, flags: string | number, mode: string | number, callback: (error: Error, fd: number) => void) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else if (typeof path !== 'number') {
      path = path.toString();
    }
    NSCFS.open(path, flags ?? 'r', mode ?? 0o666, callback);
  }

  static watch(filename: string | Buffer | URL, options: { persistent?: boolean; recursive?: boolean; encoding?: string; signal?: any }, listener?: (eventType: string, filename: string | Buffer | null) => void): FSWatcher {
    if (filename instanceof Buffer) {
      filename = filename.toString('utf-8');
    } else if (typeof filename !== 'number') {
      filename = filename.toString();
    }

    const watcher = (NSCFS as any).watch(filename, { persistent: true, recursive: false, encoding: 'utf8', ...options }, (error, object: { eventType: string; filename: string }) => {
      if (listener) {
        listener(object?.eventType, object?.filename);
      }
    });

    const ret = new FSWatcher();
    ret._native = watcher;
    return ret;
  }

  static watchFile(filename: string | Buffer | URL, options: { bigint?: boolean; persistent?: boolean; interval?: number }, listener?: (current: any, previous: any) => void): StatWatcher {
    if (filename instanceof Buffer) {
      filename = filename.toString('utf-8');
    } else if (typeof filename !== 'number') {
      filename = filename.toString();
    }

    const watcher = (NSCFS as any).watchFile(filename, { bigint: false, persistent: true, encoding: 'utf8', interval: 5007, ...options }, (error, object: { current: any; previous: any }) => {
      if (listener) {
        listener(object?.current, object?.previous);
      }
    });
    const ret = new StatWatcher();
    ret._native = watcher;
    return ret;
  }
}

export const accessSync = Fs.accessSync;
export const appendFileSync = Fs.appendFileSync;
export const chmodSync = Fs.chmodSync;
export const chownSync = Fs.chownSync;
export const closeSync = Fs.closeSync;
export const copyFileSync = Fs.copyFileSync;
export const cpSync = Fs.cpSync;
export const existsSync = Fs.existsSync;
export const fchmodSync = Fs.fchmodSync;
export const fchownSync = Fs.fchownSync;
export const fdatasyncSync = Fs.fdatasyncSync;
export const fstatSync = Fs.fstatSync;
export const lchmodSync = Fs.lchmodSync;
export const lchownSync = Fs.lchownSync;
export const lutimesSync = Fs.lutimesSync;
export const linkSync = Fs.linkSync;
export const mkdirSync = Fs.mkdirSync;
export const mkdtempSync = Fs.mkdtempSync;
export const opendirSync = Fs.opendirSync;
export const openSync = Fs.openSync;
export const readdirSync = Fs.readdirSync;
export const readFileSync = Fs.readFileSync;
export const readlinkSync = Fs.readlinkSync;
export const readSync = Fs.readSync;
export const readvSync = Fs.readvSync;
export const realpathSync = Fs.realpathSync;
export const renameSync = Fs.renameSync;
export const rmdirSync = Fs.rmdirSync;
export const rmSync = Fs.rmSync;
export const statSync = Fs.statSync;
export const statfsSync = Fs.statfsSync;
export const symlinkSync = Fs.symlinkSync;
export const truncateSync = Fs.truncateSync;
export const unlinkSync = Fs.unlinkSync;
export const utimesSync = Fs.utimesSync;
export const writeFileSync = Fs.writeFileSync;
export const writeSync = Fs.writeSync;
export const writevSync = Fs.writevSync;

export const open = Fs.open;
export const watch = Fs.watch;
export const watchFile = Fs.watchFile;
