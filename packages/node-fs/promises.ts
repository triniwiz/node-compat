require('@nativescript/node-core');

import { Buffer } from '@nativescript/node-buffer';

export class FileHandle {
  _native: any;

  appendFile(
    data: string | Buffer,
    options?: {
      encoding?: string | null;
      mode?: number;
      flag?: string;
    }
  ) {
    if (data instanceof Buffer) {
      data = data.toString('utf-8');
    } else {
      data = data.toString();
    }
    return new Promise<void>((resolve, reject) => {
      this._native.appendFile(data, options, (error) => {
        if (error) {
          reject(error);
        } else {
          resolve();
        }
      });
    });
  }

  chmod(mode: string | number = 0) {
    return new Promise<void>((resolve, reject) => {
      this._native.chmod(mode, (error) => {
        if (error) {
          reject(error);
        } else {
          resolve();
        }
      });
    });
  }

  chown(uid: number, gid: number) {
    return new Promise<void>((resolve, reject) => {
      this._native.chown(uid, gid, (error) => {
        if (error) {
          reject(error);
        } else {
          resolve();
        }
      });
    });
  }

  close() {
    return new Promise<void>((resolve, reject) => {
      this._native.close((error) => {
        if (error) {
          reject(error);
        } else {
          resolve();
        }
      });
    });
  }

  datasync() {
    return new Promise<void>((resolve, reject) => {
      this._native.datasync((error) => {
        if (error) {
          reject(error);
        } else {
          resolve();
        }
      });
    });
  }

  get fd() {
    return this._native.fd;
  }

  read(buffer: Buffer | ArrayBufferView | DataView, offset?: number, length?: number, position?: number | bigint) {
    return new Promise((resolve, reject) => {
      this._native.read(buffer, offset ?? 0, length ?? (buffer as any)?.length, position ?? -1, (error, read) => {
        if (error) {
          reject(error);
        } else {
          resolve(read);
        }
      });
    });
  }

  readFile(options?: { encoding?: string | null; flag?: string }) {
    return new Promise((resolve, reject) => {
      this._native.readFile(options, (error, read) => {
        if (error) {
          reject(error);
        } else {
          resolve(read);
        }
      });
    });
  }

  readvSync(buffers: ArrayBufferView[] | Buffer[], position?: number) {
    return new Promise((resolve, reject) => {
      this._native.readvSync(buffers, position ?? -1, (error, read) => {
        if (error) {
          reject(error);
        } else {
          resolve(read);
        }
      });
    });
  }

  stat(options?: { bigint?: boolean }) {
    return new Promise((resolve, reject) => {
      this._native.stat(options ?? {}, (error, stat) => {
        if (error) {
          reject(error);
        } else {
          resolve(stat);
        }
      });
    });
  }

  sync() {
    return new Promise<void>((resolve, reject) => {
      this._native.sync((error) => {
        if (error) {
          reject(error);
        } else {
          resolve();
        }
      });
    });
  }

  truncateSync(len: number) {
    return new Promise<void>((resolve, reject) => {
      this._native.truncate(len ?? 0, (error) => {
        if (error) {
          reject(error);
        } else {
          resolve();
        }
      });
    });
  }

  utimesSync(atime: number | string | Date, mtime: number | string | Date) {
    return new Promise<void>((resolve, reject) => {
      this._native.utimes(atime, mtime, (error) => {
        if (error) {
          reject(error);
        } else {
          resolve();
        }
      });
    });
  }

  writeFileSync(
    data: string | Buffer | ArrayBufferView | DataView,
    options?: {
      encoding?: string | null;
      mode?: number;
      flag?: string;
    }
  ) {
    return new Promise<void>((resolve, reject) => {
      this._native.writeFile(data, options, (error) => {
        if (error) {
          reject(error);
        } else {
          resolve();
        }
      });
    });
  }

  write(buffer: Buffer | ArrayBufferView | DataView, offset: number, length: number, position: number) {
    return new Promise((resolve, reject) => {
      this._native.write(buffer, offset ?? 0, length ?? -1, position ?? 0, (error, wrote) => {
        if (error) {
          reject(error);
        } else {
          resolve(wrote);
        }
      });
    });
  }

  writevSync(buffers: ArrayBufferView[] | Buffer[], position?: number) {
    return new Promise((resolve, reject) => {
      this._native.writev(buffers, position ?? 0, (error, wrote) => {
        if (error) {
          reject(error);
        } else {
          resolve(wrote);
        }
      });
    });
  }
}

function openFunc(path: string | Buffer | URL, flags: string | number = 'r', mode: string | number = 0o666): Promise<FileHandle> {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else if (typeof path !== 'number') {
    path = path.toString();
  }
  return new Promise((resolve, reject) => {
    (NSCFS as any).openHandle(path, flags ?? 'r', mode ?? 0o666, (error: Error, handle) => {
      if (error) {
        reject(error);
      } else {
        const fsHandle = new FileHandle();
        fsHandle._native = handle;
        resolve(fsHandle);
      }
    });
  });
}

export function access(path: string | Buffer, mode = 0) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }
  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).access(path, mode, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function appendFile(
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

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).appendFile(path, data, options, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function chmod(path: string | Buffer | URL, mode: string | number = 0) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).chmod(path, mode, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function chown(path: string | Buffer | URL, uid: number, gid: number) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).chown(path, uid, gid, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function close(fd: number) {
  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).close(fd, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function copyFile(src: string | Buffer | URL, dest: string | Buffer | URL, mode = 0) {
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

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).copyFile(src, dest, mode, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function cp(
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
) {
  return new Promise<void>((resolve, reject) => {
    // todo
    reject('Unsupported');
  });
}

export function exists(path: string | Buffer | URL) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise<boolean>((resolve, reject) => {
    (NSCFS as any).exists(path, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export function fchmod(fd: number, mode: string | number = 0) {
  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).fchmod(fd, mode, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function fchown(fd: number, uid: number, gid: number) {
  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).fchown(fd, uid, gid, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function fdatasync(fd: number) {
  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).fdatasync(fd, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function fstat(fd: number, options?: { bigint?: boolean }) {
  return new Promise((resolve, reject) => {
    (NSCFS as any).fstat(fd, options, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export function lchmod(path: string | Buffer | URL, mode = 0) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).lchmod(path, mode, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function lchown(path: string | Buffer | URL, uid: number, gid: number) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).lchown(path, uid, gid, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function lutimes(path: string | Buffer | URL, atime: number | string | Date, mtime: number | string | Date) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).lutimes(path, atime, mtime, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function link(existingPath: string | Buffer | URL, newPath: string | Buffer | URL) {
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

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).link(existingPath, newPath, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function mkdir(path: string | Buffer | URL, options?: { recursive?: boolean; mode?: string | number }) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).mkdir(path, { recursive: false, mode: 0o777, ...options }, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function mkdtemp(prefix: string, options?: { encoding?: string }) {
  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).mkdtemp(prefix, options, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function opendir(path: string | Buffer | URL, options?: { encoding?: string | null; bufferSize?: number; recursive?: boolean }) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }
  return new Promise((resolve, reject) => {
    (NSCFS as any).opendir(path, options, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export function readdir(
  path: string | Buffer | URL,
  options?: {
    encoding?: string;
    withFileTypes?: boolean;
    recursive?: boolean;
  }
) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise<string[] | Buffer[] | Dirent[]>((resolve, reject) => {
    (NSCFS as any).readdir(path, { encoding: 'utf8', withFileTypes: false, recursive: false, ...options }, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export function readFile(
  path: string | Buffer | URL,
  options?: {
    encoding?: string | null;
    flag?: string;
  }
) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise<string | Buffer>((resolve, reject) => {
    (NSCFS as any).readFile(path, options, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export function readlink(path: string | Buffer | URL, options?: { encoding?: string }) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise<string | Buffer>((resolve, reject) => {
    (NSCFS as any).readlink(path, options, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export function read(fd: number, buffer: Buffer | ArrayBufferView | DataView, offset?: number, length?: number, position?: number | bigint) {
  return new Promise<number>((resolve, reject) => {
    (NSCFS as any).read(fd, buffer, offset ?? 0, length ?? (buffer as any)?.length, position ?? -1, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export function readv(fd: number, buffers: ArrayBufferView[], position?: number) {
  return new Promise<number>((resolve, reject) => {
    (NSCFS as any).readv(fd, buffers, position ?? -1, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export function realpath(path: string | Buffer | URL, options?: { encoding?: string }) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise<string | Buffer>((resolve, reject) => {
    (NSCFS as any).realpath(path, options, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

// export function  realpathSync.native(path[, options])

export function rename(oldPath: string | Buffer | URL, newPath: string | Buffer | URL) {
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

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).rename(oldPath, newPath, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function rmdir(
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

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).rmdir(path, options, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function rm(
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

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).rm(path, options, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function stat(
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

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).stat(path, options, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export function statfs(path: string | Buffer | URL, options?: { bigint?: boolean }) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise((resolve, reject) => {
    (NSCFS as any).statfs(path, options, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export function symlink(target: string | Buffer | URL, path: string | Buffer | URL, type: string | null) {
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

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).symlink(target, path, type, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export function truncate(path: string | Buffer | URL, len: number) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).truncate(path, len ?? 0, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function unlink(path: string | Buffer | URL) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).unlink(path, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function utimes(path: string | Buffer | URL, atime: number | string | Date, mtime: number | string | Date) {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else {
    path = path.toString();
  }

  return new Promise((resolve, reject) => {
    (NSCFS as any).utimes(path, atime, mtime, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export function writeFile(
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

  return new Promise<void>((resolve, reject) => {
    (NSCFS as any).writeFile(file, data, options, (error) => {
      if (error) {
        reject(error);
      } else {
        resolve();
      }
    });
  });
}

export function write(fd: number, buffer: Buffer | ArrayBufferView | DataView, offset: number, length: number, position: number) {
  return new Promise<number>((resolve, reject) => {
    (NSCFS as any).write(fd, buffer, offset ?? 0, length ?? -1, position ?? -1, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export function writev(fd: number, buffers: ArrayBufferView[], position?: number) {
  return new Promise<number>((resolve, reject) => {
    (NSCFS as any).writev(fd, buffers, position ?? -1, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export const open = openFunc;
