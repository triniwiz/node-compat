//
// Created by Osei Fortune on 17/07/2023.
//

#include "FileDirentImpl.h"
#include "Caches.h"

using namespace rust;

void FileDirentImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
//    auto context = isolate->GetCurrentContext();
//    auto global = context->Global();
//    auto func = ctor->GetFunction(context).ToLocalChecked();

    //  global->Set(context, Helpers::ConvertToV8String(isolate, "NSCBuffer"), func);
}

FileDirentImpl *FileDirentImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<FileDirentImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> FileDirentImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->BufferTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, nullptr);

    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "NSCFileDirent"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);


    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "path"),
            &GetPath
    );

    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "name"),
            &GetName
    );


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "isBlockDevice"),
            v8::FunctionTemplate::New(isolate, &IsBlockDevice));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "isCharacterDevice"),
            v8::FunctionTemplate::New(isolate, &IsCharacterDevice));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "isDirectory"),
            v8::FunctionTemplate::New(isolate, &IsDirectory));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "isFIFO"),
            v8::FunctionTemplate::New(isolate, &IsFIFO));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "isFile"),
            v8::FunctionTemplate::New(isolate, &IsFile));
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "isSocket"),
            v8::FunctionTemplate::New(isolate, &IsSocket));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "isSymbolicLink"),
            v8::FunctionTemplate::New(isolate, &IsSymbolicLink));

    cache->FsDirentTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);

    return ctorTmpl;

}

void
FileDirentImpl::GetPath(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    if (ptr != nullptr) {
        auto path = fs_dirent_path(*ptr->dirent_);
        info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, path.c_str()));
        return;
    }
    info.GetReturnValue().SetUndefined();
}

void
FileDirentImpl::GetName(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    if (ptr != nullptr) {
        auto path = fs_dirent_name(*ptr->dirent_);
        info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, path.c_str()));
        return;
    }
    info.GetReturnValue().SetUndefined();
}

FileDirentImpl::FileDirentImpl(rust::Box<FileDirent> dirent) : dirent_(std::move(dirent)) {}

void FileDirentImpl::IsBlockDevice(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    bool ret = false;
    if (ptr != nullptr) {
        ret = fs_dirent_is_block_device(ptr->GetDirent());
    }
    args.GetReturnValue().Set(ret);
}

void FileDirentImpl::IsCharacterDevice(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    bool ret = false;
    if (ptr != nullptr) {
        ret = fs_dirent_is_character_device(ptr->GetDirent());
    }
    args.GetReturnValue().Set(ret);
}

void FileDirentImpl::IsDirectory(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    bool ret = false;
    if (ptr != nullptr) {
        ret = fs_dirent_is_directory(ptr->GetDirent());
    }
    args.GetReturnValue().Set(ret);
}

void FileDirentImpl::IsFIFO(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    bool ret = false;
    if (ptr != nullptr) {
        ret = fs_dirent_is_fifo(ptr->GetDirent());
    }
    args.GetReturnValue().Set(ret);
}

void FileDirentImpl::IsFile(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    bool ret = false;
    if (ptr != nullptr) {
        ret = fs_dirent_is_file(ptr->GetDirent());
    }
    args.GetReturnValue().Set(ret);
}

void FileDirentImpl::IsSocket(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    bool ret = false;
    if (ptr != nullptr) {
        ret = fs_dirent_is_socket(ptr->GetDirent());
    }
    args.GetReturnValue().Set(ret);
}

void FileDirentImpl::IsSymbolicLink(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    bool ret = false;
    if (ptr != nullptr) {
        ret = fs_dirent_is_symbolic_link(ptr->GetDirent());
    }
    args.GetReturnValue().Set(ret);
}
