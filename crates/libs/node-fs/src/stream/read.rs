use std::fs::File;

#[repr(C)]
pub enum ReadStream {
    File(File),
    Buffer(Buffer)
}
