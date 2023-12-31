//
// Created by Osei Fortune on 06/07/2023.
//

#ifndef NODECOMPATDEMO_BUFFERIMPL_H
#define NODECOMPATDEMO_BUFFERIMPL_H

#include "Common.h"
#include "Helpers.h"

class BufferImpl {
private:
    Buffer *buffer_;
    std::shared_ptr<v8::BackingStore> store_;

public:

    Buffer &GetBuffer() const {
        return *buffer_;
    }

    ~BufferImpl() {
        if (buffer_ != nullptr) {
            buffer_destroy(buffer_);
            buffer_ = nullptr;
        }
    }

    Buffer *GetBuffer() {
        return buffer_;
    }

    BufferImpl(Buffer *buffer);

    static void Init(v8::Isolate *isolate);

    static BufferImpl *GetPointer(v8::Local<v8::Object> object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static void IndexedGetter(uint32_t index, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void IndexedSetter(uint32_t index, v8::Local<v8::Value> value,
                              const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Alloc(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void
    GetBuffer(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Length(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Atob(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Btoa(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Concat(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void CopyBytesFrom(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void From(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ToString(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Fill(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteInt8(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteUInt8(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteInt16LE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteInt16BE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteUInt16LE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteUInt16BE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteInt32LE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteInt32BE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteUInt32LE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteUInt32BE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteFloatLE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteFloatBE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteDoubleLE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteDoubleBE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteBigInt64LE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteBigInt64BE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteBigUInt64LE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void WriteBigUInt64BE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadInt8(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadUInt8(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadInt16LE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadInt16BE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadUInt16LE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadUInt16BE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadInt32LE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadInt32BE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadUInt32LE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadUInt32BE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadFloatLE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadFloatBE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadDoubleLE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadDoubleBE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadBigInt64LE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadBigInt64BE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadBigUInt64LE(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ReadBigUInt64BE(const v8::FunctionCallbackInfo<v8::Value> &args);
};


#endif //NODECOMPATDEMO_BUFFERIMPL_H
