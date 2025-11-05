#!/usr/bin/env bash
# Build with versioned filename to avoid Unity lock issues

if [ -z "$1" ]; then
    echo "Usage: ./build-unity-versioned.sh /path/to/UnityProject"
    exit 1
fi

UNITY_PROJECT="$1"
PLUGINS_DIR="$UNITY_PROJECT/Assets/Plugins/x86_64"
VERSION=$(date +%s)  # Unix timestamp as version

echo "Building versioned library (v$VERSION)..."

# Build
chmod -R +w target/unity-bundle 2>/dev/null
rm -rf target/unity-bundle
./build-unity-linux-bundle.sh

# Copy with versioned name
mkdir -p "$PLUGINS_DIR"
cp target/unity-bundle/libmaschine3_hal.so "$PLUGINS_DIR/libmaschine3_hal_v$VERSION.so"
cp target/unity-bundle/libusb-1.0.so.0 "$PLUGINS_DIR/"
cp target/unity-bundle/libudev.so.1 "$PLUGINS_DIR/"
cp target/unity-bundle/libcap.so.2 "$PLUGINS_DIR/"

# Remove old versions (keep last 2)
cd "$PLUGINS_DIR"
ls -t libmaschine3_hal_v*.so | tail -n +3 | xargs rm -f 2>/dev/null

echo ""
echo "✅ Installed: libmaschine3_hal_v$VERSION.so"
echo ""
echo "⚠️  You need to update the DLL name in C# to: libmaschine3_hal_v$VERSION"
echo "    Or use a symlink approach instead"
