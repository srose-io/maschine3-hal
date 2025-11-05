# Dual-Screen Display Support - Update Summary

## Overview

The Maschine MK3 HAL Unity integration has been updated to support both displays independently. Each display is 480x272 pixels and can be written to separately or together.

## Changes Made

### 1. Rust Core Library (`src/device.rs`)

**Modified**: `write_display_framebuffer()` method

- Added `display_id: u8` parameter (0 = Left display, 1 = Right display)
- Added validation to ensure display_id is 0 or 1
- Updated packet header to use the provided display_id at byte offset 2

```rust
pub fn write_display_framebuffer(&self, display_id: u8, framebuffer_data: &[u8]) -> Result<()>
```

### 2. Rust FFI Layer (`src/ffi.rs`)

**Modified**: `mk3_write_display()` function

- Added `display_id: c_uint` parameter
- Added validation for display_id > 1 (returns `MK3_ERROR_INVALID_PARAMETER`)
- Passes display_id to the core library method

```rust
pub unsafe extern "C" fn mk3_write_display(
    device: *mut MaschineMK3,
    display_id: c_uint,
    rgb565_data: *const u8,
    data_len: c_uint,
) -> c_int
```

### 3. Unity C# Wrapper (`unity/MaschineMK3Native.cs`)

**Added**: `DisplayId` enum

```csharp
public enum DisplayId
{
    Left = 0,
    Right = 1
}
```

**Modified**: DllImport declaration

- Updated to include `displayId` parameter

**Added**: New `WriteDisplay()` overload

```csharp
// Write to specific display
public bool WriteDisplay(DisplayId displayId, byte[] rgb565Data)

// Write to left display (backward compatible)
public bool WriteDisplay(byte[] rgb565Data)
```

### 4. Unity Example Controller (`unity/MaschineExampleController.cs`)

**Added**: Inspector controls

```csharp
public bool updateLeftDisplay = true;
public bool updateRightDisplay = true;
```

**Updated**: `UpdateDisplay()` method

- Now writes to left and/or right display based on inspector settings
- Each display can be independently enabled/disabled

**Updated**: `FillDisplayColor()` method

- Now accepts `DisplayId` parameter
- Logs which display was updated

**Added**: New context menu test functions

- `Test: Fill Both Displays Red/Green/Blue` - Fill both displays with same color
- `Test: Fill Left Display Red` - Fill only left display
- `Test: Fill Right Display Blue` - Fill only right display
- `Test: Different Colors Each Screen` - Left red, right cyan

### 5. Unity Display Test (`unity/DisplayTest.cs`)

**Added**: Inspector controls

```csharp
public bool updateLeftDisplay = true;
public bool updateRightDisplay = true;
public bool mirrorContent = true;
public float hueOffset = 0.5f;
```

**Updated**: `Update()` method

- Writes to each display independently
- Can mirror content or show different content with hue offset

**Updated**: `GenerateGradient()` method

- Now accepts `baseHue` parameter for per-display customization

### 6. Documentation (`UNITY_INTEGRATION.md`)

**Updated**: Display Control section

- Added `DisplayId` enum documentation
- Added dual-screen API examples
- Documented both overloads of `WriteDisplay()`
- Added example code for:
  - Writing same content to both displays
  - Writing different content to each display

## Usage Examples

### Basic - Update Both Displays

```csharp
byte[] frameData = ConvertTextureToRGB565(myTexture);
maschine.WriteDisplay(MaschineMK3Native.DisplayId.Left, frameData);
maschine.WriteDisplay(MaschineMK3Native.DisplayId.Right, frameData);
```

### Advanced - Different Content Per Display

```csharp
byte[] leftFrame = CreateGradient(Color.red, Color.blue);
byte[] rightFrame = CreateGradient(Color.green, Color.yellow);
maschine.WriteDisplay(MaschineMK3Native.DisplayId.Left, leftFrame);
maschine.WriteDisplay(MaschineMK3Native.DisplayId.Right, rightFrame);
```

### Example Controller - Toggle Displays

In the Unity Inspector on the `MaschineExampleController` component:
- Check/uncheck `Update Left Display` to enable/disable left screen updates
- Check/uncheck `Update Right Display` to enable/disable right screen updates

### Display Test - Mirror or Offset

In the Unity Inspector on the `DisplayTest` component:
- `Mirror Content` = true: Both displays show identical content
- `Mirror Content` = false: Right display has hue shifted by `Hue Offset`

## Testing

Use the context menu functions in Unity:

1. Right-click the `MaschineExampleController` component in Inspector
2. Select from test options:
   - `Test: Fill Both Displays Red`
   - `Test: Fill Left Display Red`
   - `Test: Fill Right Display Blue`
   - `Test: Different Colors Each Screen`

## Backward Compatibility

The default `WriteDisplay(byte[])` method still works and writes to the left display, maintaining backward compatibility with existing code.

## Building

To rebuild the library with these changes:

```bash
# Linux (with Nix)
./build-unity-linux-bundle.sh /path/to/UnityProject

# Or just build the library
nix-shell --run "cargo build --release"
```

After building, copy the updated files to Unity:

```bash
# Copy C# files only (Unity can remain open)
./copy-cs-only.sh /path/to/UnityProject

# Or copy everything (close Unity first)
./update-unity.sh /path/to/UnityProject
```

## Protocol Details

The display ID is set in the packet header at byte offset 2:
- `0x00` = Left display
- `0x01` = Right display

All other packet format details remain unchanged (see `docs/MaschineMK3-Display.md`).
