use std::ffi::{c_int, c_uint, c_ushort, CString};
use std::path::PathBuf;
use std::sync::Arc;

use jni::objects::{JByteArray, JByteBuffer, JClass, JLongArray, JObject, JObjectArray, JPrimitiveArray, JString, JValue, ReleaseMode};
use jni::sys::{jboolean, jbyteArray, jint, jlong, jobjectArray, JNI_TRUE, jlongArray, jsize};
use jni::{JavaVM, JNIEnv};
use node_buffer::{Buffer, StringEncoding};
use node_fs::a_sync::AsyncClosure;
use node_fs::file_dir::FileDir;
use node_fs::file_stat::FileStat;
use node_fs::prelude::{FsEncoding, FsEncodingType};
use node_fs::sync::ReaddirResult;
use crate::fs::file_dir::build_dir;
use crate::fs::file_dirent::{build_dirent, build_dirents};
use crate::fs::{FILE_DIRENT_CLASS, JVM, STRING_CLASS};
use crate::fs::prelude::*;


#[derive(Clone, Debug)]
pub struct AsyncCallback {
    pub(crate) inner: jni::objects::GlobalRef,
}

impl AsyncCallback {
    pub fn new(callback: jni::objects::GlobalRef) -> Self {
        Self {
            inner: AsyncCallbackInner {
                inner: callback.clone()
            },
        }
    }

    pub fn on_success(&self, result: JValue) {
        if let Some(jvm) = JVM.get() {
            let mut env = jvm.attach_current_thread().unwrap();
            let _ = env.call_method(
                self.inner.inner().as_obj(),
                "onSuccess",
                "(Ljava/lang/Object;)V",
                &[result],
            );
        }
    }

    pub fn on_error(&self, result: JValue) {
        if let Some(jvm) = JVM.get() {
            let mut env = jvm.attach_current_thread().unwrap();
            let _ = env.call_method(
                self.inner.as_obj(),
                "onError",
                "(Ljava/lang/Object;)V",
                &[result],
            );
        }
    }

    pub fn clone_from_ptr(ptr: *const AsyncCallback) -> AsyncCallback {
        unsafe {
            let callback = &*ptr;
            callback.clone()
        }
    }
}

impl PartialEq for AsyncCallback {
    fn eq(&self, other: &Self) -> bool {
        match JVM.get() {
            None => false,
            Some(jvm) => {
                let mut env = jvm.attach_current_thread().unwrap();
                env.is_same_object(
                    self.inner.as_obj(), other.inner.as_obj(),
                ).unwrap_or(false)
            }
        }
    }
}

impl Eq for AsyncCallback {}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAccess(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    mode: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    node_fs::a_sync::access(get_str(&mut env, &path, "").as_ref(), mode, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAppendFileWithBytes(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    bytes: JByteArray,
    callback: jlong,
) {
    nativeAppendFileWithBytes(&mut env, fd, bytes, callback);
}

pub(crate) fn nativeAppendFileWithBytes(env: &mut JNIEnv, fd: jint, bytes: JByteArray, callback: jlong) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let bytes = env.new_global_ref(JObject::from(bytes)).unwrap();
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();
        let bytes = unsafe { JByteArray::from_raw(bytes.into_raw()) };
        let data = unsafe {
            env
                .get_array_elements_critical(&bytes, ReleaseMode::NoCopyBack)
                .unwrap()
        };

        let bytes = unsafe {
            std::slice::from_raw_parts_mut(std::mem::transmute::<*mut i8, *mut u8>(data.as_ptr()), data.len())
        };

        match node_fs::sync::append_file_with_bytes(fd, bytes) {
            Ok(_) => {
                // force drop of array to enable jni usage
                drop(data);
                callback.on_success(JObject::null().into())
            }
            Err(error) => {
                // force drop of array to enable jni usage
                drop(data);
                callback.on_error(JValue::Object(
                    error_to_jstring(error).as_obj(),
                ))
            }
        }
    });
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAppendFileWithBuffer(
    _: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let buffer = unsafe { (&*(buffer as *mut Buffer)).clone() };
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let _ = jvm.attach_current_thread().unwrap();

        let bytes = buffer.buffer();

        match node_fs::sync::append_file_with_bytes(fd, bytes) {
            Ok(_) => {
                // force drop of array to enable jni usage
                drop(data);
                callback.on_success(JObject::null().into())
            }
            Err(error) => {
                // force drop of array to enable jni usage
                drop(data);
                callback.on_error(JValue::Object(
                    error_to_jstring(error).as_obj(),
                ))
            }
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAppendFileWithString(
    _: JNIEnv,
    _: JClass,
    fd: jint,
    data: JString,
    callback: jlong,
) {
    nativeAppendFileWithString(fd, data, callback);
}

pub(crate) fn nativeAppendFileWithString(fd: jint, data: JString, callback: jlong) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    let data = get_str(&mut enn, &data, "");
    node_fs::a_sync::append_file_with_str(fd, data.as_str(), callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAppendFileWithPathBytes(
    env: JNIEnv,
    _: JClass,
    path: JString,
    bytes: JByteArray,
    mode: jint,
    flags: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let bytes = JObject::from(bytes);
    let bytes = env.new_global_ref(bytes).unwrap();
    let path = env.new_global_ref(path).unwrap();
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();
        let bytes = unsafe { JByteArray::from_raw(bytes.as_raw()) };
        let data = unsafe {
            env
                .get_array_elements_critical(&bytes, ReleaseMode::NoCopyBack)
                .unwrap()
        };

        let data = unsafe {
            std::slice::from_raw_parts_mut(
                data.as_ptr() as *mut u8,
                data.len(),
            )
        };

        let path = unsafe { JString::from_raw(path.as_raw()) };
        match node_fs::sync::append_file_with_path_bytes(
            unsafe { get_str(&mut env, &path, "").as_ref() },
            data,
            mode,
            flags,
        ) {
            Ok(_) => {
                // force drop of array to enable jni usage
                drop(data);
                callback.on_success(JObject::null().into())
            }
            Err(error) => {
                // force drop of array to enable jni usage
                drop(data);
                callback.on_error(jni::objects::JValue::Object(
                    error_to_jstring(error).as_obj(),
                ))
            }
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAppendFileWithPathBuffer(
    env: JNIEnv,
    _: JClass,
    path: JString,
    buffer: jlong,
    mode: jint,
    flags: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let buffer = unsafe { (&*(buffer as *mut Buffer)).clone() };
    let path = env.new_global_ref(path).unwrap();
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();

        let data = buffer.buffer();

        let path = unsafe { JString::from_raw(path.as_raw()) };
        match node_fs::sync::append_file_with_path_bytes(
            unsafe { get_str(&mut env, &path, "").as_ref() },
            data,
            mode,
            flags,
        ) {
            Ok(_) => {
                // force drop of array to enable jni usage
                drop(data);
                callback.on_success(JObject::null().into())
            }
            Err(error) => {
                // force drop of array to enable jni usage
                drop(data);
                callback.on_error(jni::objects::JValue::Object(
                    error_to_jstring(error).as_obj(),
                ))
            }
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeAppendFileWithPathString(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    data: JString,
    mode: jint,
    flags: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let path = get_str(&mut env, &path, "");
    let data = get_str(&mut env, &data, "");
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    node_fs::a_sync::append_file_with_path_str(path.as_str(), data.as_str(), mode, flags, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeChmod(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    mode: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let path = get_str(&mut env, &path, "");
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    node_fs::a_sync::chmod(path.as_str(), mode as u32, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeChown(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    uid: jint,
    gid: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let path = get_str(&mut env, &path, "");
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();

    node_fs::a_sync::chown(path.as_ref(), uid as u32, gid as u32, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeClose(
    _: JNIEnv,
    _: JClass,
    fd: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    node_fs::a_sync::close(fd, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeCopyFile(
    mut env: JNIEnv,
    _: JClass,
    src: JString,
    dest: JString,
    flags: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let src = get_str(&mut env, &src, "");
    let dest = get_str(&mut env, &dest, "");
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    node_fs::a_sync::copy_file(&src, &dest, flags as u32, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeCopy(
    mut env: JNIEnv,
    _: JClass,
    src: JString,
    dest: JString,
    _flags: jint,
    callback: jlong,
) {
    let _callback = callback as *const AsyncCallback;
    let _src = get_str(&mut env, &src, "");
    let _dest = get_str(&mut env, &dest, "");
    todo!()
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeExists(
    mut env: JNIEnv,
    _: JClass,
    src: JString,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<bool, std::io::Error>::new(Box::new(move |success, error| {
        if error.is_some() {
            on_success.on_error(JValue::Object(
                error_to_jstring(error.unwrap()).as_obj(),
            ))
        } else {
            let vm = JVM.get().unwrap();
            let mut env = vm.attach_current_thread().unwrap();
            on_success.on_success(to_boolean(&mut env, success.unwrap()).into())
        }
    }))
        .into_arc();

    let src = get_str(&mut env, &src, "");

    node_fs::a_sync::exists(&src, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeFchmod(
    _: JNIEnv,
    _: JClass,
    fd: jint,
    mode: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    node_fs::a_sync::fchmod(fd, mode as c_ushort, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeFchown(
    _: JNIEnv,
    _: JClass,
    fd: jint,
    uid: jint,
    gid: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    node_fs::a_sync::fchown(fd, uid as c_uint, gid as c_uint, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeFdatasync(
    _: JNIEnv,
    _: JClass,
    fd: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    node_fs::a_sync::fdatasync(fd, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeFstat(
    _: JNIEnv,
    _: JClass,
    fd: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback =
        AsyncClosure::<FileStat, std::io::Error>::new(Box::new(move |success, error| {
            if error.is_some() {
                on_success.on_error(JValue::Object(
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
    node_fs::a_sync::fstat(fd, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeFsync(
    _: JNIEnv,
    _: JClass,
    fd: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    node_fs::a_sync::fsync(fd, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeFtruncate(
    _: JNIEnv,
    _: JClass,
    fd: jint,
    len: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    node_fs::a_sync::ftruncate(fd, len.try_into().unwrap(), callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeFutimes(
    _: JNIEnv,
    _: JClass,
    fd: jint,
    atime: jlong,
    mtime: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();

    node_fs::a_sync::futimes(
        fd,
        atime.try_into().unwrap(),
        mtime.try_into().unwrap(),
        callback,
    );
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeLchmod(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    mode: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    let path = get_str(&mut env, &path, "");
    node_fs::a_sync::lchmod(&path, mode as c_ushort, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeLchown(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    uid: jint,
    gid: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    let path = get_str(&mut env, &path, "");
    node_fs::a_sync::lchown(&path, uid as c_uint, gid as c_uint, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeLutimes(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    atime: jlong,
    mtime: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    let path = get_str(&mut env, &path, "");

    node_fs::a_sync::lutimes(
        &path,
        atime.try_into().unwrap(),
        mtime.try_into().unwrap(),
        callback,
    );
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeLink(
    mut env: JNIEnv,
    _: JClass,
    existing_path: JString,
    new_path: JString,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    let existing_path = get_str(&mut env, &existing_path, "");
    let new_path = get_str(&mut env, &new_path, "");
    node_fs::a_sync::link(&existing_path, &new_path, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeLstat(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback =
        AsyncClosure::<FileStat, std::io::Error>::new(Box::new(move |success, error| {
            if error.is_some() {
                on_success.on_error(JValue::Object(
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
    let path = get_str(&mut env, &path, "");
    node_fs::a_sync::lstat(&path, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeMkdir(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    mode: c_int,
    recursive: jboolean,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    let path = get_str(&mut env, &path, "");
    node_fs::a_sync::mkdir(&path, mode as u32, recursive == JNI_TRUE, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeMkdtemp(
    mut env: JNIEnv,
    _: JClass,
    prefix: JString,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<PathBuf, std::io::Error>::new(Box::new(move |success, error| {
        if error.is_some() {
            on_success.on_error(JValue::Object(
                error_to_jstring(error.unwrap()).as_obj(),
            ))
        } else {
            let res = success.unwrap();
            let res = res.to_string_lossy();
            let jvm = JVM.get().unwrap();
            let env = jvm.attach_current_thread().unwrap();
            let res = env.new_string(res.as_ref()).unwrap();
            on_success.on_success(res.into())
        }
    }))
        .into_arc();
    let prefix = get_str(&mut env, &prefix, "");
    node_fs::a_sync::mkdtemp(&prefix, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeOpen(
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
        if error.is_some() {
            on_success.on_error(JValue::Object(
                error_to_jstring(error.unwrap()).as_obj(),
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
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeOpenDir(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<FileDir, std::io::Error>::new(Box::new(move |success, error| {
        if error.is_some() {
            on_success.on_error(JValue::Object(
                error_to_jstring(error.unwrap()).as_obj(),
            ))
        } else {
            let jvm = JVM.get().unwrap();
            let mut env = jvm.attach_current_thread().unwrap();
            on_success.on_success(build_dir(&mut env, success.unwrap()).into())
        }
    }))
        .into_arc();
    let path = get_str(&mut env, &path, "");
    node_fs::a_sync::opendir(&path, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeRead(
    _: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: jlong,
    offset: jlong,
    length: jlong,
    position: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let mut buffer = unsafe { (&*(buffer as *mut Buffer)).clone() };
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();
        let bytes = buffer.buffer_mut();
        match node_fs::sync::read(
            fd,
            bytes,
            offset.try_into().unwrap(),
            length.try_into().unwrap(),
            position.try_into().unwrap(),
        ) {
            Ok(read) => callback.on_success(to_long(&mut env, read.try_into().unwrap()).into()),
            Err(error) => callback.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            )),
        }
    });
}

pub(crate) fn nativeRead(
    env: &mut JNIEnv,
    fd: jint,
    buffer: JByteBuffer,
    offset: jlong,
    length: jlong,
    position: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let bytes = env.new_global_ref(buffer).unwrap();
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();
        let data = unsafe { JByteBuffer::from_raw(bytes.as_raw()) };
        match (env.get_direct_buffer_address(&data), env.get_direct_buffer_capacity(&data)) {
            (Ok(data), Ok(size)) => {
                let bytes = unsafe { std::slice::from_raw_parts_mut(data, size) };
                match node_fs::sync::read(
                    fd,
                    bytes,
                    offset.try_into().unwrap(),
                    length.try_into().unwrap(),
                    position.try_into().unwrap(),
                ) {
                    Ok(read) => callback.on_success(to_long(&mut env, read.try_into().unwrap()).into()),
                    Err(error) => callback.on_error(JValue::Object(
                        error_to_jstring(error).as_obj(),
                    )),
                }
            }
            (Err(error), _) => {
                callback.on_error(JValue::Object(
                    env.new_string(error.to_string()).unwrap().into()
                ))
            }
            (_, _) => {}
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadBuffer(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: JByteBuffer,
    offset: jlong,
    length: jlong,
    position: jlong,
    callback: jlong,
) {
    nativeRead(&mut env, fd, buffer, offset, length, position, callback);
}

pub(crate) fn nativeRead(
    env: &mut JNIEnv,
    fd: jint,
    buffer: JByteBuffer,
    offset: jlong,
    length: jlong,
    position: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let bytes = env.new_global_ref(buffer).unwrap();
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();
        let data = unsafe { JByteBuffer::from_raw(bytes.as_raw()) };
        match (env.get_direct_buffer_address(&data), env.get_direct_buffer_capacity(&data)) {
            (Ok(data), Ok(size)) => {
                let bytes = unsafe { std::slice::from_raw_parts_mut(data, size) };
                match node_fs::sync::read(
                    fd,
                    bytes,
                    offset.try_into().unwrap(),
                    length.try_into().unwrap(),
                    position.try_into().unwrap(),
                ) {
                    Ok(read) => callback.on_success(to_long(&mut env, read.try_into().unwrap()).into()),
                    Err(error) => callback.on_error(JValue::Object(
                        error_to_jstring(error).as_obj(),
                    )),
                }
            }
            (Err(error), _) => {
                callback.on_error(JValue::Object(
                    env.new_string(error.to_string()).unwrap().into()
                ))
            }
            (_, _) => {}
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadWithBytes(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: JByteArray,
    offset: jlong,
    length: jlong,
    position: jlong,
    callback: jlong,
) {
    nativeReadWithBytes(env, fd, buffer, offset, length, position, callback);
}

pub(crate) fn nativeReadWithBytes(
    env: JNIEnv,
    fd: jint,
    buffer: JByteArray,
    offset: jlong,
    length: jlong,
    position: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let bytes = env.new_global_ref(buffer).unwrap();
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();
        let bytes = unsafe { JByteArray::from_raw(bytes.as_raw()) };
        let data = unsafe {
            env
                .get_array_elements_critical(&bytes, ReleaseMode::NoCopyBack)
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
                callback.on_success(to_long(&mut env, read.try_into().unwrap()).into())
            }
            Err(error) => {
                // force drop of array to enable jni usage
                drop(data);
                callback.on_error(JValue::Object(
                    error_to_jstring(error).as_obj(),
                ))
            }
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReaddir(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    with_file_types: jboolean,
    encoding: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);

    match FsEncodingType::try_from(encoding) {
        Ok(encoding) => {
            let path = get_str(&mut env, &path, "");
            node_fs::a_sync::readdir(
                &path,
                with_file_types == JNI_TRUE,
                encoding,
                Box::new(move |success, error| {
                    if error.is_some() {
                        on_success.on_error(JValue::Object(
                            error_to_jstring(error.unwrap()).as_obj(),
                        ))
                    } else {
                        let jvm = JVM.get().unwrap();
                        let mut env = jvm.attach_current_thread().unwrap();
                        let success = success.unwrap();

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

                            unsafe {
                                on_success.on_success(JObject::from_raw(array.into_raw()).into())
                            }
                            return;
                        }

                        let result = match encoding {
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
                        unsafe {
                            on_success.on_success(JObject::from_raw(result).into())
                        }
                    }
                }),
            );
        }
        Err(error) => {
            on_success.on_error(JValue::Object(
                error_to_jstring(error.unwrap()).as_obj(),
            ))
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadFile(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    encoding: jint,
    flags: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);

    match FsEncodingType::try_from(encoding) {
        Ok(encoding) => {
            let callback =
                AsyncClosure::<FsEncoding, std::io::Error>::new(Box::new(move |success, error| {
                    if error.is_some() {
                        on_success.on_error(JValue::Object(
                            error_to_jstring(error.unwrap()).as_obj(),
                        ))
                    } else {
                        let jvm = JVM.get().unwrap();
                        let mut env = jvm.attach_current_thread().unwrap();

                        let value = match success.unwrap() {
                            FsEncoding::String(string) => {
                                JValue::Object(
                                    env.new_string(string.to_string_lossy()).unwrap().into()
                                )
                            }
                            FsEncoding::Buffer(buffer) => {
                                JValue::Long(Box::into_raw(
                                    Box::new(buffer)
                                ) as jlong)
                            }
                        };

                        on_success.on_success(value)
                    }
                }))
                    .into_arc();
            let path = get_str(&mut env, &path, "");
            node_fs::a_sync::read_file(&path, encoding, flags, callback);
        }
        Err(error) => {
            on_success.on_error(env.new_string(error).unwrap().into())
        }
    }
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadFileWithFd(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    encoding: jint,
    flags: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);

    match FsEncodingType::try_from(encoding) {
        Ok(encoding) => {
            let callback =
                AsyncClosure::<FsEncoding, std::io::Error>::new(Box::new(move |success, error| {
                    if error.is_some() {
                        on_success.on_error(JValue::Object(
                            error_to_jstring(error.unwrap()).as_obj(),
                        ))
                    } else {
                        let jvm = JVM.get().unwrap();
                        let mut env = jvm.attach_current_thread().unwrap();

                        let value = match success.unwrap() {
                            FsEncoding::String(string) => {
                                JValue::Object(
                                    env.new_string(string.to_string_lossy()).unwrap().into()
                                )
                            }
                            FsEncoding::Buffer(buffer) => {
                                JValue::Long(Box::into_raw(
                                    Box::new(buffer)
                                ) as jlong)
                            }
                        };

                        on_success.on_success(value)
                    }
                }))
                    .into_arc();
            node_fs::a_sync::read_file_with_fd(fd, encoding, flags, callback);
        }
        Err(error) => {
            on_success.on_success(env.new_string(error).unwrap().into())
        }
    }
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadLink(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    encoding: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);

    match FsEncodingType::try_from(encoding) {
        Ok(encoding) => {
            let callback = AsyncClosure::<FsEncoding, std::io::Error>::new(Box::new(move |success, error| {
                if error.is_some() {
                    on_success.on_error(JValue::Object(
                        error_to_jstring(error.unwrap()).as_obj(),
                    ))
                } else {
                    let res = success.unwrap();
                    let jvm = JVM.get().unwrap();
                    let env = jvm.attach_current_thread().unwrap();

                    let res = match res {
                        FsEncoding::String(string) => {
                            JValue::Object(
                                env.new_string(string.to_string_lossy()).unwrap().into()
                            )
                        }
                        FsEncoding::Buffer(buffer) => {
                            JValue::Long(Box::into_raw(
                                Box::new(buffer)
                            ) as jlong)
                        }
                    };
                    on_success.on_success(res.into())
                }
            }))
                .into_arc();
            let path = get_str(&mut env, &path, "");
            node_fs::a_sync::read_link(&path, encoding, callback);
        }
        Err(error) => {
            on_success.on_error(env.new_string(error).unwrap().into())
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeReadv(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    buffers: JLongArray,
    position: jlong,
    callback: jlong,
) {
    nativeReadv(env, fd, buffers, position, callback);
}

pub(crate) fn nativeReadv(
    env: JNIEnv,
    fd: jint,
    buffers: JLongArray,
    position: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let buffers = env.new_global_ref(buffers).unwrap();
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();
        let array = unsafe { JLongArray::from_raw(buffers.as_raw()) };

        match unsafe { env.get_array_elements_critical(&array, ReleaseMode::CopyBack) } {
            Ok(array) => {
                let array = unsafe { std::slice::from_raw_parts_mut(array.as_ptr() as *mut i64, array.len()) };

                let mut buf = array.iter()
                    .map(|value| unsafe { (&*(*i as *mut Buffer)).clone() })
                    .collect::<Vec<Buffer>>();

                match node_fs::sync::readv(fd, buf.as_mut_slice(), position.try_into().unwrap()) {
                    Ok(read) => {
                        on_success.on_success(to_long(&mut env, read.try_into().unwrap()).into());
                    }
                    Err(error) => on_success.on_error(JValue::Object(
                        error_to_jstring(error).as_obj(),
                    )),
                }
            }
            Err(error) => {}
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeRealPath(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<PathBuf, std::io::Error>::new(Box::new(move |success, error| {
        if error.is_some() {
            on_success.on_error(JValue::Object(
                error_to_jstring(error.unwrap()).as_obj(),
            ))
        } else {
            let res = success.unwrap();
            let res = res.to_string_lossy();
            let jvm = JVM.get().unwrap();
            let env = jvm.attach_current_thread().unwrap();
            let res = env.new_string(res.as_ref()).unwrap();
            on_success.on_success(res.into())
        }
    }))
        .into_arc();
    let path = get_str(&mut env, &path, "");
    node_fs::a_sync::real_path(&path, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeRename(
    mut env: JNIEnv,
    _: JClass,
    old_path: JString,
    new_path: JString,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    let old_path = get_str(&mut env, &old_path, "");
    let new_path = get_str(&mut env, &new_path, "");
    node_fs::a_sync::rename(&old_path, &new_path, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeRmdir(
    &mut env: JNIEnv,
    _: JClass,
    path: JString,
    max_retries: jint,
    recursive: jboolean,
    retry_delay: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    let path = get_str(&mut env, &path, "");
    node_fs::a_sync::rmdir(
        &path,
        max_retries,
        recursive == JNI_TRUE,
        retry_delay.try_into().unwrap(),
        callback,
    );
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeRm(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    max_retries: jint,
    recursive: jboolean,
    retry_delay: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), node_core::error::AnyError>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            let vm = JVM.get().unwrap();
            let env = vm.attach_current_thread().unwrap();

            on_success.on_error(JValue::Object(
                env.new_string(error.to_string()).unwrap().into()
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    let path = get_str(&mut env, &path, "");
    node_fs::a_sync::rm(
        &path,
        max_retries,
        recursive == JNI_TRUE,
        retry_delay.try_into().unwrap(),
        callback,
    );
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeStat(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    throw_if_no_entry: jboolean,
    callback: jlong,
) {
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
    let path = get_str(&mut env, &path, "");
    node_fs::a_sync::stat(&path, throw_if_no_entry == JNI_TRUE, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeSymlink(
    mut env: JNIEnv,
    _: JClass,
    target: JString,
    path: JString,
    type_: JString,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    let target = get_str(&mut env, &target, "");
    let path = get_str(&mut env, &path, "");
    let type_ = get_str(&mut env, &type_, "");
    node_fs::a_sync::symlink(&target, &path, &type_, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeTruncate(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    len: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    let path = get_str(&mut env, &path, "");
    node_fs::a_sync::truncate(&path, len.try_into().unwrap(), callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeUnlink(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    let path = get_str(&mut env, &path, "");
    node_fs::a_sync::unlink(&path, callback);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeUtimes(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    atime: jlong,
    mtime: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
        if let Some(error) = error {
            on_success.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(JObject::null().into())
        }
    }))
        .into_arc();
    let path = get_str(&mut env, &path, "");
    node_fs::a_sync::utimes(
        &path,
        atime.try_into().unwrap(),
        mtime.try_into().unwrap(),
        callback,
    );
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWrite(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: JByteBuffer,
    offset: jlong,
    length: jlong,
    position: jlong,
    callback: jlong,
) {
    nativeWrite(env, fd, buffer, offset, length, position, callback);
}

pub(crate) fn nativeWrite(
    env: JNIEnv,
    fd: jint,
    buffer: JByteBuffer,
    offset: jlong,
    length: jlong,
    position: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let buffer = env.new_global_ref(buffer).unwrap();

    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();

        let buffer = unsafe { JByteBuffer::from_raw(buffer.as_raw()) };

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
                    Ok(wrote) => callback.on_success(to_long(&mut env, wrote.try_into().unwrap()).into()),
                    Err(error) => callback.on_error(JValue::Object(
                        error_to_jstring(error).as_obj(),
                    )),
                }
            }
            (Err(error), _) => {
                callback.on_error(env.new_string(error.to_string()).unwrap().into())
            }
            _ => {}
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteBytes(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    buffer: JByteArray,
    offset: jlong,
    length: jlong,
    position: jlong,
    callback: jlong,
) {
    nativeWriteBytes(env, fd, buffer, offset, length, position, callback);
}

pub(crate) fn nativeWriteBytes(
    env: JNIEnv,
    fd: jint,
    buffer: JByteArray,
    offset: jlong,
    length: jlong,
    position: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let bytes = env.new_global_ref(buffer).unwrap();
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();
        let bytes = unsafe { JByteArray::from_raw(bytes.as_raw()) };
        let data = unsafe {
            env
                .get_array_elements_critical(&bytes, ReleaseMode::CopyBack)
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
                callback.on_success(to_long(&mut env, wrote.try_into().unwrap()).into())
            }
            Err(error) => {
                // force drop of array to enable jni usage
                drop(data);
                callback.on_error(JValue::Object(
                    error_to_jstring(error).as_obj(),
                ))
            }
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteString(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    string: JString,
    encoding: jint,
    position: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let string = get_str(&mut env, &string, "");

    let on_success = AsyncCallback::clone_from_ptr(callback);

    match StringEncoding::try_from(encoding) {
        Ok(encoding) => {
            let callback = AsyncClosure::<usize, std::io::Error>::new(Box::new(move |success, error| {
                if error.is_some() {
                    on_success.on_error(jni::objects::JValue::Object(
                        error_to_jstring(error.unwrap()).as_obj(),
                    ))
                } else {
                    on_success.on_success((success.unwrap() as jlong).into())
                }
            }))
                .into_arc();
            node_fs::a_sync::write_string(
                fd,
                &string,
                encoding,
                position.try_into().unwrap(),
                callback,
            );
        }
        Err(error) => {
            on_success.on_error(env.new_string(error).unwrap().into())
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithString(
    mut env: JNIEnv,
    _: JClass,
    fd: jint,
    data: JString,
    encoding: jint,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let data = get_str(&mut env, &data, "");

    let on_success = AsyncCallback::clone_from_ptr(callback);

    match StringEncoding::try_from(encoding) {
        Ok(encoding) => {
            let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
                if error.is_some() {
                    on_success.on_error(JValue::Object(
                        error_to_jstring(error.unwrap()).as_obj(),
                    ))
                } else {
                    on_success.on_success(JObject::null().into())
                }
            }))
                .into_arc();
            node_fs::a_sync::write_file_with_str(fd, &data, encoding, callback);
        }
        Err(error) => {}
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithBytes(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    data: JByteArray,
    callback: jlong,
) {
    nativeWriteFileWithBytes(env, fd, data, callback);
}

pub(crate) fn nativeWriteFileWithBytes(env: JNIEnv, fd: jint, data: JByteArray, callback: jlong) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let data = env.new_global_ref(data).unwrap();
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();
        let data = unsafe { JByteArray::from_raw(data.as_raw()) };
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
        match node_fs::sync::write_file_with_bytes(fd, bytes) {
            Ok(_) => {
                // force drop of array to enable jni usage
                drop(data);
                callback.on_success(JObject::null().into())
            }
            Err(error) => {
                // force drop of array to enable jni usage
                drop(data);
                callback.on_error(JValue::Object(
                    error_to_jstring(error).as_obj(),
                ))
            }
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithBuffer(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    data: JByteBuffer,
    callback: jlong,
) {
    nativeWriteFileWithBuffer(env, fd, data, callback);
}

pub(crate) fn nativeWriteFileWithBuffer(env: JNIEnv, fd: jint, data: JByteBuffer, callback: jlong) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let data = env.new_global_ref(data).unwrap();
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let data = unsafe { JByteBuffer::from_raw(data.as_raw()) };

        match (env.get_direct_buffer_address(&data), env.get_direct_buffer_capacity(&data)) {
            (Ok(data), Ok(size)) => {
                let bytes = unsafe { std::slice::from_raw_parts_mut(data, size) };
                match node_fs::sync::write_file_with_bytes(fd, bytes) {
                    Ok(_) => {
                        callback.on_success(JObject::null().into())
                    }
                    Err(error) => callback.on_error(JValue::Object(
                        error_to_jstring(error).as_obj(),
                    )),
                }
            }
            (Err(error), _) => {
                callback.on_error(env.new_string(error.to_string()).unwrap().into())
            }
            _ => {}
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithStringFromPath(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    data: JString,
    encoding: jint,
    mode: c_int,
    flag: c_int,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let path = get_str(&mut env, &path, "");
    let data = get_str(&mut env, &data, "");

    let on_success = AsyncCallback::clone_from_ptr(callback);

    match StringEncoding::try_from(encoding) {
        Ok(encoding) => {
            let callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
                if let Some(error) = error {
                    on_success.on_error(JValue::Object(
                        error_to_jstring(error).as_obj(),
                    ))
                } else {
                    on_success.on_success(JObject::null().into())
                }
            }))
                .into_arc();
            node_fs::a_sync::write_file_with_str_from_path(&path, &data, encoding, mode, flag, callback);
        }
        Err(error) => {
            on_success.on_error(env.new_string(error).unwrap().into())
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithBytesFromPath(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    data: JByteArray,
    mode: c_int,
    flag: c_int,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let path = get_str(&mut env, &path, "");

    let data = env.new_global_ref(data).unwrap();
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();
        let data = unsafe { JByteArray::from_raw(data.as_raw()) };
        let data = unsafe {
            env
                .get_array_elements_critical(data.as_obj().into_inner(), ReleaseMode::NoCopyBack)
                .unwrap()
        };

        let bytes = unsafe {
            std::slice::from_raw_parts_mut(
                data.as_ptr() as *mut u8,
                data.len(),
            )
        };
        match node_fs::sync::write_file_with_bytes_from_path(&path, bytes, mode, flag) {
            Ok(_) => {
                // force drop of array to enable jni usage
                drop(data);
                callback.on_success(JObject::null().into())
            }
            Err(error) => {
                // force drop of array to enable jni usage
                drop(data);
                callback.on_error(JValue::Object(
                    error_to_jstring(error).as_obj(),
                ))
            }
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileFromPath(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    data: jlong,
    mode: c_int,
    flag: c_int,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let path = get_str(&mut env, &path, "");
    let data = unsafe { (&*(data as *mut Buffer)).clone() };
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let _ = jvm.attach_current_thread().unwrap();

        match node_fs::sync::write_file_with_bytes_from_path(&path, data.buffer(), mode, flag) {
            Ok(_) => callback.on_success(JObject::null().into()),
            Err(error) => callback.on_error(JValue::Object(
                error_to_jstring(error).as_obj(),
            )),
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWriteFileWithBufferFromPath(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    data: JByteBuffer,
    mode: c_int,
    flag: c_int,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let path = get_str(&mut env, &path, "");
    let data = env.new_global_ref(data).unwrap();
    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let data = unsafe { JByteBuffer::from_raw(data.as_raw()) };

        match (env.get_direct_buffer_address(&data), env.get_direct_buffer_capacity(&data)) {
            (Ok(data), Ok(size)) => {
                let bytes = unsafe { std::slice::from_raw_parts_mut(data, size) };
                match node_fs::sync::write_file_with_bytes_from_path(&path, bytes, mode, flag) {
                    Ok(_) => callback.on_success(JObject::null().into()),
                    Err(error) => callback.on_error(JValue::Object(
                        error_to_jstring(error).as_obj(),
                    )),
                }
            }
            (Err(error), _) => {
                callback.on_error(env.new_string(error.to_string()).unwrap().into())
            }
            _ => {}
        }
    });
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWritev(
    env: JNIEnv,
    _: JClass,
    fd: jint,
    buffers: JLongArray,
    position: jlong,
    callback: jlong,
) {
    nativeWritev(env, fd, buffers, position, callback);
}

pub(crate) fn nativeWritev(
    env: JNIEnv,
    fd: jint,
    buffers: JLongArray,
    position: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let buffers = env.new_global_ref(buffers).unwrap();

    let _ = node_core::thread::spawn(move || {
        let jvm = JVM.get().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();
        let array = unsafe { JLongArray::from_raw(buffers.as_raw()) };

        unsafe {
            match env.get_array_elements_critical(&array, ReleaseMode::NoCopyBack) {
                Ok(array) => {
                    let slice = std::slice::from_raw_parts_mut(array.as_ptr(), array.len());

                    let slice = slice.iter()
                        .map(|s| &*(*s as *mut Buffer).clone())
                        .collect::<Vec<Buffer>>();

                    drop(array);

                    match node_fs::sync::writev(fd, slice, position.try_into().unwrap()) {
                        Ok(wrote) => {
                            on_success.on_success(to_long(&mut env, wrote.try_into().unwrap()).into());
                        }
                        Err(error) => on_success.on_error(JValue::Object(
                            error_to_jstring(error).as_obj(),
                        )),
                    }
                }
                Err(error) => {
                    on_success.on_error(JValue::Object(
                        env.new_string(error.to_string()).unwrap().into()
                    ))
                }
            }
        }
    });
}