# Unity Quick Start Guide

Get your Maschine MK3 working in Unity in 5 minutes!

## Step 1: Build the Native Library

### Windows
```powershell
cargo build --release
```
Output: `target\release\maschine3_hal.dll`

### Linux
```bash
cargo build --release
```
Output: `target/release/libmaschine3_hal.so`

## Step 2: Copy to Unity

### Automatic (Recommended)
```bash
# Linux/macOS
./build-unity.sh /path/to/YourUnityProject

# Windows
build-unity.bat C:\path\to\YourUnityProject
```

### Manual
1. Copy the DLL/SO to `YourUnityProject/Assets/Plugins/x86_64/`
2. Copy `unity/MaschineMK3Native.cs` to `YourUnityProject/Assets/Plugins/`
3. Copy `unity/MaschineExampleController.cs` to `YourUnityProject/Assets/Scripts/` (optional)

## Step 3: Create a Test Scene

1. Open Unity
2. Create a new empty GameObject (GameObject → Create Empty)
3. Name it "MaschineController"
4. Add the `MaschineExampleController` component to it
5. Press Play!

## Step 4: Test It

Hit pads on your Maschine MK3 - they should light up red and you'll see console output!

## What's Next?

### Write Your Own Controller

```csharp
using UnityEngine;

public class MyMaschineController : MonoBehaviour
{
    private MaschineMK3Native maschine;

    void Start()
    {
        maschine = gameObject.AddComponent<MaschineMK3Native>();

        // Handle pad hits
        maschine.OnPadHit += (pad, velocity) => {
            Debug.Log($"Pad {pad} hit!");
            // Your game logic here
        };

        // Set pad colors
        for (int i = 0; i < 16; i++)
            maschine.SetPadLED(i, Color.blue, false);
        maschine.FlushLEDs();
    }
}
```

### Common Use Cases

**Drum Machine**
```csharp
maschine.OnPadHit += (pad, velocity) => {
    audioSource.PlayOneShot(drumSamples[pad], velocity / 4095f);
};
```

**Level Selector**
```csharp
maschine.OnPadHit += (pad, velocity) => {
    SceneManager.LoadScene($"Level{pad + 1}");
};
```

**Parameter Control**
```csharp
maschine.OnKnobChanged += (knob, value, delta) => {
    playerSpeed = value / 1023f * maxSpeed;
};
```

**Button Actions**
```csharp
maschine.OnButtonPressed += (buttonId) => {
    if (buttonId == 0) // Play button
        StartGame();
};
```

## Troubleshooting

### "DllNotFoundException"
- Make sure the DLL/SO is in `Assets/Plugins/x86_64/`
- Check the file name matches your platform
- Reimport the plugin in Unity (right-click → Reimport)

### "Device not found"
- **Windows**: Install WinUSB driver using Zadig
- **Linux**: Configure udev rules (see `LINUX_SETUP.md`)
- Check USB connection

### No events firing
- Make sure the component is attached to an active GameObject
- Check console for initialization errors
- Verify the device is connected

## Full Documentation

- [UNITY_INTEGRATION.md](UNITY_INTEGRATION.md) - Complete guide with API reference
- [unity/README.md](unity/README.md) - Unity scripts documentation
- [README.md](README.md) - Main library documentation

## Support

Open an issue on GitHub if you encounter problems!

---

That's it! Your Maschine MK3 is now integrated with Unity. Have fun building! 🎮
