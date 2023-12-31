mod a_sync;
mod async_callback;
mod file_dir;
mod file_dirent;
mod file_handle;
mod file_stat;
mod file_watcher;
mod fs_watch;
mod sync;
mod prelude;


use std::collections::HashMap;
use std::ffi::c_void;
use jni::JavaVM;
use jni::objects::{GlobalRef, JValue};
use jni::sys::jint;
use once_cell::sync::OnceCell;
use node_fs::{FILE_COPY_OPTIONS_COPYFILE_EXCL, FILE_COPY_OPTIONS_COPYFILE_FICLONE, FILE_COPY_OPTIONS_COPYFILE_FICLONE_FORCE};


pub(crate) const FS_CONSTANTS_CLASS: &str = "org/nativescript/node_compat/fs/filesystem/Constants";
pub(crate) const FILE_SYSTEM_CLASS: &str = "org/nativescript/node_compat/fs/FileSystem";
pub(crate) const FILE_DIRENT_CLASS: &str = "org/nativescript/node_compat/fs/FileDirent";
pub(crate) const FILE_DIR_CLASS: &str = "org/nativescript/node_compat/fs/FileDir";
pub(crate) const FILE_STAT_CLASS: &str = "org/nativescript/node_compat/fs/FileStat";
pub(crate) const FILE_HANDLE_CLASS: &str = "org/nativescript/node_compat/fs/FileHandle";
pub(crate) const FILE_WATCHER_CLASS: &str = "org/nativescript/node_compat/fs/FileWatcher";
pub(crate) const FILE_WATCHER_EVENT_CLASS: &str =
    "org/nativescript/node_compat/fs/FileWatcher$Event";
pub(crate) const FILE_FS_WATCH_CLASS: &str = "org/nativescript/node_compat/fs/FsWatcher";
pub(crate) const FILE_FS_WATCH_EVENT_CLASS: &str =
    "org/nativescript/node_compat/fs/FsWatcher$Event";

pub(crate) const STRING_CLASS: &str = "java/lang/String";
pub(crate) const BOOLEAN_CLASS: &str = "java/lang/Boolean";
pub(crate) const INTEGER_CLASS: &str = "java/lang/Integer";
pub(crate) const LONG_CLASS: &str = "java/lang/Long";
pub(crate) const FLOAT_CLASS: &str = "java/lang/Float";
pub(crate) const DOUBLE_CLASS: &str = "java/lang/Double";
pub(crate) const OBJECT_CLASS: &str = "java/lang/Object";

pub static JVM: OnceCell<JavaVM> = OnceCell::new();

pub static JVM_CLASS_CACHE: OnceCell<parking_lot::RwLock<HashMap<&'static str, GlobalRef>>> =
    OnceCell::new();

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *const c_void) -> jint {
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Debug));

    if let Ok(mut env) = vm.get_env() {
        let clazz = env.find_class(FS_CONSTANTS_CLASS).unwrap();

        JVM_CLASS_CACHE.get_or_init(|| {
            let mut map = HashMap::new();
            map.insert(FS_CONSTANTS_CLASS, env.new_global_ref(clazz).unwrap());
            map.insert(
                FILE_SYSTEM_CLASS,
                env.new_global_ref(env.find_class(FILE_SYSTEM_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                FILE_DIRENT_CLASS,
                env.new_global_ref(env.find_class(FILE_DIRENT_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                FILE_DIR_CLASS,
                env.new_global_ref(env.find_class(FILE_DIR_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                FILE_STAT_CLASS,
                env.new_global_ref(env.find_class(FILE_STAT_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                FILE_HANDLE_CLASS,
                env.new_global_ref(env.find_class(FILE_HANDLE_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                FILE_WATCHER_CLASS,
                env.new_global_ref(env.find_class(FILE_WATCHER_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                FILE_WATCHER_EVENT_CLASS,
                env.new_global_ref(env.find_class(FILE_WATCHER_EVENT_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                FILE_FS_WATCH_CLASS,
                env.new_global_ref(env.find_class(FILE_FS_WATCH_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                FILE_FS_WATCH_EVENT_CLASS,
                env.new_global_ref(env.find_class(FILE_FS_WATCH_EVENT_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                STRING_CLASS,
                env.new_global_ref(env.find_class(STRING_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                BOOLEAN_CLASS,
                env.new_global_ref(env.find_class(BOOLEAN_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                INTEGER_CLASS,
                env.new_global_ref(env.find_class(INTEGER_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                LONG_CLASS,
                env.new_global_ref(env.find_class(LONG_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                FLOAT_CLASS,
                env.new_global_ref(env.find_class(FLOAT_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                DOUBLE_CLASS,
                env.new_global_ref(env.find_class(DOUBLE_CLASS).unwrap())
                    .unwrap(),
            );
            map.insert(
                OBJECT_CLASS,
                env.new_global_ref(env.find_class(OBJECT_CLASS).unwrap())
                    .unwrap(),
            );
            parking_lot::RwLock::new(map)
        });

        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_RDONLY", "I")
                .unwrap(),
            libc::O_RDONLY.into(),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_WRONLY", "I")
                .unwrap(),
            libc::O_WRONLY.into(),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_RDWR", "I")
                .unwrap(),
            libc::O_RDWR.into(),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_CREAT", "I")
                .unwrap(),
            libc::O_CREAT.into(),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_EXCL", "I")
                .unwrap(),
            libc::O_EXCL.into(),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_NOCTTY", "I")
                .unwrap(),
            libc::O_NOCTTY.into(),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_TRUNC", "I")
                .unwrap(),
            libc::O_TRUNC.into(),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_APPEND", "I")
                .unwrap(),
            libc::O_APPEND.into(),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_DIRECTORY", "I")
                .unwrap(),
            libc::O_DIRECTORY.into(),
        );

        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_NOATIME", "I")
                .unwrap(),
            JValue::Int(libc::MS_NOATIME as jint));

        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_NOFOLLOW", "I")
                .unwrap(),
            JValue::Int(libc::O_NOFOLLOW),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_SYNC", "I")
                .unwrap(),
            JValue::Int(libc::O_SYNC),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_DSYNC", "I")
                .unwrap(),
            JValue::Int(libc::O_DSYNC),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_SYMLINK", "I")
                .unwrap(),
            JValue::Int(-1),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_DIRECT", "I")
                .unwrap(),
            JValue::Int(0x4000),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "O_NONBLOCK", "I")
                .unwrap(),
            JValue::Int(libc::O_NONBLOCK),
        );

        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "F_OK", "I")
                .unwrap(),
            JValue::Int(libc::F_OK),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "R_OK", "I")
                .unwrap(),
            JValue::Int(libc::R_OK),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "W_OK", "I")
                .unwrap(),
            JValue::Int(libc::W_OK),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "X_OK", "I")
                .unwrap(),
            JValue::Int(libc::X_OK),
        );

        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "COPYFILE_EXCL", "I")
                .unwrap(),
            JValue::Int(FILE_COPY_OPTIONS_COPYFILE_EXCL as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "COPYFILE_FICLONE", "I")
                .unwrap(),
            JValue::Int(FILE_COPY_OPTIONS_COPYFILE_FICLONE as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "COPYFILE_FICLONE_FORCE", "I")
                .unwrap(),
            JValue::Int(FILE_COPY_OPTIONS_COPYFILE_FICLONE_FORCE as jint),
        );

        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IFMT", "I")
                .unwrap(),
            JValue::Int(libc::S_IFMT as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IFREG", "I")
                .unwrap(),
            JValue::Int(libc::S_IFREG as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IFDIR", "I")
                .unwrap(),
            JValue::Int(libc::S_IFDIR as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IFCHR", "I")
                .unwrap(),
            JValue::Int(libc::S_IFCHR as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IFBLK", "I")
                .unwrap(),
            JValue::Int(libc::S_IFBLK as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IFIFO", "I")
                .unwrap(),
            JValue::Int(libc::S_IFIFO as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IFLNK", "I")
                .unwrap(),
            JValue::Int(libc::S_IFLNK as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IFSOCK", "I")
                .unwrap(),
            JValue::Int(libc::S_IFSOCK as jint),
        );

        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IRWXU", "I")
                .unwrap(),
            JValue::Int(libc::S_IRWXU as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IRUSR", "I")
                .unwrap(),
            JValue::Int(libc::S_IRUSR as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IWUSR", "I")
                .unwrap(),
            JValue::Int(libc::S_IWUSR as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IXUSR", "I")
                .unwrap(),
            JValue::Int(libc::S_IXUSR as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IRWXG", "I")
                .unwrap(),
            JValue::Int(libc::S_IRWXG as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IRGRP", "I")
                .unwrap(),
            JValue::Int(libc::S_IRGRP as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IWGRP", "I")
                .unwrap(),
            JValue::Int(libc::S_IWGRP as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IXGRP", "I")
                .unwrap(),
            JValue::Int(libc::S_IXGRP as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IRWXO", "I")
                .unwrap(),
            JValue::Int(libc::S_IRWXO as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IROTH", "I")
                .unwrap(),
            JValue::Int(libc::S_IROTH as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IWOTH", "I")
                .unwrap(),
            JValue::Int(libc::S_IWOTH as jint),
        );
        let _ = env.set_static_field(
            &clazz,
            env.get_static_field_id(FS_CONSTANTS_CLASS, "S_IXOTH", "I")
                .unwrap(),
            JValue::Int(libc::S_IXOTH as jint),
        );
    }

    JVM.get_or_init(|| vm);

    jni::sys::JNI_VERSION_1_8
}