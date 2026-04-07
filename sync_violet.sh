#!/bin/bash

ACORE_ROOT="../azerothcore-wotlk"
DEST_DIR="$ACORE_ROOT/src/server/apps/worldserver/violet"

echo "--- [VioletCore] Beginning of assimilation: Rust build ---"
cargo build --release --quiet

mkdir -p "$DEST_DIR"

# for macOS
cp target/release/libviolet_core_override.dylib "$DEST_DIR/"
cp target/release/libviolet_core_override.a "$DEST_DIR/"

echo "--- [VioletCore] Synchronization complete. Code in: worldserver/violet ---"
