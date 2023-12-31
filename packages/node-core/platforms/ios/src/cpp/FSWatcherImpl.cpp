//
// Created by Osei Fortune on 31/12/2023.
//

#include "FSWatcherImpl.h"
#include "Caches.h"

FSWatcherImpl::FSWatcherImpl(AsyncFileWatchClosure *callback, const char *filename) : callback_(
        callback), filename_(filename) {

}


FSWatcherImpl *FSWatcherImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<FSWatcherImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> FSWatcherImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->FSWatcherImpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, nullptr);

    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "NSCFSWatcher"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "ref"),
            v8::FunctionTemplate::New(isolate, &Ref));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "unref"),
            v8::FunctionTemplate::New(isolate, &Unref));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "close"),
            v8::FunctionTemplate::New(isolate, &Close));


    cache->FSWatcherImpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);

    return ctorTmpl;

}


void FSWatcherImpl::Ref(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        fs_async_file_watcher_ref(ptr->filename_.c_str(), ptr->callback_);
    }
}


void FSWatcherImpl::Unref(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        fs_async_file_watcher_unref(ptr->filename_.c_str(), ptr->callback_);
    }
}

void FSWatcherImpl::Close(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr && !ptr->closed_) {
        fs_async_file_watcher_unref(ptr->filename_.c_str(), ptr->callback_);
        fs_async_file_watch_closure_destroy(ptr->callback_);
        ptr->callback_ = nullptr;
    }
}
