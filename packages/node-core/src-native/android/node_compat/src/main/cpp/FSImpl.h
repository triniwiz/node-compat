//
// Created by Osei Fortune on 08/07/2023.
//

#ifndef NODECOMPATDEMO_FSIMPL_H
#define NODECOMPATDEMO_FSIMPL_H

#include "Common.h"
#include "node-cxx/src/lib.rs.h"

using namespace org::nativescript::nodecompat;

class FSImpl {
public:
    static void Init(v8::Isolate *isolate);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void AccessSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void AppendFileSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadSync(const v8::FunctionCallbackInfo<v8::Value> &args);

};


#endif //NODECOMPATDEMO_FSIMPL_H
