# Maschine MK3 HAL

A cross-platform Rust hardware abstraction layer (HAL) for the Native Instruments Maschine MK3 controller.

[![Crates.io](https://img.shields.io/crates/v/maschine3-hal.svg)](https://crates.io/crates/maschine3-hal)
[![Documentation](https://docs.rs/maschine3-hal/badge.svg)](https://docs.rs/maschine3-hal)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## Features

- ✅ **Cross-Platform**: Windows and Linux support with optimized communication paths
- ✅ **Real-time Input**: Button, pad, knob, and touch strip events with state tracking
- ✅ **LED Control**: Full RGB control for pads and group buttons, brightness control for other LEDs
- ✅ **Display Graphics**: 480x272 RGB565 display rendering with optimized bulk transfers
- ✅ **High Performance**: Platform-optimized USB communication for minimal latency
- ✅ **Safe API**: Type-safe abstractions over low-level USB protocols
- ✅ **Unity Integration**: C FFI layer for Unity Engine game development (see [Unity Integration Guide](UNITY_INTEGRATION.md))

## Platform Support

| Platform | Communication | Requirements | Performance |
|----------|---------------|--------------|-------------|
| **Windows** | HID API | WinUSB driver (via Zadig) | Stable, Compatible |
| **Linux** | Direct USB | udev rules, audio group | Optimized, Low-latency |

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
maschine3-hal = "0.1.0"
```

Basic usage:

```rust
use maschine3_hal::{MaschineMK3, InputEvent, MaschineLEDColor};

// Connect to device
let mut device = MaschineMK3::new()?;

// Monitor input events  
let events = device.poll_input_events()?;
for event in events {
    match event {
        InputEvent::PadHit { pad_number, velocity } => {
            println!("Pad {} hit with velocity {}", pad_number, velocity);
            device.set_pad_led(pad_number, MaschineLEDColor::red(true))?;
        }
        InputEvent::ButtonPressed(button) => {
            println!("Button {} pressed", button.name());
        }
        _ => {}
    }
}

// Control LEDs
device.set_pad_led(0, MaschineLEDColor::blue(true))?;
device.set_button_led_color(InputElement::GroupA, MaschineLEDColor::green(true))?;

// Display graphics (RGB565 format)
let pixels = vec![Rgb565::new(255, 0, 0); 480 * 272]; // Red screen
device.send_display_image(0, pixels)?;
```

## Installation

### Prerequisites

#### Windows
- Windows 10 or later
- [Zadig](https://zadig.akeo.ie/) to install WinUSB driver
- Visual Studio Build Tools or equivalent

#### Linux
- Linux kernel 2.6+ 
- Development packages for USB and udev

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install libudev-dev libusb-1.0-0-dev build-essential
```

**Fedora/RHEL:**
```bash
sudo dnf install systemd-devel libusb1-devel gcc
```

**Arch Linux:**
```bash
sudo pacman -S systemd libusb gcc
```

### Device Setup

#### Windows
1. Connect your Maschine MK3
2. Download and run [Zadig](https://zadig.akeo.ie/)
3. Select the Maschine MK3 device
4. Install WinUSB driver (replaces Native Instruments driver)

#### Linux
1. Copy udev rules:
   ```bash
   sudo cp 99-maschine-mk3.rules /etc/udev/rules.d/
   sudo udevadm control --reload-rules
   sudo udevadm trigger
   ```

2. Add your user to the audio group:
   ```bash
   sudo usermod -a -G audio $USER
   ```

3. **Log out and log back in** for group membership to take effect.

For detailed Linux setup instructions, see [`LINUX_SETUP.md`](LINUX_SETUP.md).

## Unity Integration

This library can be used in Unity Engine projects via C FFI bindings.

**Quick Start**:
```bash
# Build the native library
cargo build --release

# Copy to Unity (or run build script with Unity project path)
./build-unity.sh /path/to/UnityProject  # Linux/macOS
build-unity.bat C:\path\to\UnityProject  # Windows
```

Then attach the `MaschineMK3Native` component to a GameObject in Unity.

See the [Unity Integration Guide](UNITY_INTEGRATION.md) for complete documentation, API reference, and examples.

## Examples

Run the included examples to test your setup:

```bash
# Basic connectivity and LED test
cargo run --example simple_test

# Real-time input monitoring
cargo run --example input_monitor

# Display color patterns
cargo run --example color_test

# LED animations
cargo run --example led_animation

# Linux-specific performance test
cargo run --example linux_platform_test  # Linux only
```

## API Documentation

### Device Management

```rust
// Connect to first available device
let mut device = MaschineMK3::new()?;

// Get device information
println!("Device: {}", device.device_info()?);
```

### Input Monitoring

```rust
// Polling approach (blocking)
let events = device.poll_input_events()?;

// Callback approach (non-blocking)
device.start_input_monitoring(|event| {
    println!("Event: {:?}", event);
})?;
```

### LED Control

```rust
// Individual pad LEDs (RGB)
device.set_pad_led(0, MaschineLEDColor::red(true))?;

// Group button LEDs (RGB)  
device.set_button_led_color(InputElement::GroupA, MaschineLEDColor::blue(true))?;

// Other button LEDs (brightness only)
device.set_button_led(InputElement::Play, 127)?;

// Bulk operations
device.set_all_pad_leds(MaschineLEDColor::white(true))?;
device.clear_all_leds()?;
```

### Display Graphics

```rust
// Send RGB565 image data
let pixels: Vec<Rgb565> = create_your_image();
device.send_display_image(0, pixels)?;

// Send RGB888 data (auto-converted)
let rgb_data: Vec<u8> = load_rgb_image();
device.send_display_rgb888(0, &rgb_data)?;

// Clear with solid color  
device.clear_display(0, 255, 0, 0)?; // Red
```

## Performance Considerations

### Linux Optimization
- Direct USB communication provides lower latency
- Automatic kernel driver handling  
- Optimized for real-time audio applications
- Consider real-time kernel for sub-millisecond latency

### Windows Compatibility
- HID API provides stable, driver-independent operation
- WinUSB driver required for display interface
- Slightly higher latency but excellent compatibility

### General Tips
- Use bulk LED updates when possible
- Pool input events at appropriate rates (typically 100-1000 Hz)
- Display updates are bandwidth-limited (~30 FPS for full-screen)

## Development

### Building

```bash
# Development build
cargo build

# Optimized build
cargo build --release

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt
```

### Documentation

Detailed protocol documentation is available in `docs/`:

- `MaschineMK3-Overview.md` - USB interfaces and endpoints
- `MaschineMK3-HIDInput.md` - Input protocol specification
- `MaschineMK3-HIDOutput.md` - LED control protocol
- `MaschineMK3-Display.md` - Display graphics protocol

## Contributing

Contributions are welcome! This is an open-source project and we'd love help with:

- **Platform Support**: Linux and macOS implementations
- **Protocol Reverse Engineering**: Additional device features
- **Performance Optimization**: Latency and throughput improvements
- **Documentation**: Examples, tutorials, and protocol docs
- **Testing**: Device compatibility and edge cases

## Troubleshooting

### Device Not Found
- **Windows**: Install WinUSB driver via Zadig
- **Linux**: Check udev rules and group membership
- **Both**: Verify USB connection, try different ports

### Permission Denied (Linux)
```bash
# Check group membership
groups $USER

# Reinstall udev rules  
sudo cp 99-maschine-mk3.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules

# Log out/in to refresh groups
```

### LED/Display Issues
- Ensure proper driver installation
- Check USB power (may need powered hub for intensive LED use)
- Verify interface claiming succeeded in debug output

## Contributing

Contributions are welcome! Please see [`CONTRIBUTING.md`](CONTRIBUTING.md) for guidelines.

### Development Setup

1. Clone the repository
2. Install platform prerequisites (see Installation above)
3. Run tests: `cargo test`
4. Run examples: `cargo run --example simple_test`

### Testing

The library includes comprehensive tests and examples:
- Unit tests for protocol parsing
- Integration tests with mock devices
- Hardware examples for real device testing

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This is an unofficial, community-developed library. It is not affiliated with or endorsed by Native Instruments. Use of this library may void your device warranty and could potentially cause device malfunction. Use at your own risk.

The library requires replacing official drivers, which may interfere with Native Instruments software. Ensure you understand the implications before proceeding.

## Acknowledgments

- Native Instruments for creating the Maschine MK3 hardware
- The Rust USB/HID community for excellent libraries and documentation
- **Protocol Documentation**: This implementation is adapted from the excellent reverse engineering work by:
  - [Drachenkaetzchen/cabl](https://github.com/Drachenkaetzchen/cabl/tree/develop/doc/hardware/maschine-mk3) - Comprehensive protocol documentation (display protocol documentation has been significantly enhanced and expanded)
  - [asutherland/ni-controllers-lib](https://github.com/asutherland/ni-controllers-lib) - Native Instruments controller research
- Contributors to USB protocol reverse engineering efforts
