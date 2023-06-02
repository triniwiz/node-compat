use std::ffi::{c_void, CString};
use std::fmt::{Debug, Display, format, Formatter};
use std::io::Write;
use std::str::Utf8Error;
use std::sync::{Arc};
use base64::Engine;
use parking_lot::RwLock;
use byteorder::{BigEndian, ByteOrder, LittleEndian};

enum BufferInner {
    Empty,
    Allocated(Arc<RwLock<Vec<u8>>>),
    Reference(Arc<RwLock<(*mut u8, usize)>>),
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

pub struct BufferBuilder {
    size: usize,
    fill: Option<Vec<u8>>,
    string: Option<CString>,
    fill_encoding: StringEncoding,
    reference: Option<(*mut u8, usize)>,
}

impl BufferBuilder {
    pub fn build(&self) -> Buffer {
        match &self.string {
            None => {}
            Some(string) => {
                let ret = match self.fill_encoding {
                    StringEncoding::Ascii => {
                        let string = string.to_string_lossy();
                        string.as_bytes().to_vec()
                    }
                    StringEncoding::Utf8 => {
                        string.as_bytes().to_vec()
                    }
                    StringEncoding::Utf16le => {
                       string
                           .to_string_lossy()
                           .to_string()
                           .encode_utf16()
                           .flat_map(|c|{
                               let mut bytes = [0; 2];
                               LittleEndian::write_u16(&mut bytes, c);
                               bytes.to_vec()
                           })
                           .collect::<Vec<u8>>()
                    }
                    StringEncoding::Ucs2 => {
                        let string = string.as_bytes();
                        let length = string.len();
                        let string = unsafe { std::slice::from_raw_parts(string.as_ptr() as *const u16, length / 2) };
                        let mut buf = vec![0_u8; length];
                        let decoded = ucs2::decode(string, buf.as_mut_slice()).unwrap_or(0);
                        buf.shrink_to(decoded);
                        buf
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
                );
            }
        }

        if let Some(buf) = self.fill.as_ref() {
            return Buffer(
                BufferInner::Allocated(
                    Arc::new(
                        RwLock::new(
                            buf.to_vec()
                        )
                    )
                )
            );
        }

        Buffer(match self.reference {
            None => {
                BufferInner::Allocated(
                    Arc::new(
                        RwLock::new(
                            vec![0_u8; self.size]
                        )
                    )
                )
            }
            Some((data, size)) => {
                BufferInner::Reference(Arc::new(
                    RwLock::new((data, size))
                ))
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

impl Default for Buffer {
    fn default() -> Self {
        Self(BufferInner::Empty)
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ret = self.to_string(Some(StringEncoding::Hex), None, None).chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "<Buffer {}>", ret)
    }
}

impl Buffer {
    pub fn write_int8(&mut self, value: i8, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];

        unsafe { std::ptr::write(buffer.as_mut_ptr(), value as u8) }
    }

    pub fn write_uint8(&mut self, value: u8, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let ptr = unsafe { buffer.as_mut_ptr().offset(offset.unwrap_or(0) as isize) };

        unsafe { std::ptr::write(ptr, value) }
    }

    pub fn write_uint16be(&mut self, value: u16, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        BigEndian::write_u16(buffer, value);
    }

    pub fn write_uint16le(&mut self, value: u16, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        LittleEndian::write_u16(buffer, value);
    }

    pub fn write_int16be(&mut self, value: i16, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        BigEndian::write_i16(buffer, value);
    }

    pub fn write_int16le(&mut self, value: i16, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        LittleEndian::write_i16(buffer, value);
    }

    pub fn write_uint32be(&mut self, value: u32, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        BigEndian::write_u32(buffer, value);
    }

    pub fn write_uint32le(&mut self, value: u32, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        LittleEndian::write_u32(buffer, value);
    }

    pub fn write_int32be(&mut self, value: i32, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        BigEndian::write_i32(buffer, value);
    }

    pub fn write_int32le(&mut self, value: i32, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        LittleEndian::write_i32(buffer, value);
    }

    pub fn write_float_be(&mut self, value: f32, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        BigEndian::write_f32(buffer, value);
    }

    pub fn write_float_le(&mut self, value: f32, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        LittleEndian::write_f32(buffer, value);
    }

    pub fn write_double_be(&mut self, value: f64, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        BigEndian::write_f64(buffer, value);
    }

    pub fn write_double_le(&mut self, value: f64, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        LittleEndian::write_f64(buffer, value);
    }

    pub fn write_big_int64be(&mut self, value: i64, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        BigEndian::write_i64(buffer, value);
    }

    pub fn write_big_int64le(&mut self, value: i64, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        LittleEndian::write_i64(buffer, value);
    }

    pub fn write_big_uint64be(&mut self, value: u64, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        BigEndian::write_u64(buffer, value);
    }

    pub fn write_big_uint64le(&mut self, value: u64, offset: Option<usize>) {
        let buffer = self.buffer_mut();
        let length = buffer.len();
        let buffer = &mut buffer[offset.unwrap_or(0)..length];
        LittleEndian::write_u64(buffer, value);
    }

    pub fn read_int8(&self, offset: Option<usize>) -> i8 {
        unsafe { *(self.buffer().get(offset.unwrap_or(0)).unwrap() as *const _ as *const i8) }
    }

    pub fn read_uint8(&self, offset: Option<usize>) -> u8 {
        unsafe { *(self.buffer().get(offset.unwrap_or(0)).unwrap()) }
    }

    pub fn read_uint16be(&self, offset: Option<usize>) -> u16 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        BigEndian::read_u16(buffer)
    }

    pub fn read_uint16le(&self, offset: Option<usize>) -> u16 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        LittleEndian::read_u16(buffer)
    }

    pub fn read_int16be(&self, offset: Option<usize>) -> i16 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        BigEndian::read_i16(buffer)
    }

    pub fn read_int16le(&self, offset: Option<usize>) -> i16 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        LittleEndian::read_i16(buffer)
    }

    pub fn read_uint32be(&self, offset: Option<usize>) -> u32 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        BigEndian::read_u32(buffer)
    }

    pub fn read_uint32le(&self, offset: Option<usize>) -> u32 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        LittleEndian::read_u32(buffer)
    }

    pub fn read_int32be(&self, offset: Option<usize>) -> i32 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        BigEndian::read_i32(buffer)
    }

    pub fn read_int32le(&self, offset: Option<usize>) -> i32 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        LittleEndian::read_i32(buffer)
    }

    pub fn read_float_be(&self, offset: Option<usize>) -> f32 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        BigEndian::read_f32(buffer)
    }

    pub fn read_float_le(&self, offset: Option<usize>) -> f32 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        LittleEndian::read_f32(buffer)
    }

    pub fn read_double_be(&self, offset: Option<usize>) -> f64 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        BigEndian::read_f64(buffer)
    }

    pub fn read_double_le(&self, offset: Option<usize>) -> f64 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        LittleEndian::read_f64(buffer)
    }

    pub fn read_big_int64be(&self, offset: Option<usize>) -> i64 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        BigEndian::read_i64(buffer)
    }

    pub fn read_big_int64le(&self, offset: Option<usize>) -> i64 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        LittleEndian::read_i64(buffer)
    }

    pub fn read_big_uint64be(&self, offset: Option<usize>) -> u64 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        BigEndian::read_u64(buffer)
    }

    pub fn read_big_uint64le(&self, offset: Option<usize>) -> u64 {
        let buffer = self.buffer();
        let length = buffer.len();
        let buffer = &buffer[offset.unwrap_or(0)..length];
        LittleEndian::read_u64(buffer)
    }

    pub fn concat(buffers: &[&[u8]], length: Option<usize>) -> Self {
        let len: usize = match length {
            Some(len) => len.min(buffers.iter().map(|buf| buf.len()).sum()),
            None => buffers.iter().map(|buf| buf.len()).sum(),
        };

        let mut result = vec![0_u8; len];

        let mut cursor = std::io::Cursor::new(&mut result);

        for buf in buffers {
            let remaining_length = match length {
                Some(len) => len - cursor.position() as usize,
                None => usize::MAX - cursor.position() as usize,
            };

            if remaining_length == 0 {
                break;
            }

            let bytes_to_write = remaining_length.min(buf.len());
            cursor.write_all(&buf[..bytes_to_write]).unwrap();
        }

        Buffer(BufferInner::Allocated(
            Arc::new(
                RwLock::new(
                    result
                )
            )
        ))
    }

    pub fn from_slice(value: &[u8]) -> Self {
        Self(
            BufferInner::Allocated(
                Arc::new(
                    RwLock::new(
                        value.to_vec()
                    )
                )
            )
        )
    }

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

    pub fn fill(&mut self, string: CString, encoding: Option<StringEncoding>) -> &mut Self {
        let ret = match encoding.unwrap_or(StringEncoding::Utf8) {
            StringEncoding::Ascii => {
                let string = string.to_string_lossy();
                string.as_bytes().to_vec()
            }
            StringEncoding::Utf8 => {
                string.as_bytes().to_vec()
            }
            StringEncoding::Utf16le => {
                let (decoded, _) = encoding_rs::UTF_8.decode_without_bom_handling(string.as_bytes());
                decoded.as_bytes().to_vec()
            }
            StringEncoding::Ucs2 => {
                let string = string.as_bytes();
                let length = string.len();
                let string = unsafe { std::slice::from_raw_parts(string.as_ptr() as *const u16, length / 2) };
                let mut buf = vec![0_u8; length];
                let decoded = ucs2::decode(string, buf.as_mut_slice()).unwrap_or(0);
                buf.shrink_to(decoded);
                buf
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
        match self.0 {
            BufferInner::Allocated(ref buf) => {
                let mut buf = buf.write();
                let _ = buf.write(ret.as_slice());
            }
            BufferInner::Reference(ref buf) => {
                let (data, size) = *buf.write();
                if data.is_null() || size == 0 {
                    return self;
                }
                let mut buf = unsafe { std::slice::from_raw_parts_mut(data, size) };
                let _ = buf.write(ret.as_slice());
            }

            _ => {}
        }
        self
    }

    pub fn length(&self) -> usize {
        match self.0 {
            BufferInner::Allocated(ref allocated) => {
                allocated.read().len()
            }
            BufferInner::Reference(ref buf) => {
                let (data, size) = *buf.read();
                if data.is_null() {
                    return 0;
                }
                size
            }
            BufferInner::Empty => 0
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
            BufferInner::Reference(ref buf) => {
                let (data, size) = *buf.read();
                if data.is_null() || size == 0 {
                    return &[];
                }
                return unsafe { std::slice::from_raw_parts(data, size) };
            }

            BufferInner::Empty => {
                return &[];
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
            BufferInner::Reference(ref buf) => {
                let (data, size) = *buf.write();
                if data.is_null() || size == 0 {
                    return &mut [];
                }
                return unsafe { std::slice::from_raw_parts_mut(data, size) };
            }

            BufferInner::Empty => {
                return &mut [];
            }
        }
    }

    pub fn to_string(&self, encoding: Option<StringEncoding>, start: Option<usize>, end: Option<usize>) -> String {
        let buf = self.buffer();
        let start = start.unwrap_or(0);
        let end = end.unwrap_or(buf.len());

        let buffer = &buf[start..end];

        match encoding {
            None => {
                encoding_rs::UTF_8.decode(buffer).0.to_string()
            }
            Some(encoding) => {
                match encoding {
                    StringEncoding::Ascii => {
                        match std::str::from_utf8(buffer) {
                            Ok(string) => {
                                string.to_string()
                            }
                            Err(_) => {
                                String::default()
                            }
                        }
                    }
                    StringEncoding::Utf8 => {
                        String::from_utf8_lossy(buffer).to_string()
                    }
                    StringEncoding::Utf16le | StringEncoding::Ucs2 => {
                        let buffer = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u16, buffer.len()) };
                        String::from_utf16_lossy(buffer).to_string()
                    }
                    StringEncoding::Base64 => {
                        base64::engine::general_purpose::STANDARD.encode(buffer)
                    }
                    StringEncoding::Latin1 | StringEncoding::Binary => {
                        let encoding = encoding_rs::Encoding::for_label(b"latin1").unwrap();
                        encoding.decode(buffer).0.to_string()
                    }
                    StringEncoding::Hex => {
                        hex::encode(buffer)
                    }
                }
            }
        }
    }
}