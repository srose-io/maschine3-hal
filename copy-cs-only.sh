#!/usr/bin/env bash
# Quick script to copy only C# files (no rebuild needed)

if [ -z "$1" ]; then
    echo "Usage: ./copy-cs-only.sh /path/to/UnityProject"
    exit 1
fi

UNITY_PROJECT="$1"

echo "Copying C# files to Unity project..."

mkdir -p "$UNITY_PROJECT/Assets/Plugins"
cp unity/MaschineMK3Native.cs "$UNITY_PROJECT/Assets/Plugins/"
echo "✓ Updated MaschineMK3Native.cs"

mkdir -p "$UNITY_PROJECT/Assets/Scripts"
cp unity/MaschineExampleController.cs "$UNITY_PROJECT/Assets/Scripts/"
echo "✓ Updated MaschineExampleController.cs"

cp unity/DiagnosticHelper.cs "$UNITY_PROJECT/Assets/Scripts/"
echo "✓ Updated DiagnosticHelper.cs"

echo ""
echo "✅ C# files updated!"
echo ""
echo "Unity should automatically detect and recompile the scripts."
echo "No need to restart Unity for C# changes."
