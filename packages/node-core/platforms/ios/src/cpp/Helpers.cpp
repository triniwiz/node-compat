//
// Created by Osei Fortune on 06/07/2023.
//

#include "Helpers.h"
#include "BufferImpl.h"
#include "FileDirentImpl.h"
#include "FileDirImpl.h"
#include "FileHandleImpl.h"

const char *Helpers::LOG_TAG = "JS";
int Helpers::m_maxLogcatObjectSize = 4096;


extern "C" {
void async_success_closure(void *data) {
    if (data != nullptr) {
        auto func = static_cast<AsyncCallback *>(data);
        if (func->isolate != nullptr && !func->isolate->IsDead()) {
            v8::Isolate *isolate = func->isolate;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = func->callback.Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);

            v8::Local<v8::Value> args[2] = {v8::Null(isolate), v8::Null(isolate)};


            callback->Call(context, context->Global(), 2,
                           args);  // ignore JS return value

            delete func;
        }
    }
}
void async_success_i32(int32_t fd, void *data) {
    if (data != nullptr) {
        auto func = static_cast<AsyncCallback *>(data);
        if (func->isolate != nullptr && !func->isolate->IsDead()) {
            v8::Isolate *isolate = func->isolate;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = func->callback.Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);

            v8::Local<v8::Value> args[2] = {v8::Null(isolate), v8::Integer::New(isolate, fd)};


            callback->Call(context, context->Global(), 2,
                           args);  // ignore JS return value

            delete func;
        }
    }
}
void async_success_bool(bool value, void *data) {
    if (data != nullptr) {
        auto func = static_cast<AsyncCallback *>(data);
        if (func->isolate != nullptr && !func->isolate->IsDead()) {
            v8::Isolate *isolate = func->isolate;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = func->callback.Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);

            v8::Local<v8::Value> args[2] = {v8::Null(isolate), v8::Boolean::New(isolate, value)};


            callback->Call(context, context->Global(), 2,
                           args);  // ignore JS return value

            delete func;
        }
    }
}
void async_success_filestat(FileStat *value, void *data) {
    if (data != nullptr) {
        auto func = static_cast<AsyncCallback *>(data);
        if (func->isolate != nullptr && !func->isolate->IsDead()) {
            v8::Isolate *isolate = func->isolate;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = func->callback.Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);

            auto val = Helpers::FileStatToJS(isolate, false, *value);

            filestat_destroy(value);

            v8::Local<v8::Value> args[2] = {v8::Null(isolate), val};


            callback->Call(context, context->Global(), 2,
                           args);  // ignore JS return value

            delete func;
        }
    }
}
void async_success_string(const char *value, void *data) {
    if (data != nullptr) {
        auto func = static_cast<AsyncCallback *>(data);
        if (func->isolate != nullptr && !func->isolate->IsDead()) {
            v8::Isolate *isolate = func->isolate;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = func->callback.Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);
            auto val = Helpers::ConvertToV8String(isolate, value);
            node_string_destroy((char *) value);

            v8::Local<v8::Value> args[2] = {v8::Null(isolate), val};


            callback->Call(context, context->Global(), 2,
                           args);  // ignore JS return value

            delete func;
        }
    }
}
void async_success_usize(uintptr_t value, void *data) {
    if (data != nullptr) {
        auto func = static_cast<AsyncCallback *>(data);
        if (func->isolate != nullptr && !func->isolate->IsDead()) {
            v8::Isolate *isolate = func->isolate;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = func->callback.Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);

            v8::Local<v8::Value> args[2] = {v8::Null(isolate), v8::Number::New(isolate, value)};


            callback->Call(context, context->Global(), 2,
                           args);  // ignore JS return value

            delete func;
        }
    }
}
void async_success_filewatch_event(FileWatchEvent *value, void *data) {
    if (data != nullptr) {
        auto func = static_cast<AsyncCallback *>(data);
        if (func->isolate != nullptr && !func->isolate->IsDead()) {
            v8::Isolate *isolate = func->isolate;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = func->callback.Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);

            auto val = v8::Object::New(isolate);
            auto current = fs_filewatch_event_current(value);
            auto previous = fs_filewatch_event_previous(value);
            auto currentJS = Helpers::FileStatToJS(isolate, false, *current);
            auto previousJS = Helpers::FileStatToJS(isolate, false, *previous);
            val->Set(context, Helpers::ConvertToV8String(isolate, "current"), currentJS);
            val->Set(context, Helpers::ConvertToV8String(isolate, "previous"), previousJS);
            fs_filewatch_event_destroy(value);

            v8::Local<v8::Value> args[2] = {v8::Null(isolate), val};


            callback->Call(context, context->Global(), 2,
                           args);  // ignore JS return value

            delete func;
        }
    }
}
void async_success_watch_event(WatchEvent *value, void *data) {
    if (data != nullptr) {
        auto func = static_cast<AsyncCallback *>(data);
        if (func->isolate != nullptr && !func->isolate->IsDead()) {
            v8::Isolate *isolate = func->isolate;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = func->callback.Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);

            auto val = v8::Object::New(isolate);
            auto event_type = fs_watch_event_event_type(value);
            auto filename = fs_watch_event_filename(value);
            auto event_typeJS = Helpers::ConvertToV8String(isolate, event_type);
            auto filenameJS = Helpers::ConvertToV8String(isolate, filename);
            val->Set(context, Helpers::ConvertToV8String(isolate, "eventType"), event_typeJS);
            val->Set(context, Helpers::ConvertToV8String(isolate, "filename"), filenameJS);
            fs_watch_event_destroy(value);

            v8::Local<v8::Value> args[2] = {v8::Null(isolate), val};


            callback->Call(context, context->Global(), 2,
                           args);  // ignore JS return value

            delete func;
        }
    }
}
void async_success_fs_encoding(FsEncoding *value, void *data) {
    if (data != nullptr) {
        auto func = static_cast<AsyncCallback *>(data);
        if (func->isolate != nullptr && !func->isolate->IsDead()) {
            v8::Isolate *isolate = func->isolate;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = func->callback.Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);

            v8::Local<v8::Value> ret;
            if (fs_encoding_is_buffer(value)) {
                auto buf = fs_encoding_get_buffer_value(value);
                auto bufferImpl = new BufferImpl(buf);

                auto ext = v8::External::New(isolate, bufferImpl);

                auto ctor = BufferImpl::GetCtor(isolate);
                auto val = ctor->GetFunction(context).ToLocalChecked()->NewInstance(
                        context).ToLocalChecked();
                val->SetInternalField(0, ext);
                ret = val;

            } else {
                auto string = fs_encoding_get_string_value(value);
                ret = Helpers::ConvertToV8String(isolate, string);
                node_string_destroy((char *) string);
            }

            fs_encoding_destroy(value);

            v8::Local<v8::Value> args[2] = {v8::Null(isolate), ret};


            callback->Call(context, context->Global(), 2,
                           args);  // ignore JS return value

            delete func;
        }
    }
}
void async_success_readdir(ReaddirResultArray *value, void *data) {
    if (data != nullptr) {
        auto func = static_cast<AsyncCallback *>(data);
        if (func->isolate != nullptr && !func->isolate->IsDead()) {
            v8::Isolate *isolate = func->isolate;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = func->callback.Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);
            auto len = value->length;
            auto size = sizeof(value->data);
            auto array = v8::Array::New(isolate, len);
            for (int i = 0; i < len; i++) {
                auto item = (&value->data)[size * i];
                switch (fs_readdir_get_type(item)) {
                    case ReaddirResultTypeString:
                        array->Set(context, i, Helpers::ConvertToV8String(isolate,
                                                                          fs_readdir_get_string_value(
                                                                                  item)));
                        break;
                    case ReaddirResultTypeBuffer: {
                        auto buf = fs_readdir_get_buffer_value(item);
                        auto bufferImpl = new BufferImpl(buf);

                        auto ext = v8::External::New(isolate, bufferImpl);

                        auto ctor = BufferImpl::GetCtor(isolate);
                        auto val = ctor->GetFunction(context).ToLocalChecked()->NewInstance(
                                context).ToLocalChecked();
                        val->SetInternalField(0, ext);

                        array->Set(context, i, val);
                    }
                        break;
                    case ReaddirResultTypeType: {
                        auto dirent = fs_readdir_get_type_value(item);
                        auto direntImpl = new FileDirentImpl(dirent);

                        auto ext = v8::External::New(isolate, direntImpl);

                        auto ctor = FileDirentImpl::GetCtor(isolate);
                        auto val = ctor->GetFunction(context).ToLocalChecked()->NewInstance(
                                context).ToLocalChecked();

                        val->SetInternalField(0, ext);

                        array->Set(context, i, val);
                    }
                        break;
                }
            }
            fs_readdir_result_array_destroy(value);

            v8::Local<v8::Value> args[2] = {v8::Null(isolate), array};


            callback->Call(context, context->Global(), 2,
                           args);  // ignore JS return value

            delete func;
        }
    }
}
void async_success_filedir(FileDir *value, void *data) {
    if (data != nullptr) {
        auto func = static_cast<AsyncCallback *>(data);
        if (func->isolate != nullptr && !func->isolate->IsDead()) {
            v8::Isolate *isolate = func->isolate;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = func->callback.Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);


            auto fileDirImpl = new FileDirImpl(value);

            auto ext = v8::External::New(isolate, fileDirImpl);

            auto ctor = FileDirImpl::GetCtor(isolate);
            auto val = ctor->GetFunction(context).ToLocalChecked()->NewInstance(
                    context).ToLocalChecked();

            val->SetInternalField(0, ext);


            v8::Local<v8::Value> args[2] = {v8::Null(isolate), val};


            callback->Call(context, context->Global(), 2,
                           args);  // ignore JS return value

            delete func;
        }
    }
}
void async_success_filehandle(FileHandle *value, void *data) {
    if (data != nullptr) {
        auto func = static_cast<AsyncCallback *>(data);
        if (func->isolate != nullptr && !func->isolate->IsDead()) {
            v8::Isolate *isolate = func->isolate;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = func->callback.Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);


            auto fileHandleImpl = new FileHandleImpl(value);

            auto ext = v8::External::New(isolate, fileHandleImpl);

            auto ctor = FileHandleImpl::GetCtor(isolate);
            auto val = ctor->GetFunction(context).ToLocalChecked()->NewInstance(
                    context).ToLocalChecked();

            val->SetInternalField(0, ext);


            v8::Local<v8::Value> args[2] = {v8::Null(isolate), val};


            callback->Call(context, context->Global(), 2,
                           args);  // ignore JS return value

            delete func;
        }
    }
}


void async_error(NodeError *error, void *data) {
    if (data != nullptr) {
        auto func = static_cast<AsyncCallback *>(data);
        if (func->isolate != nullptr && !func->isolate->IsDead()) {
            v8::Isolate *isolate = func->isolate;
            v8::Locker locker(isolate);
            v8::Isolate::Scope isolate_scope(isolate);
            v8::HandleScope handle_scope(isolate);
            v8::Local<v8::Function> callback = func->callback.Get(isolate);
            v8::Local<v8::Context> context = callback->GetCreationContextChecked();
            v8::Context::Scope context_scope(context);

            auto message = node_error_get_message(error);
            auto jsMessage = Helpers::ConvertToV8String(isolate, message);
            auto jsError = v8::Exception::Error(jsMessage);
            v8::Local<v8::Value> args[2] = {jsError, v8::Null(isolate)};

            callback->Call(context, context->Global(), 2,
                           args);  // ignore JS return value

            delete func;
        }
    }
    if (error != nullptr) {
        node_error_destroy(error);
    }
}
}


#ifdef __ANDROID__

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

#endif


void Helpers::LogToConsole(const std::string &message) {
#ifdef __ANDROID__
    sendToADBLogcat(message, android_LogPriority::ANDROID_LOG_INFO);
#endif

#ifdef __APPLE__
#endif
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

StringEncoding
Helpers::ParseEncoding(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                       StringEncoding defaultValue) {

    if (value->IsString() || value->IsStringObject()) {
        auto val = Helpers::ConvertFromV8String(isolate, value);
        if (val == "utf8" || val == "utf-8") {
            return StringEncoding::StringEncodingUtf8;
        } else if (val == "utf16le" || val == "utf-16le") {
            return StringEncoding::StringEncodingUtf16le;
        } else if (val == "latin1") {
            return StringEncoding::StringEncodingLatin1;
        } else if (val == "base64") {
            return StringEncoding::StringEncodingBase64;
        } else if (val == "base64url") {
            return StringEncoding::StringEncodingBase64Url;
        } else if (val == "hex") {
            return StringEncoding::StringEncodingHex;
        } else if (val == "binary") {
            return StringEncoding::StringEncodingBinary;
        } else if (val == "ucs2" || val == "ucs-2") {
            return StringEncoding::StringEncodingUcs2;
        }
    }

    return defaultValue;
}

FsEncodingType
Helpers::ParseFsEncoding(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                         FsEncodingType defaultValue) {

    if (value->IsString() || value->IsStringObject()) {
        auto val = Helpers::ConvertFromV8String(isolate, value);
        if (val == "utf8" || val == "utf-8") {
            return FsEncodingType::FsEncodingTypeUtf8;
        } else if (val == "utf16le" || val == "utf-16le") {
            return FsEncodingType::FsEncodingTypeUtf16le;
        } else if (val == "latin1") {
            return FsEncodingType::FsEncodingTypeLatin1;
        } else if (val == "ucs2" || val == "ucs-2") {
            return FsEncodingType::FsEncodingTypeUcs2;
        }
    }

    return defaultValue;
}

void Helpers::ParseAppendFileOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                     AppendFileOptions &options) {
    if (value->IsObject() && !value->IsNullOrUndefined()) {
        auto ctx = isolate->GetCurrentContext();
        auto val = value.As<v8::Object>();

        v8::Local<v8::Value> encodingValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "encoding")).ToLocal(&encodingValue);

        if (encodingValue->IsString()) {
            options.encoding = ParseEncoding(isolate, encodingValue,
                                             StringEncoding::StringEncodingUtf8);
        }

        v8::Local<v8::Value> modeValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "mode")).ToLocal(&modeValue);

        if (modeValue->IsNumber()) {
            options.mode = (int32_t) modeValue->NumberValue(ctx).ToChecked();
        }


        v8::Local<v8::Value> flagValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "flag")).ToLocal(&flagValue);


        if (flagValue->IsNumber()) {
            options.flag = (int32_t) flagValue->NumberValue(ctx).ToChecked();
        }
    }
}


void Helpers::ParseMkDirOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                MkDirOptions &options) {
    if (value->IsObject() && !value->IsNullOrUndefined()) {
        auto ctx = isolate->GetCurrentContext();
        auto val = value.As<v8::Object>();


        v8::Local<v8::Value> recursiveValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "recursive")).ToLocal(&recursiveValue);

        if (recursiveValue->IsBoolean()) {
            options.recursive = recursiveValue->BooleanValue(isolate);
        }

        v8::Local<v8::Value> modeValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "mode")).ToLocal(&modeValue);

        if (modeValue->IsNumber()) {
            options.mode = (int32_t) modeValue->NumberValue(ctx).ToChecked();
        }
    }
}


void Helpers::ParseMkdTempOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                  MkdTempOptions &options) {
    if (value->IsObject() && !value->IsNullOrUndefined()) {
        auto ctx = isolate->GetCurrentContext();
        auto val = value.As<v8::Object>();


        v8::Local<v8::Value> encodingValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "encoding")).ToLocal(&encodingValue);

        if (encodingValue->IsString()) {
            options.encoding = ParseEncoding(isolate, encodingValue,
                                             StringEncoding::StringEncodingUtf8);
        }
    }
}


void Helpers::ParseOpenDirOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                  OpenDirOptions &options) {
    if (value->IsObject() && !value->IsNullOrUndefined()) {
        auto ctx = isolate->GetCurrentContext();
        auto val = value.As<v8::Object>();


        v8::Local<v8::Value> encodingValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "encoding")).ToLocal(&encodingValue);

        if (encodingValue->IsString()) {
            options.encoding = ParseEncoding(isolate, encodingValue,
                                             StringEncoding::StringEncodingUtf8);
        }

        v8::Local<v8::Value> bufferSizeValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "bufferSize")).ToLocal(&bufferSizeValue);

        if (bufferSizeValue->IsNumber()) {
            options.buffer_size = (size_t) bufferSizeValue->NumberValue(ctx).ToChecked();
        } else {
            options.buffer_size = 32;
        }

        v8::Local<v8::Value> recursiveValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "recursive")).ToLocal(&recursiveValue);

        if (recursiveValue->IsBoolean()) {
            options.recursive = recursiveValue->BooleanValue(isolate);
        } else {
            options.recursive = false;
        }

    }
}

void Helpers::ParseReaddirOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                  ReaddirOptions &options) {
    if (value->IsObject() && !value->IsNullOrUndefined()) {
        auto ctx = isolate->GetCurrentContext();
        auto val = value.As<v8::Object>();


        v8::Local<v8::Value> encodingValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "encoding")).ToLocal(&encodingValue);

        if (encodingValue->IsString()) {
            options.encoding = ParseFsEncoding(isolate, encodingValue,
                                               FsEncodingType::FsEncodingTypeUtf8);
        }

        v8::Local<v8::Value> withFileTypesValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "withFileTypes")).ToLocal(
                &withFileTypesValue);

        if (withFileTypesValue->IsBoolean()) {
            options.with_file_types = (size_t) withFileTypesValue->BooleanValue(isolate);
        } else {
            options.with_file_types = false;
        }

        v8::Local<v8::Value> recursiveValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "recursive")).ToLocal(&recursiveValue);

        if (recursiveValue->IsBoolean()) {
            options.recursive = recursiveValue->BooleanValue(isolate);
        } else {
            options.recursive = false;
        }

    }
}


void Helpers::ParseReadFileOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                   ReadFileOptions &options) {
    if (value->IsObject() && !value->IsNullOrUndefined()) {
        auto ctx = isolate->GetCurrentContext();
        auto val = value.As<v8::Object>();


        v8::Local<v8::Value> encodingValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "encoding")).ToLocal(&encodingValue);

        if (encodingValue->IsString()) {
            options.encoding = ParseFsEncoding(isolate, encodingValue,
                                               FsEncodingType::FsEncodingTypeUtf8);
        }

        v8::Local<v8::Value> flagValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "flag")).ToLocal(&flagValue);


        if (flagValue->IsNumber()) {
            options.flag = (int32_t) flagValue->NumberValue(ctx).ToChecked();
        }

    }
}

void Helpers::ParseReadLinkOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                   ReadLinkOptions &options) {
    if (value->IsObject() && !value->IsNullOrUndefined()) {
        auto ctx = isolate->GetCurrentContext();
        auto val = value.As<v8::Object>();


        v8::Local<v8::Value> encodingValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "encoding")).ToLocal(&encodingValue);

        if (encodingValue->IsString()) {
            options.encoding = ParseFsEncoding(isolate, encodingValue,
                                               FsEncodingType::FsEncodingTypeUtf8);
        }
    }
}

void Helpers::ParseRealPathOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                   RealPathOptions &options) {
    if (value->IsObject() && !value->IsNullOrUndefined()) {
        auto ctx = isolate->GetCurrentContext();
        auto val = value.As<v8::Object>();


        v8::Local<v8::Value> encodingValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "encoding")).ToLocal(&encodingValue);

        if (encodingValue->IsString()) {
            options.encoding = ParseEncoding(isolate, encodingValue,
                                             StringEncoding::StringEncodingUtf8);
        }
    }
}


void Helpers::ParseRmDirOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                RmDirOptions &options) {
    if (value->IsObject() && !value->IsNullOrUndefined()) {
        auto ctx = isolate->GetCurrentContext();
        auto val = value.As<v8::Object>();


        v8::Local<v8::Value> recursiveValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "recursive")).ToLocal(&recursiveValue);

        if (recursiveValue->IsBoolean()) {
            options.recursive = recursiveValue->BooleanValue(isolate);
        }


        v8::Local<v8::Value> maxRetriedValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "max_retries")).ToLocal(&maxRetriedValue);

        if (recursiveValue->IsInt32()) {
            options.max_retries = maxRetriedValue->Int32Value(ctx).ToChecked();
        }


        v8::Local<v8::Value> retryDelayValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "retry_delay")).ToLocal(&retryDelayValue);

        if (retryDelayValue->IsNumber()) {
            options.retry_delay = (uint64_t) maxRetriedValue->NumberValue(ctx).ToChecked();
        }


    }
}

void Helpers::ParseRmOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                             RmOptions &options) {
    if (value->IsObject() && !value->IsNullOrUndefined()) {
        auto ctx = isolate->GetCurrentContext();
        auto val = value.As<v8::Object>();


        v8::Local<v8::Value> recursiveValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "recursive")).ToLocal(&recursiveValue);

        if (recursiveValue->IsBoolean()) {
            options.recursive = recursiveValue->BooleanValue(isolate);
        }


        v8::Local<v8::Value> maxRetriedValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "max_retries")).ToLocal(&maxRetriedValue);

        if (recursiveValue->IsInt32()) {
            options.max_retries = maxRetriedValue->Int32Value(ctx).ToChecked();
        }


        v8::Local<v8::Value> retryDelayValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "retry_delay")).ToLocal(&retryDelayValue);

        if (retryDelayValue->IsNumber()) {
            options.retry_delay = (uint64_t) maxRetriedValue->NumberValue(ctx).ToChecked();
        }


        v8::Local<v8::Value> forceValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "force")).ToLocal(&retryDelayValue);

        if (forceValue->IsBoolean()) {
            options.force = forceValue->BooleanValue(isolate);
        }


    }
}


void Helpers::ParseWriteFileOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                    WriteFileOptions &options) {
    if (value->IsObject() && !value->IsNullOrUndefined()) {
        auto ctx = isolate->GetCurrentContext();
        auto val = value.As<v8::Object>();


        v8::Local<v8::Value> encodingValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "encoding")).ToLocal(&encodingValue);

        if (encodingValue->IsString()) {
            options.encoding = ParseEncoding(isolate, encodingValue,
                                             StringEncoding::StringEncodingUtf8);
        }

        v8::Local<v8::Value> modeValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "mode")).ToLocal(&modeValue);

        if (modeValue->IsNumber()) {
            options.mode = (int32_t) modeValue->NumberValue(ctx).ToChecked();
        }


        v8::Local<v8::Value> flagValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "flag")).ToLocal(&flagValue);


        if (flagValue->IsNumber()) {
            options.flag = (int32_t) flagValue->NumberValue(ctx).ToChecked();
        }

    }
}


void Helpers::ParseWriteOptions(v8::Isolate *isolate, const v8::Local<v8::Value> &value,
                                WriteOptions &options) {
    if (value->IsObject() && !value->IsNullOrUndefined()) {
        auto ctx = isolate->GetCurrentContext();
        auto val = value.As<v8::Object>();

        v8::Local<v8::Value> offsetValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "offset")).ToLocal(&offsetValue);

        if (offsetValue->IsNumber()) {
            options.offset = (size_t) offsetValue->NumberValue(ctx).ToChecked();
        }

        v8::Local<v8::Value> lengthValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "length")).ToLocal(&lengthValue);

        if (lengthValue->IsNumber()) {
            options.length = (size_t) lengthValue->NumberValue(ctx).ToChecked();
        }

        v8::Local<v8::Value> positionValue;
        val->Get(ctx, Helpers::ConvertToV8String(isolate, "position")).ToLocal(&positionValue);

        if (positionValue->IsNumber()) {
            options.position = (intptr_t) positionValue->NumberValue(ctx).ToChecked();
        }

    }
}

v8::Local<v8::Object>
Helpers::FileStatToJS(v8::Isolate *isolate, bool bigInt,
                      const FileStat &stat) {
    v8::Isolate::Scope isolate_scope(isolate);
    v8::EscapableHandleScope handle_scope(isolate);
    v8::Local<v8::Object> ret = v8::Object::New(isolate);
    auto ctx = isolate->GetCurrentContext();

    if (bigInt) {
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "dev"),
                 v8::BigInt::New(isolate, stat.dev));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "ino"),
                 v8::BigInt::New(isolate, stat.ino));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "mode"),
                 v8::Int32::New(isolate, stat.mode));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "nlink"),
                 v8::BigInt::New(isolate, stat.nlink));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "uid"),
                 v8::Int32::New(isolate, stat.uid));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "gid"),
                 v8::Int32::New(isolate, stat.gid));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "rdev"),
                 v8::BigInt::New(isolate, stat.rdev));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "size"),
                 v8::BigInt::New(isolate, stat.size));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "blksize"),
                 v8::BigInt::New(isolate, stat.blksize));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "blocks"),
                 v8::BigInt::New(isolate, stat.blocks));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "atimeMs"),
                 v8::Number::New(isolate, stat.atimeMs));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "mtimeMs"),
                 v8::Number::New(isolate, stat.mtimeMs));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "ctimeMs"),
                 v8::Number::New(isolate, stat.ctimeMs));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "birthtimeMs"),
                 v8::Number::New(isolate, stat.birthtimeMs));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "birthtime"),
                 v8::Number::New(isolate, stat.birthtime));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "atime"),
                 v8::Number::New(isolate, stat.atime));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "mtime"),
                 v8::Number::New(isolate, stat.mtime));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "ctime"),
                 v8::Number::New(isolate, stat.ctime));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "isBlockDevice"),
                 v8::Boolean::New(isolate, stat.isBlockDevice));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "isCharacterDevice"),
                 v8::Boolean::New(isolate, stat.isCharacterDevice));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "isDirectory"),
                 v8::Boolean::New(isolate, stat.isDirectory));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "isFIFO"),
                 v8::Boolean::New(isolate, stat.isFIFO));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "isFile"),
                 v8::Boolean::New(isolate, stat.isFile));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "isSocket"),
                 v8::Boolean::New(isolate, stat.isSocket));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "isSymbolicLink"),
                 v8::Boolean::New(isolate, stat.isSymbolicLink));
    } else {
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "dev"),
                 v8::Number::New(isolate, (double) stat.dev));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "ino"),
                 v8::Number::New(isolate, (double) stat.ino));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "mode"),
                 v8::Int32::New(isolate, stat.mode));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "nlink"),
                 v8::Number::New(isolate, (double) stat.nlink));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "uid"),
                 v8::Int32::New(isolate, stat.uid));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "gid"),
                 v8::Int32::New(isolate, stat.gid));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "rdev"),
                 v8::Number::New(isolate, (double) stat.rdev));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "size"),
                 v8::Number::New(isolate, (double) stat.size));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "blksize"),
                 v8::Number::New(isolate, (double) stat.blksize));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "blocks"),
                 v8::Number::New(isolate, (double) stat.blocks));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "atimeMs"),
                 v8::Number::New(isolate, stat.atimeMs));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "mtimeMs"),
                 v8::Number::New(isolate, stat.mtimeMs));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "ctimeMs"),
                 v8::Number::New(isolate, stat.ctimeMs));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "birthtimeMs"),
                 v8::Number::New(isolate, stat.birthtimeMs));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "birthtime"),
                 v8::Number::New(isolate, stat.birthtime));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "atime"),
                 v8::Number::New(isolate, stat.atime));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "mtime"),
                 v8::Number::New(isolate, stat.mtime));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "ctime"),
                 v8::Number::New(isolate, stat.ctime));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "isBlockDevice"),
                 v8::Boolean::New(isolate, stat.isBlockDevice));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "isCharacterDevice"),
                 v8::Boolean::New(isolate, stat.isCharacterDevice));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "isDirectory"),
                 v8::Boolean::New(isolate, stat.isDirectory));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "isFIFO"),
                 v8::Boolean::New(isolate, stat.isFIFO));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "isFile"),
                 v8::Boolean::New(isolate, stat.isFile));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "isSocket"),
                 v8::Boolean::New(isolate, stat.isSocket));
        ret->Set(ctx, Helpers::ConvertToV8String(isolate, "isSymbolicLink"),
                 v8::Boolean::New(isolate, stat.isSymbolicLink));
    }


    return handle_scope.Escape(ret);
}

