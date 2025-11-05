# Unity Integration Changelog

## Initial Release

### Features Added

- **C FFI Layer** ([src/ffi.rs](../src/ffi.rs))
  - Complete C-compatible API for Unity P/Invoke
  - Device lifecycle management (create, free)
  - Input event polling with buffered events
  - LED control for pads and buttons
  - Display framebuffer writes (RGB565)
  - Error code based error handling

- **Unity C# Wrapper** ([MaschineMK3Native.cs](MaschineMK3Native.cs))
  - P/Invoke declarations for all native functions
  - Event-based input system (OnPadHit, OnButtonPressed, etc.)
  - Unity Color to RGB conversion utilities
  - Automatic device lifecycle management
  - Display texture conversion (RGB888 → RGB565)

- **Example Controller** ([MaschineExampleController.cs](MaschineExampleController.cs))
  - Complete working example demonstrating all features
  - Pad LED feedback with timing
  - Button and knob handling
  - Display rendering support
  - Configurable visual feedback options

- **Build Scripts**
  - Linux/macOS shell script ([build-unity.sh](../build-unity.sh))
  - Windows batch script ([build-unity.bat](../build-unity.bat))
  - Automatic copying to Unity projects

- **Documentation**
  - [UNITY_INTEGRATION.md](../UNITY_INTEGRATION.md) - Complete integration guide
  - [UNITY_QUICKSTART.md](../UNITY_QUICKSTART.md) - 5-minute quick start
  - [unity/README.md](README.md) - Unity scripts documentation
  - API reference with input element ID mappings
  - Troubleshooting guide

### API Details

#### Supported Events
- `OnPadHit(padNumber, velocity)` - Pad hit with 12-bit velocity
- `OnPadAftertouch(padNumber, pressure)` - Pad pressure/aftertouch
- `OnPadTouchRelease(padNumber)` - Release from touch without hit
- `OnPadHitRelease(padNumber)` - Release from normal hit
- `OnButtonPressed(buttonId)` - Button press
- `OnButtonReleased(buttonId)` - Button release
- `OnKnobChanged(knobId, value, delta)` - Knob/encoder change

#### Supported Controls
- 16 velocity-sensitive pads (0-15)
- 75+ buttons mapped to unique IDs
- 8 knobs (10-bit resolution)
- Main encoder (4-bit resolution)
- Audio controls (mic gain, headphone volume, master volume)
- Full RGB LED control
- 480x272 display

### Build Configuration

- Added `cdylib` crate type to [Cargo.toml](../Cargo.toml)
- Builds both static library (for Rust use) and dynamic library (for Unity)
- Release build optimizations enabled by default

### Platform Support

- **Windows**: `maschine3_hal.dll`
- **Linux**: `libmaschine3_hal.so`
- **macOS**: `libmaschine3_hal.dylib` (untested)

### Known Limitations

1. Brightness parameter in LED functions is currently ignored (uses RGB color mapping instead)
2. Display requires RGB565 format - Unity textures must be converted
3. Platform-specific USB requirements still apply (WinUSB driver on Windows, udev rules on Linux)

### Future Enhancements

Potential improvements for future versions:
- Brightness control separate from color
- Direct Unity Texture2D rendering without RGB565 conversion
- Async/coroutine-based input polling
- Unity Editor integration (custom inspectors)
- Additional example scenes (drum machine, parameter control, etc.)
