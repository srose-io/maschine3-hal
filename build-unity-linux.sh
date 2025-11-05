#!/usr/bin/env bash
# Build script for Unity on Linux - fixes library dependencies for system compatibility

set -e

echo "Building Maschine MK3 HAL for Unity (Linux)..."

# Build in nix-shell
nix-shell --run "cargo build --release"

LIB_PATH="target/release/libmaschine3_hal.so"
echo "Built: $LIB_PATH"

# Check if we have patchelf available
if command -v patchelf &> /dev/null; then
    echo ""
    echo "Patching library to use system libraries instead of Nix store..."

    # Create a copy for Unity
    UNITY_LIB="target/release/libmaschine3_hal_unity.so"
    cp "$LIB_PATH" "$UNITY_LIB"

    # Set interpreter to system default
    patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 "$UNITY_LIB" 2>/dev/null || true

    # Set rpath to system library paths
    patchelf --set-rpath '/usr/lib:/usr/lib64:/lib:/lib64:$ORIGIN' "$UNITY_LIB"

    echo "Patched library: $UNITY_LIB"
    echo ""
    echo "Checking dependencies after patching:"
    ldd "$UNITY_LIB" || true

    LIB_TO_COPY="$UNITY_LIB"
else
    echo ""
    echo "WARNING: patchelf not found. Library may not work outside Nix environment."
    echo "Install with: nix-env -i patchelf"
    echo ""
    LIB_TO_COPY="$LIB_PATH"
fi

# Check if Unity project path is provided
if [ -n "$1" ]; then
    UNITY_PROJECT="$1"
    PLUGINS_DIR="$UNITY_PROJECT/Assets/Plugins/x86_64"

    echo ""
    echo "Copying to Unity project: $UNITY_PROJECT"

    # Create plugins directory if it doesn't exist
    mkdir -p "$PLUGINS_DIR"

    # Copy library (rename to libmaschine3_hal.so)
    cp "$LIB_TO_COPY" "$PLUGINS_DIR/libmaschine3_hal.so"
    chmod +x "$PLUGINS_DIR/libmaschine3_hal.so"

    # Copy C# scripts if they don't exist
    if [ ! -f "$UNITY_PROJECT/Assets/Plugins/MaschineMK3Native.cs" ]; then
        mkdir -p "$UNITY_PROJECT/Assets/Plugins"
        cp unity/MaschineMK3Native.cs "$UNITY_PROJECT/Assets/Plugins/"
        echo "Copied MaschineMK3Native.cs"
    fi

    if [ ! -f "$UNITY_PROJECT/Assets/Scripts/MaschineExampleController.cs" ]; then
        mkdir -p "$UNITY_PROJECT/Assets/Scripts"
        cp unity/MaschineExampleController.cs "$UNITY_PROJECT/Assets/Scripts/"
        echo "Copied MaschineExampleController.cs"
    fi

    # Copy diagnostic helper
    if [ ! -f "$UNITY_PROJECT/Assets/Scripts/DiagnosticHelper.cs" ]; then
        cp unity/DiagnosticHelper.cs "$UNITY_PROJECT/Assets/Scripts/"
        echo "Copied DiagnosticHelper.cs"
    fi

    echo ""
    echo "Unity integration complete!"
    echo "Library: $PLUGINS_DIR/libmaschine3_hal.so"
    echo ""
    echo "IMPORTANT: System library dependencies required:"
    echo "  - libusb-1.0"
    echo "  - libudev"
    echo ""
    echo "Install with:"
    echo "  Ubuntu/Debian: sudo apt install libusb-1.0-0 libudev1"
    echo "  Fedora/RHEL:   sudo dnf install libusb libudev"
    echo "  Arch:          sudo pacman -S libusb systemd-libs"
else
    echo ""
    echo "To copy to Unity project automatically, run:"
    echo "  ./build-unity-linux.sh /path/to/UnityProject"
    echo ""
    echo "Or manually copy:"
    echo "  $LIB_TO_COPY → UnityProject/Assets/Plugins/x86_64/libmaschine3_hal.so"
fi
