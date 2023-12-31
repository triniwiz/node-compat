    if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo nightly+ build --target ${RUST_BUILD_TARGET} --package node-android)
    set(TARGET_DIR ${RUST_BUILD_TARGET}/debug)
else ()
    set(CARGO_CMD cargo nightly+ build --release --target ${RUST_BUILD_TARGET} --package node-android)
    set(TARGET_DIR ${RUST_BUILD_TARGET}/release)
endif ()

if (ENABLE_LTO)
    set(RUST_FLAGS "-Clinker-plugin-lto" "-Clinker=clang-14" "-Clink-arg=-fuse-ld=lld-14")
endif ()

#set(NODE_CORE_LIB ${PROJECT_NATIVE_ROOT_DIR}/target/${TARGET_DIR}/libnodecxx.a)

set(CXX_BRIDGE_PREFIX "")

include_directories(
        ${PROJECT_NATIVE_ROOT_DIR}/target/${RUST_BUILD_TARGET}/cxxbridge
)

add_custom_command(
        OUTPUT "${PROJECT_NATIVE_ROOT_DIR}/target/${RUST_BUILD_TARGET}/cxxbridge/node-cxx/src/lib.rs.cc"
        COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} RUSTFLAGS= "${RUST_FLAGS}" ${CARGO_CMD}
        WORKING_DIRECTORY ${PROJECT_NATIVE_ROOT_DIR}
)