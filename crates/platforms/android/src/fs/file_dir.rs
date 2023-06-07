use std::io::Error;
use jni::objects::{JClass, JObject};
use jni::sys::jlong;
use jni::{sys::jobject, JNIEnv};
use node_fs::file_dir::FileDir;
use crate::fs::{FILE_DIR_CLASS, JVM};
use crate::fs::prelude::*;


use super::a_sync::AsyncCallback;
use super::file_dirent::build_dirent;

pub(crate) fn build_dir<'a>(env: &mut JNIEnv<'a>, dir: FileDir) -> JObject<'a> {
    let clazz = find_class(FILE_DIR_CLASS).unwrap();
    let dir = Box::into_raw(Box::new(dir));
    env.new_object(clazz, "(J)V", &[(dir as i64).into()])
        .unwrap()
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileDir_nativeCloseSync(
    mut env: JNIEnv,
    _: JClass,
    file_dir: jlong,
) {
    let dir: *mut FileDir = file_dir as _;
    if !dir.is_null() {
        let dir = unsafe { Box::from_raw(dir) };
        let result = dir.close();
        if let Err(error) = result {
            let _ = env.throw(error.to_string());
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileDir_nativeClose(
    _: JNIEnv,
    _: JClass,
    file_dir: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let dir: *mut FileDir = file_dir as _;
    if !dir.is_null() {
        let dir = unsafe { Box::from_raw(dir) };
        dir.close_async(Box::new(move |error| {
            match error {
                None => {
                    on_success.on_success(JObject::null().into())
                }
                Some(error) => {
                    on_success.on_error(jni::objects::JValue::Object(
                        error_to_jstring(error).as_obj(),
                    ))
                }
            }
        }));
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileDir_nativePath(
    env: JNIEnv,
    _: JClass,
    file_dir: jlong,
) -> jobject {
    let dir: *mut FileDir = file_dir as _;
    if !dir.is_null() {
        let dir = unsafe { Box::from_raw(dir) };
        return env.new_string(dir.path()).unwrap().into_inner();
    }
    JObject::null().into_inner()
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileDir_nativeDispose(
    _env: JNIEnv,
    _: JClass,
    file_dir: jlong,
) {
    let dir: *mut FileDir = file_dir as _;
    if !dir.is_null() {
        let _ = unsafe { Box::from_raw(dir) };
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileDir_nativeReadSync(
    mut env: JNIEnv,
    _: JClass,
    file_dir: jlong,
) -> jobject {
    let dir: *mut FileDir = file_dir as _;
    if !dir.is_null() {
        let dir = unsafe { Box::from_raw(dir) };
        match dir.read() {
            Ok(dirent) => {
                return build_dirent(&mut env, dirent).into_inner();
            }
            Err(error) => {
                let _ = env.throw(error.to_string());
            }
        }
    }
    return JObject::null().into_inner();
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileDir_nativeRead(
    _: JNIEnv,
    _: JClass,
    file_dir: jlong,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_success = AsyncCallback::clone_from_ptr(callback);
    let dir: *mut FileDir = file_dir as _;
    if !dir.is_null() {
        let dir = unsafe { Box::from_raw(dir) };
        dir.read_async(Box::new(move |dirent, error| {
            if error.is_some() {
                on_success.on_error(jni::objects::JValue::Object(
                    error_to_jstring(error.unwrap()).as_obj(),
                ))
            } else {
                let jvm = JVM.get().unwrap();
                let mut env = jvm.attach_current_thread().unwrap();
                on_success.on_success(build_dirent(&mut env, dirent.unwrap()).into())
            }
        }));
    }
}
