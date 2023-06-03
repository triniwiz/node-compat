use std::path::Path;
use std::sync::Arc;

pub fn run() {
    let mut test_txt = std::env::current_dir().unwrap();
    test_txt.push("data/test.txt");
    let callback = Arc::new(node_fs::a_sync::AsyncClosure::<i32, std::io::Error>::new(
        Box::new(|result, error|{
            print!("result {:?} {:?}", result , error);
            match result {
                None => {}
                Some(fd) => {

                    let callback = Arc::new(node_fs::a_sync::AsyncClosure::<node_fs::file_stat::FileStat, std::io::Error>::new(
                        Box::new(|result, error|{
                            print!("result {:?} {:?}", result , error);
                            match result {
                                None => {}
                                Some(stat) => {

                                    println!("stat {:?}", stat);

                                    //node_fs::sync::close_fd(fd);

                                }
                            }
                        })
                    ));

                   node_fs::a_sync::fstat(fd, callback.clone());
                }
            }
        })
    ));
    node_fs::a_sync::open(test_txt.as_os_str().to_string_lossy().as_ref(), node_fs::FILE_ACCESS_OPTIONS_R_OK, node_fs::FILE_OPEN_OPTIONS_O_RDWR, callback.clone());
}