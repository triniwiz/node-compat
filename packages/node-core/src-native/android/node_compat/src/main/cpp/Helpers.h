//
// Created by Osei Fortune on 06/07/2023.
//

#ifndef NODECOMPATDEMO_HELPERS_H
#define NODECOMPATDEMO_HELPERS_H
#include "Common.h"
#include "node-cxx/src/lib.rs.h"

class Helpers {
public:
    static const char* LOG_TAG;

    static int m_maxLogcatObjectSize;

    static org::nativescript::nodecompat::StringEncoding ParseEncoding(v8::Isolate *isolate, const v8::Local<v8::Value> &value, org::nativescript::nodecompat::StringEncoding defaultValue);

    static void sendToADBLogcat(const std::string& message, android_LogPriority logPriority);

    static void LogToConsole(const std::string &message);

    static void ThrowIllegalConstructor(v8::Isolate *isolate);

    static v8::Local<v8::String> ConvertToV8String(v8::Isolate *isolate, const std::string &string);

    static std::string ConvertFromV8String(v8::Isolate *isolate, const v8::Local<v8::Value> &value);

    static v8::Local<v8::ArrayBuffer> ConvertToV8ArrayBuffer(v8::Isolate* isolate, const char* data, int size);

    static std::pair<void*, size_t> ConvertFromV8ArrayBuffer(v8::Isolate* isolate, v8::Local<v8::ArrayBuffer> param);

    static std::vector<v8::Local<v8::Value>> ConvertFromV8Array(v8::Isolate* isolate, const v8::Local<v8::Value>& value);

    static bool IsInstanceOf(v8::Isolate *isolate, v8::Local<v8::Value> value, const std::string& clazz);

    static void SetInternalClassName(v8::Isolate *isolate, v8::Local<v8::Object> value, const std::string& clazz);

    static void
    SetPrivate(v8::Isolate *isolate, v8::Local<v8::Object> object, const std::string& property, v8::Local<v8::Value> value);

    static v8::Local<v8::Value> GetPrivate(v8::Isolate *isolate, v8::Local<v8::Object> object, const std::string& property);
};

#endif //NODECOMPATDEMO_HELPERS_H