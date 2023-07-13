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

    static void ChmodSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ChownSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CloseSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void OpenSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ExistsSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FchmodSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FchownSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FdatasyncSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void FStatSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CopyFileSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CpSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void LchmodSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void LchownSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void LutimesSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void LinkSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void StatSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void MkdirSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void MkdtempSync(const v8::FunctionCallbackInfo<v8::Value> &args);

};


#endif //NODECOMPATDEMO_FSIMPL_H
