ARCHS_IOS = x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim
ARCHS_ANDROID = i686-linux-android x86_64-linux-android aarch64-linux-android armv7-linux-androideabi
LIB = libnodenative.dylib
XCFRAMEWORK = NodeNative.xcframework
RUST_LIB = nodenative

all:GENERATE_HEADERS ios android

ios: $(XCFRAMEWORK)

android: GENERATE_ANDROID

.PHONY: GENERATE_HEADERS
GENERATE_HEADERS:
	./tools/scripts/build-headers.sh

# PHONY keyword on make means this is not a file, just an identifier for a target
.PHONY: $(ARCHS_IOS)
$(ARCHS_IOS): %:
	cargo +nightly build -Z build-std='std,panic_abort'  -Z build-std-features=panic_immediate_abort --target $@ --release -p node-c

$(XCFRAMEWORK): $(ARCHS_IOS)
	rm -rf tmp/simulator_fat
	mkdir tmp/simulator_fat
	lipo -create $(wildcard target/x86_64-apple-ios/release/$(LIB)) $(wildcard target/aarch64-apple-ios-sim/release/$(LIB)) -output tmp/simulator_fat/$(LIB)
	xcodebuild -create-xcframework -library $(wildcard target/aarch64-apple-ios/release/$(LIB)) -headers crates/libs/node-c/include -library tmp/simulator_fat/$(LIB) -headers crates/libs/node-c/include -output tmp/$@ && ./tools/scripts/copy-ios.sh

.PHONY: $(ARCHS_ANDROID)
$(ARCHS_ANDROID): %:
	./tools/scripts/build-android.sh $@

.PHONY: GENERATE_ANDROID
GENERATE_ANDROID: $(ARCHS_ANDROID)
	./tools/scripts/copy-android.sh

.PHONY: clean
clean:
	rm -rf target rm -rf $(XCFRAMEWORK) rm -rf simulator_fat/$(LIB)

