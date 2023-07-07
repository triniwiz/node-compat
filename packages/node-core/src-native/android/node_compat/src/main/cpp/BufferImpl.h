//
// Created by Osei Fortune on 06/07/2023.
//

#ifndef NODECOMPATDEMO_BUFFERIMPL_H
#define NODECOMPATDEMO_BUFFERIMPL_H

#include "Common.h"
#include "node-cxx/src/lib.rs.h"

using namespace org::nativescript::nodecompat;

class BufferImpl {
private:
    rust::Box<Buffer> buffer_;

public:
    BufferImpl(rust::Box<Buffer> buffer);

    static void Init(v8::Isolate *isolate);

    static BufferImpl *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void Alloc(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Length(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Atob(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Btoa(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Concat(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void From(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ToString(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Fill(const v8::FunctionCallbackInfo<v8::Value> &args);
};


#endif //NODECOMPATDEMO_BUFFERIMPL_H
