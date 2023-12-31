//
// Created by Osei Fortune on 31/12/2023.
//

#ifndef NODECOMPATDEMO_FSWATCHERIMPL_H
#define NODECOMPATDEMO_FSWATCHERIMPL_H
#include "Common.h"
#include "Helpers.h"


class FSWatcherImpl {
private:
    AsyncFileWatchClosure *callback_;
    std::string filename_;
    bool closed_ = false;

public:

    FSWatcherImpl(AsyncFileWatchClosure *callback, const char *filename);

    ~FSWatcherImpl() {
        if (callback_ != nullptr) {
            fs_async_file_watch_closure_destroy(callback_);
            callback_ = nullptr;
        }
    }

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static FSWatcherImpl *GetPointer(v8::Local<v8::Object> object);


    static void Ref(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Unref(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Close(const v8::FunctionCallbackInfo<v8::Value> &args);

    AsyncFileWatchClosure &GetCallback() const {
        return *callback_;
    }
};


#endif //NODECOMPATDEMO_FSWATCHERIMPL_H
