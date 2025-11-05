#!/usr/bin/env bash
# Check Maschine MK3 USB permissions

echo "=== Maschine MK3 USB Permissions Check ==="
echo ""

# Check if device is connected
echo "1. Checking if device is connected..."
DEVICE=$(lsusb | grep "17cc:1600")
if [ -z "$DEVICE" ]; then
    echo "   ❌ Device NOT found"
    echo "   → Make sure Maschine MK3 is connected via USB"
    exit 1
else
    echo "   ✓ Device found: $DEVICE"
fi

# Get bus and device numbers
BUS=$(echo "$DEVICE" | awk '{print $2}')
DEV=$(echo "$DEVICE" | awk '{print $4}' | tr -d ':')
DEVICE_PATH="/dev/bus/usb/$BUS/$DEV"

echo ""
echo "2. Checking device permissions..."
echo "   Path: $DEVICE_PATH"
ls -la "$DEVICE_PATH"

PERMS=$(ls -l "$DEVICE_PATH" | awk '{print $1}')
GROUP=$(ls -l "$DEVICE_PATH" | awk '{print $4}')

if [[ "$PERMS" == *"rw-rw-"* ]] && [[ "$GROUP" == "audio" ]]; then
    echo "   ✓ Permissions look good (rw-rw- and group is audio)"
else
    echo "   ❌ Permissions may be incorrect"
    echo "   → Expected: crw-rw-r-- with group 'audio'"
fi

echo ""
echo "3. Checking udev rules..."
if [ -f "/etc/udev/rules.d/99-maschine-mk3.rules" ]; then
    echo "   ✓ udev rules file exists"
    cat /etc/udev/rules.d/99-maschine-mk3.rules
else
    echo "   ❌ udev rules NOT found"
    echo "   → Install with: sudo cp 99-maschine-mk3.rules /etc/udev/rules.d/"
fi

echo ""
echo "4. Checking user groups..."
GROUPS_OUTPUT=$(groups)
if echo "$GROUPS_OUTPUT" | grep -q "audio"; then
    echo "   ✓ User is in 'audio' group"
else
    echo "   ❌ User is NOT in 'audio' group"
    echo "   → Add with: sudo usermod -a -G audio $USER"
    echo "   → Then log out and log back in"
fi

echo ""
echo "5. Testing USB access..."
if [ -r "$DEVICE_PATH" ] && [ -w "$DEVICE_PATH" ]; then
    echo "   ✓ USB device is readable and writable"
else
    echo "   ❌ Cannot access USB device"
    echo "   → Check udev rules and group membership"
fi

echo ""
echo "=== Summary ==="
if [ -r "$DEVICE_PATH" ] && [ -w "$DEVICE_PATH" ] && echo "$GROUPS_OUTPUT" | grep -q "audio"; then
    echo "✅ Everything looks good! Unity should be able to access the device."
else
    echo "❌ Issues found. Fix the problems above, then:"
    echo "   1. Reload udev rules: sudo udevadm control --reload-rules && sudo udevadm trigger"
    echo "   2. If you added yourself to audio group, log out and log back in"
    echo "   3. Reconnect the Maschine MK3"
fi
