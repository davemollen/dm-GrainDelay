PACKAGE_NAME=(`./scripts/get-package-name.sh vst`)
NAME=$(echo $PACKAGE_NAME | perl -pe 's/(?<=[^\W_])_+([^\W_])|_+/-\U$1/g')
VST_NAME="$NAME.vst"
MOVE_TO="/Library/Audio/Plug-Ins/VST/$VST_NAME"
BINARY_NAME="lib$PACKAGE_NAME.dylib"

cd vst
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
lipo -create target/x86_64-apple-darwin/release/$BINARY_NAME target/aarch64-apple-darwin/release/$BINARY_NAME -output target/release/$BINARY_NAME
file target/release/$BINARY_NAME
../scripts/osx_vst_bundler.sh $NAME target/release/$BINARY_NAME 

if [ -d "$MOVE_TO" ]; then
    sudo rm -r "$MOVE_TO"
fi

if sudo mv "$VST_NAME" "$MOVE_TO"; then
    echo "Copied VST bundle to $MOVE_TO"
fi