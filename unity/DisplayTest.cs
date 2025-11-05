using UnityEngine;

/// <summary>
/// Simple display test - fills screens with animated gradient
/// Attach to the same GameObject as MaschineExampleController
/// </summary>
public class DisplayTest : MonoBehaviour
{
    [Header("Display Settings")]
    public bool updateLeftDisplay = true;
    public bool updateRightDisplay = true;
    public bool mirrorContent = true; // Same content on both displays
    public float hueOffset = 0.5f; // Hue offset for right display when not mirrored

    private MaschineMK3Native maschine;
    private byte[] displayBuffer;
    private float hueShift = 0f;

    void Start()
    {
        // Get reference to the MaschineMK3Native component
        maschine = GetComponent<MaschineMK3Native>();

        if (maschine == null)
        {
            Debug.LogError("DisplayTest requires MaschineMK3Native component!");
            enabled = false;
            return;
        }

        // Allocate display buffer (480x272 pixels, RGB565 = 2 bytes per pixel)
        displayBuffer = new byte[480 * 272 * 2];

        Debug.Log("DisplayTest initialized - will update display every frame");
    }

    void Update()
    {
        if (!maschine.IsConnected)
            return;

        // Update left display
        if (updateLeftDisplay)
        {
            GenerateGradient(hueShift);
            bool success = maschine.WriteDisplay(MaschineMK3Native.DisplayId.Left, displayBuffer);
            if (!success)
            {
                Debug.LogWarning("Left display write failed!");
            }
        }

        // Update right display
        if (updateRightDisplay)
        {
            float rightHue = mirrorContent ? hueShift : (hueShift + hueOffset) % 1f;
            GenerateGradient(rightHue);
            bool success = maschine.WriteDisplay(MaschineMK3Native.DisplayId.Right, displayBuffer);
            if (!success)
            {
                Debug.LogWarning("Right display write failed!");
            }
        }

        // Animate hue
        hueShift += Time.deltaTime * 0.1f;
        if (hueShift > 1f) hueShift = 0f;
    }

    private void GenerateGradient(float baseHue)
    {
        int pixelIndex = 0;

        for (int y = 0; y < 272; y++)
        {
            for (int x = 0; x < 480; x++)
            {
                // Create horizontal gradient with animated hue
                float t = (x / 480f + baseHue) % 1f;
                Color color = Color.HSVToRGB(t, 1f, 1f);

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

                // Write as little-endian (LSB first)
                displayBuffer[pixelIndex++] = (byte)(rgb565x & 0xFF);
                displayBuffer[pixelIndex++] = (byte)((rgb565x >> 8) & 0xFF);
            }
        }
    }

    // Diagnostic: Fill screen with solid color to test display
    [ContextMenu("Test: Fill Red")]
    public void FillRed()
    {
        FillColor(Color.red);
    }

    [ContextMenu("Test: Fill Green")]
    public void FillGreen()
    {
        FillColor(Color.green);
    }

    [ContextMenu("Test: Fill Blue")]
    public void FillBlue()
    {
        FillColor(Color.blue);
    }

    [ContextMenu("Test: Fill White")]
    public void FillWhite()
    {
        FillColor(Color.white);
    }

    private void FillColor(Color color)
    {
        if (!maschine.IsConnected)
        {
            Debug.LogError("Device not connected!");
            return;
        }

        // Convert to RGB565
        ushort r = (ushort)((int)(color.r * 31) & 0x1F);
        ushort g = (ushort)((int)(color.g * 63) & 0x3F);
        ushort b = (ushort)((int)(color.b * 31) & 0x1F);
        ushort rgb565 = (ushort)((r << 11) | (g << 5) | b);

        byte lsb = (byte)(rgb565 & 0xFF);
        byte msb = (byte)((rgb565 >> 8) & 0xFF);

        // Fill entire buffer
        for (int i = 0; i < displayBuffer.Length; i += 2)
        {
            displayBuffer[i] = lsb;
            displayBuffer[i + 1] = msb;
        }

        bool success = maschine.WriteDisplay(displayBuffer);
        Debug.Log($"Fill {color}: {(success ? "SUCCESS" : "FAILED")}");
    }
}
