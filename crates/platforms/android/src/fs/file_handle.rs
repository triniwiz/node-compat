use jni::objects::{JByteArray, JByteBuffer, JClass, JLongArray, JObject, JString};
use jni::sys::{jbyteArray, jint, jlong, jobjectArray};
use jni::JNIEnv;
use libc::c_int;
use node_buffer::StringEncoding;

use crate::a_sync::AsyncCallback;
use crate::fs::a_sync::{AsyncCallback, AsyncClosure};
use node_fs::file_handle::FileHandle;
use node_fs::file_stat::FileStat;
use crate::fs::{FILE_HANDLE_CLASS, JVM};
use crate::fs::prelude::*;

fn build_file_handle<'a>(env: &'a mut JNIEnv, handle: FileHandle) -> JObject<'a> {
    let clazz = find_class(FILE_HANDLE_CLASS).unwrap();
    let object = env.new_object(clazz, "()V", &[]).unwrap();
    let ptr = Box::into_raw(Box::new(handle));
    let _ = env.set_field(&object, "native", "J", (ptr as i64).into());
    object
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeOpenSync(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
) -> jlong {
    match node_fs::sync::open_handle_with_fd(fd) {
        Ok(handle) => (Box::into_raw(Box::new(handle)) as i64).into(),
        Err(error) => {
            let _ = env.throw(error.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeOpen(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    flags: jint,
    mode: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<c_int, std::io::Error>::new(Box::new(move |success, error| {
        if let Some(error) = error {
            on_success.on_error(jni::objects::JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            let jvm = JVM.get().unwrap();
            let mut env = jvm.attach_current_thread().unwrap();
            on_success.on_success(to_integer(&mut env, success.unwrap().into()).into())
        }
    }))
        .into_arc();
    let path = get_str(&mut env, &path, "");
    node_fs::a_sync::open(&path, flags, mode, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeAppendFileWithBytes(
    mut env: JNIEnv,
    _: JClass,
    handle: jlong,
    bytes: JByteArray,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    super::a_sync::nativeAppendFileWithBytes(&mut env, handle.fd(), bytes, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeAppendFileWithString(
    _: JNIEnv,
    _: JClass,
    handle: jlong,
    data: JString,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    super::a_sync::nativeAppendFileWithString(handle.fd(), data, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeChmod(
    _: JNIEnv,
    _: JClass,
    handle: jlong,
    mode: jint,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(jni::objects::JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(jni::objects::JObject::null().into())
        }
    }))
        .into_arc();
    handle.chmod(mode.try_into().unwrap(), callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeChown(
    _: JNIEnv,
    _: JClass,
    handle: jlong,
    uid: jint,
    gid: jint,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(jni::objects::JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    handle.chown(uid as u32, gid as u32, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeClose(
    _: JNIEnv,
    _: JClass,
    handle: jlong,
    callback: jlong,
) {
    let handle = unsafe { *Box::from_raw(handle as *mut FileHandle) };
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(jni::objects::JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(jni::objects::JObject::null().into())
        }
    }))
        .into_arc();
    handle.close(callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeDatasync(
    _: JNIEnv,
    _: JClass,
    handle: jlong,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(jni::objects::JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(jni::objects::JObject::null().into())
        }
    }))
        .into_arc();
    handle.datasync(callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeGetFd(
    _: JNIEnv,
    _: JClass,
    handle: jint,
) -> jint {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };
    handle.fd()
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeRead(
    env: JNIEnv,
    _: JClass,
    handle: jlong,
    buffer: JByteBuffer,
    offset: jlong,
    length: jlong,
    position: jlong,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };
    super::a_sync::nativeRead(env, handle.fd(), buffer, offset, length, position, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeReadWithBytes(
    env: JNIEnv,
    _: JClass,
    handle: jlong,
    buffer: JByteArray,
    offset: jlong,
    length: jlong,
    position: jlong,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };
    super::a_sync::nativeReadWithBytes(
        env,
        handle.fd(),
        buffer,
        offset,
        length,
        position,
        callback,
    );
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeReadv(
    env: JNIEnv,
    _: JClass,
    handle: jlong,
    buffers: JLongArray,
    position: jlong,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    super::a_sync::nativeReadv(env, handle.fd(), buffers, position, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeStat(
    _: JNIEnv,
    _: JClass,
    handle: jlong,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback =
        AsyncClosure::<FileStat, std::io::Error>::new(Box::new(move |success, error| {
            if error.is_some() {
                on_success.on_error(jni::objects::JValue::Object(
                    error_to_jstring(error.unwrap()).as_obj(),
                ))
            } else {
                let vm = JVM.get().unwrap();
                let mut env = vm.attach_current_thread().unwrap();
                let stat = super::file_stat::build_stat(&mut env, success.unwrap());
                on_success.on_success(stat.into())
            }
        }))
            .into_arc();

    handle.stat(callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeSync(
    _: JNIEnv,
    _: JClass,
    handle: jlong,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(jni::objects::JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(jni::objects::JObject::null().into())
        }
    }))
        .into_arc();
    handle.sync(callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeTruncate(
    _: JNIEnv,
    _: JClass,
    handle: jlong,
    len: jlong,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(jni::objects::JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(jni::objects::JObject::null().into())
        }
    }))
        .into_arc();
    handle.truncate(len.try_into().unwrap(), callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeUtimes(
    _: JNIEnv,
    _: JClass,
    handle: jlong,
    atime: jlong,
    mtime: jlong,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(jni::objects::JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(jni::objects::JObject::null().into())
        }
    }))
        .into_arc();

    handle.utimes(
        atime.try_into().unwrap(),
        mtime.try_into().unwrap(),
        callback,
    );
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeWrite(
    env: JNIEnv,
    _: JClass,
    handle: jlong,
    buffer: JByteBuffer,
    offset: jlong,
    length: jlong,
    position: jlong,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    super::a_sync::nativeWrite(env, handle.fd(), buffer, offset, length, position, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeWriteBytes(
    env: JNIEnv,
    _: JClass,
    handle: jlong,
    buffer: JByteArray,
    offset: jlong,
    length: jlong,
    position: jlong,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    super::a_sync::nativeWriteBytes(env, handle.fd(), buffer, offset, length, position, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeWriteString(
    mut env: JNIEnv,
    _: JClass,
    handle: jlong,
    string: JString,
    encoding: jint,
    position: jlong,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    let callback = callback as *const AsyncCallback;
    let string = get_str(&mut env, &string, "");

    let on_success = AsyncCallback::clone_from_ptr(callback);

    match StringEncoding::try_from(encoding) {
        Ok(encoding) => {
            let callback = AsyncClosure::<usize, std::io::Error>::new(Box::new(move |success, error| {
                if let Some(error) = error {
                    on_success.on_error(jni::objects::JValue::Object(
                        error_to_jstring(error).as_obj(),
                    ))
                } else {
                    on_success.on_success((success.unwrap() as jlong).into())
                }
            }))
                .into_arc();
            handle.write_string(&string, encoding, position.try_into().unwrap(), callback);
        }
        Err(error) => {
            on_success.on_error(env.new_string(error).unwrap().into())
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeWriteFileWithString(
    mut env: JNIEnv,
    _: JClass,
    handle: jint,
    data: JString,
    encoding: jint,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    let callback = callback as *const AsyncCallback;
    let data = get_str(&mut env, &data, "");

    let on_success = AsyncCallback::clone_from_ptr(callback);


    match StringEncoding::try_from(encoding) {
        Ok(encoding) => {
            let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
                if let Some(error) = error {
                    on_success.on_error(jni::objects::JValue::Object(
                        error_to_jstring(error).as_obj(),
                    ))
                } else {
                    on_success.on_success(JObject::null().into())
                }
            }))
                .into_arc();
            handle.write_file_with_str(&data, encoding, callback);
        }
        Err(error) => {
            on_success.on_error(env.new_string(error).unwrap().into())
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeWriteFileWithBytes(
    env: JNIEnv,
    _: JClass,
    handle: jlong,
    data: JByteArray,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    super::a_sync::nativeWriteFileWithBytes(env, handle.fd(), data, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeWriteFileWithBuffer(
    env: JNIEnv,
    _: JClass,
    handle: jlong,
    data: JByteBuffer,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    super::a_sync::nativeWriteFileWithBuffer(env, handle.fd(), data, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileHandle_nativeWritev(
    env: JNIEnv,
    _: JClass,
    handle: jlong,
    buffers: JLongArray,
    position: jlong,
    callback: jlong,
) {
    let handle = handle as *mut FileHandle;
    let handle = unsafe { &mut *handle };

    super::a_sync::nativeWritev(env, handle.fd(), buffers, position, callback);
}
