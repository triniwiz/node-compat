//
// Created by Osei Fortune on 08/07/2023.
//

#include <unistd.h>
#include "FSImpl.h"
#include "Caches.h"
#include "BufferImpl.h"

void FSImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    global->Set(context, Helpers::ConvertToV8String(isolate, "NSCFS"), func);
}

v8::Local<v8::FunctionTemplate> FSImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->FsTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, nullptr);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "NSCFS"));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "accessSync"),
            v8::FunctionTemplate::New(isolate, &AccessSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "readSync"),
            v8::FunctionTemplate::New(isolate, &ReadSync));


    cache->FsTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);

    return ctorTmpl;

}

void FSImpl::AccessSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto pathValue = args[0];
    auto mode = F_OK;
    if (!pathValue->IsString()) {
        isolate->ThrowError("Invalid Path");
    }
    try {
        auto path = Helpers::ConvertFromV8String(isolate, pathValue);
        fs_access_sync(rust::Str(path.c_str()), mode);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}


void FSImpl::ReadSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    int fd = -1;

    size_t offset = 0;
    size_t length = -1;
    rust::isize position = -1;

    auto fdValue = args[0];
    if (fdValue->IsNumber()) {
        fd = (int32_t) fdValue->NumberValue(ctx).ToChecked();
    }

    auto bufferValue = args[1];

    if (bufferValue->IsTypedArray()) {
        auto array = bufferValue.As<v8::TypedArray>();
        auto buffer = array->Buffer();
        auto store = buffer->GetBackingStore();
        auto os = array->ByteOffset();
        auto len = array->ByteLength();
        auto data = static_cast<uint8_t *>(store->Data()) + os;
        try {
            auto ret = fs_read_sync(fd, rust::Slice(data, len), offset, length, position);
            args.GetReturnValue().Set((double) ret);
            return;
        } catch (std::exception &error) {
            auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
            isolate->ThrowException(err);
        }

    } else if (bufferValue->IsDataView()) {
        auto view = bufferValue.As<v8::DataView>();
        auto buffer = view->Buffer();
        auto store = buffer->GetBackingStore();
        auto os = view->ByteOffset();
        auto len = view->ByteLength();
        auto data = static_cast<uint8_t *>(store->Data()) + os;
        try {
            auto ret = fs_read_sync(fd, rust::Slice(data, len), offset, length, position);
            args.GetReturnValue().Set((double) ret);
            return;
        } catch (std::exception &error) {
            auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
            isolate->ThrowException(err);
        }

    } else if (bufferValue->IsObject()) {
        auto ptr = BufferImpl::GetPointer(bufferValue.As<v8::Object>());

        if (ptr != nullptr) {
            auto data = buffer_buffer(ptr->GetBuffer());
            try {
                auto ret = fs_read_sync(fd, data, offset, length, position);
                args.GetReturnValue().Set((double) ret);
                return;
            } catch (std::exception &error) {
                auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
                isolate->ThrowException(err);
            }

        }
    }

    args.GetReturnValue().SetUndefined();

}
