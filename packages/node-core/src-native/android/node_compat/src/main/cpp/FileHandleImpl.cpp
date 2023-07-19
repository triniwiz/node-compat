//
// Created by Osei Fortune on 18/07/2023.
//

#include "FileHandleImpl.h"
#include "Caches.h"
#include "node-cxx/src/lib.rs.h"

using namespace rust;

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


    cache->FileHandleTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);

    return ctorTmpl;

}

FileHandleImpl::FileHandleImpl(rust::Box<FileHandle> handle) : handle_(std::move(handle)) {}

void FileHandleImpl::AppendFile(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto src = args[0];

    auto ptr = GetPointer(args.This());
    AppendFileOptions options{};
    auto optionsValue = args[2];
    Helpers::ParseAppendFileOptions(isolate, optionsValue, options);

    if (src->IsString()) {
        auto error = []() {
            Helpers::LogToConsole("error");
        };
        auto success = []() {
            Helpers::LogToConsole("success");
        };
        auto closure = fs_async_create_async_closure(&success, &error);
        fs_handle_append_file_with_str(ptr->GetFileHandle(), Helpers::ConvertFromV8String(isolate, src), options, *closure);

    } else if (src->IsTypedArray()) {

        auto error = []() {
            Helpers::LogToConsole("error");
        };
        auto success = []() {
            Helpers::LogToConsole("success");
        };
        auto closure = fs_async_create_async_closure(&success, &error);


        auto array = src.As<v8::TypedArray>();
        auto buffer = array->Buffer();
        auto store = buffer->GetBackingStore();
        auto offset = array->ByteOffset();
        auto data = static_cast<uint8_t *>(store->Data()) + offset;
        auto length = array->ByteLength();

        auto buf = buffer_from_reference(data, length);

        fs_handle_append_file_with_bytes(ptr->GetFileHandle(), *buf, options, *closure);
    }
}
