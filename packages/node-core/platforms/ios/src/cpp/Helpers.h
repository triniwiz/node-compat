//
// Created by Osei Fortune on 06/07/2023.
//

#ifndef NODECOMPATDEMO_HELPERS_H
#define NODECOMPATDEMO_HELPERS_H

#include "Common.h"

class Helpers {
public:
    static const char *LOG_TAG;

    static int m_maxLogcatObjectSize;

    static StringEncoding ParseEncoding(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                        StringEncoding defaultValue);

    static FsEncodingType ParseFsEncoding(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                          FsEncodingType defaultValue);

    static void ParseAppendFileOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                       AppendFileOptions &options);

    static void ParseMkDirOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                  MkDirOptions &options);

    static void ParseMkdTempOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                    MkdTempOptions &options);

    static void ParseOpenDirOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                    OpenDirOptions &options);

    static void ParseReaddirOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                    ReaddirOptions &options);

    static void ParseReadFileOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                     ReadFileOptions &options);

    static void ParseReadLinkOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                     ReadLinkOptions &options);

    static void ParseRealPathOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                     RealPathOptions &options);

    static void ParseRmDirOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                  RmDirOptions &options);

    static void ParseRmOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                               RmOptions &options);


    static void ParseWriteFileOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                      WriteFileOptions &options);


    static void ParseWriteOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                  WriteOptions &options);


    static v8::Local<v8::Object>
    FileStatToJS(v8::Isolate *isolate, bool bigInt, const FileStat &stat);
    
    
#ifdef __ANDROID__
    static void sendToADBLogcat(const std::string &message, android_LogPriority logPriority);
#endif
    
    static void LogToConsole(const std::string &message);

    static void ThrowIllegalConstructor(v8::Isolate *isolate);

    static v8::Local<v8::String> ConvertToV8String(v8::Isolate *isolate, const std::string &string);

    static std::string ConvertFromV8String(v8::Isolate *isolate, const v8::Local<v8::Value> &value);

    static v8::Local<v8::ArrayBuffer>
    ConvertToV8ArrayBuffer(v8::Isolate *isolate, const char *data, int size);

    static std::pair<void *, size_t>
    ConvertFromV8ArrayBuffer(v8::Isolate *isolate, v8::Local<v8::ArrayBuffer> param);

    static std::vector<v8::Local<v8::Value>>
    ConvertFromV8Array(v8::Isolate *isolate, const v8::Local<v8::Value> &value);

    static bool
    IsInstanceOf(v8::Isolate *isolate, v8::Local<v8::Value> value, const std::string &clazz);

    static void SetInternalClassName(v8::Isolate *isolate, v8::Local<v8::Object> value,
                                     const std::string &clazz);

    static void
    SetPrivate(v8::Isolate *isolate, v8::Local<v8::Object> object, const std::string &property,
               v8::Local<v8::Value> value);

    static v8::Local<v8::Value>
    GetPrivate(v8::Isolate *isolate, v8::Local<v8::Object> object, const std::string &property);
};

#endif //NODECOMPATDEMO_HELPERS_H
