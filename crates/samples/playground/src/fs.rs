use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::sync::Arc;
use node_buffer::Buffer;
use node_fs::file_handle::FileHandle;

pub fn run() {
    let mut test_txt = std::env::current_dir().unwrap();
    test_txt.push("data/test.txt");
    let callback = Arc::new(node_fs::a_sync::AsyncClosure::<FileHandle, std::io::Error>::new(
        Box::new(|result, error|{
            println!("error {:?}", error);
            match result {
                None => {}
                Some(mut handle) => {
                    println!("handle");
                    let cb =  Arc::new(node_fs::a_sync::AsyncClosure::<(), std::io::Error>::new(
                        Box::new(|result, error|{
                            println!("write error {:?}", error);
                        })));
                    let ret = node_fs::sync::read_file_with_fd(handle.fd() as i32, 0);

                    println!("data {}", ret.unwrap().to_string(None, None, None));

                    handle.append_file_with_str(" NICE!!", cb.clone());
                }
            }
        })
    ));
    FileHandle::new_async(test_txt.as_os_str().to_string_lossy().as_ref(), node_fs::FILE_ACCESS_OPTIONS_R_OK, node_fs::FILE_OPEN_OPTIONS_O_RDWR, callback.clone());
}