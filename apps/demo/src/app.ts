import { Application, knownFolders, path } from '@nativescript/core';

require('@nativescript/node-core');

import * as fs from '@nativescript/node-fs';

import { Buffer } from '@nativescript/node-buffer';

const b = require('buffer');

console.time('from');
const buffer = Buffer.from('hello world', 'utf8');
console.timeEnd('from');

console.time('from');
const a = b.Buffer.from('hello world', 'utf8');
console.timeEnd('from');

// console.time('from');
// const c = global.NSCBuffer.from("hello world", 'utf8');
// console.timeEnd('from');

console.log(buffer.toString());

console.time('toString');
console.log(buffer.toString('hex'));
// Prints: 68656c6c6f20776f726c64
console.log(buffer.toString('base64'));
// Prints: aGVsbG8gd29ybGQ=
console.timeEnd('toString');

console.time('toString');
console.log(a.toString('hex'));
// Prints: 68656c6c6f20776f726c64
console.log(a.toString('base64'));
// Prints: aGVsbG8gd29ybGQ=
console.timeEnd('toString');

console.log(Buffer.from('fhqwhgads', 'utf8'));
// Prints: <Buffer 66 68 71 77 68 67 61 64 73>
console.log(Buffer.from('fhqwhgads', 'utf16le'));
// Prints: <Buffer 66 00 68 00 71 00 77 00 68 00 67 00 61 00 64 00 73 00>

// Create a single `Buffer` from a list of three `Buffer` instances.

const buf1 = Buffer.alloc(10);
const buf2 = Buffer.alloc(14);
const buf3 = Buffer.alloc(18);
const totalLength = buf1.length + buf2.length + buf3.length;

console.log(totalLength);
// Prints: 42

const bufA = Buffer.concat([buf1, buf2, buf3], totalLength);

console.log(bufA);
// Prints: <Buffer 00 00 00 00 ...>
console.log(bufA.length);
// Prints: 42

const str = 'Node.js';
const buf = Buffer.allocUnsafe(str.length);

console.time('buf[index]');
for (let i = 0; i < str.length; i++) {
  buf[i] = str.charCodeAt(i);
}
console.timeEnd('buf[index]');
console.log('hey', buf, buf[1]);
console.log('indexed', buf.toString('utf8'));
// Prints: Node.js

const buff = Buffer.from([0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff]);
console.log(buff.length);

console.log(buff.readBigUInt64BE(0));
// Prints: 4294967295n

const u16 = new Uint16Array([0, 0xffff]);
const copy = Buffer.copyBytesFrom(u16, 1, 1);
u16[1] = 0;
console.log('copy', copy.length); // 2
console.log('copy', copy[0]); // 255
console.log('copy', copy[1]); // 255

try {
  const img = path.join(knownFolders.currentApp().path + '/images/1057903.jpg');

  fs.accessSync(img);
} catch (error) {
  console.log(error);
}

console.dir();

// try {
//   const img = path.join(knownFolders.currentApp().path + '/images/1057903.jpg');
//   const buffer = global.NSCBuffer.alloc(2_000_000);
//   console.log(buffer.length);
//   const file = new java.io.File(img);
//   const pfd = android.os.ParcelFileDescriptor.open(file, android.os.ParcelFileDescriptor.MODE_READ_ONLY);
//   const fd = pfd.detachFd();
//   const read = global.NSCFS.readSync(fd, buffer);
//   console.log('read', read);

//   // console.log(buffer.toString());
// } catch (error) {
//   console.log(error);
// }
const hello = path.join(knownFolders.currentApp().path + '/documents/hello.txt');

// try {
//   const buffer = global.NSCBuffer.alloc(1000);
//   const file = new java.io.File(hello);
//   const pfd = android.os.ParcelFileDescriptor.open(file, android.os.ParcelFileDescriptor.MODE_READ_ONLY);
//   const fd = pfd.detachFd();
//   console.time('read');
//   const read = global.NSCFS.readSync(fd, buffer);
//   console.timeEnd('read');
//   console.log('read', read);
//   console.log(buffer.toString('utf8'));

//   // console.log(buffer.toString());
// } catch (error) {
//   console.log(error);
// }

if (global.isAndroid) {
  try {
    const file = new java.io.File(hello);
    // const pfd = android.os.ParcelFileDescriptor.open(file, android.os.ParcelFileDescriptor.MODE_READ_WRITE | android.os.ParcelFileDescriptor.MODE_APPEND);
    // const fd = pfd.detachFd();
  } catch (err) {
    console.log(err);
    /* Handle the error */
  }
}

if (global.isIOS) {
}

console.time('appendFileSync');
fs.appendFileSync(hello, ' data to append', { flag: 'a' });
console.timeEnd('appendFileSync');
console.log('The "data to append" was appended to file!');
// try {
//   const file = new java.io.File(hello);
//   const pfd = android.os.ParcelFileDescriptor.open(file, android.os.ParcelFileDescriptor.MODE_READ_ONLY);
//   const fd = pfd.detachFd();
//   console.time('fstatSync');
//   const stat = fs.fstatSync(fd, { bigint: true });
//   console.timeEnd('fstatSync');
//   console.log('fstatSync ', stat);
// } catch (err) {
//   console.log(err);
//   /* Handle the error */
// }

let absolutePath;
if (global.isAndroid) {
  const file = new java.io.File(hello);
  absolutePath = file.getAbsolutePath();
}

if (global.isIOS) {
  const file = NSURL.URLWithString(hello);
  absolutePath = file.absoluteString;
}

try {
  console.time('statSync');
  const stat = fs.statSync(absolutePath, { bigint: true });
  console.timeEnd('statSync');
  console.log(
    'statSync ',
    JSON.stringify(stat, (key, value) => {
      if (value && typeof value === 'object') {
        const ret = {};
        Object.keys(value).forEach((key) => {
          const item = value[key];
          if (typeof item === 'bigint') {
            ret[key] = item.toString();
          } else {
            ret[key] = item;
          }
        });

        return ret;
      }
      return value;
    })
  );
} catch (err) {
  console.log(err);
  /* Handle the error */
}

Application.run({ moduleName: 'app-root' });
