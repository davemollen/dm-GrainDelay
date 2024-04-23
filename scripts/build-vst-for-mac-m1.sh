name="dm-GrainDelay"
binary_name="libdm_grain_delay.dylib"
vst_name="$name.vst"
move_to="/Library/Audio/Plug-Ins/VST/$vst_name"

cd vst
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
lipo -create target/x86_64-apple-darwin/release/$binary_name target/aarch64-apple-darwin/release/$binary_name -output target/release/$binary_name
file target/release/$binary_name

cd target/release
../../../scripts/osx_vst_bundler.sh $name $binary_name 

if [ -d "$move_to" ]; then
    rm -r "$move_to"
fi

if mv "$vst_name" "$move_to"; then
    echo "Copied VST bundle to $move_to"
fi