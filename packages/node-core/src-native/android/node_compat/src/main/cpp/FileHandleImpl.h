//
// Created by Osei Fortune on 18/07/2023.
//

#ifndef NODECOMPATDEMO_FILEHANDLEIMPL_H
#define NODECOMPATDEMO_FILEHANDLEIMPL_H

#include "Common.h"
#include "node-cxx/src/lib.rs.h"

using namespace org::nativescript::nodecompat;

class FileHandleImpl {
private:
    rust::Box<FileHandle> handle_;

public:
    FileHandle &GetFileHandle() {
        return *handle_;
    }

    FileHandleImpl(rust::Box<FileHandle> handle);

    static void Init(v8::Isolate *isolate);

    static FileHandleImpl *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void AppendFile(const v8::FunctionCallbackInfo<v8::Value> &args);

};


#endif //NODECOMPATDEMO_FILEHANDLEIMPL_H
