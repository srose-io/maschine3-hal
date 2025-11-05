# Unity Integration Files

This directory contains C# scripts for integrating the Maschine MK3 HAL with Unity Engine.

## Files

### MaschineMK3Native.cs
Core P/Invoke wrapper that provides C# bindings to the native Rust library. This component:
- Manages device lifecycle (initialization, cleanup)
- Provides event-based input handling
- Exposes LED and display control methods
- Handles memory management between C# and native code

**Usage**: Attach this component to a GameObject, or add it programmatically.

### MaschineExampleController.cs
Example implementation demonstrating various features:
- Pad input with visual LED feedback
- Button input handling
- Knob/encoder input
- Display texture rendering
- Event-driven architecture

**Usage**: Attach to a GameObject to see a working demo. Customize as needed for your project.

## Quick Start

1. **Build the native library** (see [UNITY_INTEGRATION.md](../UNITY_INTEGRATION.md))
   ```bash
   cargo build --release
   ```

2. **Copy files to Unity**:
   - Copy `maschine3_hal.dll` (Windows) or `libmaschine3_hal.so` (Linux) to `Assets/Plugins/x86_64/`
   - Copy `MaschineMK3Native.cs` to `Assets/Plugins/` or `Assets/Scripts/`
   - Copy `MaschineExampleController.cs` to `Assets/Scripts/` (optional)

3. **Create a GameObject** in your scene and attach `MaschineExampleController` component

4. **Run** and interact with your Maschine MK3!

## API Overview

### Events
```csharp
maschine.OnPadHit += (padNumber, velocity) => { /* ... */ };
maschine.OnButtonPressed += (buttonId) => { /* ... */ };
maschine.OnKnobChanged += (knobId, value, delta) => { /* ... */ };
```

### LED Control
```csharp
maschine.SetPadLED(0, Color.red, bright: true);
maschine.SetButtonLED(7, Color.cyan, bright: false);
maschine.FlushLEDs(); // Send to device
```

### Display
```csharp
byte[] rgb565Data = ConvertTextureToRGB565(myTexture);
maschine.WriteDisplay(rgb565Data);
```

## Documentation

See [UNITY_INTEGRATION.md](../UNITY_INTEGRATION.md) for complete documentation including:
- Detailed setup instructions
- API reference
- Input element ID mappings
- Troubleshooting guide
- Platform-specific notes

## Examples

### Simple Pad Controller
```csharp
void Start()
{
    maschine = gameObject.AddComponent<MaschineMK3Native>();
    maschine.OnPadHit += (pad, velocity) => {
        Debug.Log($"Pad {pad} hit!");
        maschine.SetPadLED(pad, Color.green, true);
        maschine.FlushLEDs();
    };
}
```

### Knob-Controlled Parameter
```csharp
void Start()
{
    maschine = gameObject.AddComponent<MaschineMK3Native>();
    maschine.OnKnobChanged += (knob, value, delta) => {
        if (knob == 15) // Knob 1
        {
            float normalized = value / 1023f;
            audioSource.volume = normalized;
        }
    };
}
```

## Requirements

- Unity 2020.3 or later
- .NET 4.x or .NET Standard 2.0+
- Maschine MK3 hardware
- Native library built for target platform

## License

MIT or Apache-2.0 (dual-licensed, same as parent project)
