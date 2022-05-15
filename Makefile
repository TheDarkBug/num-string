ANDROID_NDK_HOME = $(HOME)/Android/Sdk/ndk
GENERATED_V8A = target/aarch64-linux-android/release/libnum_string.so
GENERATED_V7A = target/armv7-linux-androideabi/release/libnum_string.so

all: dirs arm64-v8a armv7-v7a

dirs:
	mkdir -p jniLibs/{arm64-v8a,armeabi-v7a}

arm64-v8a:
	ANDROID_NDK_HOME=$(ANDROID_NDK_HOME) cargo +nightly ndk -t arm64-v8a build -Zbuild-std --release
	cp $(GENERATED_V8A) jniLibs/arm64-v8a

armv7-v7a:
	ANDROID_NDK_HOME=$(ANDROID_NDK_HOME) cargo +nightly ndk -t armeabi-v7a build -Zbuild-std --release
	cp $(GENERATED_V7A) jniLibs/armeabi-v7a

setup:
	rustup toolchain add nightly-x86_64-unknown-linux-gnu
	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
	rustup target add --toolchain nightly aarch64-linux-android armv7-linux-androideabi
	cargo install cargo-ndk
