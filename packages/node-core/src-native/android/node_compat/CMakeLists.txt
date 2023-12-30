cmake_minimum_required(VERSION 3.15.0)
set(CMAKE_VERBOSE_MAKEFILE ON)

project(nodenativev8)

set(CMAKE_CXX_STANDARD 17)

## TODO
#option(ENABLE_LTO "Enable cross language linking time optimization" ON)
#if (ENABLE_LTO)
#  include(CheckIPOSupported)
#  check_ipo_supported(RESULT supported OUTPUT error)
#  if (supported)
#    message(STATUS "IPO / LTO enabled")
#    set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)
#    add_link_options(-fuse-ld=lld-14)
#  else ()
#    message(STATUS "IPO / LTO not supported: <${error}>")
#  endif ()
#endif ()

if ("${ANDROID_ABI}" MATCHES "armeabi-v7a$")
  set(RUST_BUILD_TARGET "armv7-linux-androideabi")
elseif ("${ANDROID_ABI}" MATCHES "arm64-v8a$")
  set(RUST_BUILD_TARGET "aarch64-linux-android")
elseif ("${ANDROID_ABI}" MATCHES "x86_64$")
  set(RUST_BUILD_TARGET "x86_64-linux-android")
elseif ("${ANDROID_ABI}" MATCHES "x86$")
  set(RUST_BUILD_TARGET "i686-linux-android")
endif ()


set(COMMON_CMAKE_ARGUMENTS "-std=c++17 -pthread -Werror -Wno-unused-result -mstackrealign -fexceptions -fno-builtin-stpcpy -DV8_31BIT_SMIS_ON_64BIT_ARCH")


if ("${ANDROID_ABI}" MATCHES "arm64-v8a$" OR "${ANDROID_ABI}" MATCHES "x86_64$")
  # Enable pointer compression on 64 bit platforms
  set(COMMON_CMAKE_ARGUMENTS "${COMMON_CMAKE_ARGUMENTS} -DV8_COMPRESS_POINTERS")
endif ()

set(CPP_DIR ../../../platforms/ios/src/cpp)

include_directories(
        nodenativev8

        PUBLIC

        ${PROJECT_SOURCE_DIR}/src/main/cpp/include/libc++
        ${PROJECT_SOURCE_DIR}/src/main/cpp/include
        ${PROJECT_SOURCE_DIR}/src/main/cpp
        ${PROJECT_SOURCE_DIR}/src/main/cpp/nodenative
        ${PROJECT_SOURCE_DIR}/src/main/cpp/nodenative/include
        ${CPP_DIR}
        ${CPP_DIR}/include
)

set(CMAKE_CXX_FLAGS ${COMMON_CMAKE_ARGUMENTS})


add_library(
        nodenativev8

        SHARED

        ${CPP_DIR}/NodeCompatImpl.cpp
        ${CPP_DIR}/BufferImpl.cpp
        ${CPP_DIR}/Caches.cpp
        ${CPP_DIR}/Helpers.cpp
        ${CPP_DIR}/Bridge.cpp
        ${CPP_DIR}/FSImpl.cpp
        ${CPP_DIR}/FileDirImpl.cpp
        ${CPP_DIR}/FileDirentImpl.cpp
        ${CPP_DIR}/FileHandleImpl.cpp
        ${CPP_DIR}/NodeNativeJSIModule.cpp

        ${NODE_ANDROID_BRIDGE_CPP}
)


set_target_properties(
        nodenativev8 PROPERTIES
        CXX_STANDARD 17
        CXX_EXTENSIONS OFF
        POSITION_INDEPENDENT_CODE ON
        LINK_FLAGS -Wl,--allow-multiple-definition
)


find_library(system-log log)
find_library(system-z z)

add_custom_command(TARGET nodenativev8 POST_BUILD COMMAND ${CMAKE_STRIP} --strip-unneeded $<TARGET_FILE:nodenativev8>)

#if ("${ANDROID_ABI}" MATCHES "armeabi-v7a" OR "${ANDROID_ABI}" MATCHES "x86" AND NOT "${ANDROID_ABI}" MATCHES "x86_64")
#  # On API Level 19 and lower we need to link with android_support
#  # because it contains some implementation of functions such as "strtoll" and "strtoul"
#  target_link_libraries(nodenativev8
#          ${system-log}
#          ${system-z}
#          ${PROJECT_SOURCE_DIR}/src/main/libs/${ANDROID_ABI}/libNativeScript.so
#          ${ANDROID_NDK_ROOT}/sources/cxx-stl/llvm-libc++/libs/${ANDROID_ABI}/libandroid_support.a
#          ${NODE_ANDROID_LIB}
#          android
#          )
#
#else ()
#  target_link_libraries(nodenativev8
#          ${system-log}
#          ${system-z}
#          ${PROJECT_SOURCE_DIR}/src/main/libs/${ANDROID_ABI}/libNativeScript.so
#          ${NODE_ANDROID_LIB}
#          android
#          )
#endif ()


target_link_libraries(nodenativev8
        ${system-log}
        ${system-z}
        ${PROJECT_SOURCE_DIR}/src/main/libs/${ANDROID_ABI}/libNativeScript.so
        ${PROJECT_SOURCE_DIR}/src/main/jniLibs/${ANDROID_ABI}/libnodenative.so
        ${NODE_ANDROID_LIB}
        android
        )
