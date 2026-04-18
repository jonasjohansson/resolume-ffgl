#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
LIB_PATH="$SCRIPT_DIR/target/release/libdot_grid.dylib"

echo "=== Building DotGrid ==="
cd "$SCRIPT_DIR"
cargo build --release

if [ ! -f "$LIB_PATH" ]; then
    echo "Error: $LIB_PATH not found"
    exit 1
fi

BUNDLE_NAME="DotGrid"
FFGL_DIRS=(
    "$HOME/Library/Graphics/FreeFrame Plug-Ins"
    "$HOME/Documents/Resolume Arena/Extra Effects"
)

for DIR in "${FFGL_DIRS[@]}"; do
    BUNDLE_DIR="$DIR/$BUNDLE_NAME.bundle/Contents/MacOS"
    echo "Deploying to $BUNDLE_DIR"
    mkdir -p "$BUNDLE_DIR"
    cp "$LIB_PATH" "$BUNDLE_DIR/$BUNDLE_NAME"
done

echo "=== Done! Restart Resolume to load DotGrid ==="
