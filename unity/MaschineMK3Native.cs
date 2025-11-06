using System;
using System.Runtime.InteropServices;
using UnityEngine;

/// <summary>
/// Unity wrapper for the Maschine MK3 HAL native library.
/// Provides C# bindings for the Rust FFI interface.
/// </summary>
public class MaschineMK3Native : MonoBehaviour
{
    // Library name varies by platform
    // Note: Unity automatically adds "lib" prefix and ".so"/".dll" extension
#if UNITY_EDITOR_WIN || UNITY_STANDALONE_WIN
    private const string DLL_NAME = "maschine3_hal";
#elif UNITY_EDITOR_OSX || UNITY_STANDALONE_OSX
    private const string DLL_NAME = "maschine3_hal";
#elif UNITY_EDITOR_LINUX || UNITY_STANDALONE_LINUX
    private const string DLL_NAME = "maschine3_hal";
#else
    private const string DLL_NAME = "maschine3_hal";
#endif

    // Error codes
    public const int MK3_SUCCESS = 0;
    public const int MK3_ERROR_NULL_POINTER = -1;
    public const int MK3_ERROR_DEVICE_NOT_FOUND = -2;
    public const int MK3_ERROR_USB_ERROR = -3;
    public const int MK3_ERROR_TIMEOUT = -4;
    public const int MK3_ERROR_COMMUNICATION = -5;
    public const int MK3_ERROR_INVALID_PARAMETER = -6;
    public const int MK3_ERROR_NO_EVENTS = -7;

    // Event types
    public enum EventType
    {
        ButtonPressed = 0,
        ButtonReleased = 1,
        ButtonHeld = 2,
        KnobChanged = 3,
        AudioChanged = 4,
        PadEvent = 5,
    }

    // Pad event types
    public enum PadEventType
    {
        Hit = 0,
        TouchRelease = 1,
        HitRelease = 2,
        Aftertouch = 3,
    }

    // C-compatible input event structure
    [StructLayout(LayoutKind.Sequential)]
    public struct InputEvent
    {
        public EventType eventType;
        public int elementId;
        public ushort value;
        public int delta;
        public PadEventType padEventType;
    }

    // C-compatible RGB color structure
    [StructLayout(LayoutKind.Sequential)]
    public struct RgbColor
    {
        public byte r;
        public byte g;
        public byte b;

        public RgbColor(byte r, byte g, byte b)
        {
            this.r = r;
            this.g = g;
            this.b = b;
        }

        public static RgbColor FromUnityColor(Color color)
        {
            return new RgbColor(
                (byte)(color.r * 255),
                (byte)(color.g * 255),
                (byte)(color.b * 255)
            );
        }
    }

    // Native function imports
    [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr mk3_new();

    [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
    private static extern void mk3_free(IntPtr device);

    [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
    private static extern int mk3_poll_events(
        IntPtr device,
        [Out] InputEvent[] eventsOut,
        uint maxEvents,
        out uint eventsRead
    );

    [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
    private static extern int mk3_poll_events_fast(
        IntPtr device,
        [Out] InputEvent[] eventsOut,
        uint maxEvents,
        out uint eventsRead
    );

    [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
    private static extern int mk3_set_pad_led(
        IntPtr device,
        byte padNumber,
        RgbColor color,
        int bright
    );

    [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
    private static extern int mk3_set_button_led(
        IntPtr device,
        int buttonId,
        RgbColor color,
        int bright
    );

    [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
    private static extern int mk3_write_display(
        IntPtr device,
        uint displayId,
        byte[] rgb565Data,
        uint dataLen
    );

    [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
    private static extern int mk3_flush_leds(IntPtr device);

    [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
    private static extern int mk3_is_display_available(IntPtr device);

    [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
    private static extern int mk3_write_display_rgb888_dirty(
        IntPtr device,
        uint displayId,
        byte[] rgb888Data,
        uint dataLen
    );

    // Instance variables
    private IntPtr deviceHandle = IntPtr.Zero;
    private InputEvent[] eventBuffer = new InputEvent[64];
    private const int MAX_EVENTS_PER_POLL = 64;

    /// <summary>
    /// Check if the device is connected and initialized
    /// </summary>
    public bool IsConnected => deviceHandle != IntPtr.Zero;
    
    /// <summary>
    /// Check if the display interface is available for writing
    /// </summary>
    public bool IsDisplayAvailable
    {
        get
        {
            if (deviceHandle == IntPtr.Zero) return false;
            int result = mk3_is_display_available(deviceHandle);
            return result == 1;
        }
    }

    // Unity events for input
    public event Action<int, ushort> OnPadHit;
    public event Action<int, ushort> OnPadAftertouch;
    public event Action<int> OnPadTouchRelease;
    public event Action<int> OnPadHitRelease;
    public event Action<int> OnButtonPressed;
    public event Action<int> OnButtonReleased;
    public event Action<int, ushort, int> OnKnobChanged;

    /// <summary>
    /// Initialize and connect to the Maschine MK3 device
    /// </summary>
    public bool Initialize()
    {
        if (deviceHandle != IntPtr.Zero)
        {
            Debug.LogWarning("Device already initialized");
            return true;
        }

        deviceHandle = mk3_new();
        if (deviceHandle == IntPtr.Zero)
        {
            Debug.LogError("Failed to connect to Maschine MK3 device");
            return false;
        }

        Debug.Log("Successfully connected to Maschine MK3");
        
        // Check display interface availability
        if (IsDisplayAvailable)
        {
            Debug.Log("✓ Display interface available");
        }
        else
        {
            Debug.LogWarning("⚠ Display interface NOT available - display writes will fail!");
            Debug.LogWarning("Check Unity console for \"Could not claim display interface\" message");
        }
        return true;
    }

    /// <summary>
    /// Poll for input events from the device (fast mode with 1ms timeout)
    /// Recommended for Unity Update() loops
    /// </summary>
    public void PollEvents()
    {
        if (deviceHandle == IntPtr.Zero)
            return;

        uint eventsRead;
        // Use fast polling with 1ms timeout for Unity performance
        int result = mk3_poll_events_fast(deviceHandle, eventBuffer, MAX_EVENTS_PER_POLL, out eventsRead);

        if (result == MK3_ERROR_NO_EVENTS)
            return;

        if (result != MK3_SUCCESS)
        {
            Debug.LogError($"Error polling events: {result}");
            return;
        }

        // Process events
        for (int i = 0; i < eventsRead; i++)
        {
            ProcessEvent(eventBuffer[i]);
        }
    }

    private void ProcessEvent(InputEvent evt)
    {
        switch (evt.eventType)
        {
            case EventType.ButtonPressed:
                OnButtonPressed?.Invoke(evt.elementId);
                break;

            case EventType.ButtonReleased:
                OnButtonReleased?.Invoke(evt.elementId);
                break;

            case EventType.KnobChanged:
                OnKnobChanged?.Invoke(evt.elementId, evt.value, evt.delta);
                break;

            case EventType.PadEvent:
                switch (evt.padEventType)
                {
                    case PadEventType.Hit:
                        OnPadHit?.Invoke(evt.elementId, evt.value);
                        break;
                    case PadEventType.Aftertouch:
                        OnPadAftertouch?.Invoke(evt.elementId, evt.value);
                        break;
                    case PadEventType.TouchRelease:
                        OnPadTouchRelease?.Invoke(evt.elementId);
                        break;
                    case PadEventType.HitRelease:
                        OnPadHitRelease?.Invoke(evt.elementId);
                        break;
                }
                break;
        }
    }

    /// <summary>
    /// Set a pad LED color
    /// </summary>
    public bool SetPadLED(int padNumber, Color color, bool bright = true)
    {
        if (deviceHandle == IntPtr.Zero || padNumber < 0 || padNumber >= 16)
            return false;

        RgbColor rgb = RgbColor.FromUnityColor(color);
        int result = mk3_set_pad_led(deviceHandle, (byte)padNumber, rgb, bright ? 1 : 0);

        if (result != MK3_SUCCESS)
        {
            Debug.LogError($"Error setting pad LED: {result}");
            return false;
        }

        return true;
    }

    /// <summary>
    /// Set a button LED color
    /// </summary>
    public bool SetButtonLED(int buttonId, Color color, bool bright = true)
    {
        if (deviceHandle == IntPtr.Zero)
            return false;

        RgbColor rgb = RgbColor.FromUnityColor(color);
        int result = mk3_set_button_led(deviceHandle, buttonId, rgb, bright ? 1 : 0);

        if (result != MK3_SUCCESS)
        {
            Debug.LogError($"Error setting button LED: {result}");
            return false;
        }

        return true;
    }

    /// <summary>
    /// Flush pending LED state changes to the device
    /// </summary>
    public bool FlushLEDs()
    {
        if (deviceHandle == IntPtr.Zero)
            return false;

        int result = mk3_flush_leds(deviceHandle);
        if (result != MK3_SUCCESS)
        {
            Debug.LogError($"Error flushing LEDs: {result}");
            return false;
        }

        return true;
    }

    /// <summary>
    /// Display identifiers for the Maschine MK3
    /// </summary>
    public enum DisplayId
    {
        Left = 0,
        Right = 1
    }

    /// <summary>
    /// Write RGB565 framebuffer to a specific display (480x272 pixels)
    /// </summary>
    /// <param name="displayId">Which display to write to (Left or Right)</param>
    /// <param name="rgb565Data">RGB565 pixel data (261,120 bytes)</param>
    public bool WriteDisplay(DisplayId displayId, byte[] rgb565Data)
    {
        if (deviceHandle == IntPtr.Zero)
            return false;

        const int expectedSize = 480 * 272 * 2; // RGB565 = 2 bytes per pixel
        if (rgb565Data.Length != expectedSize)
        {
            Debug.LogError($"Invalid display data size. Expected {expectedSize}, got {rgb565Data.Length}");
            return false;
        }

        int result = mk3_write_display(deviceHandle, (uint)displayId, rgb565Data, (uint)rgb565Data.Length);
        if (result != MK3_SUCCESS)
        {
            Debug.LogError($"Error writing display: {result}");
            return false;
        }

        return true;
    }

    /// <summary>
    /// Write RGB565 framebuffer to the left display (default behavior)
    /// </summary>
    public bool WriteDisplay(byte[] rgb565Data)
    {
        return WriteDisplay(DisplayId.Left, rgb565Data);
    }

    /// <summary>
    /// Write RGB888 framebuffer with automatic dirty region detection
    /// Only sends changed pixels for better performance on incremental updates
    /// </summary>
    /// <param name="displayId">Which display to write to (Left or Right)</param>
    /// <param name="rgb888Data">RGB888 pixel data (391,680 bytes)</param>
    /// <returns>True if successful</returns>
    public bool WriteDisplayDirty(DisplayId displayId, byte[] rgb888Data)
    {
        if (deviceHandle == IntPtr.Zero)
            return false;

        const int expectedSize = 480 * 272 * 3; // RGB888 = 3 bytes per pixel
        if (rgb888Data.Length != expectedSize)
        {
            Debug.LogError($"Invalid display data size. Expected {expectedSize}, got {rgb888Data.Length}");
            return false;
        }

        int result = mk3_write_display_rgb888_dirty(deviceHandle, (uint)displayId, rgb888Data, (uint)rgb888Data.Length);
        if (result != MK3_SUCCESS)
        {
            Debug.LogError($"Error writing display with dirty detection: {result}");
            return false;
        }

        return true;
    }

    /// <summary>
    /// Write RGB888 framebuffer to the left display with dirty detection (default behavior)
    /// </summary>
    public bool WriteDisplayDirty(byte[] rgb888Data)
    {
        return WriteDisplayDirty(DisplayId.Left, rgb888Data);
    }

    // Unity lifecycle methods
    void Start()
    {
        Initialize();
    }

    void Update()
    {
        // Only poll if device is connected
        if (deviceHandle != IntPtr.Zero)
        {
            PollEvents();
        }
    }

    void OnDestroy()
    {
        if (deviceHandle != IntPtr.Zero)
        {
            mk3_free(deviceHandle);
            deviceHandle = IntPtr.Zero;
            Debug.Log("Maschine MK3 device disconnected");
        }
    }

    void OnApplicationQuit()
    {
        OnDestroy();
    }
}
