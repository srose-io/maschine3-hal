# mk3-hal

A Rust hardware abstraction layer (HAL) for the Native Instruments Maschine MK3 controller.

[![Crate](https://img.shields.io/badge/crates.io-not%20yet%20published-red)](https://crates.io/crates/mk3-hal)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-alpha-orange)](https://github.com/srose-dev/mk3-hal)

## Overview

This library provides direct, low-level USB communication with the Native Instruments Maschine MK3, bypassing Native Instruments' software. It handles:

- **Input Events**: Buttons, pads (with velocity), knobs, and touch strip
- **LED Control**: Individual button and pad LEDs with full color support
- **Display Graphics**: 480x272 RGB565 display output
- **Real-time Performance**: Optimized for low-latency audio applications

## ⚠️ Project Status

**Alpha Software** - This library is in early development. APIs may change, and some features are incomplete. Use at your own risk.

## Requirements

### Hardware

- Native Instruments Maschine MK3 controller

### Platform Support

- **Windows**: Primary supported platform
- **Linux/macOS**: Not currently supported (contributions welcome)

### Driver Setup (Windows)

**Important**: You must replace the Native Instruments USB driver with a generic WinUSB driver:

1. **Stop Native Instruments services**:

   ```
   net stop "Native Instruments Background Task Server"
   net stop "Native Instruments Shared Device Service"
   ```

2. **Install WinUSB driver** using [Zadig](https://zadig.akeo.ie/):

   - Download and run Zadig as Administrator
   - Select "Options" → "List All Devices"
   - Find "Maschine MK3" in the dropdown
   - Select "WinUSB" driver and click "Replace Driver"

3. **Verify installation**: The device should appear in Device Manager under "Universal Serial Bus devices"

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mk3-hal = { git = "https://github.com/srose-dev/mk3-hal" }
```

## Quick Start

```rust
use mk3_hal::{MaschineMK3, MaschineLEDColor, InputElement};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the device
    let mut device = MaschineMK3::new()?;
    println!("Connected: {}", device.device_info()?);

    // Monitor input events
    loop {
        let events = device.poll_input_events()?;
        for event in events {
            println!("{}", event.description());

            // Light up pads when hit
            if let mk3_hal::InputEvent::PadHit { pad_number, .. } = event {
                device.set_pad_led(pad_number, MaschineLEDColor::red(true))?;
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}
```

## Examples

The `examples/` directory contains various demonstration programs:

- **`simple_test.rs`** - Basic connectivity, input monitoring, and LED control
- **`input_monitor.rs`** - Real-time input event monitoring
- **`color_test.rs`** - Display color patterns and graphics
- **`led_animation.rs`** - LED animation patterns
- **`reactive_leds.rs`** - Reactive LED responses to input

Run an example:

```bash
cargo run --example simple_test
```

## Features

### Input System

- **Buttons**: All transport, group, and function buttons with press/release events
- **Pads**: 16 velocity-sensitive pads (0-127) with pressure detection
- **Knobs**: 8 rotary encoders with delta tracking
- **Touch Strip**: Continuous position and pressure sensing
- **Change Tracking**: Only reports actual state changes, not redundant data

### LED Control

```rust
// Individual LED control
device.set_button_led(InputElement::Play, 127)?;
device.set_pad_led(0, MaschineLEDColor::red(true))?;

// Bulk operations
device.clear_all_leds()?;
```

### Display Graphics

- **Resolution**: 480x272 pixels
- **Format**: RGB565 color space
- **Performance**: Direct USB bulk transfers for minimal latency

## Architecture

### Core Components

- **`MaschineMK3`** - Main device interface and connection management
- **Input System** - HID report parsing with state tracking and change detection
- **Output System** - LED state management and display graphics
- **USB Communication** - Cross-platform USB handling with Windows HID fallback

### USB Protocol

- **Vendor ID**: 0x17CC (Native Instruments)
- **Product ID**: 0x1600 (Maschine MK3)
- **HID Interface**: #4 (Input/Output endpoint pair)
- **Display Interface**: #5 (Bulk transfer for graphics)

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

### Development Setup

1. Fork and clone the repository
2. Set up the WinUSB driver (see Requirements above)
3. Run the examples to verify your setup
4. Make your changes and test thoroughly
5. Submit a pull request

Please ensure all code is formatted (`cargo fmt`) and passes linting (`cargo clippy`) before submitting.

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
