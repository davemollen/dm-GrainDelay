PACKAGE_NAME=(`./scripts/get-package-name.sh vst`)
NAME=$(echo $PACKAGE_NAME | perl -pe 's/dm_+([^\W_])/dm-\U$1/g' | perl -pe 's/(?<=[^\W_])_+([^\W_])/\U$1/g')
VST_NAME="$NAME.vst"
MOVE_TO="/Library/Audio/Plug-Ins/VST/$VST_NAME"
BINARY_NAME="lib$PACKAGE_NAME.dylib"

cd vst
cargo build --release
../scripts/osx_vst_bundler.sh $NAME target/release/$BINARY_NAME 

if [ -d "$MOVE_TO" ]; then
    rm -r "$MOVE_TO"
fi

if mv "$VST_NAME" "$MOVE_TO"; then
    echo "Copied VST bundle to $MOVE_TO"
fi