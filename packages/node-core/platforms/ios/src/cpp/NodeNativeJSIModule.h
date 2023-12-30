//
// Created by Osei Fortune on 29/12/2023.
//

#pragma once

#include <memory>
#include <array>
#include "BufferImpl.h"
#include "FSImpl.h"


class NodeNativeJSIModule {
public:
    static void install(v8::Isolate * isolate);
};