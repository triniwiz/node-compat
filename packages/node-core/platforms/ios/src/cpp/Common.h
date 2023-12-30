//
// Created by Osei Fortune on 05/07/2023.
//
#pragma once

#ifndef NODECOMPATDEMO_COMMON_H
#define NODECOMPATDEMO_COMMON_H

#include <stdint.h>
#include <string.h>

#ifdef __ANDROID__
#include <android/log.h>
#include "nodenative.h"
#include "include/v8.h"
#endif

#ifdef __APPLE__
#include <NativeScript/include/v8.h>

#ifdef __cplusplus
extern "C" {
#endif

#include "nodenative/include/node_c.h"

#ifdef __cplusplus
}
#endif

#endif


#endif //NODECOMPATDEMO_COMMON_H
