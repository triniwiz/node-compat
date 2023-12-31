use jni::objects::{JByteArray, JByteBuffer, JClass, JLongArray, JObject, JString, JValue, ReleaseMode};
use jni::sys::{jboolean, jbyteArray, jint, jlong, jobject, jobjectArray, JNI_TRUE, jarray, jsize, jstring};
use jni::JNIEnv;
use libc::{c_int, c_uint, c_ushort};
use node_buffer::{Buffer, StringEncoding};
use node_fs::prelude::{FsEncoding, FsEncodingType, handle_meta};
use node_fs::sync::ReaddirResult;
use crate::fs::file_dir::build_dir;
use crate::fs::file_dirent::{build_dirent, build_dirents, build_dirents_paths};
use crate::fs::file_stat::build_stat;
use crate::fs::{FILE_DIRENT_CLASS, FILE_SYSTEM_CLASS, STRING_CLASS};
use crate::fs::prelude::*;

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAccessSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    mode: jint,
) {
    let path = get_str(&mut env, &path, "");
    let result = node_fs::sync::access(&path, mode);
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAppendFileSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: jlong,
) {
    let data = unsafe { &*(buffer as *mut Buffer) };

    let result = node_fs::sync::append_file_with_buffer(fd, data);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAppendFileWithBytesSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    bytes: JByteArray,
) {
    let data = unsafe {
        env
            .get_array_elements_critical(&bytes, ReleaseMode::NoCopyBack)
            .unwrap()
    };

    let bytes = unsafe {
        std::slice::from_raw_parts_mut(
            data.as_ptr() as *mut u8,
            data.len() as usize,
        )
    };
    let result = node_fs::sync::append_file_with_bytes(fd, bytes);

    if let Err(error) = result {
        drop(data);
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAppendFileWithStringSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    data: JString,
) {
    let data = get_str(&mut env, &data, "");
    let result = node_fs::sync::append_file_with_str(fd, &data);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAppendFileWithPathBytesSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    bytes: JByteArray,
    mode: jint,
    flags: jint,
) {
    let path = get_str(&mut env, &path, "");
    let bytes = unsafe {
        env
            .get_array_elements_critical(&bytes, ReleaseMode::NoCopyBack)
            .unwrap()
    };

    let data = unsafe {
        std::slice::from_raw_parts_mut(
            data.as_ptr() as *mut u8,
            data.size().unwrap_or_default() as usize,
        )
    };

    let result = node_fs::sync::append_file_with_path_bytes(&path, data, mode, flags);

    if let Err(error) = result {
        drop(bytes);
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAppendFileWithPathStringSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    data: JString,
    mode: jint,
    flags: jint,
) {
    let path = get_str(&mut env, &path, "");
    let data = get_str(&mut env, &data, "");
    let result = node_fs::sync::append_file_with_path_str(path.as_ref(), data.as_ref(), mode, flags);
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeChmodSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    mode: jint,
) {
    let path = get_str(&mut env, &path, "");
    let result = node_fs::sync::chmod(path.as_ref(), mode as u32);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeChownSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    uid: jint,
    gid: jint,
) {
    let path = get_str(&mut env, &path, "");
    let result = node_fs::sync::chown(path.as_ref(), uid as u32, gid as u32);
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeCloseSync(
    _: JNIEnv,
    _: JClass,
    fd: jint,
) {
    node_fs::sync::close_fd(fd);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeCopyFileSync(
    mut env: JNIEnv,
    _: JClass,
    src: JString,
    dest: JString,
    flags: jint,
) {
    let src = get_str(&mut env, &src, "");
    let dest = get_str(&mut env, &dest, "");
    let result = node_fs::sync::copy_file(&src, &dest, flags as u32);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeCopySync(
    mut env: JNIEnv,
    _: JClass,
    src: JString,
    dest: JString,
    _flags: jint,
) {
    let _src = get_str(&mut env, &src, "");
    let _dest = get_str(&mut env, &dest, "");
    todo!()
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeExistsSync(
    mut env: JNIEnv,
    _: JClass,
    src: JString,
) -> jboolean {
    let src = get_str(&mut env, &src, "");
    node_fs::sync::exists(&src).into()
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeFchmodSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    mode: jint,
) {
    let result = node_fs::sync::fchmod(fd, mode as c_ushort);
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeFchownSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    uid: jint,
    gid: jint,
) {
    let result = node_fs::sync::fchown(fd, uid as c_uint, gid as c_uint);
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeFdatasyncSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
) {
    let result = node_fs::sync::fdatasync(fd);
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeFstatSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
) -> jobject {
    match node_fs::sync::fstat(fd) {
        Ok(stat) => build_stat(&mut env, handle_meta(&stat)).into_inner(),
        Err(error) => {
            let _ = env.throw(error.to_string());
            return JObject::null().into_inner();
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeFsyncSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
) {
    let result = node_fs::sync::fsync(fd);
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeFtruncateSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    len: jlong,
) {
    let result = node_fs::sync::ftruncate(fd, len.try_into().unwrap());
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeFutimesSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    atime: jlong,
    mtime: jlong,
) {
    let result = node_fs::sync::futimes(fd, atime.try_into().unwrap(), mtime.try_into().unwrap());
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeLchmodSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    mode: jint,
) {
    let path = get_str(&mut env, &path, "");
    let result = node_fs::sync::lchmod(&path, mode as c_ushort);
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeLchownSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    uid: jint,
    gid: jint,
) {
    let path = get_str(&mut env, &path, "");
    let result = node_fs::sync::lchown(&path, uid as c_uint, gid as c_uint);
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeLutimesSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    atime: jlong,
    mtime: jlong,
) {
    let path = get_str(&mut env, &path, "");
    let result = node_fs::sync::lutimes(&path, atime.try_into().unwrap(), mtime.try_into().unwrap());
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeLinkSync(
    mut env: JNIEnv,
    _: JClass,
    existing_path: JString,
    new_path: JString,
) {
    let existing_path = get_str(&mut env, &existing_path, "");
    let new_path = get_str(&mut env, &new_path, "");
    let result = node_fs::sync::link(&existing_path, &new_path);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeLstatSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
) -> jobject {
    let path = get_str(&mut env, &path, "");
    match node_fs::sync::lstat(&path) {
        Ok(stat) => build_stat(&mut env, handle_meta(&stat)).into_inner(),
        Err(error) => {
            let _ = env.throw(error.to_string());
            return JObject::null().into_inner();
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeMkdirSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    mode: c_int,
    recursive: jboolean,
) {
    let path = get_str(&mut env, &path, "");
    let result = node_fs::sync::mkdir(&path, mode as u32, recursive == JNI_TRUE);
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeMkdtempSync(
    mut env: JNIEnv,
    _: JClass,
    prefix: JString,
) -> jobject {
    let prefix = get_str(&mut env, &prefix, "");
    return match node_fs::sync::mkdtemp(&prefix) {
        Ok(result) => env.new_string(result).unwrap().into_inner(),
        Err(error) => {
            let _ = env.throw(error.to_string());
            JObject::null().into_inner()
        }
    };
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeOpenSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    flags: jint,
    mode: jint,
) -> jint {
    let path = get_str(&mut env, &path, "");
    match node_fs::sync::open(&path, flags, mode) {
        Ok(fd) => fd.into(),
        Err(error) => {
            let _ = env.throw(error.to_string());
            return 0;
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeOpenDirSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
) -> jobject {
    let path = get_str(&mut env, &path, "");
    return match node_fs::sync::opendir(&path) {
        Ok(dir) => build_dir(&mut env, dir).into_inner(),
        Err(error) => {
            let _ = env.throw(error.to_string());
            JObject::null().into_inner()
        }
    };
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: jlong,
    offset: jlong,
    length: jlong,
    position: jlong,
) -> jlong {
    let buffer = unsafe { &mut *(buffer as *mut Buffer) };
    return match node_fs::sync::read(
        fd,
        buffer.buffer_mut(),
        offset.try_into().unwrap(),
        length.try_into().unwrap(),
        position.try_into().unwrap(),
    ) {
        Ok(read) => (read as jlong).into(),
        Err(error) => {
            let _ = env.throw(error.to_string());
            0
        }
    };
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadWithBytesSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: JByteArray,
    offset: jlong,
    length: jlong,
    position: jlong,
) -> jlong {
    let data = unsafe {
        env
            .get_array_elements_critical(&buffer, ReleaseMode::CopyBack)
            .unwrap()
    };

    let bytes = unsafe {
        std::slice::from_raw_parts_mut(
            data.as_ptr() as *mut u8,
            data.len(),
        )
    };
    match node_fs::sync::read(
        fd,
        bytes,
        offset.try_into().unwrap(),
        length.try_into().unwrap(),
        position.try_into().unwrap(),
    ) {
        Ok(read) => {
            // force drop of array to enable jni usage
            drop(data);
            read as jlong
        }
        Err(error) => {
            // force drop of array to enable jni usage
            drop(data);
            let _ = env.throw(error.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadWithBufferSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: JByteBuffer,
    offset: jlong,
    length: jlong,
    position: jlong,
) -> jlong {
    match (env.get_direct_buffer_address(&buffer), env.get_direct_buffer_capacity(&buffer)) {
        (Ok(data), Ok(size)) => {
            let bytes = unsafe { std::slice::from_raw_parts_mut(data, size) };
            return match node_fs::sync::read(
                fd,
                bytes,
                offset.try_into().unwrap(),
                length.try_into().unwrap(),
                position.try_into().unwrap(),
            ) {
                Ok(read) => (read as jlong).into(),
                Err(error) => {
                    let _ = env.throw(error.to_string());
                    0
                }
            };
        }
        _ => 0
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReaddir(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    with_file_types: jboolean,
    encoding: jint,
) -> jarray {
    match FsEncodingType::try_from(encoding) {
        Ok(encoding) => {
            let path = get_str(&mut env, &path, "");
            match node_fs::sync::readdir(&path, with_file_types == JNI_TRUE, encoding) {
                Ok(success) => {
                    if with_file_types {
                        let dirent = find_class(FILE_DIRENT_CLASS).unwrap();
                        let array = env.new_object_array(
                            success.len() as jsize,
                            dirent,
                            JObject::null(),
                        ).unwrap();

                        for (i, result) in success.into_iter().enumerate() {
                            let dirent = match result {
                                ReaddirResult::Type(result) => {
                                    build_dirent(&mut env, result)
                                }
                                _ => {
                                    unreachable!()
                                }
                            };
                            let _ = env.set_object_array_element(&array, i as jsize, dirent);
                        }

                        return array.into_raw();
                    }

                    return match encoding {
                        FsEncodingType::Utf8 | FsEncodingType::Utf16le | FsEncodingType::Ucs2 | FsEncodingType::Latin1 | FsEncodingType::Ascii => {
                            let string = find_class(STRING_CLASS).unwrap();
                            let array = env.new_object_array(
                                success.len() as jsize,
                                string,
                                JObject::null(),
                            ).unwrap();

                            for (i, result) in success.iter().enumerate() {
                                let result = match result {
                                    ReaddirResult::String(result) => {
                                        env.new_string(result).unwrap()
                                    }
                                    _ => {
                                        unreachable!()
                                    }
                                };

                                let _ = env.set_object_array_element(&array, i as jsize, result);
                            }

                            array.into_raw()
                        }

                        FsEncodingType::Buffer => {
                            let array = env.new_long_array(
                                success.len() as jsize,
                            ).unwrap();

                            let result = success.into_iter()
                                .map(|result| {
                                    match result {
                                        ReaddirResult::Buffer(result) => {
                                            Box::into_raw(
                                                Box::new(result)
                                            ) as jlong
                                        }
                                        _ => {
                                            unreachable!()
                                        }
                                    }
                                })
                                .collect::<Vec<jlong>>();

                            let _ = env.set_long_array_region(&array, i as jsize, result.as_slice());

                            array.into_raw()
                        }
                    };
                }
                Err(error) => {
                    let _ = env.throw(error.to_string());
                    build_dirents(&mut env, vec![])
                }
            }
        }
        Err(error) => {
            let _ = env.throw(error);
        }
    }
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadFileSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    encoding: jint,
    flags: jint,
) -> jobject {
    let path = get_str(&mut env, &path, "");
    match FsEncodingType::try_from(encoding) {
        Ok(encoding) => {
            match node_fs::sync::read_file(&path, encoding, flags) {
                Ok(mut buf) => {
                    match buf {
                        FsEncoding::String(string) => {
                            env.new_string(string.to_string_lossy()).unwrap().into_raw()
                        }
                        FsEncoding::Buffer(buffer) => {
                            return JValue::Long(Box::into_raw(
                                Box::new(
                                    buffer
                                )
                            ) as jlong).into();
                        }
                    }
                }
                Err(error) => {
                    let _ = env.throw(error.to_string());
                    JObject::null().into_inner()
                }
            }
        }
        Err(error) => {
            let _ = env.throw(error);
            JObject::null().into_inner()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadFileWithFdSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    encoding: jint,
    flags: jint,
) -> jobject {
    match FsEncodingType::try_from(encoding) {
        Ok(encoding) => {
            match node_fs::sync::read_file_with_fd(fd, encoding, flags) {
                Ok(mut buf) => {
                    match buf {
                        FsEncoding::String(string) => {
                            env.new_string(string.to_string_lossy()).unwrap().into_raw()
                        }
                        FsEncoding::Buffer(buffer) => {
                            return JValue::Long(Box::into_raw(
                                Box::new(
                                    buffer
                                )
                            ) as jlong).into();
                        }
                    }
                }
                Err(error) => {
                    let _ = env.throw(error.to_string());
                    JObject::null().into_inner()
                }
            }
        }
        Err(error) => {
            let _ = env.throw(error);
            JObject::null().into_inner()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadLinkSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    encoding: jint,
) -> jobject {
    match FsEncodingType::try_from(encoding) {
        Ok(encoding) => {
            let path = get_str(&mut env, &path, "");
            match node_fs::sync::read_link(&path, encoding) {
                Ok(link) => {
                    match link {
                        FsEncoding::String(string) => {
                            env.new_string(string.to_string_lossy()).unwrap().into_raw()
                        }
                        FsEncoding::Buffer(buffer) => {
                            return JValue::Long(Box::into_raw(
                                Box::new(
                                    buffer
                                )
                            ) as jlong).into();
                        }
                    }
                }
                Err(error) => {
                    let _ = env.throw(error.to_string());
                    JObject::null().into_inner()
                }
            }
        }
        Err(error) => {
            let _ = env.throw(error);
            JObject::null().into_inner()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadvSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    buffers: JLongArray,
    position: jlong,
) -> jlong {
    match unsafe { env.get_array_elements_critical(&buffers, ReleaseMode::CopyBack) } {
        Ok(array) => {
            let array = unsafe { std::slice::from_raw_parts_mut(array.as_ptr() as *mut i64, array.len()) };

            let mut buf = array.iter()
                .map(|value| unsafe { (&*(*i as *mut Buffer)).clone() })
                .collect::<Vec<Buffer>>();

            match node_fs::sync::readv(fd, buf.as_mut_slice(), position.try_into().unwrap()) {
                Ok(read) => read as jlong,
                Err(error) => {
                    let _ = env.throw(error.to_string());
                    0
                }
            }
        }
        Err(error) => {
            let _ = env.throw(error.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeRealPathSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
) -> jobject {
    let path = get_str(&mut env, &path, "");
    match node_fs::sync::real_path(&path) {
        Ok(buf) => env.new_string(buf.to_string_lossy()).unwrap().into_inner(),
        Err(error) => {
            let _ = env.throw(error.to_string());
            JObject::null().into_inner()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeRenameSync(
    mut env: JNIEnv,
    _: JClass,
    old_path: JString,
    new_path: JString,
) {
    let old_path = get_str(&mut env, &old_path, "");
    let new_path = get_str(&mut env, &new_path, "");
    let result = node_fs::sync::rename(&old_path, &new_path);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeRmdirSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    max_retries: jint,
    recursive: jboolean,
    retry_delay: jlong,
) {
    let path = get_str(&mut env, &path, "");
    let result = node_fs::sync::rmdir(
        &path,
        max_retries,
        recursive == JNI_TRUE,
        retry_delay.try_into().unwrap(),
    );
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeRmSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    max_retries: jint,
    recursive: jboolean,
    retry_delay: jlong,
) {
    let path = get_str(&mut env, &path, "");
    let result = node_fs::sync::rm(
        &path,
        max_retries,
        recursive == JNI_TRUE,
        retry_delay.try_into().unwrap(),
    );

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeStatSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    throw_if_no_entry: jboolean,
) -> jobject {
    let path = get_str(&mut env, &path, "");
    let meta = node_fs::sync::stat(&path);
    match meta {
        Ok(stat) => build_stat(&mut env, handle_meta(&stat)).into_inner(),
        Err(error) => {
            if throw_if_no_entry == JNI_TRUE && error.kind() == std::io::ErrorKind::NotFound {
                let _ = env.throw(error.to_string());
            }
            JObject::null().into_inner()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeSymlinkSync(
    mut env: JNIEnv,
    _: JClass,
    target: JString,
    path: JString,
    type_: JString,
) {
    let target = get_str(&mut env, &target, "");
    let path = get_str(&mut env, &path, "");
    let type_ = get_str(&mut env, &type_, "");
    let result = node_fs::sync::symlink(&target, &path, &type_);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeTruncateSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    len: jlong,
) {
    let path = get_str(&mut env, &path, "");
    let result = node_fs::sync::truncate(&path, len.try_into().unwrap());

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeUnlinkSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
) {
    let path = get_str(&mut env, &path, "");
    let result = node_fs::sync::unlink(&path);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

// todo watch

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeUtimesSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    atime: jlong,
    mtime: jlong,
) {
    let path = get_str(&mut env, &path, "");
    let result = node_fs::sync::utimes(&path, atime.try_into().unwrap(), mtime.try_into().unwrap());

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: jlong,
    offset: jlong,
    length: jlong,
    position: jlong,
) -> jlong {
    let buffer = unsafe { &mut *(buffer as *mut Buffer) };
    match node_fs::sync::write(
        fd,
        buffer.buffer_mut(),
        offset.try_into().unwrap(),
        length.try_into().unwrap(),
        position.try_into().unwrap(),
    ) {
        Ok(wrote) => wrote as jlong,
        Err(error) => {
            let _ = env.throw(error.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteBufferSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: JByteBuffer,
    offset: jlong,
    length: jlong,
    position: jlong,
) -> jlong {
    match (env.get_direct_buffer_address(&buffer), env.get_direct_buffer_capacity(&buffer)) {
        (Ok(data), Ok(size)) => {
            let bytes = unsafe { std::slice::from_raw_parts_mut(data, size) };
            match node_fs::sync::write(
                fd,
                bytes,
                offset.try_into().unwrap(),
                length.try_into().unwrap(),
                position.try_into().unwrap(),
            ) {
                Ok(wrote) => wrote as jlong,
                Err(error) => {
                    let _ = env.throw(error.to_string());
                    0
                }
            }
        }
        (Err(error), _) => {
            let _ = env.throw(error.to_string());
            0
        }
        _ => { 0 }
    }
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteBytesSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    data: JByteArray,
    offset: jlong,
    length: jlong,
    position: jlong,
) -> jlong {
    let data = unsafe {
        env
            .get_array_elements_critical(&data, ReleaseMode::NoCopyBack)
            .unwrap()
    };

    let bytes = unsafe {
        std::slice::from_raw_parts_mut(
            data.as_ptr() as *mut u8,
            data.len(),
        )
    };
    match node_fs::sync::write(
        fd,
        bytes,
        offset.try_into().unwrap(),
        length.try_into().unwrap(),
        position.try_into().unwrap(),
    ) {
        Ok(wrote) => {
            // force drop of array to enable jni usage
            drop(data);
            wrote as jlong
        }
        Err(error) => {
            // force drop of array to enable jni usage
            drop(data);
            let _ = env.throw(error.to_string());
            0_i64
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteStringSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    string: JString,
    encoding: jint,
    position: jlong,
) -> jlong {
    match StringEncoding::try_from(encoding) {
        Ok(encoding) => {
            let string = get_str(&mut env, &string, "");
            match node_fs::sync::write_string(fd, &string, encoding, position.try_into().unwrap()) {
                Ok(wrote) => wrote as jlong,
                Err(error) => {
                    let _ = env.throw(error.to_string());
                    0
                }
            }
        }
        Err(error) => {
            let _ = env.throw(error);
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: jlong,
) {
    let data = unsafe { &*(buffer as *mut Buffer) };

    let bytes = data.buffer();

    let result = node_fs::sync::write_file_with_bytes(fd, bytes);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithStringSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    data: JString,
    encoding: jint,
) {
    match StringEncoding::try_from(encoding) {
        Ok(encoding) => {
            let data = get_str(&mut env, &data, "");

            let result = node_fs::sync::write_file_with_str(fd, &data, encoding);

            if let Err(error) = result {
                let _ = env.throw(error.to_string());
            }
        }
        Err(error) => {
            let _ = env.throw(error);
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithBytesSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    data: JByteArray,
) {
    let data = unsafe {
        env
            .get_array_elements_critical(&data, ReleaseMode::NoCopyBack)
            .unwrap()
    };

    let bytes = unsafe {
        std::slice::from_raw_parts_mut(
            data.as_ptr() as *mut u8,
            data.len(),
        )
    };
    let result = node_fs::sync::write_file_with_bytes(fd, bytes);

    if let Err(error) = result {
        drop(data);
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithBufferSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    data: JByteBuffer,
) {
    match (env.get_direct_buffer_address(&data), env.get_direct_buffer_capacity(&data)) {
        (Ok(data), Ok(size)) => {
            let bytes = unsafe { std::slice::from_raw_parts_mut(data, size) };
            let result = node_fs::sync::write_file_with_bytes(fd, bytes);

            if let Err(error) = result {
                let _ = env.throw(error.to_string());
            }
        }
        (Err(error), _) => {
            let _ = env.throw(error.to_string());
        }
        _ => {}
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithStringFromPathSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    data: JString,
    encoding: jint,
    mode: c_int,
    flag: c_int,
) {
    match StringEncoding::try_from(encoding) {
        Ok(encoding) => {
            let path = get_str(&mut env, &path, "");
            let data = get_str(&mut env, &data, "");

            let result = node_fs::sync::write_file_with_str_from_path(&path, &data, encoding, mode, flag);

            if let Err(error) = result {
                let _ = env.throw(error.to_string());
            }
        }
        Err(error) => {
            let _ = env.throw(error);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithBytesFromPathSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    data: JByteArray,
    mode: c_int,
    flag: c_int,
) {
    let path = get_str(&mut env, &path, "");

    let data = unsafe {
        env
            .get_array_elements_critical(&data, ReleaseMode::NoCopyBack)
            .unwrap()
    };
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(
            data.as_ptr() as *mut u8,
            data.len(),
        )
    };
    let result = node_fs::sync::write_file_with_bytes_from_path(&path, bytes, mode, flag);

    if let Err(error) = result {
        drop(data);
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithBufferFromPathSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    data: JByteBuffer,
    mode: c_int,
    flag: c_int,
) {
    match (env.get_direct_buffer_address(&data), env.get_direct_buffer_capacity(&data)) {
        (Ok(data), Ok(size)) => {
            let path = get_str(&mut env, &path, "");

            let bytes = unsafe { std::slice::from_raw_parts_mut(data, size) };
            let result = node_fs::sync::write_file_with_bytes_from_path(&path, bytes, mode, flag);

            if let Err(error) = result {
                let _ = env.throw(error.to_string());
            }
        }
        (Err(error), _) => {
            let _ = env.throw(error.to_string());
        }
        _ => {}
    }
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileFromPathSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    data: jlong,
    mode: c_int,
    flag: c_int,
) {
    let path = get_str(&mut env, &path, "");

    let bytes = unsafe { &*(data as *mut Buffer) };
    let result = node_fs::sync::write_file_with_buffer_from_path(&path, bytes, mode, flag);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWritevSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    buffers: JLongArray,
    position: jlong,
) -> jlong {
    match env.get_array_elements_critical(&buffers, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let slice = std::slice::from_raw_parts_mut(array.as_ptr(), array.len());

            let slice = slice.iter()
                .map(|s| &*(*s as *mut Buffer).clone())
                .collect::<Vec<Buffer>>();

            drop(array);

            match node_fs::sync::writev(fd, slice, position.try_into().unwrap()) {
                Ok(wrote) => wrote as jlong,
                Err(error) => {
                    let _ = env.throw(error.to_string());
                    0
                }
            }
        }
        Err(error) => {
            let _ = env.throw(error.to_string());
            0
        }
    }
}
