//
// Created by Osei Fortune on 05/07/2023.
//

#include "NodeCompatImpl.h"

void NodeCompatImpl::Init(v8::Isolate *isolate) {
    v8::Locker locker(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
}
