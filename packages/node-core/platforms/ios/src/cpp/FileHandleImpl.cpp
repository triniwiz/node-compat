//
// Created by Osei Fortune on 18/07/2023.
//

#include "FileHandleImpl.h"
#include "Caches.h"
#include "BufferImpl.h"

void FileHandleImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
//    auto context = isolate->GetCurrentContext();
//    auto global = context->Global();
//    auto func = ctor->GetFunction(context).ToLocalChecked();
//
//    global->Set(context, Helpers::ConvertToV8String(isolate, "NSCBuffer"), func);
}

FileHandleImpl *FileHandleImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<FileHandleImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> FileHandleImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->FileHandleTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, nullptr);

    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "NSCFileHandle"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "appendFile"),
            v8::FunctionTemplate::New(isolate, &AppendFile));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "chmod"),
            v8::FunctionTemplate::New(isolate, &Chmod));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "chown"),
            v8::FunctionTemplate::New(isolate, &Chown));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "close"),
            v8::FunctionTemplate::New(isolate, &Close));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "datasync"),
            v8::FunctionTemplate::New(isolate, &DataSync));


    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "fd"),
            &GetFd);

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "read"),
            v8::FunctionTemplate::New(isolate, &Read));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readFile"),
            v8::FunctionTemplate::New(isolate, &ReadFile));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readv"),
            v8::FunctionTemplate::New(isolate, &Readv));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "stat"),
            v8::FunctionTemplate::New(isolate, &Stat));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "sync"),
            v8::FunctionTemplate::New(isolate, &Sync));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "truncate"),
            v8::FunctionTemplate::New(isolate, &Truncate));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "utimes"),
            v8::FunctionTemplate::New(isolate, &Utimes));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeFile"),
            v8::FunctionTemplate::New(isolate, &WriteFile));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "write"),
            v8::FunctionTemplate::New(isolate, &Write));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writev"),
            v8::FunctionTemplate::New(isolate, &Writev));



    cache->FileHandleTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);

    return ctorTmpl;

}

FileHandleImpl::FileHandleImpl(FileHandle *handle) : handle_(handle) {}

void FileHandleImpl::AppendFile(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto src = args[0];

    auto ptr = GetPointer(args.This());
    AppendFileOptions options{};
    auto optionsValue = args[1];
    Helpers::ParseAppendFileOptions(isolate, optionsValue, options);

    v8::Global<v8::Function> func(isolate, args[2].As<v8::Function>());
    auto cb = new AsyncCallback{
            isolate,
            std::move(func)
    };

    if (src->IsString()) {
        auto closure = fs_async_create_async_closure((void *) async_success_closure,
                                                     (void *) async_error, cb);
        auto source = Helpers::ConvertFromV8String(isolate, src);
        fs_handle_append_file_with_str(ptr->handle_, source.c_str(), options, closure);

    } else if (src->IsTypedArray()) {

        auto closure = fs_async_create_async_closure((void *) async_success_closure,
                                                     (void *) async_error, cb);

        auto array = src.As<v8::TypedArray>();
        auto buffer = array->Buffer();
        auto store = buffer->GetBackingStore();
        auto offset = array->ByteOffset();
        auto data = static_cast<uint8_t *>(store->Data()) + offset;
        auto length = array->ByteLength();

        auto buf = buffer_from_reference(data, length);

        fs_handle_append_file_with_bytes(ptr->handle_, buf, options, closure);
    }
}

void FileHandleImpl::Chmod(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto mode = args[0]->NumberValue(ctx).ToChecked();

    auto ptr = GetPointer(args.This());

    v8::Global<v8::Function> func(isolate, args[1].As<v8::Function>());
    auto cb = new AsyncCallback{
            isolate,
            std::move(func)
    };

    auto closure = fs_async_create_async_closure((void *) async_success_closure,
                                                 (void *) async_error, cb);
    fs_handle_chmod(ptr->handle_, (uint16_t) mode, closure);

}

void FileHandleImpl::Chown(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto uid = args[0]->NumberValue(ctx).ToChecked();
    auto gid = args[1]->NumberValue(ctx).ToChecked();

    auto ptr = GetPointer(args.This());

    v8::Global<v8::Function> func(isolate, args[2].As<v8::Function>());
    auto cb = new AsyncCallback{
            isolate,
            std::move(func)
    };

    auto closure = fs_async_create_async_closure((void *) async_success_closure,
                                                 (void *) async_error, cb);
    fs_handle_chown(ptr->handle_, (uint32_t) uid, (uint32_t) gid, closure);

}

void FileHandleImpl::Close(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();

    auto ptr = GetPointer(args.This());

    v8::Global<v8::Function> func(isolate, args[0].As<v8::Function>());
    auto cb = new AsyncCallback{
            isolate,
            std::move(func)
    };

    auto closure = fs_async_create_async_closure((void *) async_success_closure,
                                                 (void *) async_error, cb);
    fs_handle_close(ptr->handle_, closure);

}

void FileHandleImpl::DataSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();

    auto ptr = GetPointer(args.This());

    v8::Global<v8::Function> func(isolate, args[0].As<v8::Function>());
    auto cb = new AsyncCallback{
            isolate,
            std::move(func)
    };

    auto closure = fs_async_create_async_closure((void *) async_success_closure,
                                                 (void *) async_error, cb);
    fs_handle_datasync(ptr->handle_, closure);

}

void FileHandleImpl::GetFd(v8::Local<v8::String> name,
                           const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        auto fd = fs_handle_fd(ptr->handle_);
        info.GetReturnValue().Set(fd);
        return;
    }
    info.GetReturnValue().Set(0);
}

void FileHandleImpl::Read(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();

    auto ptr = GetPointer(args.This());

    size_t offset = 0;
    size_t length = -1;
    intptr_t position = -1;

    auto bufferValue = args[0];

    auto offsetValue = args[1];

    if (offsetValue->IsNumber()) {
        offset = (size_t) offsetValue->NumberValue(ctx).ToChecked();
    }

    auto lengthValue = args[2];
    if (lengthValue->IsNumber()) {
        length = (size_t) lengthValue->NumberValue(ctx).ToChecked();
    }

    auto positionValue = args[3];

    if (positionValue->IsBigInt()) {
        position = (intptr_t) positionValue->ToBigInt(ctx).ToLocalChecked()->Int64Value();
    } else if (positionValue->IsNumber()) {

    }

    v8::Global<v8::Function> func(isolate, args[4].As<v8::Function>());
    auto cb = new AsyncCallback{
            isolate,
            std::move(func)
    };


    auto closure = fs_async_create_async_usize_closure((void *) async_success_usize,
                                                       (void *) async_error, cb);

    if (bufferValue->IsTypedArray()) {
        auto array = bufferValue.As<v8::TypedArray>();
        auto buffer = array->Buffer();
        auto store = buffer->GetBackingStore();
        auto os = array->ByteOffset();
        auto len = array->ByteLength();
        auto data = static_cast<uint8_t *>(store->Data()) + os;
        fs_handle_read_bytes(ptr->handle_, data, len, offset, length, position, closure);

    } else if (bufferValue->IsDataView()) {
        auto view = bufferValue.As<v8::DataView>();
        auto buffer = view->Buffer();
        auto store = buffer->GetBackingStore();
        auto os = view->ByteOffset();
        auto len = view->ByteLength();
        auto data = static_cast<uint8_t *>(store->Data()) + os;
        fs_handle_read_bytes(ptr->handle_, data, len, offset, length, position, closure);

    } else if (bufferValue->IsObject()) {
        auto buffer = BufferImpl::GetPointer(bufferValue.As<v8::Object>());

        if (buffer != nullptr) {
            auto data = buffer->GetBuffer();
            fs_handle_read(ptr->handle_, data, offset, length, position, closure);
        }
    }

    args.GetReturnValue().SetUndefined();

}

void FileHandleImpl::ReadFile(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();

    auto ptr = GetPointer(args.This());

    ReadFileOptions options{};

    Helpers::ParseReadFileOptions(isolate, args[0], options);


    v8::Global<v8::Function> func(isolate, args[1].As<v8::Function>());
    auto cb = new AsyncCallback{
            isolate,
            std::move(func)
    };


    auto closure = fs_async_create_async_fs_encoding_closure((void *) async_success_fs_encoding,
                                                             (void *) async_error, cb);


    fs_handle_read_file(ptr->handle_, options, closure);

}

void FileHandleImpl::Readv(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();

    auto ptr = GetPointer(args.This());

    auto value = args[0];

    int64_t position = -1;
    auto positionValue = args[1];

    if (positionValue->IsNumber()) {
        position = (int64_t) positionValue->NumberValue(ctx).ToChecked();
    }

    std::vector<uint8_t *> vec;
    std::vector<size_t> vec_length;

    v8::Global<v8::Function> func(isolate, args[2].As<v8::Function>());
    auto cb = new AsyncCallback{
            isolate,
            std::move(func)
    };


    auto closure = fs_async_create_async_usize_closure((void *) async_success_usize,
                                                       (void *) async_error, cb);

    if (value->IsArray()) {
        auto array = value.As<v8::Array>();
        auto len = array->Length();
        bool hasError = false;
        vec.reserve(len);
        vec_length.reserve(len);
        for (int i = 0; i < len; i++) {
            auto itemValue = array->Get(ctx, i);
            if (itemValue.IsEmpty()) {
                hasError = true;
                break;
            }
            auto item = itemValue.ToLocalChecked();
            if (item->IsArrayBufferView()) {
                auto buffer = item.As<v8::ArrayBufferView>();
                auto length = buffer->ByteLength();
                auto arrayBuffer = buffer->Buffer();
                auto offset = buffer->ByteOffset();
                auto store = arrayBuffer->GetBackingStore();
                auto data = static_cast<uint8_t *>(store->Data()) + offset;
                vec.push_back(data);
                vec_length.push_back(length);
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

        auto size = vec.size();
        fs_handle_readv_slice(ptr->handle_, vec.data(), vec_length.data(), size, position, closure);

        return;
    }

    // todo throw
}

void FileHandleImpl::Stat(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    bool bigint = false;

    auto optionsValue = args[0];
    if (optionsValue->IsObject()) {
        auto options = optionsValue.As<v8::Object>();
        v8::Local<v8::Value> bigintValue;

        options->Get(ctx, Helpers::ConvertToV8String(isolate, "bigint")).ToLocal(&bigintValue);

        if (bigintValue->IsBoolean()) {
            bigint = bigintValue->BooleanValue(isolate);
        }
    }


    v8::Global<v8::Function> func(isolate, args[1].As<v8::Function>());
    auto cb = new AsyncCallback{
            isolate,
            std::move(func)
    };


    auto closure = fs_async_create_async_file_stat_closure((void *) async_success_filestat,
                                                           (void *) async_error, cb);


    fs_handle_stat(ptr->handle_, closure);

}

void FileHandleImpl::Sync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();

    auto ptr = GetPointer(args.This());

    v8::Global<v8::Function> func(isolate, args[0].As<v8::Function>());
    auto cb = new AsyncCallback{
            isolate,
            std::move(func)
    };

    auto closure = fs_async_create_async_closure((void *) async_success_closure,
                                                 (void *) async_error, cb);
    fs_handle_sync(ptr->handle_, closure);

}

void FileHandleImpl::Truncate(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();

    auto ptr = GetPointer(args.This());

    uint64_t len = 0;

    auto lenValue = args[0];
    if (lenValue->IsNumber()) {
        auto value = lenValue->NumberValue(ctx).ToChecked();
        if (value > -1) {
            len = (uint64_t) value;
        }
    }

    v8::Global<v8::Function> func(isolate, args[1].As<v8::Function>());
    auto cb = new AsyncCallback{
            isolate,
            std::move(func)
    };

    auto closure = fs_async_create_async_closure((void *) async_success_closure,
                                                 (void *) async_error, cb);

    fs_handle_truncate(ptr->handle_, len, closure);

}

void FileHandleImpl::Utimes(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();

    auto ptr = GetPointer(args.This());

    int64_t atime = 0;
    auto atimeValue = args[0];

    if (atimeValue->IsNumber()) {
        atime = (int64_t) atimeValue->NumberValue(ctx).ToChecked();
    } else if (atimeValue->IsDate()) {
        atime = (int64_t) atimeValue.As<v8::Date>()->ValueOf();
    } else if (atimeValue->IsString()) {
        atime = (int64_t) std::stod(Helpers::ConvertFromV8String(isolate, atimeValue));
    }

    int64_t mtime = 0;
    auto mtimeValue = args[1];

    if (mtimeValue->IsNumber()) {
        mtime = (int64_t) mtimeValue->NumberValue(ctx).ToChecked();
    } else if (mtimeValue->IsDate()) {
        mtime = (int64_t) mtimeValue.As<v8::Date>()->ValueOf();
    } else if (mtimeValue->IsString()) {
        mtime = (int64_t) std::stod(Helpers::ConvertFromV8String(isolate, mtimeValue));
    }


    v8::Global<v8::Function> func(isolate, args[2].As<v8::Function>());
    auto cb = new AsyncCallback{
            isolate,
            std::move(func)
    };

    auto closure = fs_async_create_async_closure((void *) async_success_closure,
                                                 (void *) async_error, cb);

    fs_handle_utimes(ptr->handle_, atime, mtime, closure);
}

void FileHandleImpl::WriteFile(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();

    auto ptr = GetPointer(args.This());


    auto dataValue = args[0];

    WriteFileOptions options{};

    Helpers::ParseWriteFileOptions(isolate, args[1], options);


    v8::Global<v8::Function> func(isolate, args[2].As<v8::Function>());
    auto cb = new AsyncCallback{
            isolate,
            std::move(func)
    };

    auto closure = fs_async_create_async_closure((void *) async_success_closure,
                                                 (void *) async_error, cb);


    if (dataValue->IsString()) {
        auto data = Helpers::ConvertFromV8String(isolate,
                                                 dataValue);

        fs_handle_write_file_with_str(ptr->handle_, data.c_str(), options, closure);
    } else if (dataValue->IsTypedArray()) {
        auto array = dataValue.As<v8::TypedArray>();
        auto buffer = array->Buffer();
        auto store = buffer->GetBackingStore();
        auto os = array->ByteOffset();
        auto len = array->ByteLength();
        auto data = static_cast<uint8_t *>(store->Data()) + os;
        fs_handle_write_file_with_bytes_slice(ptr->handle_, data, len, options, closure);
    }

}

void FileHandleImpl::Write(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();

    auto ptr = GetPointer(args.This());


    auto bufferValue = args[0];

    WriteOptions options{};

    v8::Local<v8::Value> offsetValue = args[1];

    if (offsetValue->IsNumber()) {
        options.offset = (size_t) offsetValue->NumberValue(ctx).ToChecked();
    }

    v8::Local<v8::Value> lengthValue = args[2];

    if (lengthValue->IsNumber()) {
        options.length = (size_t) lengthValue->NumberValue(ctx).ToChecked();
    }

    v8::Local<v8::Value> positionValue = args[3];

    if (positionValue->IsNumber()) {
        options.position = (intptr_t) positionValue->NumberValue(ctx).ToChecked();
    }

    v8::Global<v8::Function> func(isolate, args[4].As<v8::Function>());
    auto cb = new AsyncCallback{
            isolate,
            std::move(func)
    };

    auto closure = fs_async_create_async_usize_closure((void *) async_success_usize,
                                                       (void *) async_error, cb);


    auto array = bufferValue.As<v8::TypedArray>();
    auto buffer = array->Buffer();
    auto store = buffer->GetBackingStore();
    auto os = array->ByteOffset();
    auto len = array->ByteLength();
    auto data = static_cast<const uint8_t *>(store->Data()) + os;
    fs_handle_write_bytes(ptr->handle_, data, len, options, closure);

}

void FileHandleImpl::Writev(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());


    int64_t position = -1;
    auto positionValue = args[1];

    if (positionValue->IsNumber()) {
        position = (int64_t) positionValue->NumberValue(ctx).ToChecked();
    }

    std::vector<const uint8_t *> vec;
    std::vector<size_t> vec_length;

    auto value = args[0];

    if (value->IsArray()) {
        auto array = value.As<v8::Array>();
        auto len = array->Length();
        bool hasError = false;
        vec.reserve(len);
        vec_length.reserve(len);
        for (int i = 0; i < len; i++) {
            auto itemValue = array->Get(ctx, i);
            if (itemValue.IsEmpty()) {
                hasError = true;
                break;
            }
            auto item = itemValue.ToLocalChecked();
            if (item->IsArrayBufferView()) {
                auto buffer = item.As<v8::ArrayBufferView>();
                auto length = buffer->ByteLength();
                auto arrayBuffer = buffer->Buffer();
                auto offset = buffer->ByteOffset();
                auto store = arrayBuffer->GetBackingStore();
                auto data = static_cast<uint8_t *>(store->Data()) + offset;
                vec.push_back(data);
                vec_length.push_back(length);
            }

        }
//        if (hasError) {
//            auto buffer = buffer_alloc(0);
//            auto bufferImpl = new BufferImpl(std::move(buffer));
//            auto ext = v8::External::New(isolate, bufferImpl);
//
//            auto ctor = GetCtor(isolate);
//            auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
//            ret->SetInternalField(0, ext);
//            args.GetReturnValue().Set(ret);
//            return;
//        }


        v8::Global<v8::Function> func(isolate, args[2].As<v8::Function>());
        auto cb = new AsyncCallback{
                isolate,
                std::move(func)
        };

        auto closure = fs_async_create_async_usize_closure((void *) async_success_usize,
                                                           (void *) async_error, cb);

        fs_handle_writev_slice(ptr->handle_, vec.data(), vec_length.data(), vec.size(),
                               position, closure);


        return;
    }

}
