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


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "opendirSync"),
            v8::FunctionTemplate::New(isolate, &OpenDirSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "readdirSync"),
            v8::FunctionTemplate::New(isolate, &ReaddirSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "readFileSync"),
            v8::FunctionTemplate::New(isolate, &ReadFileSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "readLinkSync"),
            v8::FunctionTemplate::New(isolate, &ReadLinkSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "readvSync"),
            v8::FunctionTemplate::New(isolate, &ReadvSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "realpathSync"),
            v8::FunctionTemplate::New(isolate, &RealpathSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "renameSync"),
            v8::FunctionTemplate::New(isolate, &RenameSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "rmdirSync"),
            v8::FunctionTemplate::New(isolate, &RmdirSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "rmSync"),
            v8::FunctionTemplate::New(isolate, &RmSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "statfsSync"),
            v8::FunctionTemplate::New(isolate, &StatfsSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "symlinkSync"),
            v8::FunctionTemplate::New(isolate, &SymlinkSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "truncateSync"),
            v8::FunctionTemplate::New(isolate, &TruncateSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "unlinkSync"),
            v8::FunctionTemplate::New(isolate, &UnlinkSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "utimesSync"),
            v8::FunctionTemplate::New(isolate, &UtimesSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeFileSync"),
            v8::FunctionTemplate::New(isolate, &WriteFileSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeSync"),
            v8::FunctionTemplate::New(isolate, &WriteSync));


    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "writevSync"),
            v8::FunctionTemplate::New(isolate, &WritevSync));

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "open"),
            v8::FunctionTemplate::New(isolate, &Open));

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
        fs_access_sync(path.c_str(), mode);
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
                auto dst = Helpers::ConvertFromV8String(isolate, dest);
                auto source = Helpers::ConvertFromV8String(isolate, src);
                fs_append_file_with_path_string_sync(
                        dst.c_str(),
                        source.c_str(),
                        options
                );
            } else if (src->IsObject()) {

                auto dst = Helpers::ConvertFromV8String(isolate, dest);

                auto ptr = BufferImpl::GetPointer(src.As<v8::Object>());

                if (ptr != nullptr) {
                    fs_append_file_with_path_sync(
                            dst.c_str(),
                            ptr->GetBuffer(),
                            options
                    );

                } else {
                    // todo throw error
                }
            }
        } else if (dest->IsNumber()) {
            if (src->IsString()) {
                auto source = Helpers::ConvertFromV8String(isolate, src);
                fs_append_file_with_string_sync(
                        (int32_t) dest->NumberValue(ctx).ToChecked(),
                        source.c_str(),
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
                    auto source = Helpers::ConvertFromV8String(isolate, src);
                    fs_append_file_with_buffer_string_sync(
                            destPtr->GetBuffer(),
                            source.c_str(),
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
        fs_chmod_sync(path.c_str(), (int32_t) mode);
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
        fs_chown_sync(path.c_str(), (uint32_t) uid, (uint32_t) gid);
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
    isolate->GetCurrentContext();
    auto pathValue = args[0];
    std::string path;
    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }
    try {
        fs_exists_sync(path.c_str());
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
        fs_chown_sync(path.c_str(), uid, gid);
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
        args.GetReturnValue().Set(Helpers::FileStatToJS(isolate, bigint, *stat));
        filestat_destroy(stat);
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
        fs_copy_file_sync(src.c_str(), dest.c_str(), mode);
        args.GetReturnValue().SetUndefined();
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::CpSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    // auto isolate = args.GetIsolate();

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
        fs_lchmod_sync(path.c_str(), mode);
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
        fs_lchown_sync(path.c_str(), uid, gid);
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
        fs_lutimes_sync(path.c_str(), atime, mtime);
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
        fs_link_sync(existingPath.c_str(), newPath.c_str());
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
        auto stat = fs_stat_sync(path.c_str());
        args.GetReturnValue().Set(Helpers::FileStatToJS(isolate, bigint, *stat));
        filestat_destroy(stat);
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }
}

void FSImpl::MkdirSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    isolate->GetCurrentContext();
    auto pathValue = args[0];
    std::string path;
    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }

    MkDirOptions options{};

    Helpers::ParseMkDirOptions(isolate, args[1], options);

    try {
        fs_mkdir_sync(path.c_str(), options);
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
        fs_mkdtemp_sync(prefix.c_str(), options);
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
    intptr_t position = -1;

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
        position = (intptr_t) positionValue->ToBigInt(ctx).ToLocalChecked()->Int64Value();
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
            auto ret = fs_read_sync(fd, data, len, offset, length, position);
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
            auto ret = fs_read_sync(fd, data, len, offset, length, position);
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
            auto data_length = buffer_length(ptr->GetBuffer());
            try {
                auto ret = fs_read_sync(fd, data, data_length, offset, length, position);
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
        auto dir = fs_opendir_sync(path.c_str(), options);
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
        auto ret = fs_open_sync(path.c_str(), flag, mode);
        args.GetReturnValue().Set(ret);
        return;
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::ReaddirSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];

    std::string path;

    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }

    ReaddirOptions options{};

    Helpers::ParseReaddirOptions(isolate, args[1], options);

    try {
        auto readdir = fs_readdir_sync(path.c_str(), options);
        auto result = readdir->data;
        auto size = readdir->length;
        auto array = v8::Array::New(isolate, (int) size);
        for (int32_t i = 0; i < size; i++) {
            auto value = (&result)[i];
            auto type = fs_readdir_get_type(value);

            if (type == ReaddirResultType::ReaddirResultTypeString) {
                auto string_value = fs_readdir_get_string_value(value);
                array->Set(ctx, i, Helpers::ConvertToV8String(isolate, string_value));
            } else {
                auto buffer = fs_readdir_get_buffer_value(value);

                auto bufferImpl = new BufferImpl(std::move(buffer));
                auto ext = v8::External::New(isolate, bufferImpl);

                auto ctor = BufferImpl::GetCtor(isolate);
                auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(
                        ctx).ToLocalChecked();
                ret->SetInternalField(0, ext);
                array->Set(ctx, i, ret);
            }
        }
        args.GetReturnValue().Set(array);
        return;
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::ReadFileSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];

    std::string path;

    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }

    ReadFileOptions options{};

    Helpers::ParseReadFileOptions(isolate, args[1], options);

    try {
        auto ret = fs_read_file_sync(path.c_str(), options);
        bool is_buffer = fs_encoding_is_buffer(ret);

        if (!is_buffer) {
            auto buffer = fs_encoding_get_buffer_value(ret);
            auto bufferImpl = new BufferImpl(std::move(buffer));
            auto ext = v8::External::New(isolate, bufferImpl);

            auto ctor = BufferImpl::GetCtor(isolate);
            auto instance = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(
                    ctx).ToLocalChecked();
            instance->SetInternalField(0, ext);
            args.GetReturnValue().Set(instance);
        } else {
            auto val = fs_encoding_get_string_value(ret);
            args.GetReturnValue().Set(Helpers::ConvertToV8String(isolate,
                                                                 val));
        }

        return;
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::ReadLinkSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];

    std::string path;

    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }

    ReadLinkOptions options{};

    Helpers::ParseReadLinkOptions(isolate, args[1], options);

    try {
        auto ret = fs_read_link_sync(path.c_str(), options);

        bool is_buffer = fs_encoding_is_buffer(ret);

        if (!is_buffer) {
            auto buffer = fs_encoding_get_buffer_value(ret);
            auto bufferImpl = new BufferImpl(buffer);
            auto ext = v8::External::New(isolate, bufferImpl);

            auto ctor = BufferImpl::GetCtor(isolate);
            auto instance = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(
                    ctx).ToLocalChecked();
            instance->SetInternalField(0, ext);
            args.GetReturnValue().Set(instance);
        } else {
            auto val = fs_encoding_get_string_value(ret);
            args.GetReturnValue().Set(Helpers::ConvertToV8String(isolate,
                                                                 val));
        }

        return;
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::ReadvSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto fdValue = args[0];

    int32_t fd = -1;

    if (fdValue->IsInt32()) {
        fd = fdValue->Int32Value(ctx).ToChecked();
    }


    auto value = args[1];

    int64_t position = -1;
    auto positionValue = args[2];

    if (positionValue->IsNumber()) {
        position = (int64_t) positionValue->NumberValue(ctx).ToChecked();
    }

    std::vector<uint8_t *> vec;
    std::vector<size_t> vec_length;

    if (value->IsArray()) {
        auto array = value.As<v8::Array>();
        auto len = array->Length();
        bool hasError = false;
        vec.reserve(len);
        vec_length.reserve(len);
        for (int i = 0; i < len; i++) {
            auto itemValue = array->Get(ctx, i);
            if (itemValue.IsEmpty()) {
                hasError = true;
                break;
            }
            auto item = itemValue.ToLocalChecked();
            if (item->IsArrayBufferView()) {
                auto buffer = item.As<v8::ArrayBufferView>();
                auto length = buffer->ByteLength();
                auto arrayBuffer = buffer->Buffer();
                auto offset = buffer->ByteOffset();
                auto store = arrayBuffer->GetBackingStore();
                auto data = static_cast<uint8_t *>(store->Data()) + offset;
                vec.push_back(data);
                vec_length.push_back(length);
            }

        }
        if (hasError) {
            auto buffer = buffer_alloc(0);
            auto bufferImpl = new BufferImpl(std::move(buffer));
            auto ext = v8::External::New(isolate, bufferImpl);

            auto ctor = GetCtor(isolate);
            auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
            ret->SetInternalField(0, ext);
            args.GetReturnValue().Set(ret);
            return;
        }

        auto size = vec.size();
        try {
            auto ret = fs_readv_sync_slice(fd, vec.data(), vec_length.data(), size, position);
            args.GetReturnValue().Set((int32_t) ret);
            return;
        } catch (std::exception &error) {
            auto err = v8::Exception::Error(
                    Helpers::ConvertToV8String(isolate, error.what()));
            isolate->ThrowException(err);
        }

        return;
    }

    // todo throw
    args.GetReturnValue().Set(0);
}

void FSImpl::RealpathSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto pathValue = args[0];

    std::string path;

    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }

    RealPathOptions options{};

    Helpers::ParseRealPathOptions(isolate, args[1], options);

    try {
        auto ret = fs_real_path_sync(path.c_str(), options);
        args.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, ret));
        return;
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::RenameSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto oldPathValue = args[0];

    std::string oldPath;

    if (oldPathValue->IsString()) {
        oldPath = Helpers::ConvertFromV8String(isolate, oldPathValue);
    }

    auto newPathValue = args[0];

    std::string newPath;

    if (newPathValue->IsString()) {
        newPath = Helpers::ConvertFromV8String(isolate, newPathValue);
    }

    try {
        fs_rename_sync(oldPath.c_str(), newPath.c_str());
        args.GetReturnValue().SetUndefined();
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::RmdirSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];

    std::string path;

    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }

    RmDirOptions options{};

    Helpers::ParseRmDirOptions(isolate, args[1], options);

    try {
        fs_rmdir_sync(path.c_str(), options);
        args.GetReturnValue().SetUndefined();
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::RmSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];

    std::string path;

    RmOptions options{};

    Helpers::ParseRmOptions(isolate, args[1], options);

    try {
        fs_rm_sync(path.c_str(), options);
        args.GetReturnValue().SetUndefined();
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::StatfsSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto fdValue = args[0];

    int32_t fd = -1;

    if (fdValue->IsInt32()) {
        fd = fdValue->Int32Value(ctx).ToChecked();
    }

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
        auto stat = fs_fstat_sync(fd);
        auto ret = Helpers::FileStatToJS(isolate, bigint, *stat);
        filestat_destroy(stat);
        args.GetReturnValue().Set(ret);
        return;
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::SymlinkSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto targetValue = args[0];

    std::string target;

    if (targetValue->IsString()) {
        target = Helpers::ConvertFromV8String(isolate, targetValue);
    }

    auto pathValue = args[1];
    std::string path;

    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }

    auto typeValue = args[2];
    std::string type;

    if (typeValue->IsString()) {
        type = Helpers::ConvertFromV8String(isolate, typeValue);
    }


    try {
        fs_symlink_sync(target.c_str(), path.c_str(), type.c_str());
        args.GetReturnValue().SetUndefined();
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::TruncateSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];

    std::string path;

    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }

    uint64_t len = 0;

    auto lenValue = args[1];
    if (lenValue->IsNumber()) {
        auto value = lenValue->NumberValue(ctx).ToChecked();
        if (value > -1) {
            len = (uint64_t) value;
        }
    }

    try {
        fs_truncate_sync(path.c_str(), len);
        args.GetReturnValue().SetUndefined();
        return;
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::UnlinkSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];

    std::string path;

    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }

    try {
        fs_unlink_sync(path.c_str());
        args.GetReturnValue().SetUndefined();
        return;
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::UtimesSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathValue = args[0];

    std::string path;

    if (pathValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathValue);
    }

    int64_t atime = 0;
    auto atimeValue = args[1];

    if (atimeValue->IsNumber()) {
        atime = (int64_t) atimeValue->NumberValue(ctx).ToChecked();
    } else if (atimeValue->IsDate()) {
        atime = (int64_t) atimeValue.As<v8::Date>()->ValueOf();
    } else if (atimeValue->IsString()) {
        atime = (int64_t) std::stod(Helpers::ConvertFromV8String(isolate, atimeValue));
    }

    int64_t mtime = 0;
    auto mtimeValue = args[1];

    if (mtimeValue->IsNumber()) {
        mtime = (int64_t) mtimeValue->NumberValue(ctx).ToChecked();
    } else if (mtimeValue->IsDate()) {
        mtime = (int64_t) mtimeValue.As<v8::Date>()->ValueOf();
    } else if (mtimeValue->IsString()) {
        mtime = (int64_t) std::stod(Helpers::ConvertFromV8String(isolate, mtimeValue));
    }


    try {
        fs_utimes_sync(path.c_str(), atime, mtime);
        args.GetReturnValue().SetUndefined();
        return;
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::WriteFileSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto pathOrFdValue = args[0];

    std::string path;
    int32_t fd = -1;

    if (pathOrFdValue->IsString()) {
        path = Helpers::ConvertFromV8String(isolate, pathOrFdValue);
    } else if (pathOrFdValue->IsInt32()) {
        fd = pathOrFdValue->Int32Value(ctx).ToChecked();
    }


    auto dataValue = args[1];

    WriteFileOptions options{};

    Helpers::ParseWriteFileOptions(isolate, args[2], options);

    try {
        if (fd != -1) {
            if (dataValue->IsString()) {
                auto data = Helpers::ConvertFromV8String(isolate, dataValue);
                fs_write_file_with_str_sync(fd, data.c_str(),
                                            options);
            } else if (dataValue->IsTypedArray()) {
                auto array = dataValue.As<v8::TypedArray>();
                auto buffer = array->Buffer();
                auto store = buffer->GetBackingStore();
                auto os = array->ByteOffset();
                auto len = array->ByteLength();
                auto data = static_cast<uint8_t *>(store->Data()) + os;
                fs_write_file_with_bytes_sync(fd, data, len, options);
            }
        } else {
            if (dataValue->IsString()) {
                auto data = Helpers::ConvertFromV8String(isolate,
                                                         dataValue);
                fs_write_file_with_str_from_path_sync(path.c_str(), data.c_str(),
                                                      options);
            } else if (dataValue->IsTypedArray()) {
                auto array = dataValue.As<v8::TypedArray>();
                auto buffer = array->Buffer();
                auto store = buffer->GetBackingStore();
                auto os = array->ByteOffset();
                auto len = array->ByteLength();
                auto data = static_cast<uint8_t *>(store->Data()) + os;
                fs_write_file_with_bytes_from_path_sync(path.c_str(), data, len,
                                                        options);
            }
        }
        args.GetReturnValue().SetUndefined();
        return;
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::WriteSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto fdValue = args[0];

    int32_t fd = -1;

    if (fdValue->IsInt32()) {
        fd = fdValue->Int32Value(ctx).ToChecked();
    }

    auto bufferValue = args[1];

    WriteOptions options{};

    v8::Local<v8::Value> offsetValue = args[2];

    if (offsetValue->IsNumber()) {
        options.offset = (size_t) offsetValue->NumberValue(ctx).ToChecked();
    }

    v8::Local<v8::Value> lengthValue = args[3];

    if (lengthValue->IsNumber()) {
        options.length = (size_t) lengthValue->NumberValue(ctx).ToChecked();
    }

    v8::Local<v8::Value> positionValue = args[4];

    if (positionValue->IsNumber()) {
        options.position = (intptr_t) positionValue->NumberValue(ctx).ToChecked();
    }


    try {

        auto array = bufferValue.As<v8::TypedArray>();
        auto buffer = array->Buffer();
        auto store = buffer->GetBackingStore();
        auto os = array->ByteOffset();
        auto len = array->ByteLength();
        auto data = static_cast<uint8_t *>(store->Data()) + os;
        auto ret = fs_write_sync(fd, data, len, options);
        args.GetReturnValue().Set((int32_t) ret);
        return;
    } catch (std::exception &error) {
        auto err = v8::Exception::Error(
                Helpers::ConvertToV8String(isolate, error.what()));
        isolate->ThrowException(err);
    }

}

void FSImpl::WritevSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto fdValue = args[0];

    int32_t fd = -1;


    int64_t position = -1;
    auto positionValue = args[2];

    if (positionValue->IsNumber()) {
        position = (int64_t) positionValue->NumberValue(ctx).ToChecked();
    }

    std::vector<const uint8_t *> vec;
    std::vector<size_t> vec_length;

    auto value = args[1];

    if (value->IsArray()) {
        auto array = value.As<v8::Array>();
        auto len = array->Length();
        bool hasError = false;
        vec.reserve(len);
        vec_length.reserve(len);
        for (int i = 0; i < len; i++) {
            auto itemValue = array->Get(ctx, i);
            if (itemValue.IsEmpty()) {
                hasError = true;
                break;
            }
            auto item = itemValue.ToLocalChecked();
            if (item->IsArrayBufferView()) {
                auto buffer = item.As<v8::ArrayBufferView>();
                auto length = buffer->ByteLength();
                auto arrayBuffer = buffer->Buffer();
                auto offset = buffer->ByteOffset();
                auto store = arrayBuffer->GetBackingStore();
                auto data = static_cast<uint8_t *>(store->Data()) + offset;
                vec.push_back(data);
                vec_length.push_back(length);
            }

        }
        if (hasError) {
            auto buffer = buffer_alloc(0);
            auto bufferImpl = new BufferImpl(std::move(buffer));
            auto ext = v8::External::New(isolate, bufferImpl);

            auto ctor = GetCtor(isolate);
            auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
            ret->SetInternalField(0, ext);
            args.GetReturnValue().Set(ret);
            return;
        }


        try {
            auto ret = fs_writev_sync_slice(fd, vec.data(), vec_length.data(), vec.size(),
                                            position);
            args.GetReturnValue().Set((int32_t) ret);
            return;
        } catch (std::exception &error) {
            auto err = v8::Exception::Error(
                    Helpers::ConvertToV8String(isolate, error.what()));
            isolate->ThrowException(err);
        }

        return;
    }


    args.GetReturnValue().Set(0);

}


void FSImpl::Open(const v8::FunctionCallbackInfo<v8::Value> &args) {
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

    auto success = [](int32_t fd) {
        Helpers::LogToConsole("fd success");
    };
    auto error = []() {};
    auto callback = fs_async_create_async_i32_closure(&success, &error);
    fs_async_open(path.c_str(), flag, mode, callback);

}
