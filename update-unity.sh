#!/usr/bin/env bash
# Helper script to update Unity plugin during development

if [ -z "$1" ]; then
    echo "Usage: ./update-unity.sh /path/to/UnityProject"
    exit 1
fi

UNITY_PROJECT="$1"
PLUGINS_DIR="$UNITY_PROJECT/Assets/Plugins/x86_64"

echo "=== Unity Plugin Update Script ==="
echo ""

# Check if Unity is running
if pgrep -x "Unity" > /dev/null; then
    echo "⚠️  WARNING: Unity is running!"
    echo ""
    echo "Unity locks native plugins when loaded. Please:"
    echo "  1. Close Unity (File → Exit)"
    echo "  2. Run this script again"
    echo "  3. Reopen Unity"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 1
    fi
    echo ""
    echo "⚠️  Continuing... Unity may need to be restarted if it crashes"
    echo ""
fi

# Build and copy
echo "Building library..."
chmod -R +w target/unity-bundle 2>/dev/null
rm -rf target/unity-bundle
./build-unity-linux-bundle.sh "$UNITY_PROJECT"

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Update complete!"
    echo ""
    echo "Next steps:"
    echo "  1. Open Unity (or restart if it was running)"
    echo "  2. Wait for script recompilation"
    echo "  3. Test your changes"
else
    echo ""
    echo "❌ Build failed"
    exit 1
fi
