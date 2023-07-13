declare class NSCFS {
  static accessSync(path: string | NSCBuffer | URL, mode: number);
  static appendFileSync(
    path: string | NSCBuffer | URL | number,
    data: string | NSCBuffer,
    options?: {
      encoding: string | null;
      mode: number;
      flag: string;
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

  static lchmodSync(path: string | Buffer | URL, mode: number);

  static lchownSync(path: string | Buffer | URL, uid: number, gid: number);

  static lutimesSync(path: string | Buffer | URL, atime: number | string | Date, mtime: number | string | Date);

  static linkSync(existingPath: string | Buffer | URL, newPath: string | Buffer | URL);

  static statSync(path: string | Buffer | URL, options?: { bigint?: boolean });

  static mkdirSync(path: string | Buffer | URL, options?: { recursive?: boolean; mode: string | number });

  static mkdtempSync(prefix: string, options?: { encoding?: string });
}
