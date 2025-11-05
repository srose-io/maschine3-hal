# Unity Integration Guide

This guide explains how to use the Maschine MK3 HAL library in Unity Engine projects.

## Overview

The library provides a C FFI (Foreign Function Interface) layer that Unity can call through P/Invoke. This allows you to interface with the Maschine MK3 hardware controller directly from Unity C# scripts.

## Prerequisites

- Unity 2020.3 or later (any version with .NET 4.x or .NET Standard 2.0+)
- Rust toolchain installed (for building the native library)
- Windows 10/11 or Linux (platform where you'll run Unity)
- Maschine MK3 hardware controller

### Platform-Specific Requirements

#### Windows
- WinUSB driver installed for the Maschine MK3 (see main README)
- Visual Studio Build Tools (for Rust compilation)

#### Linux
- Proper udev rules configured (see `LINUX_SETUP.md`)
- libudev-dev installed

## Step 1: Build the Native Library

### Windows

```powershell
# Build release version for maximum performance
cargo build --release

# The output will be in target\release\maschine3_hal.dll
```

### Linux

```bash
# Build release version
cargo build --release

# The output will be in target/release/libmaschine3_hal.so
```

The built library will be:
- Windows: `target/release/maschine3_hal.dll`
- Linux: `target/release/libmaschine3_hal.so`
- macOS: `target/release/libmaschine3_hal.dylib` (untested)

## Step 2: Copy Native Library to Unity

Copy the built native library to your Unity project's `Assets/Plugins` directory:

### Unity Project Structure
```
YourUnityProject/
├── Assets/
│   ├── Plugins/
│   │   ├── x86_64/                    # 64-bit plugins
│   │   │   ├── maschine3_hal.dll      # Windows
│   │   │   └── libmaschine3_hal.so    # Linux
│   │   └── MaschineMK3Native.cs       # C# wrapper script
│   └── Scripts/
│       └── MaschineController.cs      # Your game logic
```

### Plugin Import Settings in Unity

1. Select the `.dll` or `.so` file in Unity's Project window
2. In the Inspector, configure:
   - **Windows DLL**:
     - Platform: Windows x86_64
     - Load on startup: ✓
   - **Linux SO**:
     - Platform: Linux x86_64
     - Load on startup: ✓

## Step 3: Add C# Wrapper to Unity

Copy the provided C# wrapper script to your Unity project:

```
unity/MaschineMK3Native.cs → Assets/Plugins/MaschineMK3Native.cs
```

This script provides:
- Native function P/Invoke declarations
- Unity-friendly C# wrapper methods
- Event-based input handling
- Automatic lifecycle management

## Step 4: Create Your Controller Script

Here's an example Unity script that uses the Maschine MK3:

```csharp
using UnityEngine;

public class MaschineController : MonoBehaviour
{
    private MaschineMK3Native maschine;

    void Start()
    {
        // Get the native wrapper component
        maschine = gameObject.AddComponent<MaschineMK3Native>();

        // Subscribe to input events
        maschine.OnPadHit += HandlePadHit;
        maschine.OnButtonPressed += HandleButtonPressed;
        maschine.OnKnobChanged += HandleKnobChanged;

        // Set initial LED colors
        for (int i = 0; i < 16; i++)
        {
            maschine.SetPadLED(i, Color.blue, true);
        }
        maschine.FlushLEDs();
    }

    void HandlePadHit(int padNumber, ushort velocity)
    {
        Debug.Log($"Pad {padNumber} hit with velocity {velocity}");

        // Light up the pad
        maschine.SetPadLED(padNumber, Color.red, true);
        maschine.FlushLEDs();
    }

    void HandleButtonPressed(int buttonId)
    {
        Debug.Log($"Button {buttonId} pressed");

        // Example: Button 0 is Play button
        if (buttonId == 0)
        {
            Debug.Log("Play button pressed!");
        }
    }

    void HandleKnobChanged(int knobId, ushort value, int delta)
    {
        Debug.Log($"Knob {knobId} changed to {value} (delta: {delta})");
    }
}
```

## API Reference

### Initialization

The `MaschineMK3Native` component automatically initializes on `Start()` and cleans up on `OnDestroy()`.

```csharp
bool Initialize()
```

Returns `true` if the device was found and initialized successfully.

### Input Events

Subscribe to these events to handle input:

```csharp
event Action<int, ushort> OnPadHit;           // padNumber, velocity
event Action<int, ushort> OnPadAftertouch;    // padNumber, pressure
event Action<int> OnPadRelease;               // padNumber
event Action<int> OnButtonPressed;            // buttonId
event Action<int> OnButtonReleased;           // buttonId
event Action<int, ushort, int> OnKnobChanged; // knobId, value, delta
```

### LED Control

```csharp
bool SetPadLED(int padNumber, Color color, bool bright = true)
bool SetButtonLED(int buttonId, Color color, bool bright = true)
bool FlushLEDs()  // Must call this to send LED updates to device
```

**Important**: LED changes are buffered. You must call `FlushLEDs()` to send them to the device.

### Display Control

The Maschine MK3 has **two independent displays**, each 480x272 pixels. You can write to them separately or together.

```csharp
// Display ID enum
public enum DisplayId
{
    Left = 0,
    Right = 1
}

// Write to a specific display
bool WriteDisplay(DisplayId displayId, byte[] rgb565Data)

// Write to left display (default)
bool WriteDisplay(byte[] rgb565Data)
```

Each display requires a 480x272 RGB565 framebuffer. The byte array must be exactly 261,120 bytes (480 × 272 × 2).

#### Dual-Screen Example

```csharp
// Update both screens with the same content
byte[] frameData = ConvertTextureToRGB565(myTexture);
maschine.WriteDisplay(MaschineMK3Native.DisplayId.Left, frameData);
maschine.WriteDisplay(MaschineMK3Native.DisplayId.Right, frameData);

// Or different content on each screen
byte[] leftFrame = CreateGradient(Color.red, Color.blue);
byte[] rightFrame = CreateGradient(Color.green, Color.yellow);
maschine.WriteDisplay(MaschineMK3Native.DisplayId.Left, leftFrame);
maschine.WriteDisplay(MaschineMK3Native.DisplayId.Right, rightFrame);
```

#### Converting Unity Texture to RGB565

```csharp
public byte[] ConvertTextureToRGB565(Texture2D texture)
{
    if (texture.width != 480 || texture.height != 272)
    {
        Debug.LogError("Texture must be 480x272");
        return null;
    }

    byte[] rgb565 = new byte[480 * 272 * 2];
    Color[] pixels = texture.GetPixels();

    for (int i = 0; i < pixels.Length; i++)
    {
        Color pixel = pixels[i];

        // Convert to RGB888
        byte red = (byte)(pixel.r * 255);
        byte green = (byte)(pixel.g * 255);
        byte blue = (byte)(pixel.b * 255);

        // Convert to Maschine's custom RGB565x format
        // Pack as: GGGB BBBB RRRR RGGG
        ushort r5 = (ushort)(red >> 3);
        ushort g3high = (ushort)(green >> 5);
        ushort glow = (ushort)((green >> 2) & 7);
        ushort b5 = (ushort)(blue >> 3);

        ushort rgb565x = (ushort)((glow << 13) | (b5 << 8) | (r5 << 3) | g3high);

        // Write as little-endian
        rgb565[i * 2] = (byte)(rgb565x & 0xFF);
        rgb565[i * 2 + 1] = (byte)((rgb565x >> 8) & 0xFF);
    }

    return rgb565;
}
```

## Input Element IDs

### Buttons (0-75)

| ID Range | Buttons |
|----------|---------|
| 0-6 | Transport: Play, Rec, Stop, Restart, Erase, Tap, Follow |
| 7-14 | Groups: A, B, C, D, E, F, G, H |
| 15-23 | Knobs: 1-8, Main Encoder |
| 24-26 | Audio: Mic Gain, Headphone Volume, Master Volume |
| 27-40 | Mode: Notes, Volume, Swing, Tempo, Note Repeat, Lock, Pad Mode, Keyboard, Chords, Step, Fixed Vel, Scene, Pattern, Events |
| 41-48 | Navigation: Variation, Duplicate, Select, Solo, Mute, Pitch, Mod, Perform |
| 49-56 | Display Buttons: 1-8 |
| 57-68 | System: Channel/MIDI, Arranger, Browser/Plugin, Arrow Left/Right, File/Save, Settings, Macro, Plugin, Mixer, Sampling, Auto |
| 69-73 | Encoder: Push, Up, Down, Left, Right |
| 74 | Shift |

### Pads (0-15)

Pads are numbered 0-15, corresponding to the 4x4 pad grid.

## Example: Simple Drum Sampler

```csharp
using UnityEngine;

public class DrumSampler : MonoBehaviour
{
    private MaschineMK3Native maschine;
    public AudioClip[] drumSamples = new AudioClip[16]; // Assign in Inspector
    private AudioSource audioSource;

    void Start()
    {
        maschine = gameObject.AddComponent<MaschineMK3Native>();
        audioSource = gameObject.AddComponent<AudioSource>();

        maschine.OnPadHit += PlayDrum;

        // Set pad colors to indicate loaded samples
        for (int i = 0; i < 16; i++)
        {
            Color padColor = drumSamples[i] != null ? Color.green : Color.gray;
            maschine.SetPadLED(i, padColor, false);
        }
        maschine.FlushLEDs();
    }

    void PlayDrum(int padNumber, ushort velocity)
    {
        if (drumSamples[padNumber] != null)
        {
            // Convert 12-bit velocity (0-4095) to volume (0-1)
            float volume = velocity / 4095f;
            audioSource.PlayOneShot(drumSamples[padNumber], volume);

            // Flash pad LED
            maschine.SetPadLED(padNumber, Color.red, true);
            maschine.FlushLEDs();
            Invoke(nameof(ResetPadColor), 0.1f);
        }
    }

    void ResetPadColor()
    {
        for (int i = 0; i < 16; i++)
        {
            Color padColor = drumSamples[i] != null ? Color.green : Color.gray;
            maschine.SetPadLED(i, padColor, false);
        }
        maschine.FlushLEDs();
    }
}
```

## Troubleshooting

### "DllNotFoundException: maschine3_hal"

**Cause**: Unity can't find the native library.

**Solutions**:
1. Ensure the `.dll`/`.so` is in `Assets/Plugins/x86_64/`
2. Check the library name matches the platform:
   - Windows: `maschine3_hal.dll`
   - Linux: `libmaschine3_hal.so`
3. Verify plugin import settings in Unity Inspector
4. Try reimporting the plugin (right-click → Reimport)

### Device Not Found

**Cause**: USB device not accessible or driver issues.

**Solutions**:
- Windows: Install WinUSB driver using Zadig
- Linux: Configure udev rules (see `LINUX_SETUP.md`)
- Check USB cable connection
- Try different USB port (USB 3.0 recommended)

### Build Errors on Windows

**Cause**: Rust can't find Windows SDK or build tools.

**Solutions**:
1. Install Visual Studio Build Tools
2. Install Windows 10 SDK
3. Run build from "x64 Native Tools Command Prompt for VS"

### Performance Issues

**Tips**:
- Build the native library with `--release` flag
- Don't call `FlushLEDs()` too frequently (max 60 Hz)
- Pool input events instead of processing immediately
- Use Unity's Job System for heavy processing

## Platform-Specific Notes

### Windows

- The library uses HID API for compatibility without driver signing
- Display interface may require WinUSB driver (see main README)
- USB permissions are automatic with proper drivers

### Linux

- Requires udev rules for non-root access
- Direct USB access provides better performance
- Some distributions may need additional permissions

### Editor vs Build

- The native library works in both Unity Editor and builds
- For builds, ensure plugins are included in build settings
- Test thoroughly in standalone builds before distribution

## Additional Resources

- [Main README](README.md) - Library overview and setup
- [Protocol Documentation](docs/) - USB protocol details
- [Examples](examples/) - Rust usage examples
- [Linux Setup](LINUX_SETUP.md) - Linux-specific configuration

## License

This library is dual-licensed under MIT or Apache-2.0, same as the main library.
