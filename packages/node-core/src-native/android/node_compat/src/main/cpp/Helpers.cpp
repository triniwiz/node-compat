//
// Created by Osei Fortune on 06/07/2023.
//

#include "Helpers.h"

const char *Helpers::LOG_TAG = "JS";
int Helpers::m_maxLogcatObjectSize = 4096;

void Helpers::sendToADBLogcat(const std::string &message, android_LogPriority logPriority) {
    // limit the size of the message that we send to logcat using the predefined value in package.json
    auto messageToLog = message;
    if (messageToLog.length() > m_maxLogcatObjectSize) {
        messageToLog = messageToLog.erase(m_maxLogcatObjectSize, std::string::npos);
        messageToLog = messageToLog + "...";
    }

    // split strings into chunks of 4000 characters
    // __android_log_write can't send more than 4000 to the stdout at a time
    auto messageLength = messageToLog.length();
    int maxStringLength = 4000;

    if (messageLength < maxStringLength) {
        __android_log_write(logPriority, Helpers::LOG_TAG, messageToLog.c_str());
    } else {
        for (int i = 0; i < messageLength; i += maxStringLength) {
            auto messagePart = messageToLog.substr(i, maxStringLength);

            __android_log_write(logPriority, Helpers::LOG_TAG, messagePart.c_str());
        }
    }
}

void Helpers::LogToConsole(const std::string &message) {
    sendToADBLogcat(message, android_LogPriority::ANDROID_LOG_INFO);
}

void Helpers::ThrowIllegalConstructor(v8::Isolate *isolate) {
    auto msg = ConvertToV8String(isolate, "Illegal constructor");
    auto err = v8::Exception::TypeError(msg);
    isolate->ThrowException(err);
}

v8::Local<v8::String> Helpers::ConvertToV8String(v8::Isolate *isolate, const std::string &string) {
    return v8::String::NewFromUtf8(isolate, string.c_str()).ToLocalChecked();
}

std::string Helpers::ConvertFromV8String(v8::Isolate *isolate, const v8::Local<v8::Value> &value) {
    if (value.IsEmpty()) {
        return {};
    }

    if (value->IsStringObject()) {
        v8::Local<v8::String> obj = value.As<v8::StringObject>()->ValueOf();
        return ConvertFromV8String(isolate, obj);
    }

    v8::String::Utf8Value result(isolate, value);

    const char *val = *result;

    if (val == nullptr) {
        return {};
    }

    return {*result};
}

v8::Local<v8::ArrayBuffer>
Helpers::ConvertToV8ArrayBuffer(v8::Isolate *isolate, const char *data, int size) {
    v8::Local<v8::ArrayBuffer> arrayBuffer = v8::ArrayBuffer::New(isolate, size);
    std::shared_ptr<v8::BackingStore> arrayBufferContents = arrayBuffer->GetBackingStore();

    memcpy(arrayBufferContents->Data(), data, size);

    return arrayBuffer;
}

std::pair<void *, size_t>
Helpers::ConvertFromV8ArrayBuffer(v8::Isolate *isolate, v8::Local<v8::ArrayBuffer> param) {
    void *data = param->GetBackingStore()->Data();
    size_t length = param->ByteLength();

    return std::make_pair(data, length);
}

std::vector<v8::Local<v8::Value>>
Helpers::ConvertFromV8Array(v8::Isolate *isolate, const v8::Local<v8::Value> &value) {
    std::vector<v8::Local<v8::Value>> vec;
    auto context = isolate->GetCurrentContext();

    if (value.IsEmpty()) {
        return {};
    }

    if (value->IsArray()) {
        v8::Local<v8::Array> arr = v8::Local<v8::Array>::Cast(value);
        uint32_t length = arr->Length();

        v8::Local<v8::Object> obj = arr.As<v8::Object>();
        v8::Local<v8::Array> keys = obj->GetPropertyNames(context).ToLocalChecked();
        uint32_t keysLength = keys->Length();

        for (uint32_t i = 0; i < keysLength; i++) {
            v8::Local<v8::Value> key = keys->Get(context, i).ToLocalChecked();
            v8::Local<v8::Value> value = obj->Get(context, key).ToLocalChecked();
        }

        for (uint32_t i = 0; i < length; i++) {
            v8::MaybeLocal<v8::Value> maybeElement = arr->Get(isolate->GetCurrentContext(),
                                                              Helpers::ConvertToV8String(isolate,
                                                                                         std::to_string(
                                                                                                 i)));
            if (maybeElement.IsEmpty()) {
                continue;
            }
            v8::Local<v8::Value> element = maybeElement.ToLocalChecked();
            vec.push_back(element);
        }
    } else {}

    return vec;
}

bool
Helpers::IsInstanceOf(v8::Isolate *isolate, v8::Local<v8::Value> value, const std::string &clazz) {
    auto context = isolate->GetCurrentContext();

    if (value.IsEmpty()) {
        return false;
    }

    if (value->IsNullOrUndefined()) {
        return false;
    }

    if (!value->IsObject()) {
        return false;
    }

    //    auto key = v8::Private::New(isolate,
    //                                Helpers::ConvertToV8String(isolate,
    //                                                           "class_name"));
    //    auto instance = value->GetPrivate(context, key);
    //    if(instance.IsEmpty()){
    //        return false;
    //    }
    //
    //    auto to_cmp = Helpers::ConvertFromV8String(isolate, instance.ToLocalChecked()->ToString(context).ToLocalChecked());
    //    return std::strcmp(clazz.c_str(), to_cmp.c_str()) == 0;

    v8::TryCatch tryCatch(isolate);
    v8::Local<v8::Value> object;

    if (context->Global()
            ->GetRealNamedProperty(context, Helpers::ConvertToV8String(isolate, clazz))
            .ToLocal(&object)) {

        if (object->IsFunction()) {
            auto name = object.As<v8::Function>()->GetName();
            v8::String::Utf8Value a(isolate, name.As<v8::String>());
            std::string a_val(*a, a.length());

            if (value->IsFunction()) {
                auto value_name = value.As<v8::Function>()->GetName();
                v8::String::Utf8Value b(isolate, value_name.As<v8::String>());
                std::string b_val(*b, b.length());
                if (std::strcmp(
                        a_val.c_str(),
                        b_val.c_str()) !=
                    0) {
                    return false;
                }
            }

            if (name->IsString()) {
                if (std::strcmp(a_val.c_str(), clazz.c_str()) ==
                    0) {
                    return true;
                }
            }
        }
        if (object->IsObject() &&
            value->ToObject(context).ToLocalChecked()->InstanceOf(context,
                                                                  object.As<v8::Object>()).FromMaybe(
                    false)) {
            return true;
        }
    }

    if (tryCatch.HasCaught())
        tryCatch.Reset();
    return false;
}

void Helpers::SetInternalClassName(v8::Isolate *isolate, v8::Local<v8::Object> value,
                                   const std::string &clazz) {
    auto context = isolate->GetCurrentContext();
    value->SetPrivate(context,
                      v8::Private::New(isolate, Helpers::ConvertToV8String(isolate, "class_name")),
                      Helpers::ConvertToV8String(isolate, clazz));
}

void
Helpers::SetPrivate(v8::Isolate *isolate, v8::Local<v8::Object> object, const std::string &property,
                    v8::Local<v8::Value> value) {
    auto context = isolate->GetCurrentContext();
    auto key = v8::Private::ForApi(isolate, Helpers::ConvertToV8String(isolate, property));
    object->SetPrivate(context, key, value);
}

v8::Local<v8::Value> Helpers::GetPrivate(v8::Isolate *isolate, v8::Local<v8::Object> object,
                                         const std::string &property) {
    auto context = isolate->GetCurrentContext();
    auto key = v8::Private::ForApi(isolate, Helpers::ConvertToV8String(isolate, property));
    auto value = object->GetPrivate(context, key);
    if (value.IsEmpty()) {
        return v8::Undefined(isolate);
    } else {
        return value.ToLocalChecked();
    }
}

org::nativescript::nodecompat::StringEncoding
Helpers::ParseEncoding(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                       org::nativescript::nodecompat::StringEncoding defaultValue) {

    if (value->IsString() || value->IsStringObject()) {
        auto val = Helpers::ConvertFromV8String(isolate, value);
        if (val == "utf8" || val == "utf-8") {
            return org::nativescript::nodecompat::StringEncoding::Utf8;
        } else if (val == "utf16le" || val == "utf-16le") {
            return org::nativescript::nodecompat::StringEncoding::Utf16le;
        } else if (val == "latin1") {
            return org::nativescript::nodecompat::StringEncoding::Latin1;
        } else if (val == "base64") {
            return org::nativescript::nodecompat::StringEncoding::Base64;
        } else if (val == "base64url") {
            return org::nativescript::nodecompat::StringEncoding::Base64Url;
        } else if (val == "hex") {
            return org::nativescript::nodecompat::StringEncoding::Hex;
        } else if (val == "binary") {
            return org::nativescript::nodecompat::StringEncoding::Binary;
        } else if (val == "ucs2" || val == "ucs-2") {
            return org::nativescript::nodecompat::StringEncoding::Ucs2;
        }
    }

    return defaultValue;
}

void Helpers::ParseAppendFileOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                     org::nativescript::nodecompat::AppendFileOptions &options) {
    if (value->IsObject() && !value->IsNullOrUndefined()) {
        auto ctx = isolate->GetCurrentContext();
        auto val = value.As<v8::Object>();

        v8::Local<v8::Value> encodingValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "encoding")).ToLocal(&encodingValue);

        if (encodingValue->IsString()) {
            options.encoding = ParseEncoding(isolate, encodingValue,
                                             org::nativescript::nodecompat::StringEncoding::Utf8);
        }

        v8::Local<v8::Value> modeValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "mode")).ToLocal(&modeValue);

        if (modeValue->IsNumber()) {
            options.mode = (int32_t) modeValue->NumberValue(ctx).ToChecked();
        }


        v8::Local<v8::Value> flagValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "flag")).ToLocal(&flagValue);


        if(flagValue->IsNumber()){
            options.flag = (int32_t) flagValue->NumberValue(ctx).ToChecked();
        }
    }
}
