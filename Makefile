ANDROID_LIBS_DIR = ../android/app/src/main/jniLibs
.PHONY: langs

all: dirs arm64-v8a armv7-v7a current_platform load langs

dirs:
	mkdir -p $(ANDROID_LIBS_DIR)/{arm64-v8a,armeabi-v7a} ../assets/langs

arm64-v8a:
	ANDROID_NDK_HOME=${ANDROID_HOME}/ndk cargo +nightly ndk -t arm64-v8a build -Zbuild-std --release

armv7-v7a:
	ANDROID_NDK_HOME=${ANDROID_HOME}/ndk cargo +nightly ndk -t armeabi-v7a build -Zbuild-std --release

current_platform:
	cargo build --lib --release

load:
	cp target/aarch64-linux-android/release/libnum_string.so $(ANDROID_LIBS_DIR)/arm64-v8a
	cp target/armv7-linux-androideabi/release/libnum_string.so $(ANDROID_LIBS_DIR)/armeabi-v7a
	cp target/release/libnum_string.so ../lib

langs:
	cp src/langs/english.txt ../assets/langs
	cp src/langs/italian.txt ../assets/langs

setup:
	rustup toolchain add nightly-x86_64-unknown-linux-gnu
	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
	rustup target add --toolchain nightly aarch64-linux-android armv7-linux-androideabi
	cargo install cargo-ndk
