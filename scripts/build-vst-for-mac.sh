name="dm-GrainDelay"
binary_name="libdm_grain_delay.dylib"
vst_name="$name.vst"
move_to="/Library/Audio/Plug-Ins/VST/$vst_name"

cd vst
cargo build --release
cd target/release
../../../scripts/osx_vst_bundler.sh $name $binary_name 

if [ -d "$move_to" ]; then
    rm -r "$move_to"
fi

if mv "$vst_name" "$move_to"; then
    echo "Copied VST bundle to $move_to"
fi