//
// Created by Osei Fortune on 17/07/2023.
//

#include "FileDirImpl.h"
#include "Caches.h"

using namespace rust;

void FileDirImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    auto func = ctor->GetFunction(context).ToLocalChecked();

  //  global->Set(context, Helpers::ConvertToV8String(isolate, "NSCBuffer"), func);
}

FileDirImpl *FileDirImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<FileDirImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> FileDirImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->BufferTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, nullptr);

    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "NSCFileDir"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);


    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "path"),
            &GetPath
    );


    // use alloc
    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "allocUnsafe"),
            v8::FunctionTemplate::New(isolate, &Alloc));


    tmpl->SetAccessor(
            Helpers::ConvertToV8String(isolate, "length"),
            &Length
    );

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "atob"),
            v8::FunctionTemplate::New(isolate, &Atob)
    );

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "btoa"),
            v8::FunctionTemplate::New(isolate, &Btoa)
    );

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "concat"),
            v8::FunctionTemplate::New(isolate, &Concat)
    );

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "copyBytesFrom"),
            v8::FunctionTemplate::New(isolate, &CopyBytesFrom)
    );

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "from"),
            v8::FunctionTemplate::New(isolate, &From)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "toString"),
            v8::FunctionTemplate::New(isolate, &ToString));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "fill"),
            v8::FunctionTemplate::New(isolate, &Fill));


    // write
    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeInt8"),
            v8::FunctionTemplate::New(isolate, &WriteInt8));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeUInt8"),
            v8::FunctionTemplate::New(isolate, &WriteUInt8));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeInt16LE"),
            v8::FunctionTemplate::New(isolate, &WriteInt16LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeInt16BE"),
            v8::FunctionTemplate::New(isolate, &WriteInt16BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeUInt16LE"),
            v8::FunctionTemplate::New(isolate, &WriteUInt16LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeUInt16BE"),
            v8::FunctionTemplate::New(isolate, &WriteUInt16BE));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeInt32LE"),
            v8::FunctionTemplate::New(isolate, &WriteInt32LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeInt32BE"),
            v8::FunctionTemplate::New(isolate, &WriteInt32BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeUInt32LE"),
            v8::FunctionTemplate::New(isolate, &WriteUInt32LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeUInt32BE"),
            v8::FunctionTemplate::New(isolate, &WriteUInt32BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeFloatLE"),
            v8::FunctionTemplate::New(isolate, &WriteFloatLE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeFloatBE"),
            v8::FunctionTemplate::New(isolate, &WriteFloatBE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeDoubleLE"),
            v8::FunctionTemplate::New(isolate, &WriteDoubleLE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeDoubleBE"),
            v8::FunctionTemplate::New(isolate, &WriteDoubleBE));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeBigInt64LE"),
            v8::FunctionTemplate::New(isolate, &WriteBigInt64LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeBigInt64BE"),
            v8::FunctionTemplate::New(isolate, &WriteBigInt64BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeBigUInt64LE"),
            v8::FunctionTemplate::New(isolate, &WriteBigUInt64LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "writeBigUInt64BE"),
            v8::FunctionTemplate::New(isolate, &WriteBigUInt64BE));


    // read

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readInt8"),
            v8::FunctionTemplate::New(isolate, &ReadInt8));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readUInt8"),
            v8::FunctionTemplate::New(isolate, &ReadUInt8));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readInt16LE"),
            v8::FunctionTemplate::New(isolate, &ReadInt16LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readInt16BE"),
            v8::FunctionTemplate::New(isolate, &ReadInt16BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readUInt16LE"),
            v8::FunctionTemplate::New(isolate, &ReadUInt16LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readUInt16BE"),
            v8::FunctionTemplate::New(isolate, &ReadUInt16BE));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readInt32LE"),
            v8::FunctionTemplate::New(isolate, &ReadInt32LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readInt32BE"),
            v8::FunctionTemplate::New(isolate, &ReadInt32BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readUInt32LE"),
            v8::FunctionTemplate::New(isolate, &ReadUInt32LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readUInt32BE"),
            v8::FunctionTemplate::New(isolate, &ReadUInt32BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readFloatLE"),
            v8::FunctionTemplate::New(isolate, &ReadFloatLE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readFloatBE"),
            v8::FunctionTemplate::New(isolate, &ReadFloatBE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readDoubleLE"),
            v8::FunctionTemplate::New(isolate, &ReadDoubleLE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readDoubleBE"),
            v8::FunctionTemplate::New(isolate, &ReadDoubleBE));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readBigInt64LE"),
            v8::FunctionTemplate::New(isolate, &ReadBigInt64LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readBigInt64BE"),
            v8::FunctionTemplate::New(isolate, &ReadBigInt64BE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readBigUInt64LE"),
            v8::FunctionTemplate::New(isolate, &ReadBigUInt64LE));

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "readBigUInt64BE"),
            v8::FunctionTemplate::New(isolate, &ReadBigUInt64BE));


    tmpl->SetIndexedPropertyHandler(
            &IndexedGetter,
            &IndexedSetter
    );

    cache->FsDirTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);

    return ctorTmpl;

}

void
FileDirImpl::GetPath(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    auto isolate = info.GetIsolate();
    if (ptr != nullptr) {
        auto path = fs_dir_path(*ptr->dir_);
        info.GetReturnValue().Set(Helpers::ConvertToV8String(isolate, path.c_str()));
        return;
    }
    info.GetReturnValue().SetUndefined();
}


void
FileDirImpl::CloseSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        try {
            fs_dir_close_sync(ptr->GetDir());
        } catch (std::exception &error) {
            auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
            isolate->ThrowException(err);
        }
    }
    args.GetReturnValue().SetUndefined();
}


void
FileDirImpl::ReadSync(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto ptr = GetPointer(args.This());
    if (ptr != nullptr) {
        try {
            fs_dir_read_sync(ptr->GetDir());
        } catch (std::exception &error) {
            auto err = v8::Exception::Error(Helpers::ConvertToV8String(isolate, error.what()));
            isolate->ThrowException(err);
        }
    }
    args.GetReturnValue().SetUndefined();
}

FileDirImpl::FileDirImpl(rust::Box<FileDir> dir) : dir_(std::move(dir)) {}
