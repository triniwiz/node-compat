#!/bin/bash

cbindgen --config crates/libs/node-c/cbindgen.toml  crates/libs/node-c/src/lib.rs -l c > crates/libs/node-c/include/node_c.h
