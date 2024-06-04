HEADERS = capi/include
SWIFT_PACKAGE = swift/libefb-ios
CARGO_FLAGS = --manifest-path Cargo.toml --release --lib

all: clean xcframework

macos:
	@cargo build $(CARGO_FLAGS) --target aarch64-apple-darwin
	@cargo build $(CARGO_FLAGS) --target x86_64-apple-darwin
	@mkdir -p target/universal-apple-darwin
	@lipo -create -output \
               target/universal-apple-darwin/libefb.a \
               target/aarch64-apple-darwin/release/libefb.a \
               target/x86_64-apple-darwin/release/libefb.a

ios:
	@cargo build $(CARGO_FLAGS) --target aarch64-apple-ios
	@cargo build $(CARGO_FLAGS) --target x86_64-apple-ios
	@cargo build $(CARGO_FLAGS) --target aarch64-apple-ios-sim
	@mkdir -p target/universal-apple-ios-sim
	@lipo -create -output \
               target/universal-apple-ios-sim/libefb.a \
	       target/aarch64-apple-ios-sim/release/libefb.a \
	       target/x86_64-apple-ios/release/libefb.a

xcframework: macos ios
	@mkdir -p $(SWIFT_PACKAGE)/Frameworks
	@xcodebuild -create-xcframework \
	    -library target/universal-apple-darwin/libefb.a \
	    -headers capi/include/ \
	    -library target/aarch64-apple-ios/release/libefb.a \
	    -headers capi/include/ \
	    -library target/universal-apple-ios-sim/libefb.a \
	    -headers $(HEADERS) \
	    -output $(SWIFT_PACKAGE)/Frameworks/EFB.xcframework

clean:
	@$(RM) -rf target/universal-apple-darwin
	@$(RM) -rf target/universal-apple-ios
	@$(RM) -rf $(SWIFT_PACKAGE)/Frameworks/EFB.xcframework
