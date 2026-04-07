#!/bin/bash

ACORE_ROOT="../azerothcore-wotlk"
DEST_DIR="$ACORE_ROOT/src/server/apps/worldserver/violet"

echo "--- [VioletCore] Beginning of assimilation: Rust build on Arch Linux ---"

RUSTFLAGS="-C target-cpu=native" cargo build --release

mkdir -p "$DEST_DIR"

# Linux: .so & .a
if [ -f "target/release/libviolet_core_override.so" ]; then
    cp target/release/libviolet_core_override.so "$DEST_DIR/"
    echo "[VioletCore] .so synchronized."
fi

if [ -f "target/release/libviolet_core_override.a" ]; then
    cp target/release/libviolet_core_override.a "$DEST_DIR/"
    echo "[VioletCore] .a synchronized."
fi

echo "--- [VioletCore] Synchronization complete. Code in: worldserver/violet ---"