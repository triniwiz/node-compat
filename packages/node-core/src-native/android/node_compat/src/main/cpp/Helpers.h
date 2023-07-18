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

    static org::nativescript::nodecompat::FsEncodingType ParseFsEncoding(v8::Isolate *isolate, const v8::Local<v8::Value> &value, org::nativescript::nodecompat::FsEncodingType defaultValue);

    static void ParseAppendFileOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                       org::nativescript::nodecompat::AppendFileOptions &options);

    static void ParseMkDirOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                       org::nativescript::nodecompat::MkDirOptions &options);

    static void ParseMkdTempOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                  org::nativescript::nodecompat::MkdTempOptions &options);

    static void ParseOpenDirOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                      org::nativescript::nodecompat::OpenDirOptions &options);

    static void ParseReaddirOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                    org::nativescript::nodecompat::ReaddirOptions &options);

    static void ParseReadFileOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                    org::nativescript::nodecompat::ReadFileOptions &options);

    static void ParseReadLinkOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                     org::nativescript::nodecompat::ReadLinkOptions &options);

    static void ParseRealPathOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                     org::nativescript::nodecompat::RealPathOptions &options);

    static void ParseRmDirOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                     org::nativescript::nodecompat::RmDirOptions &options);

    static void ParseRmOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                  org::nativescript::nodecompat::RmOptions &options);


    static void ParseWriteFileOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                               org::nativescript::nodecompat::WriteFileOptions &options);



    static void ParseWriteOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                      org::nativescript::nodecompat::WriteOptions &options);


    static v8::Local<v8::Object> FileStatToJS(v8::Isolate *isolate, bool bigInt, const org::nativescript::nodecompat::FileStat &stat);

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
