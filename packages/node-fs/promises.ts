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

function openFunc(path: string | Buffer | URL, flags: string | number, mode: string | number): Promise<FileHandle> {
  if (path instanceof Buffer) {
    path = path.toString('utf-8');
  } else if (typeof path !== 'number') {
    path = path.toString();
  }
  return new Promise((resolve, reject) => {
    NSCFS.openHandle(path, flags ?? 'r', mode ?? 0o666, (error: Error, handle) => {
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

export const open = openFunc;
