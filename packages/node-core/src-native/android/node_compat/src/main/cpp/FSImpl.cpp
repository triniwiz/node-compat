//
// Created by Osei Fortune on 08/07/2023.
//

#include <unistd.h>
#include "FSImpl.h"
#include "Caches.h"
#include "BufferImpl.h"
#include "FileDirImpl.h"

void FSImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    global->Set(context, Helpers::ConvertToV8String(isolate, "NSCFS"), func);
}

v8::Local<v8::FunctionTemplate> FSImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->FsTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, nullptr);

    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "NSCFS"));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "accessSync"),
            v8::FunctionTemplate::New(isolate, &AccessSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "readSync"),
            v8::FunctionTemplate::New(isolate, &ReadSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "chmodSync"),
            v8::FunctionTemplate::New(isolate, &ChmodSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "chownSync"),
            v8::FunctionTemplate::New(isolate, &ChownSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "closeSync"),
            v8::FunctionTemplate::New(isolate, &CloseSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "appendFileSync"),
            v8::FunctionTemplate::New(isolate, &AppendFileSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "existsSync"),
            v8::FunctionTemplate::New(isolate, &ExistsSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "fchmodSync"),
            v8::FunctionTemplate::New(isolate, &FchmodSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "fchownSync"),
            v8::FunctionTemplate::New(isolate, &FchownSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "fdatasyncSync"),
            v8::FunctionTemplate::New(isolate, &FdatasyncSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "fstatSync"),
            v8::FunctionTemplate::New(isolate, &FStatSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "copyFileSync"),
            v8::FunctionTemplate::New(isolate, &CopyFileSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "cpSync"),
            v8::FunctionTemplate::New(isolate, &CpSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "lchmodSync"),
            v8::FunctionTemplate::New(isolate, &LchmodSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "lchownSync"),
            v8::FunctionTemplate::New(isolate, &LchownSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "lutimesSync"),
            v8::FunctionTemplate::New(isolate, &LutimesSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "linkSync"),
            v8::FunctionTemplate::New(isolate, &LinkSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "statSync"),
            v8::FunctionTemplate::New(isolate, &StatSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "mkdirSync"),
            v8::FunctionTemplate::New(isolate, &MkdirSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "mkdtempSync"),
            v8::FunctionTemplate::New(isolate, &MkdtempSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "openSync"),
            v8::FunctionTemplate::New(isolate, &OpenSync));


    cache->FsTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);

    return ctorTmpl;

}

void FSImpl::AccessSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto pathValue = args[0];
    auto mode = F_OK;
    if (!pathValue->IsString()) {
        isolate->ThrowError("Invalid Path");
    }
    try {
        auto path = Helpers::ConvertFromV8String(isolate, pathValue);
        fs_access_sync(rust::Str(path.c_str()), mode);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::AppendFileSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto dest = args[0];
    auto src = args[1];
    AppendFileOptions options{};
    auto optionsValue = args[2];
    Helpers::ParseAppendFileOptions(isolate, optionsValue, options);

    try {
        if (dest->IsString()) {
            if (src->IsString()) {
                fs_append_file_with_path_string_sync(
                        Helpers::ConvertFromV8String(isolate, dest),
                        Helpers::ConvertFromV8String(isolate, src),
                        options
                );
            } else if (src->IsObject()) {

                auto ptr = BufferImpl::GetPointer(src.As<v8::Object>());

                if (ptr != nullptr) {
                    fs_append_file_with_path_sync(
                            Helpers::ConvertFromV8String(isolate, dest),
                            ptr->GetBuffer(),
                            options
                    );

                } else {
                    // todo throw error
                }
            }
        } else if (dest->IsNumber()) {
            if (src->IsString()) {
                fs_append_file_with_string_sync(
                        (int32_t) dest->NumberValue(ctx).ToChecked(),
                        Helpers::ConvertFromV8String(isolate, src),
                        options
                );
            } else if (src->IsObject()) {

                auto ptr = BufferImpl::GetPointer(src.As<v8::Object>());

                if (ptr != nullptr) {
                    fs_append_file_sync(
                            (int32_t) dest->NumberValue(ctx).ToChecked(),
                            ptr->GetBuffer(),
                            options
                    );

                } else {
                    // todo throw error
                }
            }
        } else if (dest->IsObject()) {
            auto destPtr = BufferImpl::GetPointer(dest.As<v8::Object>());

            if (destPtr != nullptr) {
                if (src->IsString()) {
                    fs_append_file_with_buffer_string_sync(
                            destPtr->GetBuffer(),
                            Helpers::ConvertFromV8String(isolate, src),
                            options
                    );
                } else if (src->IsObject()) {

                    auto ptr = BufferImpl::GetPointer(src.As<v8::Object>());

                    if (ptr != nullptr) {
                        fs_append_file_with_buffer_buffer_sync(
                                destPtr->GetBuffer(),
                                ptr->GetBuffer(),
                                options
                        );

                    } else {
                        // todo throw error
                    }

                } else {
                    // todo throw error
                }
            }
        }
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::ChmodSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];
    auto mode = args[0]->NumberValue(ctx).ToChecked();
    if (!pathValue->IsString()) {
        isolate->ThrowError("Invalid Path");
    }
    try {
        auto path = Helpers::ConvertFromV8String(isolate, pathValue);
        fs_chmod_sync(rust::Str(path.c_str()), (int32_t) mode);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::ChownSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];
    auto uid = args[1]->NumberValue(ctx).ToChecked();
    auto gid = args[2]->NumberValue(ctx).ToChecked();

    if (!pathValue->IsString()) {
        isolate->ThrowError("Invalid Path");
    }
    try {
        auto path = Helpers::ConvertFromV8String(isolate, pathValue);
        fs_chown_sync(rust::Str(path.c_str()), (uint32_t) uid, (uint32_t) gid);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::CloseSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto fd = args[0]->NumberValue(ctx).ToChecked();

    try {
        fs_close_sync((int32_t) fd);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::ExistsSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];
    std::string path;
    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }
    try {
        fs_exists_sync(path);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::FchmodSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto fd = args[0]->NumberValue(ctx).ToChecked();
    uint32_t mode = 0;

    auto modeValue = args[1];
    if (modeValue->IsNumber()) {
        mode = modeValue->Uint32Value(ctx).ToChecked();
    }
    try {
        fs_fchmod_sync((int32_t) fd, mode);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::FchownSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();

    auto path = Helpers::ConvertFromV8String(isolate, args[0]);
    uint32_t uid = 0;
    uint32_t gid = 0;
    auto uidValue = args[1];
    auto gidValue = args[2];

    if (uidValue->IsUint32()) {
        uid = uidValue->Uint32Value(ctx).ToChecked();
    }

    if (gidValue->IsUint32()) {
        gid = gidValue->Uint32Value(ctx).ToChecked();
    }

    try {
        fs_chown_sync(path, uid, gid);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::FdatasyncSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto fd = args[0]->NumberValue(ctx).ToChecked();

    try {
        fs_fdatasync_sync((int32_t) fd);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::FStatSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto fd = args[0]->NumberValue(ctx).ToChecked();
    bool bigint = false;
    auto optionsValue = args[1];
    if (optionsValue->IsObject()) {
        auto options = optionsValue.As<v8::Object>();
        v8::Local<v8::Value> bigintValue;

        options->Get(ctx, Helpers::ConvertToV8String(isolate, "bigint")).ToLocal(&bigintValue);

        if (bigintValue->IsBoolean()) {
            bigint = bigintValue->BooleanValue(isolate);
        }
    }
    try {
        auto stat = fs_fstat_sync((int32_t) fd);
        args.GetReturnValue().Set(Helpers::FileStatToJS(isolate, bigint, stat));
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::CopyFileSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto srcValue = args[0];
    auto src = Helpers::ConvertFromV8String(isolate, srcValue);

    auto destValue = args[1];
    auto dest = Helpers::ConvertFromV8String(isolate, destValue);

    int32_t mode = 0;

    auto modeValue = args[2];
    if (modeValue->IsNumber()) {
        mode = modeValue->Int32Value(ctx).ToChecked();
    }

    try {
        fs_copy_file_sync(src, dest, mode);
        args.GetReturnValue().SetUndefined();
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::CpSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();

    args.GetReturnValue().SetUndefined();
}

void FSImpl::LchmodSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];
    std::string path;
    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }
    uint32_t mode = 0;
    auto modeValue = args[1];

    if (modeValue->IsUint32()) {
        mode = modeValue->Uint32Value(ctx).ToChecked();
    }

    try {
        fs_lchmod_sync(path, mode);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::LchownSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];
    std::string path;
    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }
    uint32_t uid = 0;
    auto uidValue = args[1];

    if (uidValue->IsUint32()) {
        uid = uidValue->Uint32Value(ctx).ToChecked();
    }

    uint32_t gid = 0;
    auto gidValue = args[1];

    if (gidValue->IsUint32()) {
        gid = gidValue->Uint32Value(ctx).ToChecked();
    }

    try {
        fs_lchown_sync(path, uid, gid);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::LutimesSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];
    std::string path;
    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }
    int64_t atime = 0;
    int64_t mtime = 0;

    auto atimeValue = args[1];

    if (atimeValue->IsBigInt()) {
        atime = atimeValue->ToBigInt(ctx).ToLocalChecked()->Int64Value();
    } else if (atimeValue->IsDate()) {
        atime = (int64_t) atimeValue.As<v8::Date>()->ValueOf();
    } else {
        atime = (int64_t) atimeValue->NumberValue(ctx).ToChecked();
    }

    auto mtimeValue = args[1];

    if (mtimeValue->IsBigInt()) {
        mtime = mtimeValue->ToBigInt(ctx).ToLocalChecked()->Int64Value();
    } else if (mtimeValue->IsDate()) {
        mtime = (int64_t) mtimeValue.As<v8::Date>()->ValueOf();
    } else {
        mtime = (int64_t) mtimeValue->NumberValue(ctx).ToChecked();
    }


    try {
        fs_lutimes_sync(path, atime, mtime);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::LinkSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto existingPathValue = args[0];
    auto existingPath = Helpers::ConvertFromV8String(isolate, existingPathValue);

    auto newPathValue = args[1];
    auto newPath = Helpers::ConvertFromV8String(isolate, newPathValue);


    try {
        fs_link_sync(existingPath, newPath);
        args.GetReturnValue().SetUndefined();
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::StatSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto path = Helpers::ConvertFromV8String(isolate, args[0]);
    bool bigint = false;
    auto optionsValue = args[1];
    if (optionsValue->IsObject()) {
        auto options = optionsValue.As<v8::Object>();
        v8::Local<v8::Value> bigintValue;

        options->Get(ctx, Helpers::ConvertToV8String(isolate, "bigint")).ToLocal(&bigintValue);

        if (bigintValue->IsBoolean()) {
            bigint = bigintValue->BooleanValue(isolate);
        }
    }
    try {
        auto stat = fs_stat_sync(path);
        args.GetReturnValue().Set(Helpers::FileStatToJS(isolate, bigint, stat));
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::MkdirSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];
    std::string path;
    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }

    MkDirOptions options{};

    Helpers::ParseMkDirOptions(isolate, args[1], options);

    try {
        fs_mkdir_sync(path, options);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::MkdtempSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto prefixValue = args[0];
    std::string prefix;
    if (prefixValue->IsString()) {
        prefix = Helpers::ConvertFromV8String(isolate, prefixValue);
    }


    MkdTempOptions options{};

    Helpers::ParseMkdTempOptions(isolate, args[1], options);

    try {
        fs_mkdtemp_sync(prefix, options);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}


void FSImpl::ReadSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    int fd = -1;

    size_t offset = 0;
    size_t length = -1;
    rust::isize position = -1;

    auto fdValue = args[0];
    if (fdValue->IsNumber()) {
        fd = (int32_t) fdValue->NumberValue(ctx).ToChecked();
    }

    auto bufferValue = args[1];

    auto offsetValue = args[2];

    if (offsetValue->IsNumber()) {
        offset = (size_t) offsetValue->NumberValue(ctx).ToChecked();
    }

    auto lengthValue = args[3];
    if (lengthValue->IsNumber()) {
        length = (size_t) lengthValue->NumberValue(ctx).ToChecked();
    }

    auto positionValue = args[4];

    if (positionValue->IsBigInt()) {
        position = (rust::isize) positionValue->ToBigInt(ctx).ToLocalChecked()->Int64Value();
    } else if (positionValue->IsNumber()) {

    }

    if (bufferValue->IsTypedArray()) {
        auto array = bufferValue.As<v8::TypedArray>();
        auto buffer = array->Buffer();
        auto store = buffer->GetBackingStore();
        auto os = array->ByteOffset();
        auto len = array->ByteLength();
        auto data = static_cast<uint8_t *>(store->Data()) + os;
        try {
            auto ret = fs_read_sync(fd, rust::Slice(data, len), offset, length, position);
            args.GetReturnValue().Set((double) ret);
            return;
        } catch (std::exception &error) {
            auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
            isolate->ThrowException(err);
        }

    } else if (bufferValue->IsDataView()) {
        auto view = bufferValue.As<v8::DataView>();
        auto buffer = view->Buffer();
        auto store = buffer->GetBackingStore();
        auto os = view->ByteOffset();
        auto len = view->ByteLength();
        auto data = static_cast<uint8_t *>(store->Data()) + os;
        try {
            auto ret = fs_read_sync(fd, rust::Slice(data, len), offset, length, position);
            args.GetReturnValue().Set((double) ret);
            return;
        } catch (std::exception &error) {
            auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
            isolate->ThrowException(err);
        }

    } else if (bufferValue->IsObject()) {
        auto ptr = BufferImpl::GetPointer(bufferValue.As<v8::Object>());

        if (ptr != nullptr) {
            auto data = buffer_buffer(ptr->GetBuffer());
            try {
                auto ret = fs_read_sync(fd, data, offset, length, position);
                args.GetReturnValue().Set((double) ret);
                return;
            } catch (std::exception &error) {
                auto err = v8::Exception::Error(
                        Helpers::ConvertToV8String(isolate, error.what()));
                isolate->ThrowException(err);
            }

        }
    }

    args.GetReturnValue().SetUndefined();

}

void FSImpl::OpenDirSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];

    std::string path;

    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }

    OpenDirOptions options{};
    Helpers::ParseOpenDirOptions(isolate, args[1], options);

    try {
        auto dir = fs_opendir_sync(path, options);
        auto ctor = FileDirImpl::GetCtor(isolate)->New(isolate);
        auto func = ctor->GetFunction(ctx).ToLocalChecked();
        auto ret = func->NewInstance(ctx).ToLocalChecked();
        auto fileDirImpl = new FileDirImpl(std::move(dir));
        auto ext = v8::External::New(isolate, fileDirImpl);
        ret->SetInternalField(0, ext);

        args.GetReturnValue().Set(ret);
        return;
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::OpenSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];

    std::string path;

    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }

    auto flagValue = args[1];
    int32_t flag = O_RDONLY;

    if (flagValue->IsInt32()) {
        flag = flagValue->Int32Value(ctx).ToChecked();
    }
    int32_t mode = 438;
    auto modeValue = args[2];

    if (modeValue->IsInt32()) {
        mode = modeValue->Int32Value(ctx).ToChecked();
    }

    try {
        auto ret = fs_open_sync(path, flag, mode);
        args.GetReturnValue().Set(ret);
        return;
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}
