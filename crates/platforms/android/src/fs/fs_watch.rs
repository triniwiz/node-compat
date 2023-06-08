use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::NonNull;
use std::sync::Arc;

use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jlong, jobject, JNI_TRUE, jint};
use jni::JNIEnv;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use node_fs::a_sync::{AsyncClosure, close, WatchEvent};
use node_fs::prelude::FsEncodingType;
use crate::fs::a_sync::AsyncCallback;
use crate::fs::{FILE_FS_WATCH_CLASS, FILE_FS_WATCH_EVENT_CLASS, JVM};
use crate::fs::prelude::*;

type WatcherCallbackMap = Arc<Mutex<HashMap<AsyncCallback, FsWatchCallback>>>;

#[allow(dead_code)]
pub struct FsWatchCallback {
    callback: AsyncCallback,
    inner: Arc<AsyncClosure<WatchEvent, std::io::Error>>,
}

fn watcher_callback_map() -> &'static WatcherCallbackMap {
    static INSTANCE: OnceCell<WatcherCallbackMap> = OnceCell::new();
    INSTANCE.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
}

fn build_string_op<'a>(env: &JNIEnv<'a>, value: Option<NonNull<c_char>>) -> Option<JString<'a>> {
    if let Some(value) = value {
        let string = unsafe { CStr::from_ptr(value.as_ptr()) };
        if let Ok(res) = env.new_string(string.to_string_lossy().as_ref()) {
            return Some(res);
        }
    }
    None
}

fn build_file_watch_event<'a>(env: &mut JNIEnv<'a>, event: WatchEvent) -> JObject<'a> {
    let clazz = find_class(FILE_FS_WATCH_EVENT_CLASS).unwrap();
    let object = env.new_object(clazz, "()V", &[]).unwrap();
    let filename = build_string_op(&env, event.filename).unwrap_or(JObject::null().into());
    let event_type = build_string_op(&env, event.event_type).unwrap_or(JObject::null().into());
    let _ = env.set_field(&object, "fileName", "Ljava/lang/String;", filename.into());
    let _ = env.set_field(&object, "eventType", "Ljava/lang/String;", event_type.into());
    object
}

fn build_fs_watch<'a>(env: &mut JNIEnv<'a>, file_name: &str, callback: jlong) -> JObject<'a> {
    let clazz = find_class(FILE_FS_WATCH_CLASS).unwrap();
    let object = env.new_object(clazz, "()V", &[]).unwrap();
    let _ = env.set_field(
        &object,
        "fileName",
        "Ljava/lang/String;",
        env.new_string(file_name).unwrap().into(),
    );
    let _ = env.set_field(&object, "native", "J", callback.into());
    object
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeWatch(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    persistent: jboolean,
    recursive: jboolean,
    encoding: jint,
    callback: jlong,
) -> jobject {
    let cb = callback;
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);

    match FsEncodingType::try_from(encoding) {
        Ok(encoding) => {
            let on_success = callback.clone();

            let item = FsWatchCallback {
                callback: callback.clone(),
                inner: Arc::new(AsyncClosure {
                    callback: Box::new(move |event, error| {
                        let jvm = JVM.get().unwrap();
                        let mut env = jvm.attach_current_thread().unwrap();
                        if error.is_some() {
                            on_success.on_error(jni::objects::JValue::Object(
                                error_to_jstring(error.unwrap()).as_obj(),
                            ))
                        } else {
                            on_success.on_success(build_file_watch_event(&mut env, event.unwrap()).into())
                        }
                    }),
                }),
            };


            let inner = Arc::clone(&item.inner);
            // call on another thread ?
            let mut map = watcher_callback_map().lock();
            let _ = map.insert(callback, item);

            let path = get_str(&mut env, &path, "");

            node_fs::a_sync::watch(
                path.as_ref(),
                persistent == JNI_TRUE,
                recursive == JNI_TRUE,
                encoding,
                inner,
            );
        }
        Err(error) => {
            callback.on_error(env.new_string(error).unwrap().into())
        }
    }


    build_fs_watch(&mut env, path.as_ref(), cb).into_inner()
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FileSystem_nativeUnwatchFile(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);

    let mut map = watcher_callback_map().lock();

    if let Some(cb) = map.get(&callback).map(|c| Arc::clone(&c.inner)) {
        map.remove(&callback);
        node_fs::a_sync::watcher_unref(get_str(&mut env, &path, "").as_ref(), cb);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FsWatcher_nativeUnref(
    mut env: JNIEnv,
    _: JClass,
    filename: JString,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);

    let mut map = watcher_callback_map().lock();

    if let Some(cb) = map.get(&callback).map(|c| Arc::clone(&c.inner)) {
        map.remove(&callback);
        node_fs::a_sync::watcher_unref(get_str(&mut env, &filename, "").as_ref(), cb);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_FsWatcher_nativeRef(
    mut env: JNIEnv,
    _: JClass,
    filename: JString,
    callback: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);

    let on_success = Arc::clone(&callback);

    let mut map = watcher_callback_map().lock();

    if map.contains_key(&callback) {
        return;
    }

    let inner = Arc::new(AsyncClosure::new(Box::new(move |event, error| {
        let jvm = JVM.get().unwrap();
        let mut env = jvm.attach_current_thread().unwrap();
        if let Some(error) = error {
            on_success.on_error(jni::objects::JValue::Object(
                error_to_jstring(error).as_obj(),
            ))
        } else {
            on_success.on_success(build_file_watch_event(&mut env, event.unwrap()).into())
        }
    })));

    let item = FsWatchCallback {
        callback: callback.clone(),
        inner: Arc::clone(&inner),
    };

    map.insert(callback, item);
    node_fs::a_sync::watcher_ref(get_str(&mut env, &filename, "").as_ref(), inner);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node_1compat_fs_WatcherEvent_nativeClose(
    mut env: JNIEnv,
    _: JClass,
    filename: JString,
    callback: jlong,
    on_close: jlong,
) {
    let callback = callback as *const AsyncCallback;
    let on_close = on_close as *const AsyncCallback;
    let callback = AsyncCallback::clone_from_ptr(callback);
    let on_close = AsyncCallback::clone_from_ptr(on_close);

    let map = watcher_callback_map().lock();
    if let Some(item) = map.get(&callback).map(|c| Arc::clone(&c.inner)) {
        let close_callback = AsyncClosure::<(), std::io::Error>::new(Box::new(move |_, error| {
            if let Some(error) = error {
                on_close.on_error(jni::objects::JValue::Object(
                    error_to_jstring(error).as_obj(),
                ))
            } else {
                on_close.on_success(JObject::null().into())
            }
        }))
            .into_arc();
        node_fs::a_sync::watcher_close(get_str(&mut env, &filename, "").as_ref(), item, close_callback);
    }
}
