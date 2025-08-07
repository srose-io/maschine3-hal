# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Instructions

# If you run into a lifetime issue, don't make short sighted refactors just to get it to compile. If a clean simple solution isn't obvious, stop and ask for help!

# Don't put in workarounds just to get something to build, focus on the root cause of the issue.

# Don't use cutesey descriptions for while you are working like "divining" or "moseying", just be professional use "thinking"

# You are on a windows machine, commands like rm and ls don't work, neither do forward slashes between paths

## Project Overview

This is a Rust hardware abstraction layer (HAL) for the Native Instruments Maschine MK3 controller. It provides low-level USB HID communication with the device, handling button/pad inputs, LED outputs, and display graphics.

## Key Architecture

### Core Components

- **Device Management** (`src/device.rs`): Handles USB device connection and interface claiming. Uses `rusb` for cross-platform USB access and `hidapi` on Windows for HID communication.

- **Input System** (`src/input.rs`): Parses HID input reports for buttons, pads, knobs, and touch strip. Implements state tracking with change detection.

- **Output System** (`src/output.rs`): Manages LED states for buttons/pads and display graphics (480x272 RGB565 format).

- **Platform-Specific** (`src/ni_ipc.rs`): Windows-only IPC communication with Native Instruments services.

### USB Communication

- **Vendor ID**: 0x17CC
- **Product ID**: 0x1600
- **HID Interface**: #4 (endpoints 0x83 input, 0x03 output)
- **Display Interface**: #5 (endpoint 0x04 bulk transfer)

## Common Development Commands

### Build and Test

```bash
# Build the library
cargo build

# Build with release optimizations
cargo build --release

# Run all tests
cargo test

# Run a specific example
cargo run --example simple_test
cargo run --example input_monitor
cargo run --example color_test

# Check code without building
cargo check

# Run linter
cargo clippy

# Format code
cargo fmt
```

### Working with Examples

The `examples/` directory contains various test programs:

- `simple_test.rs` - Basic connectivity and LED test
- `input_monitor.rs` - Real-time input monitoring
- `color_test.rs` - Display color patterns
- `led_animation.rs` - LED animation patterns
- `debug_input.rs` - Detailed input debugging

## Protocol Documentation

Detailed USB protocol documentation is in `docs/`:

- `MaschineMK3-Overview.md` - USB endpoint overview
- `MaschineMK3-HIDInput.md` - Input protocol (buttons, pads, knobs)
- `MaschineMK3-HIDOutput.md` - LED output protocol
- `MaschineMK3-Display.md` - Display graphics protocol

## Windows Development Notes

On Windows, the library uses `hidapi` for HID communication since direct USB access requires driver installation. The display interface may require WinUSB driver installation via Zadig for full functionality.

## Dependencies

- `rusb` - Cross-platform USB library
- `hidapi` - HID API for Windows compatibility
- `thiserror` - Error handling
- `windows` (Windows only) - For IPC communication with NI services
