# SPDX-License-Identifier: Apache-2.0
# Copyright 2024 Joe Pearson
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

CARGO_MANIFEST_DIR = ../../../
LIBEFB_C = ../../c/
HEADERS = $(LIBEFB_C)/include
SWIFT_PACKAGE = .
CARGO_FLAGS = --manifest-path $(LIBEFB_C)/Cargo.toml --release --lib

all: clean xcframework

macos:
	@cargo build $(CARGO_FLAGS) --target aarch64-apple-darwin
	@cargo build $(CARGO_FLAGS) --target x86_64-apple-darwin
	@mkdir -p $(CARGO_MANIFEST_DIR)/target/universal-apple-darwin
	@lipo -create -output \
               $(CARGO_MANIFEST_DIR)/target/universal-apple-darwin/libefb_c.a \
               $(CARGO_MANIFEST_DIR)/target/aarch64-apple-darwin/release/libefb_c.a \
               $(CARGO_MANIFEST_DIR)/target/x86_64-apple-darwin/release/libefb_c.a

ios:
	@cargo build $(CARGO_FLAGS) --target aarch64-apple-ios
	@cargo build $(CARGO_FLAGS) --target x86_64-apple-ios
	@cargo build $(CARGO_FLAGS) --target aarch64-apple-ios-sim
	@mkdir -p $(CARGO_MANIFEST_DIR)/target/universal-apple-ios-sim
	@lipo -create -output \
               $(CARGO_MANIFEST_DIR)/target/universal-apple-ios-sim/libefb_c.a \
	       $(CARGO_MANIFEST_DIR)/target/aarch64-apple-ios-sim/release/libefb_c.a \
	       $(CARGO_MANIFEST_DIR)/target/x86_64-apple-ios/release/libefb_c.a

xcframework: macos ios
	@mkdir -p $(SWIFT_PACKAGE)/Frameworks
	@xcodebuild -create-xcframework \
	    -library $(CARGO_MANIFEST_DIR)/target/universal-apple-darwin/libefb_c.a \
	    -headers $(HEADERS)/ \
	    -library $(CARGO_MANIFEST_DIR)/target/aarch64-apple-ios/release/libefb_c.a \
	    -headers $(HEADERS)/ \
	    -library $(CARGO_MANIFEST_DIR)/target/universal-apple-ios-sim/libefb_c.a \
	    -headers $(HEADERS) \
	    -output $(SWIFT_PACKAGE)/Frameworks/efb.xcframework

clean:
	@$(RM) -rf $(CARGO_MANIFEST_DIR)/target/universal-apple-darwin
	@$(RM) -rf $(CARGO_MANIFEST_DIR)/target/universal-apple-ios
	@$(RM) -rf $(SWIFT_PACKAGE)/Frameworks/efb.xcframework
