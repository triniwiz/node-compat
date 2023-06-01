use std::ffi::{c_void, CString};
use std::sync::{Arc};
use base64::Engine;
use parking_lot::RwLock;

#[repr(transparent)]
enum BufferInner {
   Allocated(Arc<RwLock<Vec<u8>>>),
    Reference(*mut u8, usize)
}

#[repr(C)]
pub enum StringEncoding {
    Ascii,
    Utf8,
    Utf16le,
    Ucs2,
    Base64,
    Latin1,
    Binary,
    Hex,
}

struct BufferBuilder{
    size: usize,
    fill: Option<Vec<u8>>,
    string: Option<CString>,
    fill_encoding: StringEncoding,
    reference: Option<(*mut u8, usize)>
}

impl BufferBuilder {
    pub fn build(&self) -> Buffer {
        match &self.string {
            None => {  }
            Some(string) => {
                let ret = match self.fill_encoding {
                    StringEncoding::Ascii => {
                       let string =  string.to_string_lossy();
                        string.as_bytes().to_vec()
                    }
                    StringEncoding::Utf8 => {
                        string.into_bytes()
                    }
                    StringEncoding::Utf16le => {
                        let (decoded, _) = encoding_rs::UTF_8.decode_without_bom_handling(string.as_bytes());
                        decoded.as_bytes().to_vec()
                    }
                    StringEncoding::Ucs2 => {
                        let string = string.as_bytes();
                        let length = string.len();
                        let string = unsafe {std::slice::from_raw_parts(string.as_ptr() as *const u16, length / 2)};
                        let mut buf = vec![0_u8; length ];
                        let decoded = ucs2::decode(string, buf.as_mut_slice()).unwrap_or(0);
                        buf.shrink_to(decoded)
                    }
                    StringEncoding::Base64 => {
                        // todo error
                        base64::engine::general_purpose::STANDARD.decode(string.as_bytes()).unwrap()
                    }
                    StringEncoding::Binary | StringEncoding::Latin1 => {
                        let (decoded, _) = encoding_rs::UTF_8.decode_without_bom_handling(string.as_bytes());
                        decoded.as_bytes().to_vec()
                    }
                    StringEncoding::Hex => {
                        // todo error
                        hex::decode(string.as_bytes()).unwrap()
                    }
                };

               return Buffer(
                   BufferInner::Allocated(
                       Arc::new(
                           RwLock::new(
                               ret
                           )
                       )
                   )
               )
            }
        }

        if self.fill.is_some() {
            let buf = self.fill.unwrap();
            return Buffer(
                BufferInner::Allocated(
                    Arc::new(
                        RwLock::new(
                            buf
                        )
                    )
                )
            )
        }

        Buffer(match self.reference {
            None => {
                BufferInner::Allocated(
                    Arc::new(
                        RwLock::new(
                            Vec::with_capacity(self.size)
                        )
                    )
                )
            }
            Some((data, size)) => {
                BufferInner::Reference(data, size)
            }
        })
    }

    pub fn size(&mut self, value: usize) -> &mut Self {
        self.size = value;
        self.fill = None;
        self.string = None;
        self.fill_encoding = StringEncoding::Utf8;
        self
    }

    pub fn fill_text(&mut self, value: CString, encoding: StringEncoding) -> &mut Self {
        self.fill_encoding = encoding;
        self.string = Some(value);
        self
    }

    pub fn reference(&mut self, data: *mut u8, size: usize) -> &mut Self {
        self.reference = Some((data, size));
        self.string = None;
        self
    }
}

#[repr(C)]
pub struct Buffer(BufferInner);

impl Buffer {

    pub fn from_vec(value: Vec<u8>) -> Self {
        Self(
            BufferInner::Allocated(
                Arc::new(
                    RwLock::new(
                        value
                    )
                )
            )
        )
    }

    pub fn builder() -> BufferBuilder {
        BufferBuilder {
            size: 0,
            fill: None,
            string: None,
            fill_encoding: StringEncoding::Utf8,
            reference: None,
        }
    }

    pub fn from_string(value: CString, encoding: StringEncoding) -> Self {
       Self::builder()
           .fill_text(value, encoding)
           .build()
    }

    pub fn length(&self) -> usize {
        match self.0 {
            BufferInner::Allocated(ref allocated) => {
                allocated.read().len()
            }
            BufferInner::Reference(data, size) =>{
                if data.is_null() {
                    return 0;
                }
                size
            }
        }
    }

    pub fn buffer(&self) -> &[u8] {
        match self.0 {
            BufferInner::Allocated(ref buf) => {
                let buf = buf.read();
                unsafe {
                    std::slice::from_raw_parts(buf.as_ptr(), buf.len())
                }
            }
            BufferInner::Reference(data, size) => {
                if data.is_null() || size == 0 {
                    return &[]
                }
                return unsafe {std::slice::from_raw_parts(data, size)}
            }
        }
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        match self.0 {
            BufferInner::Allocated(ref buf) => {
                let mut buf = buf.write();
                unsafe {
                    std::slice::from_raw_parts_mut(buf.as_mut_ptr(), buf.len())
                }
            }
            BufferInner::Reference(data, size) => {
                if data.is_null() || size == 0 {
                    return &mut []
                }
                return unsafe {std::slice::from_raw_parts_mut(data, size)}
            }
        }
    }
}