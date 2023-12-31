package org.nativescript.node_compat.fs

class FileStat {
  var dev: Long = 0
    internal set
  var ino: Long = 0
    internal set
  var mode = 0
    internal set
  var nlink: Long = 0
    internal set
  var uid = 0
    internal set
  var gid = 0
    internal set
  var rdev: Long = 0
    internal set
  var size: Long = 0
    internal set
  var blksize: Long = 0
    internal set
  var blocks: Long = 0
    internal set
  var atimeMs = 0.0
    internal set
  var mtimeMs = 0.0
    internal set
  var ctimeMs = 0.0
    internal set
  var birthtimeMs = 0.0
    internal set
  var birthtime = 0.0
    internal set
  var atime = 0.0
    internal set
  var mtime = 0.0
    internal set
  var ctime = 0.0
    internal set
  var isBlockDevice = false
    internal set
  var isCharacterDevice = false
    internal set
  var isDirectory = false
    internal set
  var isFIFO = false
    internal set
  var isFile = false
    internal set
  var isSocket = false
    internal set
  var isSymbolicLink = false
    internal set
}
