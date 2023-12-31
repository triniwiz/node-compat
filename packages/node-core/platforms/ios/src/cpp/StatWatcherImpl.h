//
// Created by Osei Fortune on 31/12/2023.
//

#ifndef NODECOMPATDEMO_STATWATCHERIMPL_H
#define NODECOMPATDEMO_STATWATCHERIMPL_H

#include "Common.h"
#include "Helpers.h"

class StatWatcherImpl {
private:
    AsyncWatchClosure *callback_;
    std::string filename_;

public:

    StatWatcherImpl(AsyncWatchClosure *callback, const char *filename);

    ~StatWatcherImpl() {
        if (callback_ != nullptr) {
            fs_async_watch_closure_destroy(callback_);
            callback_ = nullptr;
        }
    }

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static StatWatcherImpl *GetPointer(v8::Local<v8::Object> object);


    static void Ref(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Unref(const v8::FunctionCallbackInfo<v8::Value> &args);

    AsyncWatchClosure &GetCallback() const {
        return *callback_;
    }
};


#endif //NODECOMPATDEMO_STATWATCHERIMPL_H
