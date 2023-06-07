use jni::objects::{JByteBuffer, JClass, JObject, JString, JValue, ReleaseMode};
use jni::sys::{jboolean, jbyteArray, jint, jlong, jobject, jobjectArray, JNI_TRUE};
use jni::JNIEnv;
use libc::{c_int, c_uint, c_ushort};
use node_fs::prelude::handle_meta;
use crate::fs::file_dir::build_dir;
use crate::fs::file_dirent::{build_dirents, build_dirents_paths};
use crate::fs::file_stat::build_stat;
use crate::fs::FILE_SYSTEM_CLASS;
use crate::fs::prelude::*;

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAccessSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    mode: jint,
) {
    let path = get_str(path, "");
    let result = node_fs::sync::access(&path, mode);
    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAppendFileWithBytesSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    bytes: jbyteArray,
) {
    let data = env
        .get_primitive_array_critical(bytes, ReleaseMode::NoCopyBack)
        .unwrap();
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(
            data.as_ptr() as *mut u8,
            data.size().unwrap_or_default() as usize,
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
    let data = get_str(data, "");
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
    bytes: jbyteArray,
    mode: jint,
    flags: jint,
) {
    let path = get_str(path, "");
    let data = env
        .get_primitive_array_critical(bytes, ReleaseMode::NoCopyBack)
        .unwrap();
    let data = unsafe {
        std::slice::from_raw_parts_mut(
            data.as_ptr() as *mut u8,
            data.size().unwrap_or_default() as usize,
        )
    };
    let result = node_fs::sync::append_file_with_path_bytes(&path, data, mode, flags);

    if let Err(error) = result {
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
    let path = get_str(path, "");
    let data = get_str(data, "");
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
    let path = get_str(path, "");
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
    let path = get_str(path, "");
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
    let src = get_str(src, "");
    let dest = get_str(dest, "");
    let result = node_fs::sync::copy_file(&src, &dest, flags as u32);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeCopySync(
    _: JNIEnv,
    _: JClass,
    src: JString,
    dest: JString,
    _flags: jint,
) {
    let _src = get_str(src, "");
    let _dest = get_str(dest, "");
    todo!()
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeExistsSync(
    _: JNIEnv,
    _: JClass,
    src: JString,
) -> jboolean {
    let src = get_str(src, "");
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
    let path = get_str(path, "");
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
    let path = get_str(path, "");
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
    let path = get_str(path, "");
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
    let existing_path = get_str(existing_path, "");
    let new_path = get_str(new_path, "");
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
    let path = get_str(path, "");
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
    let path = get_str(path, "");
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
    let prefix = get_str(prefix, "");
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
    let path = get_str(path, "");
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
    let path = get_str(path, "");
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
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadWithBytesSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: jbyteArray,
    offset: jlong,
    length: jlong,
    position: jlong,
) -> jlong {
    let data = env
        .get_primitive_array_critical(buffer, ReleaseMode::NoCopyBack)
        .unwrap();
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(
            data.as_ptr() as *mut u8,
            data.size().unwrap_or_default() as usize,
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
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReaddirWithFileTypesSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    encoding: JString,
) -> jobjectArray {
    let path = get_str(path, "");
    let encoding = get_str(encoding, "");
    match node_fs::sync::readdir_with_file_types(&path, &encoding) {
        Ok(dirent) => build_dirents(&mut env, dirent),
        Err(error) => {
            let _ = env.throw(error.to_string());
            build_dirents(&mut env, vec![])
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReaddirWithFileSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    encoding: JString,
) -> jobjectArray {
    let path = get_str(path, "");
    let encoding = get_str(encoding, "");
    match node_fs::sync::readdir_with_file(&path, &encoding) {
        Ok(dir) => build_dirents_paths(&mut env, dir).into(),
        Err(error) => {
            let _ = env.throw(error.to_string());
            build_dirents_paths(&mut env, vec![]).into()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadFileSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    flags: jint,
) -> jobject {
    read_file(env, path, flags, false)
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadFileBytesSync(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    flags: jint,
) -> jobject {
    read_file(env, path, flags, true)
}

fn read_file(mut env: JNIEnv, path: JString, flags: jint, to_bytes: bool) -> jobject {
    let path = get_str(path, "");
    match node_fs::sync::read_file(&path, flags) {
        Ok(mut buf) => {
            if to_bytes {
                env.byte_array_from_slice(buf.as_mut_slice())
                    .unwrap()
                    .into()
            } else {
                let db = unsafe { env.new_direct_byte_buffer(buf.as_mut_slice(), buf.length()).unwrap() };
                let buf = Box::into_raw(Box::new(buf));
                let clazz = find_class(FILE_SYSTEM_CLASS).unwrap();
                let db: JValue = db.into();
                env.call_static_method(
                    clazz,
                    "watchItem",
                    "(JLjava/nio/ByteBuffer;)V",
                    &[(buf as i64).into(), db],
                )
                    .unwrap();
                db.l().unwrap().into_inner()
            }
        }
        Err(error) => {
            let _ = env.throw(error.to_string());
            JObject::null().into_inner()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadFileWithFdSync(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    flags: jint,
) -> jobject {
    read_file_with_fd(env, fd, flags, false)
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadFileBytesWithFdSync(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    flags: jint,
) -> jobject {
    read_file_with_fd(env, fd, flags, true)
}

fn read_file_with_fd(env: JNIEnv, fd: jint, flags: jint, to_bytes: bool) -> jobject {
    match node_fs::sync::read_file_with_fd(fd, flags) {
        Ok(mut buf) => {
            if to_bytes {
                env.byte_array_from_slice(buf.as_mut_slice()).unwrap()
            } else {
                let db = env.new_direct_byte_buffer(buf.as_mut_slice()).unwrap();
                let buf = Box::into_raw(Box::new(buf));
                let clazz = find_class(FILE_SYSTEM_CLASS).unwrap();
                let db: JValue = db.into();
                let _ = env.call_static_method(
                    clazz,
                    "watchItem",
                    "(JLjava/nio/ByteBuffer)V",
                    &[(buf as i64).into(), db],
                );
                db.l().unwrap().into_inner()
            }
        }
        Err(error) => {
            let _ = env.throw(error.to_string());
            JObject::null().into_inner()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadLinkSync(
    env: JNIEnv,
    _: JClass,
    path: JString,
    encoding: JString,
) -> jobject {
    let path = get_str(path, "");
    let encoding = get_str(encoding, "");
    match node_fs::sync::read_link(&path, &encoding) {
        Ok(link) => env.new_string(link.to_string_lossy()).unwrap().into_inner(),
        Err(error) => {
            let _ = env.throw(error.to_string());
            JObject::null().into_inner()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadvSync(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    buffers: jobjectArray,
    position: jlong,
) -> jlong {
    let size = env.get_array_length(buffers).unwrap_or_default();
    let mut buf = Vec::<ByteBufMut>::with_capacity(size.try_into().unwrap());
    for i in 0..size {
        let bytebuf = JByteBuffer::from(env.get_object_array_element(buffers, i).unwrap());
        let address = env.get_direct_buffer_address(bytebuf).unwrap();
        buf.push(ByteBufMut::new(address.as_mut_ptr(), address.len()))
    }
    match node_fs::sync::readv(fd, buf.as_mut_slice(), position.try_into().unwrap()) {
        Ok(read) => read as jlong,
        Err(error) => {
            let _ = env.throw(error.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeRealPathSync(
    env: JNIEnv,
    _: JClass,
    path: JString,
) -> jobject {
    let path = get_str(path, "");
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
    env: JNIEnv,
    _: JClass,
    old_path: JString,
    new_path: JString,
) {
    let old_path = get_str(old_path, "");
    let new_path = get_str(new_path, "");
    let result = node_fs::sync::rename(&old_path, &new_path);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeRmdirSync(
    env: JNIEnv,
    _: JClass,
    path: JString,
    max_retries: jint,
    recursive: jboolean,
    retry_delay: jlong,
) {
    let path = get_str(path, "");
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
    env: JNIEnv,
    _: JClass,
    path: JString,
    max_retries: jint,
    recursive: jboolean,
    retry_delay: jlong,
) {
    let path = get_str(path, "");
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
    env: JNIEnv,
    _: JClass,
    path: JString,
    throw_if_no_entry: jboolean,
) -> jobject {
    let path = get_str(path, "");
    let meta = node_fs::sync::stat(&path);
    match meta {
        Ok(stat) => build_stat(&env, handle_meta(&stat)).into_inner(),
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
    env: JNIEnv,
    _: JClass,
    target: JString,
    path: JString,
    type_: JString,
) {
    let target = get_str(target, "");
    let path = get_str(path, "");
    let type_ = get_str(type_, "");
    let result = node_fs::sync::symlink(&target, &path, &type_);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeTruncateSync(
    env: JNIEnv,
    _: JClass,
    path: JString,
    len: jlong,
) {
    let path = get_str(path, "");
    let result = node_fs::sync::truncate(&path, len.try_into().unwrap());

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeUnlinkSync(
    env: JNIEnv,
    _: JClass,
    path: JString,
) {
    let path = get_str(path, "");
    let result = node_fs::sync::unlink(&path);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

// todo watch

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeUtimesSync(
    env: JNIEnv,
    _: JClass,
    path: JString,
    atime: jlong,
    mtime: jlong,
) {
    let path = get_str(path, "");
    let result = node_fs::sync::utimes(&path, atime.try_into().unwrap(), mtime.try_into().unwrap());

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteSync(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: JByteBuffer,
    offset: jlong,
    length: jlong,
    position: jlong,
) -> jlong {
    let bytes = env.get_direct_buffer_address(buffer).unwrap();
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

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteBytesSync(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: jbyteArray,
    offset: jlong,
    length: jlong,
    position: jlong,
) -> jlong {
    let data = env
        .get_primitive_array_critical(buffer, ReleaseMode::NoCopyBack)
        .unwrap();
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(
            data.as_ptr() as *mut u8,
            data.size().unwrap_or_default() as usize,
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
    env: JNIEnv,
    _: JClass,
    fd: jint,
    string: JString,
    encoding: JString,
    position: jlong,
) -> jlong {
    let string = get_str(string, "");
    let encoding = get_str(encoding, "");
    match node_fs::sync::write_string(fd, &string, &encoding, position.try_into().unwrap()) {
        Ok(wrote) => wrote as jlong,
        Err(error) => {
            let _ = env.throw(error.to_string());
            0_i64
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithStringSync(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    data: JString,
    encoding: JString,
) {
    let data = get_str(data, "");
    let encoding = get_str(encoding, "");
    let result = node_fs::sync::write_file_with_str(fd, &data, &encoding);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithBytesSync(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    data: jbyteArray,
) {
    let data = env
        .get_primitive_array_critical(data, ReleaseMode::NoCopyBack)
        .unwrap();
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(
            data.as_ptr() as *mut u8,
            data.size().unwrap_or_default() as usize,
        )
    };
    let result = node_fs::sync::write_file_with_bytes(fd, bytes);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithBufferSync(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    data: JByteBuffer,
) {
    let bytes = env.get_direct_buffer_address(data).unwrap();
    let result = node_fs::sync::write_file_with_bytes(fd, bytes);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithStringFromPathSync(
    env: JNIEnv,
    _: JClass,
    path: JString,
    data: JString,
    encoding: JString,
    mode: c_int,
    flag: c_int,
) {
    let path = get_str(path, "");
    let data = get_str(data, "");
    let encoding = get_str(encoding, "");

    let result = node_fs::sync::write_file_with_str_from_path(&path, &data, &encoding, mode, flag);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithBytesFromPathSync(
    env: JNIEnv,
    _: JClass,
    path: JString,
    data: jbyteArray,
    encoding: JString,
    mode: c_int,
    flag: c_int,
) {
    let path = get_str(path, "");
    let encoding = get_str(encoding, "");

    let data = env
        .get_primitive_array_critical(data, ReleaseMode::NoCopyBack)
        .unwrap();
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(
            data.as_ptr() as *mut u8,
            data.size().unwrap_or_default() as usize,
        )
    };
    let result = node_fs::sync::write_file_with_bytes_from_path(&path, bytes, &encoding, mode, flag);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithBufferFromPathSync(
    env: JNIEnv,
    _: JClass,
    path: JString,
    data: JByteBuffer,
    encoding: JString,
    mode: c_int,
    flag: c_int,
) {
    let path = get_str(path, "");
    let encoding = get_str(encoding, "");

    let bytes = env.get_direct_buffer_address(data).unwrap();
    let result = node_fs::sync::write_file_with_bytes_from_path(&path, bytes, &encoding, mode, flag);

    if let Err(error) = result {
        let _ = env.throw(error.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWritevSync(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    buffers: jobjectArray,
    position: jlong,
) -> jlong {
    let size = env.get_array_length(buffers).unwrap_or_default();
    let mut buf = Vec::<ByteBuf>::with_capacity(size.try_into().unwrap());
    for i in 0..size {
        let bytebuf = JByteBuffer::from(env.get_object_array_element(buffers, i).unwrap());
        let address = env.get_direct_buffer_address(bytebuf).unwrap();
        buf.push(ByteBuf::new(address.as_ptr(), address.len()))
    }
    match node_fs::sync::writev(fd, buf, position.try_into().unwrap()) {
        Ok(wrote) => wrote as jlong,
        Err(error) => {
            let _ = env.throw(error.to_string());
            0
        }
    }
}
