#!/bin/bash
# Build script for Unity integration
# This builds the native library as a dynamic library for Unity P/Invoke

set -e

echo "Building Maschine MK3 HAL for Unity..."

# Build release version
cargo build --release

# Determine platform
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    LIB_NAME="libmaschine3_hal.so"
    LIB_PATH="target/release/$LIB_NAME"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    LIB_NAME="libmaschine3_hal.dylib"
    LIB_PATH="target/release/$LIB_NAME"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    LIB_NAME="maschine3_hal.dll"
    LIB_PATH="target/release/$LIB_NAME"
else
    echo "Unknown platform: $OSTYPE"
    exit 1
fi

echo "Built: $LIB_PATH"

# Check if Unity project path is provided
if [ -n "$1" ]; then
    UNITY_PROJECT="$1"
    PLUGINS_DIR="$UNITY_PROJECT/Assets/Plugins/x86_64"

    echo "Copying to Unity project: $UNITY_PROJECT"

    # Create plugins directory if it doesn't exist
    mkdir -p "$PLUGINS_DIR"

    # Copy library
    cp "$LIB_PATH" "$PLUGINS_DIR/"

    # Copy C# scripts if they don't exist
    if [ ! -f "$UNITY_PROJECT/Assets/Plugins/MaschineMK3Native.cs" ]; then
        cp unity/MaschineMK3Native.cs "$UNITY_PROJECT/Assets/Plugins/"
        echo "Copied MaschineMK3Native.cs"
    fi

    if [ ! -f "$UNITY_PROJECT/Assets/Scripts/MaschineExampleController.cs" ]; then
        mkdir -p "$UNITY_PROJECT/Assets/Scripts"
        cp unity/MaschineExampleController.cs "$UNITY_PROJECT/Assets/Scripts/"
        echo "Copied MaschineExampleController.cs"
    fi

    echo "Unity integration complete!"
    echo "Library: $PLUGINS_DIR/$LIB_NAME"
else
    echo ""
    echo "To copy to Unity project automatically, run:"
    echo "  ./build-unity.sh /path/to/UnityProject"
    echo ""
    echo "Or manually copy:"
    echo "  $LIB_PATH → UnityProject/Assets/Plugins/x86_64/"
    echo "  unity/MaschineMK3Native.cs → UnityProject/Assets/Plugins/"
    echo "  unity/MaschineExampleController.cs → UnityProject/Assets/Scripts/"
fi
