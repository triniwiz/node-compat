//
// Created by Osei Fortune on 06/07/2023.
//

#include "BufferImpl.h"
#include "Caches.h"
#include "node-cxx/src/lib.rs.h"

using namespace rust;

void BufferImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    global->Set(context, Helpers::ConvertToV8String(isolate, "NSCBuffer"), func);
}

BufferImpl *BufferImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<BufferImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> BufferImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->BufferTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, nullptr);

    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "NSCBuffer"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "alloc"),
            v8::FunctionTemplate::New(isolate, &Alloc));

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "buffer"),
            &GetBuffer
    );


    // use alloc
    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "allocUnsafe"),
            v8::FunctionTemplate::New(isolate, &Alloc));


    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "length"),
            &Length
    );

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "atob"),
            v8::FunctionTemplate::New(isolate, &Atob)
    );

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "btoa"),
            v8::FunctionTemplate::New(isolate, &Btoa)
    );

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "concat"),
            v8::FunctionTemplate::New(isolate, &Concat)
    );

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "copyBytesFrom"),
            v8::FunctionTemplate::New(isolate, &CopyBytesFrom)
    );

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "from"),
            v8::FunctionTemplate::New(isolate, &From)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "toString"),
            v8::FunctionTemplate::New(isolate, &ToString));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "fill"),
            v8::FunctionTemplate::New(isolate, &Fill));


    // write
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeInt8"),
            v8::FunctionTemplate::New(isolate, &WriteInt8));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeUInt8"),
            v8::FunctionTemplate::New(isolate, &WriteUInt8));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeInt16LE"),
            v8::FunctionTemplate::New(isolate, &WriteInt16LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeInt16BE"),
            v8::FunctionTemplate::New(isolate, &WriteInt16BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeUInt16LE"),
            v8::FunctionTemplate::New(isolate, &WriteUInt16LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeUInt16BE"),
            v8::FunctionTemplate::New(isolate, &WriteUInt16BE));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeInt32LE"),
            v8::FunctionTemplate::New(isolate, &WriteInt32LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeInt32BE"),
            v8::FunctionTemplate::New(isolate, &WriteInt32BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeUInt32LE"),
            v8::FunctionTemplate::New(isolate, &WriteUInt32LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeUInt32BE"),
            v8::FunctionTemplate::New(isolate, &WriteUInt32BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeFloatLE"),
            v8::FunctionTemplate::New(isolate, &WriteFloatLE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeFloatBE"),
            v8::FunctionTemplate::New(isolate, &WriteFloatBE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeDoubleLE"),
            v8::FunctionTemplate::New(isolate, &WriteDoubleLE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeDoubleBE"),
            v8::FunctionTemplate::New(isolate, &WriteDoubleBE));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeBigInt64LE"),
            v8::FunctionTemplate::New(isolate, &WriteBigInt64LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeBigInt64BE"),
            v8::FunctionTemplate::New(isolate, &WriteBigInt64BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeBigUInt64LE"),
            v8::FunctionTemplate::New(isolate, &WriteBigUInt64LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeBigUInt64BE"),
            v8::FunctionTemplate::New(isolate, &WriteBigUInt64BE));


    // read

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readInt8"),
            v8::FunctionTemplate::New(isolate, &ReadInt8));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readUInt8"),
            v8::FunctionTemplate::New(isolate, &ReadUInt8));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readInt16LE"),
            v8::FunctionTemplate::New(isolate, &ReadInt16LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readInt16BE"),
            v8::FunctionTemplate::New(isolate, &ReadInt16BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readUInt16LE"),
            v8::FunctionTemplate::New(isolate, &ReadUInt16LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readUInt16BE"),
            v8::FunctionTemplate::New(isolate, &ReadUInt16BE));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readInt32LE"),
            v8::FunctionTemplate::New(isolate, &ReadInt32LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readInt32BE"),
            v8::FunctionTemplate::New(isolate, &ReadInt32BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readUInt32LE"),
            v8::FunctionTemplate::New(isolate, &ReadUInt32LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readUInt32BE"),
            v8::FunctionTemplate::New(isolate, &ReadUInt32BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readFloatLE"),
            v8::FunctionTemplate::New(isolate, &ReadFloatLE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readFloatBE"),
            v8::FunctionTemplate::New(isolate, &ReadFloatBE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readDoubleLE"),
            v8::FunctionTemplate::New(isolate, &ReadDoubleLE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readDoubleBE"),
            v8::FunctionTemplate::New(isolate, &ReadDoubleBE));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readBigInt64LE"),
            v8::FunctionTemplate::New(isolate, &ReadBigInt64LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readBigInt64BE"),
            v8::FunctionTemplate::New(isolate, &ReadBigInt64BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readBigUInt64LE"),
            v8::FunctionTemplate::New(isolate, &ReadBigUInt64LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readBigUInt64BE"),
            v8::FunctionTemplate::New(isolate, &ReadBigUInt64BE));


    tmpl->SetIndexedPropertyHandler(
            &IndexedGetter,
            &IndexedSetter
    );

    cache->BufferTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);

    return ctorTmpl;

}

BufferImpl::BufferImpl(rust::Box<Buffer> buffer) : buffer_(std::move(buffer)) {


    auto slice = buffer_buffer(*buffer_);
    auto clone = buffer_clone(*buffer_);
    auto raw = clone.into_raw();
    this->store_ = v8::ArrayBuffer::NewBackingStore(static_cast<void *>(slice.data()),
                                                    slice.length(), [](void *data, size_t length,
                                                                       void *deleter_data) {
                auto ptr = static_cast<Buffer *>(deleter_data);
                rust::Box<Buffer>::from_raw(ptr);
            }, raw);
}


void BufferImpl::Alloc(const v8::FunctionCallbackInfo<v8::Value> &args) {
    int count = args.Length();
    auto firstArg = args[0];
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    if (count == 1 && firstArg->IsNumber()) {
        auto buffer = buffer_alloc((size_t) firstArg->NumberValue(ctx).ToChecked());
        auto bufferImpl = new BufferImpl(std::move(buffer));
        auto ext = v8::External::New(isolate, bufferImpl);

        auto ctor = GetCtor(isolate);
        auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
        ret->SetInternalField(0, ext);
        args.GetReturnValue().Set(ret);
    }
    if (count == 2) {
        auto size = (size_t) firstArg->NumberValue(ctx).ToChecked();
        auto fill = args[1];
        if (fill->IsString() || fill->IsStringObject()) {
            auto fillValue = Helpers::ConvertFromV8String(isolate, fill);
            auto buffer = buffer_alloc_with_size_string_encoding(size, rust::Str(fillValue),
                                                                 StringEncoding::Utf8);

            auto bufferImpl = new BufferImpl(std::move(buffer));
            auto ext = v8::External::New(isolate, bufferImpl);

            auto ctor = GetCtor(isolate);
            auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
            ret->SetInternalField(0, ext);
            args.GetReturnValue().Set(ret);
        } else if (fill->IsUint8Array()) {
            auto array = fill.As<v8::Uint8Array>();
            auto len = array->ByteLength();
            auto arrayBuffer = array->Buffer();
            auto offset = array->ByteOffset();
            auto store = arrayBuffer->GetBackingStore();
            auto data = static_cast<uint8_t *>(store->Data()) + offset;

            auto buffer = buffer_from_reference(data, len);

            auto bufferImpl = new BufferImpl(std::move(buffer));
            auto ext = v8::External::New(isolate, bufferImpl);

            auto ctor = GetCtor(isolate);
            auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
            ret->SetInternalField(0, ext);
            args.GetReturnValue().Set(ret);
        } else if (fill->IsNumber()) {

        }
    }
}

void
BufferImpl::GetBuffer(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    if (ptr != nullptr) {
        auto len = buffer_length(*ptr->buffer_);
        auto buffer = v8::ArrayBuffer::New(isolate, ptr->store_);
        auto ret = v8::Uint8Array::New(buffer, 0, len);
        info.GetReturnValue().Set(ret);
        return;
    }
    info.GetReturnValue().SetUndefined();
}

void
BufferImpl::Length(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        info.GetReturnValue().Set((double) buffer_length(*ptr->buffer_));
        return;
    }
    info.GetReturnValue().Set(0);
}

void BufferImpl::Atob(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto value = args[0];
    if (value->IsString() || value->IsStringObject()) {
        auto str = Helpers::ConvertFromV8String(isolate, value);
        auto ret = buffer_atob(rust::Str(str));
        args.GetReturnValue().Set(
                Helpers::ConvertToV8String(isolate, ret.c_str())
        );
        return;
    }
    args.GetReturnValue().SetEmptyString();
}

void BufferImpl::Btoa(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto value = args[0];
    if (value->IsString() || value->IsStringObject()) {
        auto str = Helpers::ConvertFromV8String(isolate, value);
        auto ret = buffer_btoa(rust::Str(str));
        args.GetReturnValue().Set(
                Helpers::ConvertToV8String(isolate, ret.c_str())
        );
        return;
    }
    args.GetReturnValue().SetEmptyString();
}

void BufferImpl::Concat(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto value = args[0];
    auto cl = args[1];
    size_t concatLength = -1;

    if (cl->IsNumber()) {
        concatLength = (size_t) cl->NumberValue(ctx).ToChecked();
    }

    if (value->IsArray()) {
        auto array = value.As<v8::Array>();
        auto len = array->Length();
        bool hasError = false;
        std::vector<rust::Slice<const uint8_t>> vec;
        vec.reserve(len);
        for (int i = 0; i < len; i++) {
            auto itemValue = array->Get(ctx, i);
            if (itemValue.IsEmpty()) {
                hasError = true;
                break;
            }
            auto item = itemValue.ToLocalChecked();
            if (item->IsUint8Array()) {
                auto buffer = item.As<v8::Uint8Array>();
                auto length = buffer->ByteLength();
                auto arrayBuffer = buffer->Buffer();
                auto offset = buffer->ByteOffset();
                auto store = arrayBuffer->GetBackingStore();
                auto data = static_cast<const uint8_t *>(store->Data()) + offset;
                rust::Slice<const uint8_t> slice = rust::Slice(data, length);
                vec.push_back(slice);
            } else if (item->IsObject()) {
                auto ptr = GetPointer(item->ToObject(ctx).ToLocalChecked());
                if (ptr != nullptr) {
                    auto buffer = buffer_buffer(*ptr->buffer_);
                    rust::Slice<const uint8_t> slice(buffer.data(), buffer.size());
                    vec.push_back(slice);
                }
            }

        }
        if (hasError) {
            auto buffer = buffer_alloc(0);
            auto bufferImpl = new BufferImpl(std::move(buffer));
            auto ext = v8::External::New(isolate, bufferImpl);

            auto ctor = GetCtor(isolate);
            auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
            ret->SetInternalField(0, ext);
            args.GetReturnValue().Set(ret);
            return;
        }


        BufferImpl *bufferImpl;

        rust::Slice<const rust::Slice<const uint8_t>> val(vec.data(), vec.size());
        if (concatLength != -1) {
            auto buffer = buffer_concat_length(val, concatLength);
            bufferImpl = new BufferImpl(std::move(buffer));
        } else {
            auto buffer = buffer_concat(val);
            bufferImpl = new BufferImpl(std::move(buffer));
        }


        auto ext = v8::External::New(isolate, bufferImpl);
        auto ctor = GetCtor(isolate);
        auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
        ret->SetInternalField(0, ext);
        args.GetReturnValue().Set(ret);

        return;
    }

    auto buffer = buffer_alloc(0);
    auto bufferImpl = new BufferImpl(std::move(buffer));
    auto ext = v8::External::New(isolate, bufferImpl);

    auto ctor = GetCtor(isolate);
    auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
    ret->SetInternalField(0, ext);
    args.GetReturnValue().Set(ret);

}

void BufferImpl::CopyBytesFrom(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto value = args[0];
    if (value->IsTypedArray()) {
        auto array = value.As<v8::TypedArray>();
        auto buffer = array->Buffer();
        auto store = buffer->GetBackingStore();
        auto offset = array->ByteOffset();
        auto data = static_cast<const uint8_t *>(store->Data()) + offset;
        auto length = array->ByteLength();
        auto buf = buffer_from_slice(rust::Slice(data, length));

        auto bufferImpl = new BufferImpl(std::move(buf));
        auto ext = v8::External::New(isolate, bufferImpl);

        auto ctor = GetCtor(isolate);
        auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
        ret->SetInternalField(0, ext);
        args.GetReturnValue().Set(ret);
        return;
    }

    args.GetReturnValue().SetUndefined();
}

void BufferImpl::From(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto value = args[0];
    if (value->IsString() || value->IsStringObject()) {
        auto encodingValue = args[1];
        StringEncoding encoding = Helpers::ParseEncoding(isolate, encodingValue,
                                                         StringEncoding::Utf8);
        auto str = Helpers::ConvertFromV8String(isolate, value);
        auto buffer = buffer_from_string(rust::Str(str), encoding);

        auto bufferImpl = new BufferImpl(std::move(buffer));
        auto ext = v8::External::New(isolate, bufferImpl);

        auto ctor = GetCtor(isolate);
        auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
        ret->SetInternalField(0, ext);
        args.GetReturnValue().Set(ret);
        return;
    } else if (value->IsArrayBuffer()) {
        auto arrayBuffer = value.As<v8::ArrayBuffer>();
        auto store = arrayBuffer->GetBackingStore();
        auto data = static_cast<const uint8_t *>(store->Data());
        auto buffer = buffer_from_slice(rust::Slice(data, arrayBuffer->ByteLength()));

        auto bufferImpl = new BufferImpl(std::move(buffer));
        auto ext = v8::External::New(isolate, bufferImpl);

        auto ctor = GetCtor(isolate);
        auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
        ret->SetInternalField(0, ext);
        args.GetReturnValue().Set(ret);
        return;
    } else if (value->IsArray()) {
        auto array = value.As<v8::Array>();
        auto len = array->Length();
        std::vector<uint8_t> vector;
        vector.reserve(len);
        bool hasError = false;
        for (int i = 0; i < len; i++) {
            v8::Local<v8::Value> itemValue;
            hasError = !array->Get(ctx, i).ToLocal(&itemValue);
            if (itemValue->IsNumber()) {
                vector.push_back((uint8_t) itemValue->NumberValue(ctx).ToChecked());
            }
        }
        if (hasError) {
            args.GetReturnValue().SetUndefined();
            return;
        }
        rust::Slice<const uint8_t> slice(vector.data(), vector.size());

        auto buffer = buffer_from_slice(slice);

        auto bufferImpl = new BufferImpl(std::move(buffer));
        auto ext = v8::External::New(isolate, bufferImpl);

        auto ctor = GetCtor(isolate);
        auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
        ret->SetInternalField(0, ext);
        args.GetReturnValue().Set(ret);
        return;
    }

}

void BufferImpl::ToString(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto len = args.Length();

    auto ptr = GetPointer(args.This());
    StringEncoding encoding = StringEncoding::Utf8;
    rust::isize start = 0;
    rust::isize end = -1;
    if (ptr != nullptr) {
        auto encodingValue = args[0];
        auto startValue = args[1];
        auto endValue = args[2];

        if (len == 0) {
            auto ret = buffer_to_print_string(*ptr->buffer_);
            args.GetReturnValue().Set(
                    Helpers::ConvertToV8String(isolate, ret.c_str())
            );
            return;
        }

        if (len > 0) {
            encoding = Helpers::ParseEncoding(isolate, encodingValue, StringEncoding::Utf8);
        }

        if (len > 1 && startValue->IsNumber()) {
            start = (rust::isize) startValue->NumberValue(ctx).ToChecked();
        }

        if (len > 2 && endValue->IsNumber()) {
            end = (rust::isize) endValue->NumberValue(ctx).ToChecked();
        }

        auto ret = buffer_to_string(*ptr->buffer_, encoding, start, end);
        args.GetReturnValue().Set(
                Helpers::ConvertToV8String(isolate, ret.c_str())
        );
        return;
    }
    return args.GetReturnValue().SetEmptyString();
}

void BufferImpl::Fill(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto len = args.Length();

    auto ptr = GetPointer(args.This());
    StringEncoding encoding = StringEncoding::Utf8;
    rust::isize start = 0;
    rust::isize end = -1;
    if (ptr != nullptr) {
        auto encodingValue = args[0];
        auto startValue = args[1];
        auto endValue = args[2];
        if (len > 0) {
            encoding = Helpers::ParseEncoding(isolate, encodingValue, StringEncoding::Utf8);
        }

        if (len > 1 && startValue->IsNumber()) {
            start = (rust::isize) startValue->NumberValue(ctx).ToChecked();
        }

        if (len > 2 && endValue->IsNumber()) {
            end = (rust::isize) endValue->NumberValue(ctx).ToChecked();
        }

        auto ret = buffer_to_string(*ptr->buffer_, encoding, start, end);
        args.GetReturnValue().Set(
                Helpers::ConvertToV8String(isolate, ret.c_str())
        );
        return;
    }
}

void BufferImpl::WriteInt8(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsNumber()) {
            buffer_write_int8(*ptr->buffer_, (int8_t) value->NumberValue(ctx).ToChecked(), offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteUInt8(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsNumber()) {
            buffer_write_uint8(*ptr->buffer_, (int8_t) value->NumberValue(ctx).ToChecked(), offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteInt16LE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsNumber()) {
            buffer_write_int16le(*ptr->buffer_, (int16_t) value->NumberValue(ctx).ToChecked(),
                                 offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteInt16BE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsNumber()) {
            buffer_write_int16be(*ptr->buffer_, (int16_t) value->NumberValue(ctx).ToChecked(),
                                 offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteUInt16LE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsNumber()) {
            buffer_write_uint16le(*ptr->buffer_, (uint16_t) value->NumberValue(ctx).ToChecked(),
                                  offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteUInt16BE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsNumber()) {
            buffer_write_uint16be(*ptr->buffer_, (int16_t) value->NumberValue(ctx).ToChecked(),
                                  offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteInt32LE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsNumber()) {
            buffer_write_int32le(*ptr->buffer_, (int32_t) value->NumberValue(ctx).ToChecked(),
                                 offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteInt32BE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsNumber()) {
            buffer_write_int32be(*ptr->buffer_, (int32_t) value->NumberValue(ctx).ToChecked(),
                                 offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteUInt32LE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsNumber()) {
            buffer_write_uint32le(*ptr->buffer_, (uint32_t) value->NumberValue(ctx).ToChecked(),
                                  offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteUInt32BE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsNumber()) {
            buffer_write_uint32be(*ptr->buffer_, (int32_t) value->NumberValue(ctx).ToChecked(),
                                  offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteFloatLE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsNumber()) {
            buffer_write_float_le(*ptr->buffer_, (float) value->NumberValue(ctx).ToChecked(),
                                  offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteFloatBE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsNumber()) {
            buffer_write_float_be(*ptr->buffer_, (float) value->NumberValue(ctx).ToChecked(),
                                  offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteDoubleLE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsNumber()) {
            buffer_write_double_le(*ptr->buffer_, value->NumberValue(ctx).ToChecked(), offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteDoubleBE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsNumber()) {
            buffer_write_double_be(*ptr->buffer_, value->NumberValue(ctx).ToChecked(), offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteBigInt64LE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsBigInt()) {
            buffer_write_big_int64le(*ptr->buffer_,
                                     value->ToBigInt(ctx).ToLocalChecked()->Int64Value(), offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteBigInt64BE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsBigInt()) {
            buffer_write_big_int64be(*ptr->buffer_,
                                     value->ToBigInt(ctx).ToLocalChecked()->Int64Value(), offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteBigUInt64LE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsBigInt()) {
            buffer_write_big_uint64le(*ptr->buffer_,
                                      value->ToBigInt(ctx).ToLocalChecked()->Uint64Value(), offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}

void BufferImpl::WriteBigUInt64BE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto value = args[0];
        auto offsetValue = args[1];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }
        if (value->IsBigInt()) {
            buffer_write_big_uint64be(*ptr->buffer_,
                                      value->ToBigInt(ctx).ToLocalChecked()->Uint64Value(), offset);
        }
    }
    args.GetReturnValue().SetUndefined();
}


void BufferImpl::ReadInt8(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_int8(*ptr->buffer_, offset);
        args.GetReturnValue().Set((int32_t) value);
        return;
    }
    args.GetReturnValue().Set(0);
}

void BufferImpl::ReadUInt8(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_uint8(*ptr->buffer_, offset);
        args.GetReturnValue().Set((int32_t) value);
        return;
    }
    args.GetReturnValue().Set(0);
}

void BufferImpl::ReadInt16LE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_int16le(*ptr->buffer_, offset);
        args.GetReturnValue().Set((int32_t) value);
        return;
    }
    args.GetReturnValue().Set(0);
}

void BufferImpl::ReadInt16BE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_int16be(*ptr->buffer_, offset);
        args.GetReturnValue().Set((int32_t) value);
        return;
    }
    args.GetReturnValue().Set(0);
}

void BufferImpl::ReadUInt16LE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_uint16le(*ptr->buffer_, offset);
        args.GetReturnValue().Set((int32_t) value);
        return;
    }
    args.GetReturnValue().Set(0);
}

void BufferImpl::ReadUInt16BE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_uint16be(*ptr->buffer_, offset);
        args.GetReturnValue().Set((int32_t) value);
        return;
    }
    args.GetReturnValue().Set(0);
}

void BufferImpl::ReadInt32LE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_int32le(*ptr->buffer_, offset);
        args.GetReturnValue().Set(value);
        return;
    }
    args.GetReturnValue().Set(0);
}

void BufferImpl::ReadInt32BE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_int32be(*ptr->buffer_, offset);
        args.GetReturnValue().Set(value);
        return;
    }
    args.GetReturnValue().Set(0);
}

void BufferImpl::ReadUInt32LE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_uint32le(*ptr->buffer_, offset);
        args.GetReturnValue().Set(value);
        return;
    }
    args.GetReturnValue().Set(0);
}

void BufferImpl::ReadUInt32BE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_uint32be(*ptr->buffer_, offset);
        args.GetReturnValue().Set((int32_t) value);
        return;
    }
    args.GetReturnValue().Set(0);
}

void BufferImpl::ReadFloatLE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_float_le(*ptr->buffer_, offset);
        args.GetReturnValue().Set((double) value);
        return;
    }
    args.GetReturnValue().Set(0);
}

void BufferImpl::ReadFloatBE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_float_be(*ptr->buffer_, offset);
        args.GetReturnValue().Set((double) value);
        return;
    }
    args.GetReturnValue().Set(0);
}

void BufferImpl::ReadDoubleLE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_double_le(*ptr->buffer_, offset);
        args.GetReturnValue().Set(value);
        return;
    }
    args.GetReturnValue().Set(0);
}

void BufferImpl::ReadDoubleBE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_double_be(*ptr->buffer_, offset);
        args.GetReturnValue().Set(value);
        return;
    }
    args.GetReturnValue().Set(0);
}

void BufferImpl::ReadBigInt64LE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_big_int64le(*ptr->buffer_, offset);
        args.GetReturnValue().Set(v8::BigInt::New(isolate, value));
        return;
    }
    args.GetReturnValue().Set(v8::BigInt::New(isolate, 0));
}

void BufferImpl::ReadBigInt64BE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_big_int64be(*ptr->buffer_, offset);
        args.GetReturnValue().Set(v8::BigInt::New(isolate, value));
        return;
    }
    args.GetReturnValue().Set(v8::BigInt::New(isolate, 0));
}

void BufferImpl::ReadBigUInt64LE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_big_uint64le(*ptr->buffer_, offset);
        args.GetReturnValue().Set(v8::BigInt::NewFromUnsigned(isolate, value));
        return;
    }
    args.GetReturnValue().Set(v8::BigInt::New(isolate, 0));
}

void BufferImpl::ReadBigUInt64BE(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        auto offsetValue = args[0];
        rust::isize offset = 0;
        if (offsetValue->IsNumber()) {
            offset = (rust::isize) offsetValue->NumberValue(ctx).ToChecked();
        }

        auto value = buffer_read_big_uint64be(*ptr->buffer_, offset);
        args.GetReturnValue().Set(v8::BigInt::NewFromUnsigned(isolate, value));
        return;
    }
    args.GetReturnValue().Set(v8::BigInt::New(isolate, 0));
}

void BufferImpl::IndexedGetter(uint32_t index, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.Holder());
    if (ptr != nullptr) {
        auto buffer = buffer_buffer(*ptr->buffer_);
        auto size = buffer.size();
        if (buffer.empty()) {
            info.GetReturnValue().SetUndefined();
            return;
        }
        if ((size_t) index < size) {
            auto value = buffer[(size_t) index];
            info.GetReturnValue().Set((uint32_t) value);
            return;
        }
    }
    info.GetReturnValue().SetUndefined();
}

void BufferImpl::IndexedSetter(uint32_t index, v8::Local<v8::Value> value,
                               const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto isolate = info.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(info.Holder());
    if (ptr != nullptr) {
        auto buffer = buffer_buffer(*ptr->buffer_);

        if (buffer.empty()) {
            info.GetReturnValue().SetUndefined();
            return;
        }

        if (((size_t) index) < buffer.size() && value->IsNumber()) {
            auto val = (uint8_t) value->NumberValue(ctx).ToChecked();
            buffer[(size_t) index] = val;
            info.GetReturnValue().SetUndefined();
            return;
        }
    }
    info.GetReturnValue().SetUndefined();
}


