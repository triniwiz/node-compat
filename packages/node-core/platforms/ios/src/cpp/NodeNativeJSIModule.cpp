//
// Created by Osei Fortune on 29/12/2023.
//

#include "NodeNativeJSIModule.h"

void NodeNativeJSIModule::install(v8::Isolate *isolate) {
     BufferImpl::Init(isolate);
    FSImpl::Init(isolate);
}
