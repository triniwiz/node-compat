//
// Created by Osei Fortune on 17/07/2023.
//

#ifndef NODECOMPATDEMO_FILEDIRENTIMPL_H
#define NODECOMPATDEMO_FILEDIRENTIMPL_H

#include "Common.h"
#include "node-cxx/src/lib.rs.h"

using namespace org::nativescript::nodecompat;

class FileDirentImpl {
private:
    rust::Box<FileDirent> dirent_;

public:

    FileDirent &GetDirent() {
        return *dirent_;
    }

    FileDirentImpl(rust::Box<FileDirent> dir);

    static void Init(v8::Isolate *isolate);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static FileDirentImpl *GetPointer(v8::Local<v8::Object> object);

    static void GetName(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void GetPath(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void IsBlockDevice(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsCharacterDevice(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsDirectory(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsFIFO(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsFile(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsSocket(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void IsSymbolicLink(const v8::FunctionCallbackInfo<v8::Value> &args);
};


#endif //NODECOMPATDEMO_FILEDIRENTIMPL_H
