//
// Created by Osei Fortune on 18/07/2023.
//

#ifndef NODECOMPATDEMO_FILEHANDLEIMPL_H
#define NODECOMPATDEMO_FILEHANDLEIMPL_H

#include "Common.h"
#include "Helpers.h"

class FileHandleImpl {
private:
    FileHandle *handle_;

public:
    FileHandle &GetFileHandle() {
        return *handle_;
    }

    FileHandleImpl(FileHandle *handle);

    ~FileHandleImpl() {
        if (handle_ != nullptr) {
            fs_filehandle_destroy(handle_);
            handle_ = nullptr;
        }
    }

    static void Init(v8::Isolate *isolate);

    static FileHandleImpl *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void AppendFile(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Chmod(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Chown(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Close(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DataSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void GetFd(v8::Local<v8::String> name,
                      const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Read(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadFile(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Readv(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Stat(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Sync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Truncate(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Utimes(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteFile(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Write(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Writev(const v8::FunctionCallbackInfo<v8::Value> &args);

};


#endif //NODECOMPATDEMO_FILEHANDLEIMPL_H
