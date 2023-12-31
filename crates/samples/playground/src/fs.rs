use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::sync::Arc;
use node_buffer::Buffer;
use node_fs::a_sync::{FileWatchEvent, WatchEvent};
use node_fs::file_handle::FileHandle;
use node_fs::FsEncodingType;
use node_fs::sync::{AppendFileOptions, ReadFileOptions};

pub fn run() {
    let mut test_txt = std::env::current_dir().unwrap();
    test_txt.push("data/test.txt");
    let callback = Arc::new(node_fs::a_sync::AsyncClosure::<FileHandle, std::io::Error>::new(
        Box::new(|result, error| {
            println!("error {:?}", error);
            match result {
                None => {}
                Some(mut handle) => {
                    println!("handle");
                    let cb = Arc::new(node_fs::a_sync::AsyncClosure::<(), std::io::Error>::new(
                        Box::new(|result, error| {
                            println!("write error {:?}", error);
                        })));

                    let mut options = ReadFileOptions::default();
                    options.set_encoding(FsEncodingType::Utf8);
                    let ret = node_fs::sync::read_file_with_fd(handle.fd() as i32, options);

                    println!("data {:?}", ret.unwrap().get_string_value());

                    let opts = AppendFileOptions::default();

                    handle.append_file_with_str(" NICE!!", opts, cb.clone());
                }
            }
        })
    ));
    FileHandle::new_async(test_txt.as_os_str().to_string_lossy().as_ref(), node_fs::FILE_ACCESS_OPTIONS_R_OK, node_fs::FILE_OPEN_OPTIONS_O_RDWR, callback.clone());


    // let watch_callback = Arc::new(node_fs::a_sync::AsyncClosure::<FileWatchEvent, std::io::Error>::new(
    //     Box::new(|result, error|{
    //         println!("watch error {:?}", error);
    //         match result {
    //             None => {}
    //             Some(event) => {
    //                 println!("watch event {:?}", event);
    //             }
    //         }
    //     })
    // ));
    //
    // node_fs::a_sync::watch_file(test_txt.as_os_str().to_string_lossy().as_ref(), false, true, 0, watch_callback);
    //


    let watch_callback = Arc::new(node_fs::a_sync::AsyncClosure::<WatchEvent, std::io::Error>::new(
        Box::new(|result, error| {
            if let Some(error) = error {
                println!("watch error {:?}", error);
            }
            match result {
                None => {}
                Some(event) => {
                    println!("watch event {:?}", event);
                }
            }
        })
    ));

    let mut current = std::env::current_dir().unwrap().to_string_lossy().as_ref().to_string() + "/";

    node_fs::a_sync::watch(current.as_str(), false, true, FsEncodingType::Utf8,  watch_callback);


    loop {}
}