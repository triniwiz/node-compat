//
// Created by Osei Fortune on 31/12/2023.
//

#include "StatWatcherImpl.h"
#include "Caches.h"

StatWatcherImpl::StatWatcherImpl(AsyncWatchClosure *callback, const char *filename) : callback_(
        callback), filename_(filename) {

}


StatWatcherImpl *StatWatcherImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<StatWatcherImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> StatWatcherImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->StatWatcherImpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, nullptr);

    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "NSCStatWatcher"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "ref"),
            v8::FunctionTemplate::New(isolate, &Ref));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "unref"),
            v8::FunctionTemplate::New(isolate, &Unref));


    cache->StatWatcherImpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);

    return ctorTmpl;

}


void StatWatcherImpl::Ref(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        fs_async_watcher_ref(ptr->filename_.c_str(), ptr->callback_);
    }
}


void StatWatcherImpl::Unref(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        fs_async_watcher_unref(ptr->filename_.c_str(), ptr->callback_);
    }
}
