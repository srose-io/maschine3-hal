#!/usr/bin/env bash
# Build script for Unity on Linux - bundles all dependencies for self-contained deployment

set -e

echo "Building Maschine MK3 HAL for Unity (Linux - with bundled dependencies)..."

# Build in nix-shell
nix-shell --run "cargo build --release"

LIB_PATH="target/release/libmaschine3_hal.so"
echo "Built: $LIB_PATH"

# Create Unity output directory
UNITY_BUILD_DIR="target/unity-bundle"
mkdir -p "$UNITY_BUILD_DIR"

echo ""
echo "Bundling dependencies..."

# Copy the main library
UNITY_LIB="$UNITY_BUILD_DIR/libmaschine3_hal.so"
cp "$LIB_PATH" "$UNITY_LIB"

# Copy libusb from Nix store
nix-shell --run "cp /nix/store/98lr76bxps01kka3gxkq51v0hjvw6iag-libusb-1.0.29/lib/libusb-1.0.so.0 $UNITY_BUILD_DIR/"
echo "Bundled: libusb-1.0.so.0"

# Copy libudev from Nix store
nix-shell --run "cp /nix/store/jmb7npbaakci23xwr58azqil5b7hv1gy-systemd-minimal-libs-257.9/lib/libudev.so.1 $UNITY_BUILD_DIR/"
echo "Bundled: libudev.so.1"

# Copy libcap (dependency of libudev)
nix-shell --run "cp /nix/store/lqkp4qsndv7zv832sirra5qkvwm05qqy-libcap-2.76-lib/lib/libcap.so.2 $UNITY_BUILD_DIR/"
echo "Bundled: libcap.so.2"

# Make files writable (Nix store files are read-only)
chmod +w "$UNITY_BUILD_DIR"/*.so*

# Patch the library to look for dependencies in the same directory ($ORIGIN)
if command -v patchelf &> /dev/null || nix-shell --run "command -v patchelf" &> /dev/null; then
    echo ""
    echo "Patching library to use bundled dependencies..."

    # Use patchelf from nix-shell
    nix-shell --run "patchelf --set-rpath '\$ORIGIN' '$UNITY_LIB'"
    nix-shell --run "patchelf --set-rpath '\$ORIGIN' '$UNITY_BUILD_DIR/libusb-1.0.so.0'"
    nix-shell --run "patchelf --set-rpath '\$ORIGIN' '$UNITY_BUILD_DIR/libudev.so.1'"
    nix-shell --run "patchelf --set-rpath '\$ORIGIN' '$UNITY_BUILD_DIR/libcap.so.2'"

    echo "Patched all libraries to use \$ORIGIN (same directory)"
else
    echo "WARNING: patchelf not available"
fi

echo ""
echo "Verifying dependencies:"
ldd "$UNITY_LIB" 2>&1 | head -20

echo ""
echo "Bundle created in: $UNITY_BUILD_DIR/"
ls -lh "$UNITY_BUILD_DIR/"

# Check if Unity project path is provided
if [ -n "$1" ]; then
    UNITY_PROJECT="$1"
    PLUGINS_DIR="$UNITY_PROJECT/Assets/Plugins/x86_64"

    echo ""
    echo "Copying to Unity project: $UNITY_PROJECT"

    # Create plugins directory if it doesn't exist
    mkdir -p "$PLUGINS_DIR"

    # Copy all bundled libraries
    cp "$UNITY_BUILD_DIR"/*.so* "$PLUGINS_DIR/"
    chmod +x "$PLUGINS_DIR"/*.so*

    echo "Copied all libraries to: $PLUGINS_DIR/"
    ls -lh "$PLUGINS_DIR"/*.so*

    mkdir -p "$UNITY_PROJECT/Assets/Plugins"
    cp unity/MaschineMK3HardwareService.cs "$UNITY_PROJECT/Assets/GrowthSim/Scripts/Control/Hardware/"
    echo "Updated MaschineMK3HardwareService.cs"

    # Copy C# scripts (always overwrite to get updates)
    mkdir -p "$UNITY_PROJECT/Assets/Plugins"
    cp unity/MaschineMK3Native.cs "$UNITY_PROJECT/Assets/Plugins/"
    echo "Updated MaschineMK3Native.cs"

    cp unity/DiagnosticHelper.cs "$UNITY_PROJECT/Assets/Scripts/"
    echo "Updated DiagnosticHelper.cs"

    echo ""
    echo "✅ Unity integration complete!"
    echo ""
    echo "In Unity, configure plugin settings:"
    echo "  1. Select each .so file in Assets/Plugins/x86_64/"
    echo "  2. Inspector → Select platforms: Linux x86_64"
    echo "  3. Inspector → Load on startup: ✓"
    echo "  4. Click Apply"
    echo ""
    echo "All dependencies are bundled - no system libraries required!"
else
    echo ""
    echo "To copy to Unity project automatically, run:"
    echo "  ./build-unity-linux-bundle.sh /path/to/UnityProject"
    echo ""
    echo "Or manually copy ALL files from:"
    echo "  $UNITY_BUILD_DIR/*.so* → UnityProject/Assets/Plugins/x86_64/"
    echo ""
    echo "⚠️  IMPORTANT: Copy ALL .so files, not just libmaschine3_hal.so"
fi
