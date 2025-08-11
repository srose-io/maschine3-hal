# Linux Setup Guide for Maschine MK3 HAL

This guide explains how to set up your Linux system to work with the Native Instruments Maschine MK3 controller.

## Prerequisites

- Linux kernel 2.6.x or later
- User account with sudo privileges
- Native Instruments Maschine MK3 hardware

## Installation Steps

### 1. Install System Dependencies

For Ubuntu/Debian:
```bash
sudo apt update
sudo apt install libudev-dev libusb-1.0-0-dev
```

For Fedora/RHEL:
```bash
sudo dnf install systemd-devel libusb1-devel
```

For Arch Linux:
```bash
sudo pacman -S systemd libusb
```

### 2. Set Up USB Permissions

The Maschine MK3 requires special USB permissions to be accessed by non-root users.

#### Option A: Using the provided udev rules (Recommended)

1. Copy the udev rules file:
```bash
sudo cp 99-maschine-mk3.rules /etc/udev/rules.d/
```

2. Add your user to the audio group:
```bash
sudo usermod -a -G audio $USER
```

3. Reload udev rules:
```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

4. **Log out and log back in** for group membership to take effect.

#### Option B: Manual udev rules

If you prefer to create the rules manually:

1. Create the udev rules file:
```bash
sudo nano /etc/udev/rules.d/99-maschine-mk3.rules
```

2. Add the following content:
```
# Native Instruments Maschine MK3
SUBSYSTEM=="usb", ATTRS{idVendor}=="17cc", ATTRS{idProduct}=="1600", GROUP="audio", MODE="0664"
KERNEL=="hidraw*", ATTRS{idVendor}=="17cc", ATTRS{idProduct}=="1600", GROUP="audio", MODE="0664"
```

3. Follow steps 2-4 from Option A.

### 3. Verify Setup

1. Connect your Maschine MK3 to your computer.

2. Check that the device is detected:
```bash
lsusb | grep "17cc:1600"
```
You should see: `Bus XXX Device XXX: ID 17cc:1600 Native Instruments Maschine MK3`

3. Check permissions:
```bash
ls -la /dev/bus/usb/*/
```
Look for devices with group `audio` and mode `664`.

4. Test with the provided examples:
```bash
cargo run --example simple_test
```

## Troubleshooting

### Permission Denied Errors

If you get "Permission denied" errors:

1. Verify your user is in the audio group:
```bash
groups $USER
```

2. Check if udev rules are properly installed:
```bash
cat /etc/udev/rules.d/99-maschine-mk3.rules
```

3. Restart udev and reconnect the device:
```bash
sudo systemctl restart systemd-udevd
# Unplug and replug the Maschine MK3
```

### Device Not Found

If the device is not detected:

1. Check USB connection and try different ports
2. Verify the device works on other systems
3. Check kernel logs: `dmesg | tail`
4. Try running with sudo (temporarily, for testing only)

### Driver Conflicts

If you have Native Instruments software installed:

1. The system may have loaded kernel drivers that conflict
2. The library will automatically detach them, but you may see warnings
3. This is normal and expected behavior

### Audio Group Not Found

On some distributions, the audio group might not exist:

1. Create the group: `sudo groupadd audio`
2. Add your user: `sudo usermod -a -G audio $USER`
3. Or modify the udev rules to use a different group like `plugdev`

## Performance Tips

### Real-time Priority

For low-latency applications, consider:

1. Installing `rtirq` package
2. Setting up real-time kernel
3. Configuring audio group limits in `/etc/security/limits.conf`

### USB Buffer Sizes

For high-throughput applications, you may need to adjust USB buffer sizes:

```bash
echo 1000 | sudo tee /sys/module/usbcore/parameters/usbfs_memory_mb
```

## Security Considerations

The provided udev rules allow members of the `audio` group to access the Maschine MK3. This is the recommended approach as it:

- Provides necessary access without requiring root privileges
- Follows Linux audio system conventions  
- Maintains reasonable security boundaries

Avoid using MODE="0666" (world-writable) unless absolutely necessary.

## Supported Distributions

This library has been tested on:

- Ubuntu 20.04+
- Fedora 35+
- Arch Linux
- Debian 11+

Other distributions should work but may require minor adjustments to package names or group memberships.