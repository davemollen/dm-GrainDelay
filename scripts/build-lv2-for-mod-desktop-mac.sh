#!/bin/bash
binary_name_to_replace="libdm_graindelay.so"
binary_name="libdm_graindelay.dylib"
lv2_folder="dm-GrainDelay.lv2"
move_to="$lv2_folder/$binary_name"

# compile binary
cd lv2
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
MACOSX_DEPLOYMENT_TARGET=10.15 cargo build --release --target x86_64-apple-darwin
MACOSX_DEPLOYMENT_TARGET=10.15 cargo build --release --target aarch64-apple-darwin
lipo -create target/x86_64-apple-darwin/release/$binary_name target/aarch64-apple-darwin/release/$binary_name -output target/release/$binary_name
file target/release/$binary_name

# move compiled binary
if [ -d "$move_to" ]; then
    rm -r "$move_to"
fi
if mv target/release/$binary_name $move_to; then
    echo "Copied lv2 binary to $move_to"
fi

# replace <binary_name>.so with <binary_name>.dylib in manifest.ttl
perl -pi -e "s|$binary_name_to_replace|$binary_name|" $lv2_folder/manifest.ttl

# move lv2 plugin
cp -R $lv2_folder "/Applications/MOD Desktop.app/Contents/LV2"
echo "Copied lv2 plugin to MOD Desktop"