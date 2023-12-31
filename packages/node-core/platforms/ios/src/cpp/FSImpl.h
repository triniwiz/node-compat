//
// Created by Osei Fortune on 08/07/2023.
//

#ifndef NODECOMPATDEMO_FSIMPL_H
#define NODECOMPATDEMO_FSIMPL_H

#include "Common.h"
#include "fcntl.h"
#include "Helpers.h"

class FSImpl {
public:
    static void Init(v8::Isolate *isolate);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void AccessSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void AppendFileSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ChmodSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ChownSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CloseSync(const v8::FunctionCallbackInfo<v8::Value> &args);

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

    static void MkdirSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void MkdtempSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void OpenDirSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReaddirSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadFileSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadLinkSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadvSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RealpathSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RenameSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RmdirSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void RmSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void StatSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void StatfsSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SymlinkSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void TruncateSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void UnlinkSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void UtimesSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteFileSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WritevSync(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Open(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void OpenHandle(const v8::FunctionCallbackInfo<v8::Value> &args);

};


#endif //NODECOMPATDEMO_FSIMPL_H
