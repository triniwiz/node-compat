extern crate core;

use std::ffi::CString;
use jni::JNIEnv;
use jni::objects::{JByteBuffer, JClass, JObject, JObjectArray, JPrimitiveArray, JString, ReleaseMode};
use jni::sys::{jbyte, jdouble, jfloat, jint, jlong, jobject, jobjectArray, jshort, jstring};
use node_buffer::{Buffer, StringEncoding};

fn get_offset(offset: jlong) -> Option<usize> {
    if offset < 0 {
        return None;
    }
    Some(offset as usize)
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeAlloc(
    mut env: JNIEnv,
    _: JClass,
    size: jlong,
    text: JString,
    encoding: jint,
) -> jlong {
    let buffer = if text.is_null() {
        Buffer::builder()
            .size(size as usize)
            .build()
    } else {
        match env.get_string(&text) {
            Ok(text) => {
                match StringEncoding::try_from(encoding) {
                    Ok(encoding) => {
                        let text = CString::new(text.to_string_lossy().to_string()).unwrap();
                        Buffer::builder()
                            .size(size as usize)
                            .fill_text(text, encoding)
                            .build()
                    }
                    Err(error) => {
                        let _ = env.throw(error);
                        Buffer::default()
                    }
                }
            }
            Err(error) => {
                let _ = env.throw(error.to_string());
                Buffer::default()
            }
        }
    };

    Box::into_raw(Box::new(buffer)) as jlong
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeConcat(
    mut env: JNIEnv,
    _: JClass,
    array: jobjectArray,
    length: jlong,
) -> jlong {
    unsafe {
        let array = JObjectArray::from_raw(array);
        let len = env.get_array_length(&array).unwrap_or_default();
        let mut slice = Vec::with_capacity(length as usize);
        for i in 0..len {
            match env.get_object_array_element(&array, i) {
                Ok(buf) => {
                    let buf = JByteBuffer::from_raw(buf.as_raw());
                    let data = env.get_direct_buffer_address(&buf);
                    let size = env.get_direct_buffer_capacity(&buf);

                    let buf = match (data, size) {
                        (Ok(data), Ok(size)) => {
                            std::slice::from_raw_parts(data, size)
                        }
                        _ => {
                            &[]
                        }
                    };

                    slice.push(buf);
                }
                Err(_) => {}
            }
        }

        Box::into_raw(
            Box::new(
                Buffer::concat(slice.as_slice(), get_offset(length))
            )
        ) as jlong
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeFromBuffer(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
) -> jlong {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return Box::into_raw(
            Box::new(
                Buffer::default()
            )
        ) as jlong;
    }

    let mut buffer = unsafe { &mut *buffer };

    Box::into_raw(
        Box::new(
            Buffer::from_buffer(&buffer)
        )
    ) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeAtob(
    mut env: JNIEnv,
    _: JClass,
    text: JString,
) -> jstring {
    match env.get_string(&text) {
        Ok(text) => {
            let text = CString::new(text.to_string_lossy().to_string()).unwrap();
            let text = Buffer::atob(text);
            env.new_string(text).unwrap().into_raw()
        }
        Err(_) => {
            env.new_string("").unwrap().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeBtoa(
    mut env: JNIEnv,
    _: JClass,
    text: JString,
) -> jstring {
    match env.get_string(&text) {
        Ok(text) => {
            let text = CString::new(text.to_string_lossy().to_string()).unwrap();
            let text = Buffer::btoa(text);
            env.new_string(text).unwrap().into_raw()
        }
        Err(_) => {
            env.new_string("").unwrap().into_raw()
        }
    }
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeFillString(
    mut env: JNIEnv,
    _: JClass,
    buffer: jlong,
    text: JString,
    encoding: jint,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    if let Ok(text) = env.get_string(&text) {
        match StringEncoding::try_from(encoding) {
            Ok(encoding) => {
                let text = CString::new(text.to_string_lossy().to_string()).unwrap();
                buffer.fill(text, Some(encoding));
            }
            Err(error) => {
                let _ = env.throw(error);
            }
        }
    }
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeToString(
    mut env: JNIEnv,
    _: JClass,
    buffer: jlong,
    encoding: jint,
    start: jlong,
    end: jlong,
) -> jstring {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return env.new_string("").unwrap().into_raw();
    }
    let mut buffer = unsafe { &mut *buffer };

    match StringEncoding::try_from(encoding) {
        Ok(encoding) => {
           let string = buffer.as_string(Some(encoding), get_offset(start), get_offset(end));
            env.new_string(string).unwrap().into_raw()
        }
        Err(error) => {
            let _ = env.throw(error);
            env.new_string("").unwrap().into_raw()
        }
    }
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeLength(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
) -> jlong {
    let buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.length() as jlong
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeBuffer(
    mut env: JNIEnv,
    _: JClass,
    buffer: jlong,
) -> jobject {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return JObject::null().into_raw();
    }

    let mut buffer = unsafe { &mut *buffer };

    let buffer = buffer.buffer_mut();

    let data = buffer.as_mut_ptr();

    let size = buffer.len();

    unsafe {
        env.new_direct_byte_buffer(data, size)
            .map(|b| b.into_raw())
            .unwrap_or(JObject::null().into_raw())
    }
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteInt8(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jbyte,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_int8(value, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteUInt8(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jbyte,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_uint8(value as u8, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteUInt16BE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jshort,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_uint16be(value as u16, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteUInt16LE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jshort,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_uint16le(value as u16, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteInt16BE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jshort,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_int16be(value, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteInt16LE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jshort,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_int16le(value, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteUInt32BE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jint,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_uint32be(value as u32, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteUInt32LE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jint,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_uint32le(value as u32, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteInt32BE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jint,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_int32be(value, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteInt32LE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jint,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_int32le(value, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteFloatBE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jfloat,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_float_be(value, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteFloatLE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jfloat,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_float_le(value, get_offset(offset));
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteDoubleBE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jdouble,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_double_be(value, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteDoubleLE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jdouble,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_double_le(value, get_offset(offset));
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteUInt64BE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jlong,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_big_uint64be(value as u64, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteUInt64LE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jlong,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_big_uint64le(value as u64, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteInt64BE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jlong,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_big_int64be(value, get_offset(offset));
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeWriteInt64LE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    value: jlong,
    offset: jlong,
) {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        return;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.write_big_int64le(value, get_offset(offset));
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadInt8(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jbyte {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_int8(get_offset(offset))
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadUInt8(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jbyte {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_uint8(get_offset(offset)) as jbyte
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadUInt16BE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jshort {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_uint16be(get_offset(offset)) as jshort
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadUInt16LE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jshort {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_uint16le(get_offset(offset)) as jshort
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadInt16BE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jshort {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_int16be(get_offset(offset))
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadInt16LE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jshort {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_int16le(get_offset(offset))
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadUInt32BE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jint {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_uint32be(get_offset(offset)) as jint
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadUInt32LE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jint {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_uint32le(get_offset(offset)) as jint
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadInt32BE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jint {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_int32be(get_offset(offset))
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadInt32LE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jint {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_int32le(get_offset(offset))
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadFloatBE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jfloat {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0.;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_float_be(get_offset(offset))
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadFloatLE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jfloat {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0.;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_float_le(get_offset(offset))
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadDoubleBE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jdouble {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0.;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_double_be(get_offset(offset))
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadDoubleLE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jdouble {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0.;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_double_le(get_offset(offset))
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadUInt64BE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jlong {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_big_uint64be(get_offset(offset)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadUInt64LE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jlong {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_big_uint64le(get_offset(offset)) as jlong
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadInt64BE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jlong {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_big_int64be(get_offset(offset))
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_node1compat_buffer_Buffer_nativeReadInt64LE(
    _env: JNIEnv,
    _: JClass,
    buffer: jlong,
    offset: jlong,
) -> jlong {
    let mut buffer = unsafe { Buffer::get_ptr(buffer) };

    if buffer.is_null() {
        // throw ??
        return 0;
    }

    let mut buffer = unsafe { &mut *buffer };

    buffer.read_big_int64le(get_offset(offset))
}