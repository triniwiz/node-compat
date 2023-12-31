use std::ffi::{c_void, CStr, CString};
use log::error;
use node_c::NodeError;

mod buffer;
mod fs;



extern "C" fn success(fd: i32, data: *mut c_void) {
    println!("done {:?} {:?}", fd, data);
}

extern "C" fn error(error: *mut NodeError, data: *mut c_void) {
    let message = unsafe {node_c::node_error_get_message(error)};
    let message = unsafe {CStr::from_ptr(message)};
    println!("error {:?} {:?}", message, data);
}

fn main() {
    let mut test_txt = std::env::current_dir().unwrap();
    test_txt.push("data/test.txt");
    let a = test_txt.to_string_lossy();
    let path = CString::new(a.to_string()).unwrap();
    let data = vec!["Osei"];
    let ptr_success = success as *mut _;
    let ptr_error = error  as *mut _;
    let cb = unsafe { node_c::fs_async_create_async_i32_closure(ptr_success, ptr_error, data.as_ptr() as *mut c_void) };

    std::mem::forget(data);

    node_c::fs_async_open(path.as_ptr(), 00000000, 0o666, cb);
    //buffer::run();
    // fs::run();
    loop {

    }
}
