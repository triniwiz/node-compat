#! /bin/bash
LIB_NAME="nodenative"
FRAMEWORK="NodeNative"

cp -r tmp/$FRAMEWORK.xcframework packages/node-core/platforms/ios
rm -rf tmp/$FRAMEWORK.xcframework
rm tmp/simulator_fat/lib$LIB_NAME.dylib