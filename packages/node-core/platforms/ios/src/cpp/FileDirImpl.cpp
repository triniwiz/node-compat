//
// Created by Osei Fortune on 17/07/2023.
//

#include "FileDirImpl.h"
#include "Caches.h"
#include "FileDirentImpl.h"


void FileDirImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
//    auto context = isolate->GetCurrentContext();
//    auto global = context->Global();
//    auto func = ctor->GetFunction(context).ToLocalChecked();

    //  global->Set(context, Helpers::ConvertToV8String(isolate, "NSCBuffer"), func);
}

FileDirImpl *FileDirImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<FileDirImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> FileDirImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->BufferTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, nullptr);

    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "NSCFileDir"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);


    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "path"),
            &GetPath
    );


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "closeSync"),
            v8::FunctionTemplate::New(isolate, &CloseSync));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readSync"),
            v8::FunctionTemplate::New(isolate, &ReadSync));

    cache->FsDirTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);

    return ctorTmpl;

}

void
FileDirImpl::GetPath(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    if (ptr != nullptr) {
        auto path = fs_dir_path(ptr->dir_);
        info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, path));
        return;
    }
    info.GetReturnValue().SetUndefined();
}


void
FileDirImpl::CloseSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        try {
            fs_dir_close_sync(ptr->dir_);
        } catch (std::exception &error) {
            auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
            isolate->ThrowException(err);
        }
    }
    args.GetReturnValue().SetUndefined();
}


void
FileDirImpl::ReadSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        try {
            auto dirent = fs_dir_read_sync(ptr->dir_);

            auto ctor = FileDirentImpl::GetCtor(isolate)->New(isolate);
            auto func = ctor->GetFunction(ctx).ToLocalChecked();
            auto ret = func->NewInstance(ctx).ToLocalChecked();
            auto fileDirentImpl = new FileDirentImpl(std::move(dirent));
            auto ext = v8::External::New(isolate, fileDirentImpl);
            ret->SetInternalField(0, ext);

            args.GetReturnValue().Set(ret);
            return;
        } catch (std::exception &error) {
            auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
            isolate->ThrowException(err);
        }
    }
}

FileDirImpl::FileDirImpl(FileDir *dir) : dir_(dir) {}
