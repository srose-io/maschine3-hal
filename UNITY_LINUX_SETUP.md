# Unity Linux Setup Guide

This guide covers Linux-specific setup for using the Maschine MK3 HAL with Unity.

## Problem: Nix Dependencies

When building in a Nix shell, the library links against Nix store paths which Unity cannot access. We need to patch the library to use system libraries instead.

## Prerequisites

### System Libraries Required

Install these system libraries (outside of Nix):

**Ubuntu/Debian:**
```bash
sudo apt install libusb-1.0-0 libudev1
```

**Fedora/RHEL:**
```bash
sudo dnf install libusb libudev
```

**Arch Linux:**
```bash
sudo pacman -S libusb systemd-libs
```

### udev Rules

You still need udev rules for device access:

```bash
sudo cp 99-maschine-mk3.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
sudo usermod -a -G audio $USER
```

**Important:** Log out and log back in for group changes to take effect.

## Building for Unity

### Step 1: Build and Patch

Use the special Linux build script:

```bash
./build-unity-linux.sh
```

This script:
1. Builds the library in nix-shell
2. Uses `patchelf` to modify library paths
3. Creates `target/release/libmaschine3_hal_unity.so`

### Step 2: Verify Dependencies

Check that system libraries are found:

```bash
ldd target/release/libmaschine3_hal_unity.so
```

You should see:
```
libusb-1.0.so.0 => /usr/lib/x86_64-linux-gnu/libusb-1.0.so.0 (0x...)
libudev.so.1 => /usr/lib/x86_64-linux-gnu/libudev.so.1 (0x...)
```

If you see "not found", install the missing system libraries.

### Step 3: Copy to Unity

**Automatic:**
```bash
./build-unity-linux.sh /path/to/YourUnityProject
```

**Manual:**
```bash
mkdir -p YourUnityProject/Assets/Plugins/x86_64
cp target/release/libmaschine3_hal_unity.so YourUnityProject/Assets/Plugins/x86_64/libmaschine3_hal.so
```

### Step 4: Configure in Unity

1. In Unity, select `Assets/Plugins/x86_64/libmaschine3_hal.so`
2. In the Inspector, configure:
   - ✅ **Select platforms for plugin**: Linux x86_64
   - ✅ **Load on startup**: Checked
   - ✅ **CPU**: x86_64

3. Click **Apply**

### Step 5: Test

1. Attach the `DiagnosticHelper` component to a GameObject
2. Press Play in Unity
3. Check the Console for diagnostic output

If you see "SUCCESS! mk3_new() returned a valid pointer", it's working!

## Troubleshooting

### Error: "libusb-1.0.so.0 => not found"

**Cause:** System libusb not installed

**Solution:**
```bash
# Ubuntu/Debian
sudo apt install libusb-1.0-0

# Fedora
sudo dnf install libusb

# Arch
sudo pacman -S libusb
```

### Error: "libudev.so.1 => not found"

**Cause:** System libudev not installed

**Solution:**
```bash
# Ubuntu/Debian
sudo apt install libudev1

# Fedora
sudo dnf install systemd-libs

# Arch
sudo pacman -S systemd-libs
```

### Error: DllNotFoundException in Unity

**Causes:**
1. Library not in correct location
2. Wrong file name
3. Missing system dependencies

**Solutions:**

1. **Check file location:**
   ```bash
   ls -la YourUnityProject/Assets/Plugins/x86_64/libmaschine3_hal.so
   ```

2. **Check dependencies:**
   ```bash
   ldd YourUnityProject/Assets/Plugins/x86_64/libmaschine3_hal.so
   ```

3. **Check permissions:**
   ```bash
   chmod +x YourUnityProject/Assets/Plugins/x86_64/libmaschine3_hal.so
   ```

4. **Use DiagnosticHelper:**
   - Add `DiagnosticHelper.cs` to your project
   - Attach to a GameObject
   - Check Console output

### Device Not Found (mk3_new returns NULL)

**Causes:**
1. Device not connected
2. udev rules not configured
3. User not in audio group

**Solutions:**

1. **Check USB connection:**
   ```bash
   lsusb | grep "17cc:1600"
   ```
   Should show: `Bus XXX Device XXX: ID 17cc:1600 Native Instruments Maschine MK3`

2. **Check udev rules:**
   ```bash
   ls -la /etc/udev/rules.d/99-maschine-mk3.rules
   ```

3. **Check group membership:**
   ```bash
   groups | grep audio
   ```
   If not in audio group: `sudo usermod -a -G audio $USER` then logout/login

4. **Test permissions:**
   ```bash
   ls -la /dev/bus/usb/$(lsusb | grep 17cc:1600 | awk '{print $2"/"$4}' | tr -d ':')
   ```
   Should show `crw-rw-r--` with group `audio`

## Unity Editor vs Standalone Builds

### In Editor
- Uses system libraries directly
- Requires all dependencies installed
- Hot-reload works after fixing errors

### Standalone Builds
- Bundles the .so with the game
- Still requires system libraries on target machine
- Include dependency instructions in your game's README

## Performance Notes

1. **Build Mode:** Always use `--release` for Unity
   ```bash
   nix-shell --run "cargo build --release"
   ```

2. **USB Latency:** Direct USB access on Linux provides better performance than Windows HID API

3. **Event Polling:** Don't call `PollEvents()` more than 60-120 Hz

## Alternative: Static Linking (Advanced)

For fully standalone builds, you can statically link libusb:

1. Edit `Cargo.toml`:
   ```toml
   [dependencies]
   rusb = { version = "0.9", features = ["vendored"] }
   ```

2. Build:
   ```bash
   nix-shell --run "cargo build --release"
   ```

This increases binary size but reduces runtime dependencies.

## Summary Checklist

Before running in Unity:
- ✅ System libraries installed (`libusb-1.0-0`, `libudev1`)
- ✅ udev rules configured
- ✅ User in `audio` group (logout/login required)
- ✅ Library built and patched with `build-unity-linux.sh`
- ✅ Library copied to `Assets/Plugins/x86_64/`
- ✅ Plugin import settings configured in Unity
- ✅ Device connected via USB

## Resources

- [Main Unity Integration Guide](UNITY_INTEGRATION.md)
- [Linux Setup Guide](LINUX_SETUP.md)
- [Quick Start](UNITY_QUICKSTART.md)
