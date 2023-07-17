//
// Created by Osei Fortune on 06/07/2023.
//

#ifndef NODECOMPATDEMO_CACHES_H
#define NODECOMPATDEMO_CACHES_H


#include "Common.h"
#include "ConcurrentMap.h"

class Caches {
public:
    Caches(v8::Isolate *isolate);

    ~Caches();

    static std::shared_ptr <Caches> Get(v8::Isolate *isolate);

    static void Remove(v8::Isolate *isolate);

    void SetContext(v8::Local<v8::Context> context);

    v8::Local<v8::Context> GetContext();


    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> BufferTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> FsTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> FsStatTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);

    std::unique_ptr <v8::Persistent<v8::FunctionTemplate>> FsDirTmpl = std::unique_ptr<v8::Persistent<v8::FunctionTemplate>>(
            nullptr);
private:
    static std::shared_ptr <ConcurrentMap<v8::Isolate *,
            std::shared_ptr < Caches>>>
    perIsolateCaches_;
    v8::Isolate *isolate_;
    std::shared_ptr <v8::Persistent<v8::Context>> context_;
};

#endif //NODECOMPATDEMO_CACHES_H
