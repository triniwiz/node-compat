//
// Created by Osei Fortune on 06/07/2023.
//

#include "BufferImpl.h"
#include "Caches.h"
#include "node-cxx/src/lib.rs.h"

void BufferImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    auto ctor = GetCtor(isolate);
    auto context = isolate->GetCurrentContext();
    auto global = context->Global();
    auto func = ctor->GetFunction(context).ToLocalChecked();

    global->Set(context, Helpers::ConvertToV8String(isolate, "NSCBuffer"), func);
}

BufferImpl *BufferImpl::GetPointer(v8::Local<v8::Object> object) {
    auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
    if (ptr == nullptr) {
        return nullptr;
    }
    return static_cast<BufferImpl *>(ptr);
}

v8::Local<v8::FunctionTemplate> BufferImpl::GetCtor(v8::Isolate *isolate) {
    auto cache = Caches::Get(isolate);
    auto ctor = cache->BufferTmpl.get();
    if (ctor != nullptr) {
        return ctor->Get(isolate);
    }

    v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate, nullptr);

    ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
    ctorTmpl->SetClassName(Helpers::ConvertToV8String(isolate, "NSCBuffer"));

    auto tmpl = ctorTmpl->InstanceTemplate();
    tmpl->SetInternalFieldCount(1);

    ctorTmpl->Set(
            Helpers::ConvertToV8String(isolate, "alloc"),
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
            Helpers::ConvertToV8String(isolate, "from"),
            v8::FunctionTemplate::New(isolate, &From)
    );

    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "toString"),
            v8::FunctionTemplate::New(isolate, &ToString));


    tmpl->Set(
            Helpers::ConvertToV8String(isolate, "fill"),
            v8::FunctionTemplate::New(isolate, &Fill));

    cache->BufferTmpl =
            std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
    return ctorTmpl;

}

BufferImpl::BufferImpl(rust::Box<Buffer> buffer) : buffer_(std::move(buffer)) {}

void BufferImpl::Alloc(const v8::FunctionCallbackInfo<v8::Value> &args) {
    int count = args.Length();
    auto firstArg = args[0];
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    if (count == 1 && firstArg->IsNumber()) {
        auto buffer = buffer_alloc((size_t) firstArg->NumberValue(ctx).ToChecked());
        auto bufferImpl = new BufferImpl(std::move(buffer));
        auto ext = v8::External::New(isolate, bufferImpl);

        auto ctor = GetCtor(isolate);
        auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
        ret->SetInternalField(0, ext);
        args.GetReturnValue().Set(ret);
    }
    if (count == 2) {
        auto size = (size_t) firstArg->NumberValue(ctx).ToChecked();
        auto fill = args[1];
        if (fill->IsString() || fill->IsStringObject()) {
            auto fillValue = Helpers::ConvertFromV8String(isolate, fill);
            auto buffer = buffer_alloc_with_size_string_encoding(size, rust::Str(fillValue),
                                                                 StringEncoding::Utf8);

            auto bufferImpl = new BufferImpl(std::move(buffer));
            auto ext = v8::External::New(isolate, bufferImpl);

            auto ctor = GetCtor(isolate);
            auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
            ret->SetInternalField(0, ext);
            args.GetReturnValue().Set(ret);
        } else if (fill->IsUint8Array()) {
            auto array = fill.As<v8::Uint8Array>();
            auto len = array->ByteLength();
            auto arrayBuffer = array->Buffer();
            auto offset = array->ByteOffset();
            auto store = arrayBuffer->GetBackingStore();
            auto data = static_cast<uint8_t *>(store->Data()) + offset;

            auto buffer = buffer_from_reference(data, len);

            auto bufferImpl = new BufferImpl(std::move(buffer));
            auto ext = v8::External::New(isolate, bufferImpl);

            auto ctor = GetCtor(isolate);
            auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
            ret->SetInternalField(0, ext);
            args.GetReturnValue().Set(ret);
        } else if (fill->IsNumber()) {

        }
    }
}

void
BufferImpl::Length(v8::Local<v8::String> name, const v8::PropertyCallbackInfo<v8::Value> &info) {
    auto ptr = GetPointer(info.This());
    if (ptr != nullptr) {
        info.GetReturnValue().Set((double) buffer_length(*ptr->buffer_));
        return;
    }
    info.GetReturnValue().Set(0);
}

void BufferImpl::Atob(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto value = args[0];
    if (value->IsString() || value->IsStringObject()) {
        auto str = Helpers::ConvertFromV8String(isolate, value);
        auto ret = buffer_atob(rust::Str(str));
        args.GetReturnValue().Set(
                Helpers::ConvertToV8String(isolate, ret.c_str())
        );
        return;
    }
    args.GetReturnValue().SetEmptyString();
}

void BufferImpl::Btoa(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto value = args[0];
    if (value->IsString() || value->IsStringObject()) {
        auto str = Helpers::ConvertFromV8String(isolate, value);
        auto ret = buffer_btoa(rust::Str(str));
        args.GetReturnValue().Set(
                Helpers::ConvertToV8String(isolate, ret.c_str())
        );
        return;
    }
    args.GetReturnValue().SetEmptyString();
}

void BufferImpl::Concat(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto value = args[0];
    auto cl = args[1];
    size_t concatLength = -1;

    if (cl->IsNumber()) {
        concatLength = (size_t) cl->NumberValue(ctx).ToChecked();
    }

    if (value->IsArray()) {
        auto array = value.As<v8::Array>();
        auto len = array->Length();
        bool hasError = false;
        std::vector<rust::Slice<const uint8_t>> vec;
        vec.reserve(len);
        for (int i = 0; i < len; i++) {
            auto itemValue = array->Get(ctx, i);
            if (itemValue.IsEmpty()) {
                hasError = true;
                break;
            }
            auto item = itemValue.ToLocalChecked();
            if (item->IsUint8Array()) {
                auto buffer = item.As<v8::Uint8Array>();
                auto length = buffer->ByteLength();
                auto arrayBuffer = buffer->Buffer();
                auto offset = buffer->ByteOffset();
                auto store = arrayBuffer->GetBackingStore();
                auto data = static_cast<const uint8_t *>(store->Data()) + offset;
                rust::Slice<const uint8_t> slice = rust::Slice(data, length);
                vec.push_back(slice);
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


        BufferImpl *bufferImpl;

        rust::Slice<const rust::Slice<const uint8_t>> val(vec.data(), vec.size());
        if (concatLength != -1) {
            auto buffer = buffer_concat_length(val, concatLength);
            bufferImpl = new BufferImpl(std::move(buffer));
        } else {
            auto buffer = buffer_concat(val);
            bufferImpl = new BufferImpl(std::move(buffer));
        }


        auto ext = v8::External::New(isolate, bufferImpl);
        auto ctor = GetCtor(isolate);
        auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
        ret->SetInternalField(0, ext);
        args.GetReturnValue().Set(ret);

        return;
    }

    auto buffer = buffer_alloc(0);
    auto bufferImpl = new BufferImpl(std::move(buffer));
    auto ext = v8::External::New(isolate, bufferImpl);

    auto ctor = GetCtor(isolate);
    auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
    ret->SetInternalField(0, ext);
    args.GetReturnValue().Set(ret);

}

void BufferImpl::From(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto value = args[0];
    if (value->IsString() || value->IsStringObject()) {
        auto encodingValue = args[1];
        StringEncoding encoding = Helpers::ParseEncoding(isolate, encodingValue,
                                                         StringEncoding::Utf8);
        auto str = Helpers::ConvertFromV8String(isolate, value);
        auto buffer = buffer_from_string(rust::Str(str), encoding);

        auto bufferImpl = new BufferImpl(std::move(buffer));
        auto ext = v8::External::New(isolate, bufferImpl);

        auto ctor = GetCtor(isolate);
        auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
        ret->SetInternalField(0, ext);
        args.GetReturnValue().Set(ret);
        return;
    } else if (value->IsArrayBuffer()) {
        auto arrayBuffer = value.As<v8::ArrayBuffer>();
        auto store = arrayBuffer->GetBackingStore();
        auto data = static_cast<const uint8_t *>(store->Data());
        auto buffer = buffer_from_slice(rust::Slice(data, arrayBuffer->ByteLength()));

        auto bufferImpl = new BufferImpl(std::move(buffer));
        auto ext = v8::External::New(isolate, bufferImpl);

        auto ctor = GetCtor(isolate);
        auto ret = ctor->GetFunction(ctx).ToLocalChecked()->NewInstance(ctx).ToLocalChecked();
        ret->SetInternalField(0, ext);
        args.GetReturnValue().Set(ret);
        return;
    }

}

void BufferImpl::ToString(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto len = args.Length();

    auto ptr = GetPointer(args.This());
    StringEncoding encoding = StringEncoding::Utf8;
    rust::isize start = 0;
    rust::isize end = -1;
    if (ptr != nullptr) {
        auto encodingValue = args[0];
        auto startValue = args[1];
        auto endValue = args[2];

        if(len == 0){
            auto ret = buffer_to_print_string(*ptr->buffer_);
            args.GetReturnValue().Set(
                    Helpers::ConvertToV8String(isolate, ret.c_str())
            );
            return;
        }

        if (len > 0) {
            encoding = Helpers::ParseEncoding(isolate, encodingValue, StringEncoding::Utf8);
        }

        if (len > 1 && startValue->IsNumber()) {
            start = (rust::isize) startValue->NumberValue(ctx).ToChecked();
        }

        if (len > 2 && endValue->IsNumber()) {
            end = (rust::isize) endValue->NumberValue(ctx).ToChecked();
        }

        auto ret = buffer_to_string(*ptr->buffer_, encoding, start, end);
        args.GetReturnValue().Set(
                Helpers::ConvertToV8String(isolate, ret.c_str())
        );
        return;
    }
    return args.GetReturnValue().SetEmptyString();
}

void BufferImpl::Fill(const v8::FunctionCallbackInfo<v8::Value> &args) {
    auto isolate = args.GetIsolate();
    auto ctx = isolate->GetCurrentContext();
    auto len = args.Length();

    auto ptr = GetPointer(args.This());
    StringEncoding encoding = StringEncoding::Utf8;
    rust::isize start = 0;
    rust::isize end = -1;
    if (ptr != nullptr) {
        auto encodingValue = args[0];
        auto startValue = args[1];
        auto endValue = args[2];
        if (len > 0) {
            encoding = Helpers::ParseEncoding(isolate, encodingValue, StringEncoding::Utf8);
        }

        if (len > 1 && startValue->IsNumber()) {
            start = (rust::isize) startValue->NumberValue(ctx).ToChecked();
        }

        if (len > 2 && endValue->IsNumber()) {
            end = (rust::isize) endValue->NumberValue(ctx).ToChecked();
        }

        auto ret = buffer_to_string(*ptr->buffer_, encoding, start, end);
        args.GetReturnValue().Set(
                Helpers::ConvertToV8String(isolate, ret.c_str())
        );
        return;
    }
}
