using UnityEngine;

/// <summary>
/// Example controller demonstrating various Maschine MK3 features in Unity.
/// Attach this to a GameObject to see the hardware in action.
/// </summary>
public class MaschineExampleController : MonoBehaviour
{
    private MaschineMK3Native maschine;

    [Header("Visual Feedback")]
    public bool enablePadFeedback = true;
    public bool enableButtonFeedback = true;
    public bool enableKnobFeedback = true;

    [Header("Pad Settings")]
    public Color padIdleColor = new Color(0.1f, 0.1f, 0.3f);
    public Color padHitColor = Color.red;
    public float padResetDelay = 0.2f;

    [Header("Display Test")]
    public bool enableAnimatedGradient = true;
    public float gradientSpeed = 1.0f;
    public bool updateLeftDisplay = true;
    public bool updateRightDisplay = true;

    private int lastHitPad = -1;
    private float padResetTimer = 0f;
    private Texture2D displayTexture;
    private float animationTime = 0f;

    void Start()
    {
        // Add the native wrapper component (it will auto-initialize in its Start())
        maschine = gameObject.AddComponent<MaschineMK3Native>();

        // Wait one frame for the component to initialize
        StartCoroutine(InitializeAfterFrame());
    }

    private System.Collections.IEnumerator InitializeAfterFrame()
    {
        yield return null; // Wait one frame

        // Check if initialization succeeded
        if (!maschine.IsConnected)
        {
            Debug.LogError("Failed to initialize Maschine MK3 - device not found");
            Debug.LogError("Check: 1) Device connected, 2) udev rules installed, 3) User in 'audio' group");
            enabled = false;
            yield break;
        }

        Debug.Log("Maschine MK3 initialized successfully!");

        // Check display interface availability
        if (maschine.IsDisplayAvailable)
        {
            Debug.Log("✓ Display interface available - display updates will work");
        }
        else
        {
            Debug.LogWarning("⚠ Display interface NOT available!");
            Debug.LogWarning("Display writes will fail. Check native console output for details.");
        }

        // Subscribe to all input events
        maschine.OnPadHit += HandlePadHit;
        maschine.OnPadAftertouch += HandlePadAftertouch;
        maschine.OnPadTouchRelease += HandlePadRelease;
        maschine.OnPadHitRelease += HandlePadRelease;
        maschine.OnButtonPressed += HandleButtonPressed;
        maschine.OnButtonReleased += HandleButtonReleased;
        maschine.OnKnobChanged += HandleKnobChanged;

        // Initialize pad LEDs to idle color
        InitializePadLEDs();

        // Create display texture for animated gradient (480x272 for Maschine MK3)
        displayTexture = new Texture2D(480, 272, TextureFormat.RGB24, false);

        Debug.Log("Maschine MK3 Example Controller initialized");
    }

    void Update()
    {
        // Handle pad LED reset timer
        if (lastHitPad >= 0 && padResetTimer > 0f)
        {
            padResetTimer -= Time.deltaTime;
            if (padResetTimer <= 0f)
            {
                ResetPadLED(lastHitPad);
                lastHitPad = -1;
            }
        }

        // Update animated gradient display
        if (enableAnimatedGradient && displayTexture != null && maschine != null && maschine.IsConnected)
        {
            animationTime += Time.deltaTime * gradientSpeed;
            GenerateAnimatedGradient();
            UpdateDisplay();
        }
    }

    private void InitializePadLEDs()
    {
        for (int i = 0; i < 16; i++)
        {
            maschine.SetPadLED(i, padIdleColor, false);
        }
        maschine.FlushLEDs();
    }

    // ===== Event Handlers =====

    private void HandlePadHit(int padNumber, ushort velocity)
    {
        if (!enablePadFeedback) return;

        // Velocity is 12-bit (0-4095), convert to 0-1 for logging
        float normalizedVelocity = velocity / 4095f;
        Debug.Log($"Pad {padNumber} hit - Velocity: {velocity} ({normalizedVelocity:F2})");

        // Light up the pad
        maschine.SetPadLED(padNumber, padHitColor, true);
        maschine.FlushLEDs();

        // Set timer to reset pad color
        lastHitPad = padNumber;
        padResetTimer = padResetDelay;
    }

    private void HandlePadAftertouch(int padNumber, ushort pressure)
    {
        if (!enablePadFeedback) return;

        float normalizedPressure = pressure / 4095f;
        Debug.Log($"Pad {padNumber} aftertouch - Pressure: {pressure} ({normalizedPressure:F2})");

        // Modulate color brightness based on pressure
        Color modulatedColor = Color.Lerp(padIdleColor, padHitColor, normalizedPressure);
        maschine.SetPadLED(padNumber, modulatedColor, normalizedPressure > 0.5f);
        maschine.FlushLEDs();
    }

    private void HandlePadRelease(int padNumber)
    {
        if (!enablePadFeedback) return;

        Debug.Log($"Pad {padNumber} released");
        ResetPadLED(padNumber);
    }

    private void HandleButtonPressed(int buttonId)
    {
        if (!enableButtonFeedback) return;

        Debug.Log($"Button {buttonId} pressed - {GetButtonName(buttonId)}");

        // Light up specific buttons
        if (buttonId >= 7 && buttonId <= 14) // Group buttons A-H
        {
            maschine.SetButtonLED(buttonId, Color.cyan, true);
            maschine.FlushLEDs();
        }

        // Special button actions
        switch (buttonId)
        {
            case 0: // Play
                Debug.Log("▶ Play button pressed");
                break;
            case 2: // Stop
                Debug.Log("■ Stop button pressed");
                break;
            case 1: // Rec
                Debug.Log("● Record button pressed");
                break;
            case 74: // Shift
                Debug.Log("⇧ Shift button pressed");
                break;
        }
    }

    private void HandleButtonReleased(int buttonId)
    {
        if (!enableButtonFeedback) return;

        Debug.Log($"Button {buttonId} released - {GetButtonName(buttonId)}");

        // Turn off group button LEDs on release
        if (buttonId >= 7 && buttonId <= 14)
        {
            maschine.SetButtonLED(buttonId, Color.black, false);
            maschine.FlushLEDs();
        }
    }

    private void HandleKnobChanged(int knobId, ushort value, int delta)
    {
        if (!enableKnobFeedback) return;

        // Knobs are 10-bit (0-1023), Main Encoder is 4-bit (0-15)
        float normalized = (knobId == 23) ? value / 15f : value / 1023f;
        Debug.Log($"Knob {knobId} changed - Value: {value} ({normalized:F2}), Delta: {delta}");

        // Example: Use first knob to control something in your game
        if (knobId == 15) // Knob 1
        {
            // Do something with the knob value
            // e.g., Control audio volume, camera rotation, etc.
        }
    }

    // ===== Helper Methods =====

    private void ResetPadLED(int padNumber)
    {
        maschine.SetPadLED(padNumber, padIdleColor, false);
        maschine.FlushLEDs();
    }

    private void GenerateAnimatedGradient()
    {
        // Fill entire screen with red
        for (int y = 0; y < displayTexture.height; y++)
        {
            for (int x = 0; x < displayTexture.width; x++)
            {
                displayTexture.SetPixel(x, y, Color.red);
            }
        }
        displayTexture.Apply();
    }

    private void UpdateDisplay()
    {
        if (displayTexture == null) return;

        if (!maschine.IsDisplayAvailable)
        {
            Debug.LogWarning("Display interface not available - skipping display update");
            return;
        }

        byte[] rgb565 = ConvertTextureToRGB565(displayTexture);
        if (rgb565 != null)
        {
            // Update left display if enabled
            if (updateLeftDisplay)
            {
                bool success = maschine.WriteDisplay(MaschineMK3Native.DisplayId.Left, rgb565);
                if (!success)
                {
                    Debug.LogError("Failed to write to left display!");
                }
            }

            // Update right display if enabled
            if (updateRightDisplay)
            {
                bool success = maschine.WriteDisplay(MaschineMK3Native.DisplayId.Right, rgb565);
                if (!success)
                {
                    Debug.LogError("Failed to write to right display!");
                }
            }
        }
    }

    private byte[] ConvertTextureToRGB565(Texture2D texture)
    {
        // Maschine MK3 display is 480x272
        if (texture.width != 480 || texture.height != 272)
        {
            Debug.LogError($"Display texture must be 480x272. Current: {texture.width}x{texture.height}");
            return null;
        }

        byte[] rgb565 = new byte[480 * 272 * 2];
        Color[] pixels = texture.GetPixels();

        for (int i = 0; i < pixels.Length; i++)
        {
            Color pixel = pixels[i];

            // Convert to RGB888 (0-255)
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

            // Store as little-endian
            rgb565[i * 2] = (byte)(rgb565x & 0xFF);
            rgb565[i * 2 + 1] = (byte)((rgb565x >> 8) & 0xFF);
        }

        return rgb565;
    }

    private string GetButtonName(int buttonId)
    {
        // Quick lookup for button names (partial list)
        switch (buttonId)
        {
            case 0: return "Play";
            case 1: return "Rec";
            case 2: return "Stop";
            case 3: return "Restart";
            case 4: return "Erase";
            case 5: return "Tap";
            case 6: return "Follow";
            case 7: return "Group A";
            case 8: return "Group B";
            case 9: return "Group C";
            case 10: return "Group D";
            case 11: return "Group E";
            case 12: return "Group F";
            case 13: return "Group G";
            case 14: return "Group H";
            case 15: return "Knob 1";
            case 16: return "Knob 2";
            case 17: return "Knob 3";
            case 18: return "Knob 4";
            case 19: return "Knob 5";
            case 20: return "Knob 6";
            case 21: return "Knob 7";
            case 22: return "Knob 8";
            case 23: return "Main Encoder";
            case 74: return "Shift";
            default: return $"Button {buttonId}";
        }
    }

    // ===== Diagnostic Functions =====

    [ContextMenu("Test: Fill Both Displays Red")]
    public void TestFillRed()
    {
        FillDisplayColor(MaschineMK3Native.DisplayId.Left, Color.red);
        FillDisplayColor(MaschineMK3Native.DisplayId.Right, Color.red);
    }

    [ContextMenu("Test: Fill Both Displays Green")]
    public void TestFillGreen()
    {
        FillDisplayColor(MaschineMK3Native.DisplayId.Left, Color.green);
        FillDisplayColor(MaschineMK3Native.DisplayId.Right, Color.green);
    }

    [ContextMenu("Test: Fill Both Displays Blue")]
    public void TestFillBlue()
    {
        FillDisplayColor(MaschineMK3Native.DisplayId.Left, Color.blue);
        FillDisplayColor(MaschineMK3Native.DisplayId.Right, Color.blue);
    }

    [ContextMenu("Test: Fill Left Display Red")]
    public void TestFillLeftRed()
    {
        FillDisplayColor(MaschineMK3Native.DisplayId.Left, Color.red);
    }

    [ContextMenu("Test: Fill Right Display Blue")]
    public void TestFillRightBlue()
    {
        FillDisplayColor(MaschineMK3Native.DisplayId.Right, Color.blue);
    }

    [ContextMenu("Test: Different Colors Each Screen")]
    public void TestDifferentColors()
    {
        FillDisplayColor(MaschineMK3Native.DisplayId.Left, Color.red);
        FillDisplayColor(MaschineMK3Native.DisplayId.Right, Color.cyan);
    }

    [ContextMenu("Test: Check Display Status")]
    public void CheckDisplayStatus()
    {
        if (maschine == null || !maschine.IsConnected)
        {
            Debug.LogError("Device not connected!");
            return;
        }

        Debug.Log("=== Display Status ===");
        Debug.Log($"Device Connected: {maschine.IsConnected}");
        Debug.Log($"Display Available: {maschine.IsDisplayAvailable}");

        if (maschine.IsDisplayAvailable)
        {
            Debug.Log("✓ Display interface is working!");
        }
        else
        {
            Debug.LogError("✗ Display interface is NOT available!");
            Debug.LogError("Check the terminal/console where Unity was launched for native library messages");
        }
    }

    private void FillDisplayColor(MaschineMK3Native.DisplayId displayId, Color color)
    {
        if (maschine == null || !maschine.IsConnected)
        {
            Debug.LogError("Device not connected!");
            return;
        }

        if (!maschine.IsDisplayAvailable)
        {
            Debug.LogError("Display interface not available!");
            return;
        }

        // Convert to RGB888
        byte red = (byte)(color.r * 255);
        byte green = (byte)(color.g * 255);
        byte blue = (byte)(color.b * 255);

        // Convert to Maschine's custom RGB565x format
        ushort r5 = (ushort)(red >> 3);
        ushort g3high = (ushort)(green >> 5);
        ushort glow = (ushort)((green >> 2) & 7);
        ushort b5 = (ushort)(blue >> 3);

        ushort rgb565x = (ushort)((glow << 13) | (b5 << 8) | (r5 << 3) | g3high);

        byte lsb = (byte)(rgb565x & 0xFF);
        byte msb = (byte)((rgb565x >> 8) & 0xFF);

        // Create buffer and fill with color
        byte[] buffer = new byte[480 * 272 * 2];
        for (int i = 0; i < buffer.Length; i += 2)
        {
            buffer[i] = lsb;
            buffer[i + 1] = msb;
        }

        bool success = maschine.WriteDisplay(displayId, buffer);
        string displayName = displayId == MaschineMK3Native.DisplayId.Left ? "Left" : "Right";
        Debug.Log($"Fill {displayName} Display {color}: {(success ? "SUCCESS" : "FAILED")}");
    }

    void OnDestroy()
    {
        // Cleanup is handled automatically by MaschineMK3Native component
        Debug.Log("Maschine Example Controller destroyed");
    }
}
