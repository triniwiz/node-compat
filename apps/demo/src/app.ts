import { Application } from '@nativescript/core';

require('@nativescript/node-core');
const b = require('buffer');

console.dir(global.NSCBuffer);

console.time('from');
const buffer = global.NSCBuffer.from('hello world', 'utf8');
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

console.log(global.NSCBuffer.from('fhqwhgads', 'utf8'));
// Prints: <Buffer 66 68 71 77 68 67 61 64 73>
console.log(global.NSCBuffer.from('fhqwhgads', 'utf16le'));
// Prints: <Buffer 66 00 68 00 71 00 77 00 68 00 67 00 61 00 64 00 73 00>

Application.run({ moduleName: 'app-root' });
