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

get_filename_component(PROJECT_ROOT_DIR ${PROJECT_SOURCE_DIR}/../../../../.. REALPATH)

get_filename_component(PROJECT_NATIVE_ROOT_DIR ${PROJECT_ROOT_DIR}/.. DIRECTORY)


MESSAGE("# NEW_PATH: " ${PROJECT_NATIVE_ROOT_DIR})

#set(PROJECT_NATIVE_ROOT_DIR ${PROJECT_NATIVE_ROOT_DIR})


set(CARGO_MANIFEST ${PROJECT_NATIVE_ROOT_DIR}/Cargo.toml)
set(CARGO_TARGET_DIR ${PROJECT_NATIVE_ROOT_DIR}/target)
set(CURRENT_TARGET_DIR ${CARGO_TARGET_DIR}/${RUST_BUILD_TARGET})


set(NODE_ANDROID_SOURCE_FILE ${PROJECT_NATIVE_ROOT_DIR}/crates/libs/node-cxx/src/lib.rs)
set(NODE_ANDROID_BRIDGE_CPP ${CARGO_TARGET_DIR}/cxxbridge/node-cxx/src/lib.rs.cc)

set(COMMON_CMAKE_ARGUMENTS "-std=c++17 -pthread -Werror -Wno-unused-result -mstackrealign -fexceptions -fno-builtin-stpcpy -DV8_31BIT_SMIS_ON_64BIT_ARCH")

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(RUST_BUILD_TYPE)
  set(NODE_ANDROID_LIB ${CURRENT_TARGET_DIR}/debug)
else ()
  set(RUST_BUILD_TYPE --release)
  set(NODE_ANDROID_LIB ${CURRENT_TARGET_DIR}/release)
  set(COMMON_CMAKE_ARGUMENTS "${COMMON_CMAKE_ARGUMENTS} -O3")
endif ()


set(NODE_ANDROID_LIB ${NODE_ANDROID_LIB}/${CMAKE_STATIC_LIBRARY_PREFIX}nodenative${CMAKE_SHARED_LIBRARY_SUFFIX})

if ("${ANDROID_ABI}" MATCHES "arm64-v8a$" OR "${ANDROID_ABI}" MATCHES "x86_64$")
  # Enable pointer compression on 64 bit platforms
  set(COMMON_CMAKE_ARGUMENTS "${COMMON_CMAKE_ARGUMENTS} -DV8_COMPRESS_POINTERS")
endif ()

include_directories(
        nodenativev8

        PUBLIC

        ${CARGO_TARGET_DIR}/cxxbridge

        ${PROJECT_SOURCE_DIR}/src/main/cpp/include/libc++
        ${PROJECT_SOURCE_DIR}/src/main/cpp/include
        ${PROJECT_SOURCE_DIR}/src/main/cpp
)

set(CMAKE_CXX_FLAGS ${COMMON_CMAKE_ARGUMENTS})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
        OUTPUT ${NODE_ANDROID_BRIDGE_CPP} ${NODE_ANDROID_LIB}
        #COMMAND cargo +nightly build -Z build-std='std,panic_abort' -Z build-std-features='panic_immediate_abort' --manifest-path ${CARGO_MANIFEST} --target ${RUST_BUILD_TARGET} ${RUST_BUILD_TYPE} -p node-android
        COMMAND cargo +nightly build -Z build-std='std' --manifest-path ${CARGO_MANIFEST} --target ${RUST_BUILD_TARGET} ${RUST_BUILD_TYPE} -p node-android
        # COMMAND cargo build --manifest-path ${CARGO_MANIFEST} --target ${RUST_BUILD_TARGET} ${RUST_BUILD_TYPE} -p node-cxx
        DEPENDS ${NODE_ANDROID_SOURCE_FILE}
        USES_TERMINAL
)


add_library(
        nodenativev8

        SHARED

        src/main/cpp/rust/cxx.cc
        src/main/cpp/NodeCompatImpl.cpp
        src/main/cpp/BufferImpl.cpp
        src/main/cpp/Caches.cpp
        src/main/cpp/Helpers.cpp
        src/main/cpp/Bridge.cpp
        src/main/cpp/FSImpl.cpp
        src/main/cpp/FileDirImpl.cpp
        src/main/cpp/FileDirentImpl.cpp

        ${NODE_ANDROID_BRIDGE_CPP}
)


set_target_properties(
        nodenativev8 PROPERTIES
        CXX_STANDARD 17
        CXX_EXTENSIONS OFF
        POSITION_INDEPENDENT_CODE ON
        LINK_FLAGS -Wl,--allow-multiple-definition
)


MESSAGE("# General cmake Info")
MESSAGE("# PROJECT_SOURCE_DIR: " ${PROJECT_SOURCE_DIR})
MESSAGE("# CMAKE_VERSION: " ${CMAKE_VERSION})
MESSAGE("# CMAKE_C_COMPILER_ID: " ${CMAKE_C_COMPILER_ID})
MESSAGE("# CMAKE_CXX_COMPILER_ID: " ${CMAKE_CXX_COMPILER_ID})
MESSAGE("# CMAKE_C_FLAGS: " ${CMAKE_C_FLAGS})
MESSAGE("# CMAKE_CXX_FLAGS: " ${CMAKE_CXX_FLAGS})

MESSAGE("# CARGO_MANIFEST: " ${CARGO_MANIFEST})
MESSAGE("# CARGO_TARGET_DIR: " ${CARGO_TARGET_DIR})
MESSAGE("# CURRENT_TARGET_DIR: " ${CURRENT_TARGET_DIR})
MESSAGE("# NODE_ANDROID_SOURCE_FILE: " ${NODE_ANDROID_SOURCE_FILE})
MESSAGE("# NODE_ANDROID_BRIDGE_CPP: " ${NODE_ANDROID_BRIDGE_CPP})
MESSAGE("# ANDROID_NDK_ROOT: " ${ANDROID_NDK_ROOT})
MESSAGE("# NODE_ANDROID_LIB: " ${NODE_ANDROID_LIB})
MESSAGE("# PROJECT_ROOT_DIR: " ${PROJECT_ROOT_DIR})
MESSAGE("# PROJECT_SOURCE_DIR: " ${PROJECT_SOURCE_DIR})

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
        ${NODE_ANDROID_LIB}
        android
        )


add_custom_command(TARGET nodenativev8
        POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        ${NODE_ANDROID_LIB}
        ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/libnodenative.so
        )
