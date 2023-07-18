declare interface Dirent {
  isBlockDevice(): boolean;
  isCharacterDevice(): boolean;
  isDirectory(): boolean;
  isFIFO(): boolean;
  isFile(): boolean;
  isSocket(): boolean;
  isSymbolicLink(): boolean;
  readonly name: string | NSCBuffer;
  readonly path: string;
}

declare interface Dir {}

declare interface Stats {}

declare class NSCFS {
  static accessSync(path: string | NSCBuffer | URL, mode: number);
  static appendFileSync(
    path: string | NSCBuffer | URL | number,
    data: string | NSCBuffer,
    options?: {
      encoding?: string | null;
      mode?: number;
      flag?: string;
    }
  );

  static chmodSync(path: string | NSCBuffer | URL, mode: string | number);

  static chownSync(path: string | NSCBuffer | URL, uid: number, gid: number);

  static closeSync(fd: number);

  static copyFileSync(src: string | NSCBuffer | URL, dest: string | NSCBuffer | URL, mode?: number);

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
  );

  static existsSync(path: string | NSCBuffer | URL): boolean;

  static fchmodSync(fd: number, mode: string | number);

  static fchownSync(fd: number, uid: number, gid: number);

  static fdatasyncSync(fd: number);

  static fstatSync(fd: number, options?: { bigint?: boolean });

  static lchmodSync(path: string | NSCBuffer | URL, mode: number);

  static lchownSync(path: string | NSCBuffer | URL, uid: number, gid: number);

  static lutimesSync(path: string | NSCBuffer | URL, atime: number | string | Date, mtime: number | string | Date);

  static linkSync(existingPath: string | NSCBuffer | URL, newPath: string | NSCBuffer | URL);

  static statSync(path: string | NSCBuffer | URL, options?: { bigint?: boolean });

  static mkdirSync(path: string | NSCBuffer | URL, options?: { recursive?: boolean; mode: string | number });

  static mkdtempSync(prefix: string, options?: { encoding?: string });

  static opendirSync(path: string | NSCBuffer | URL, options?: { encoding?: string | null; bufferSize?: number; recursive?: boolean });

  static openSync(path: string | NSCBuffer | URL, flags: string | number, mode: number): number;

  static readdirSync(
    path: string | NSCBuffer | URL,
    options?: {
      encoding?: string;
      withFileTypes?: boolean;
      recursive?: boolean;
    }
  ): string[] | NSCBuffer[] | fs.Dirent[];

  static readFileSync(
    path: string | NSCBuffer | URL,
    options?: {
      encoding?: string | null;
      flag?: string;
    }
  ): string | NSCBuffer;

  static readlinkSync(path: string | NSCBuffer | URL, options?: { encoding?: string }): string | NSCBuffer;

  static readSync(fd: number, buffer: NSCBuffer | TypedArray | DataView, offset?: number, length?: number, position?: number | BigInt): number;

  static readvSync(fd: number, buffers: ArrayBufferView[], position?: number): number;

  static realpathSync(path: string | NSCBuffer | URL, options?: { encoding?: string }): string | NSCBuffer;

  // static realpathSync.native(path[, options])

  static renameSync(oldPath: string | NSCBuffer | URL, newPath: string | NSCBuffer | URL);

  static rmdirSync(
    path: string | NSCBuffer | URL,
    options?: {
      maxRetries?: number;
      recursive?: boolean;
      retryDelay?: number;
    }
  );

  static rmSync(
    path: string | NSCBuffer | URL,
    options?: {
      force?: boolean;
      maxRetries?: number;
      recursive?: boolean;
      retryDelay?: number;
    }
  );

  static statfsSync(path: string | NSCBuffer | URL, options?: { bigint?: boolean });

  static symlinkSync(target: string | NSCBuffer | URL, path: string | NSCBuffer | URL, type: string | null);

  static truncateSync(path: string | NSCBuffer | URL, len: number);

  static unlinkSync(path: string | NSCBuffer | URL);

  static utimesSync(path: string | NSCBuffer | URL, atime: number | string | Date, mtime: number | string | Date);
  static writeFileSync(
    file: string | NSCBuffer | URL | number,
    data: string | NSCBuffer | TypedArray | DataView,
    options?: {
      encoding?: string | null;
      mode?: number;
      flag?: string;
    }
  );

  static writeSync(fd: number, buffer: NSCBuffer | TypedArray | DataView, offset: number, length: number, position: number): number;

  static writevSync(fd: number, buffers: ArrayBufferView[], position?: number): number;
}
