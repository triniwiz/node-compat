//
// Created by Osei Fortune on 17/07/2023.
//

#ifndef NODECOMPATDEMO_FILEDIRIMPL_H
#define NODECOMPATDEMO_FILEDIRIMPL_H

#include "Common.h"
#include "node-cxx/src/lib.rs.h"

using namespace org::nativescript::nodecompat;

class FileDirImpl {
private:
    rust::Box<FileDir> dir_;
public:

    FileDir &GetDir() {
        return *dir_;
    }

    FileDirImpl(rust::Box<FileDir> dir);

    static void Init(v8::Isolate *isolate);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static FileDirImpl *GetPointer(v8::Local<v8::Object> object);

    static void GetPath(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void CloseSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadSync(const v8::FunctionCallbackInfo<v8::Value> &args);

};


#endif //NODECOMPATDEMO_FILEDIRIMPL_H
