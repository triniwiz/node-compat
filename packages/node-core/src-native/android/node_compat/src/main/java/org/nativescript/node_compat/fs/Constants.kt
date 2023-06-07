package org.nativescript.node_compat.fs

object Constants {
  init {
    System.loadLibrary("nodeandroid")
  }

  /**
   * Flag indicating to open a file for read-only access.
   */
  var O_RDONLY = 0

  /**
   * Flag indicating to open a file for write-only access.
   */
  var O_WRONLY = 0

  /**
   * Flag indicating to open a file for read-write access.
   */
  var O_RDWR = 0

  /**
   * Flag indicating to create the file if it does not already exist.
   */
  var O_CREAT = 0

  /**
   * Flag indicating that opening a file should fail if the O_CREAT flag is set and the file already exists.
   */
  var O_EXCL = 0

  /**
   * Flag indicating that if path identifies a terminal device, opening the path shall not cause that terminal to become the controlling terminal for the process (if the process does not already have one).
   */
  var O_NOCTTY = 0

  /***
   * Flag indicating that if the file exists and is a regular file, and the file is opened successfully for write access, its length shall be truncated to zero.
   */
  var O_TRUNC = 0

  /**
   * Flag indicating that data will be appended to the end of the file.
   */
  var O_APPEND = 0

  /**
   * Flag indicating that the open should fail if the path is not a directory.
   */
  var O_DIRECTORY = 0

  /**
   * Flag indicating reading accesses to the file system will no longer result in an update to the atime information associated with the file. This flag is available on Linux operating systems only.
   */
  var O_NOATIME = 0

  /**
   * Flag indicating that the open should fail if the path is a symbolic link.
   */
  var O_NOFOLLOW = 0

  /**
   * Flag indicating that the file is opened for synchronized I/O with write operations waiting for file integrity.
   */
  var O_SYNC = 0

  /**
   * Flag indicating that the file is opened for synchronized I/O with write operations waiting for data integrity.
   */
  var O_DSYNC = 0

  /**
   * Flag indicating to open the symbolic link itself rather than the resource it is pointing to.
   */
  var O_SYMLINK = 0

  /**
   * When set, an attempt will be made to minimize caching effects of file I/O.
   */
  var O_DIRECT = 0

  /**
   * Flag indicating to open the file in nonblocking mode when possible.
   */
  var O_NONBLOCK = 0

  /**
   * Flag indicating that the file is visible to the calling process. This is useful for determining if a file exists, but says nothing about rwx permissions. Default if no mode is specified.
   */
  var F_OK = 0

  /**
   * Flag indicating that the file can be read by the calling process.
   */
  var R_OK = 0

  /**
   * Flag indicating that the file can be written by the calling process.
   */
  var W_OK = 0

  /**
   * Flag indicating that the file can be executed by the calling process.
   */
  var X_OK = 0

  /**
   * If present, the copy operation will fail with an error if the destination path already exists.
   */
  var COPYFILE_EXCL = 0

  /**
   * If present, the copy operation will attempt to create a copy-on-write reflink. If the underlying platform does not support copy-on-write, then a fallback copy mechanism is used.
   */
  var COPYFILE_FICLONE = 0

  /**
   * If present, the copy operation will attempt to create a copy-on-write reflink. If the underlying platform does not support copy-on-write, then the operation will fail with an error.
   */
  var COPYFILE_FICLONE_FORCE = 0

  /**
   * Bit mask used to extract the file type code.
   */
  var S_IFMT = 0

  /**
   * File type constant for a regular file.
   */
  var S_IFREG = 0

  /**
   * File type constant for a directory.
   */
  var S_IFDIR = 0

  /**
   * File type constant for a character-oriented device file.
   */
  var S_IFCHR = 0

  /**
   * File type constant for a block-oriented device file.
   */
  var S_IFBLK = 0

  /**
   * File type constant for a FIFO/pipe.
   */
  var S_IFIFO = 0

  /**
   * File type constant for a symbolic link.
   */
  var S_IFLNK = 0

  /**
   * File type constant for a socket.
   */
  var S_IFSOCK = 0

  /**
   * File mode indicating readable, writable, and executable by owner.
   */
  var S_IRWXU = 0

  /**
   * File mode indicating readable by owner.
   */
  var S_IRUSR = 0

  /**
   * File mode indicating writable by owner.
   */
  var S_IWUSR = 0

  /**
   * File mode indicating executable by owner.
   */
  var S_IXUSR = 0

  /**
   * File mode indicating readable, writable, and executable by group.
   */
  var S_IRWXG = 0

  /**
   * File mode indicating readable by group.
   */
  var S_IRGRP = 0

  /**
   * File mode indicating writable by group.
   */
  var S_IWGRP = 0

  /**
   * File mode indicating executable by group.
   */
  var S_IXGRP = 0

  /**
   * File mode indicating readable, writable, and executable by others.
   */
  var S_IRWXO = 0

  /**
   * File mode indicating readable by others.
   */
  var S_IROTH = 0

  /**
   * File mode indicating writable by others.
   */
  var S_IWOTH = 0

  /**
   * File mode indicating executable by others.
   */
  var S_IXOTH = 0
}
