import { Buffer } from '@nativescript/node-buffer';

export interface IFileStat {
  dev: number | BigInt;
  ino: number | BigInt;
  mode: number;
  nlink: number | BigInt;
  uid: number;
  gid: number;
  rdev: number | BigInt;
  size: number | BigInt;
  blksize: number | BigInt;
  blocks: number | BigInt;
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

export function parseFlags(flags: string) {
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

class Fs {
  static accessSync(path: string | Buffer, mode: number = 0) {
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

  static copyFileSync(src: string | Buffer | URL, dest: string | Buffer | URL, mode: number = 0) {
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

  static lchmodSync(path: string | Buffer | URL, mode: number = 0) {
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

  static statSync(path: string | Buffer | URL, options?: { bigint?: boolean }) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.statSync(path, options);
  }

  static mkdirSync(path: string | Buffer | URL, options?: { recursive?: boolean; mode: string | number }) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.mkdirSync(path, options);
  }

  static mkdtempSync(prefix: string, options?: { encoding?: string }) {
    if (path instanceof Buffer) {
      path = path.toString('utf-8');
    } else {
      path = path.toString();
    }

    return NSCFS.mkdtempSync(prefix, options);
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

export const statSync = Fs.statSync;

export const mkdirSync = Fs.mkdirSync;

export const mkdtempSync = Fs.mkdtempSync;
