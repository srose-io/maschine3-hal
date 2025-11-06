using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Threading;
using UnityEngine;

namespace Cradle.Engine
{
    /// <summary>
    /// Background thread service for Maschine MK3 hardware I/O
    /// Moves polling and output processing off Unity's main thread for better performance
    /// </summary>
    public class MaschineMK3HardwareService : MonoBehaviour
    {
        // Native library imports
        private const string DLL_NAME = "maschine3_hal";

        [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr mk3_new();

        [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
        private static extern void mk3_free(IntPtr device);

        [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
        private static extern int mk3_poll_events_fast(
            IntPtr device,
            [Out] MaschineMK3Native.InputEvent[] eventsOut,
            uint maxEvents,
            out uint eventsRead
        );

        [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
        private static extern int mk3_set_pad_led(
            IntPtr device,
            byte padNumber,
            MaschineMK3Native.RgbColor color,
            int bright
        );

        [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
        private static extern int mk3_set_button_led(
            IntPtr device,
            int buttonId,
            MaschineMK3Native.RgbColor color,
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
        private static extern int mk3_write_display_rgb888(
            IntPtr device,
            uint displayId,
            byte[] rgb888Data,
            uint dataLen
        );

        [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
        private static extern int mk3_write_display_region_rgb888(
            IntPtr device,
            uint displayId,
            uint x,
            uint y,
            uint width,
            uint height,
            byte[] rgb888Data,
            uint dataLen
        );

        [DllImport(DLL_NAME, CallingConvention = CallingConvention.Cdecl)]
        private static extern int mk3_write_display_rgb888_dirty(
            IntPtr device,
            uint displayId,
            byte[] rgb888Data,
            uint dataLen
        );

        // Constants
        private const int MK3_SUCCESS = 0;
        private const int MK3_ERROR_NO_EVENTS = -7;
        private const int MAX_EVENTS_PER_POLL = 64;

        // Thread-safe queues for bidirectional communication
        private ThreadSafeQueue<MaschineMK3InputEvent> _inputQueue;
        private ThreadSafeQueue<MaschineMK3OutputCommand> _ledQueue; // For LEDs (small, need all)

        // Double-buffered display writes (large, only need latest)
        // RGB888 buffers (3 bytes per pixel: 480 * 272 * 3 = 391,680 bytes each)
        private byte[] _pendingLeftDisplay;
        private byte[] _pendingRightDisplay;
        private byte[] _sendingLeftDisplay;
        private byte[] _sendingRightDisplay;
        private volatile bool _hasLeftDisplayUpdate;
        private volatile bool _hasRightDisplayUpdate;
        private volatile bool _useLeftDirtyDetection;
        private volatile bool _useRightDirtyDetection;
        private readonly object _leftDisplayLock = new object();
        private readonly object _rightDisplayLock = new object();

        // Native device handle
        private IntPtr _deviceHandle = IntPtr.Zero;
        private MaschineMK3Native.InputEvent[] _eventBuffer;

        // Background threads
        private Thread _inputThread;
        private Thread _outputThread;
        private volatile bool _running;
        private readonly object _usbLock = new object(); // Protect USB access
        private System.Diagnostics.Stopwatch _timeSource; // Thread-safe time source

        // Performance tracking
        private int _eventsProcessedThisFrame;
        private int _commandsQueuedThisFrame;

        [Header("Threading Settings")]
        [Tooltip("Polling interval in milliseconds (1-10ms recommended)")]
        public int pollingIntervalMs = 1;

        [Tooltip("Enable debug logging for threading diagnostics")]
        public bool enableDebugLogging = false;

        // Public events (fired on main thread after processing queue)
        public event Action<int, ushort> OnPadHit;
        public event Action<int, ushort> OnPadAftertouch;
        public event Action<int> OnPadTouchRelease;
        public event Action<int> OnPadHitRelease;
        public event Action<int> OnButtonPressed;
        public event Action<int> OnButtonReleased;
        public event Action<int, ushort, int> OnKnobChanged;

        // Status properties
        public bool IsConnected => _deviceHandle != IntPtr.Zero;
        public bool IsDisplayAvailable => _deviceHandle != IntPtr.Zero && mk3_is_display_available(_deviceHandle) == 1;
        public int EventsProcessedThisFrame => _eventsProcessedThisFrame;
        public int CommandsQueuedThisFrame => _commandsQueuedThisFrame;
        public int PendingLedCommands => _ledQueue.ApproximateCount;
        public bool HasPendingDisplayUpdate => _hasLeftDisplayUpdate || _hasRightDisplayUpdate;

        void Awake()
        {
            _inputQueue = new ThreadSafeQueue<MaschineMK3InputEvent>();
            _ledQueue = new ThreadSafeQueue<MaschineMK3OutputCommand>();
            _eventBuffer = new MaschineMK3Native.InputEvent[MAX_EVENTS_PER_POLL];

            // Pre-allocate double-buffered display buffers (480x272 * 3 bytes RGB888 = 391,680 bytes each)
            _pendingLeftDisplay = new byte[480 * 272 * 3];
            _pendingRightDisplay = new byte[480 * 272 * 3];
            _sendingLeftDisplay = new byte[480 * 272 * 3];
            _sendingRightDisplay = new byte[480 * 272 * 3];
        }

        void Start()
        {
            // Initialize native device
            _deviceHandle = mk3_new();

            if (_deviceHandle == IntPtr.Zero)
            {
                Debug.LogError("MaschineMK3HardwareService: Failed to initialize - device not connected");
                enabled = false;
                return;
            }

            bool displayAvailable = mk3_is_display_available(_deviceHandle) == 1;
            Debug.Log($"MaschineMK3HardwareService: Device connected (Display: {(displayAvailable ? "Available" : "Unavailable")})");

            // Initialize thread-safe time source
            _timeSource = System.Diagnostics.Stopwatch.StartNew();

            // Start background threads (separate input and output for better responsiveness)
            _running = true;

            _inputThread = new Thread(InputThreadLoop)
            {
                Name = "Maschine MK3 Input",
                IsBackground = true,
                Priority = System.Threading.ThreadPriority.AboveNormal // Input is high priority
            };
            _inputThread.Start();

            _outputThread = new Thread(OutputThreadLoop)
            {
                Name = "Maschine MK3 Output",
                IsBackground = true,
                Priority = System.Threading.ThreadPriority.Normal
            };
            _outputThread.Start();

            Debug.Log($"MaschineMK3HardwareService: Background threads started (interval: {pollingIntervalMs}ms)");
        }

        void Update()
        {
            // Main thread: Process queued input events
            _eventsProcessedThisFrame = 0;
            _commandsQueuedThisFrame = 0;

            ProcessInputQueue();
        }

        void OnDestroy()
        {
            // Stop background threads
            _running = false;

            if (_inputThread != null && _inputThread.IsAlive)
            {
                if (!_inputThread.Join(1000))
                {
                    Debug.LogWarning("MaschineMK3HardwareService: Input thread did not terminate cleanly");
                }
            }

            if (_outputThread != null && _outputThread.IsAlive)
            {
                if (!_outputThread.Join(1000))
                {
                    Debug.LogWarning("MaschineMK3HardwareService: Output thread did not terminate cleanly");
                }
            }

            // Free native device
            if (_deviceHandle != IntPtr.Zero)
            {
                mk3_free(_deviceHandle);
                _deviceHandle = IntPtr.Zero;
            }

            if (enableDebugLogging)
            {
                Debug.Log("MaschineMK3HardwareService: Background thread stopped, device freed");
            }
        }

        // Input thread: High-frequency polling for responsive input
        private void InputThreadLoop()
        {
            try
            {
                while (_running)
                {
                    if (_deviceHandle != IntPtr.Zero)
                    {
                        lock (_usbLock)
                        {
                            PollInputEvents();
                        }
                    }

                    Thread.Sleep(pollingIntervalMs); // Fast polling (1ms)
                }
            }
            catch (Exception ex)
            {
                Debug.LogError($"[MaschineMK3HardwareService] Input thread exception: {ex}");
            }
        }

        // Output thread: Lower-frequency processing for LED/display updates
        private void OutputThreadLoop()
        {
            try
            {
                while (_running)
                {
                    if (_deviceHandle != IntPtr.Zero)
                    {
                        lock (_usbLock)
                        {
                            ProcessOutputCommands();
                        }
                    }

                    Thread.Sleep(1); // Process output every 1ms (reduced from 5ms for lower latency)
                }
            }
            catch (Exception ex)
            {
                Debug.LogError($"[MaschineMK3HardwareService] Output thread exception: {ex}");
            }
        }

        // Called on background thread
        private void PollInputEvents()
        {
            uint eventsRead;
            int result = mk3_poll_events_fast(_deviceHandle, _eventBuffer, MAX_EVENTS_PER_POLL, out eventsRead);

            if (result == MK3_ERROR_NO_EVENTS)
                return;

            if (result != MK3_SUCCESS)
            {
                if (enableDebugLogging)
                {
                    Debug.LogError($"[MaschineMK3HardwareService] Poll error: {result}");
                }
                return;
            }

            // Process events and queue them (use Stopwatch for thread-safe timestamps)
            float timestamp = (float)_timeSource.Elapsed.TotalSeconds;
            for (int i = 0; i < eventsRead; i++)
            {
                ProcessNativeEvent(_eventBuffer[i], timestamp);
            }

            if (enableDebugLogging && eventsRead > 0)
            {
                Debug.Log($"[MaschineMK3HardwareService] Polled {eventsRead} events, queued to main thread");
            }
        }

        // Called on background thread
        private void ProcessNativeEvent(MaschineMK3Native.InputEvent evt, float timestamp)
        {
            switch (evt.eventType)
            {
                case MaschineMK3Native.EventType.ButtonPressed:
                    _inputQueue.Enqueue(MaschineMK3InputEvent.ButtonPressed(evt.elementId, timestamp));
                    break;

                case MaschineMK3Native.EventType.ButtonReleased:
                    _inputQueue.Enqueue(MaschineMK3InputEvent.ButtonReleased(evt.elementId, timestamp));
                    break;

                case MaschineMK3Native.EventType.KnobChanged:
                    _inputQueue.Enqueue(MaschineMK3InputEvent.KnobChanged(evt.elementId, evt.value, evt.delta, timestamp));
                    break;

                case MaschineMK3Native.EventType.PadEvent:
                    switch (evt.padEventType)
                    {
                        case MaschineMK3Native.PadEventType.Hit:
                            _inputQueue.Enqueue(MaschineMK3InputEvent.PadHit(evt.elementId, evt.value, timestamp));
                            break;
                        case MaschineMK3Native.PadEventType.Aftertouch:
                            _inputQueue.Enqueue(MaschineMK3InputEvent.PadAftertouch(evt.elementId, evt.value, timestamp));
                            break;
                        case MaschineMK3Native.PadEventType.TouchRelease:
                            _inputQueue.Enqueue(MaschineMK3InputEvent.PadTouchRelease(evt.elementId, timestamp));
                            break;
                        case MaschineMK3Native.PadEventType.HitRelease:
                            _inputQueue.Enqueue(MaschineMK3InputEvent.PadHitRelease(evt.elementId, timestamp));
                            break;
                    }
                    break;
            }
        }

        // Called on background thread
        private void ProcessOutputCommands()
        {
            if (_deviceHandle == IntPtr.Zero)
                return;

            // Process LED commands from queue
            var ledCommands = _ledQueue.Swap();
            int ledCount = ledCommands.Count;

            while (ledCommands.Count > 0)
            {
                var cmd = ledCommands.Dequeue();
                var color = new MaschineMK3Native.RgbColor(cmd.R, cmd.G, cmd.B);

                switch (cmd.CommandType)
                {
                    case MaschineMK3OutputCommand.Type.SetPadLED:
                        mk3_set_pad_led(_deviceHandle, (byte)cmd.ElementId, color, cmd.Bright ? 1 : 0);
                        break;

                    case MaschineMK3OutputCommand.Type.SetButtonLED:
                        mk3_set_button_led(_deviceHandle, cmd.ElementId, color, cmd.Bright ? 1 : 0);
                        break;

                    case MaschineMK3OutputCommand.Type.FlushLEDs:
                        mk3_flush_leds(_deviceHandle);
                        break;
                }
            }

            if (enableDebugLogging && ledCount > 0)
            {
                Debug.Log($"[MaschineMK3HardwareService] Processed {ledCount} LED commands");
            }

            // Process display updates (double-buffered, no blocking of main thread)
            // Swap buffers if new data available, then send outside the lock
            bool sendLeft = false;
            bool sendRight = false;

            if (_hasLeftDisplayUpdate)
            {
                lock (_leftDisplayLock)
                {
                    if (_hasLeftDisplayUpdate)
                    {
                        // Swap buffers
                        var temp = _sendingLeftDisplay;
                        _sendingLeftDisplay = _pendingLeftDisplay;
                        _pendingLeftDisplay = temp;
                        _hasLeftDisplayUpdate = false;
                        sendLeft = true;
                    }
                }
            }

            if (_hasRightDisplayUpdate)
            {
                lock (_rightDisplayLock)
                {
                    if (_hasRightDisplayUpdate)
                    {
                        // Swap buffers
                        var temp = _sendingRightDisplay;
                        _sendingRightDisplay = _pendingRightDisplay;
                        _pendingRightDisplay = temp;
                        _hasRightDisplayUpdate = false;
                        sendRight = true;
                    }
                }
            }

            // Send outside locks (USB write can be slow, doesn't block main thread)
            if (sendLeft)
            {
                int result;
                if (_useLeftDirtyDetection)
                {
                    result = mk3_write_display_rgb888_dirty(_deviceHandle, (uint)MaschineMK3Native.DisplayId.Left, _sendingLeftDisplay, (uint)_sendingLeftDisplay.Length);
                }
                else
                {
                    result = mk3_write_display_rgb888(_deviceHandle, (uint)MaschineMK3Native.DisplayId.Left, _sendingLeftDisplay, (uint)_sendingLeftDisplay.Length);
                }

                if (result != MK3_SUCCESS && enableDebugLogging)
                {
                    Debug.LogError($"[MaschineMK3HardwareService] ERROR writing left display: {result}");
                }
            }

            if (sendRight)
            {
                int result;
                if (_useRightDirtyDetection)
                {
                    result = mk3_write_display_rgb888_dirty(_deviceHandle, (uint)MaschineMK3Native.DisplayId.Right, _sendingRightDisplay, (uint)_sendingRightDisplay.Length);
                }
                else
                {
                    result = mk3_write_display_rgb888(_deviceHandle, (uint)MaschineMK3Native.DisplayId.Right, _sendingRightDisplay, (uint)_sendingRightDisplay.Length);
                }

                if (result != MK3_SUCCESS && enableDebugLogging)
                {
                    Debug.LogError($"[MaschineMK3HardwareService] ERROR writing right display: {result}");
                }
            }
        }

        // Called on main thread
        private void ProcessInputQueue()
        {
            var events = _inputQueue.Swap();
            int eventCount = events.Count;

            while (events.Count > 0)
            {
                var evt = events.Dequeue();
                DispatchInputEvent(evt);
                _eventsProcessedThisFrame++;
            }

            if (enableDebugLogging && eventCount > 0)
            {
                Debug.Log($"[MaschineMK3HardwareService] Dispatched {eventCount} input events");
            }
        }

        // Called on main thread
        private void DispatchInputEvent(MaschineMK3InputEvent evt)
        {
            if (enableDebugLogging)
            {
                Debug.Log($"[MaschineMK3HardwareService] Dispatching event: {evt}");
            }

            switch (evt.EventType)
            {
                case MaschineMK3InputEvent.Type.PadHit:
                    OnPadHit?.Invoke(evt.ElementId, evt.Value);
                    break;

                case MaschineMK3InputEvent.Type.PadAftertouch:
                    OnPadAftertouch?.Invoke(evt.ElementId, evt.Value);
                    break;

                case MaschineMK3InputEvent.Type.PadTouchRelease:
                    OnPadTouchRelease?.Invoke(evt.ElementId);
                    break;

                case MaschineMK3InputEvent.Type.PadHitRelease:
                    OnPadHitRelease?.Invoke(evt.ElementId);
                    break;

                case MaschineMK3InputEvent.Type.ButtonPressed:
                    OnButtonPressed?.Invoke(evt.ElementId);
                    break;

                case MaschineMK3InputEvent.Type.ButtonReleased:
                    OnButtonReleased?.Invoke(evt.ElementId);
                    break;

                case MaschineMK3InputEvent.Type.KnobChanged:
                    OnKnobChanged?.Invoke(evt.ElementId, evt.Value, evt.Delta);
                    break;
            }
        }

        // Public API for output (called from main thread)

        public void SetPadLED(int padNumber, Color color, bool bright = true)
        {
            if (enableDebugLogging)
            {
                Debug.Log($"[MaschineMK3HardwareService] Queueing SetPadLED: pad={padNumber}, color={color}, bright={bright}");
            }
            _ledQueue.Enqueue(MaschineMK3OutputCommand.SetPadLED(padNumber, color, bright));
            _commandsQueuedThisFrame++;
        }

        public void SetButtonLED(int buttonId, Color color, bool bright = true)
        {
            _ledQueue.Enqueue(MaschineMK3OutputCommand.SetButtonLED(buttonId, color, bright));
            _commandsQueuedThisFrame++;
        }

        public void FlushLEDs()
        {
            if (enableDebugLogging)
            {
                Debug.Log($"[MaschineMK3HardwareService] Queueing FlushLEDs");
            }
            _ledQueue.Enqueue(MaschineMK3OutputCommand.FlushLEDs());
            _commandsQueuedThisFrame++;
        }

        public void WriteDisplay(MaschineMK3Native.DisplayId displayId, byte[] rgb565Data)
        {
            // Double-buffered write - lock only for buffer swap, not USB transfer
            if (displayId == MaschineMK3Native.DisplayId.Left)
            {
                lock (_leftDisplayLock)
                {
                    Array.Copy(rgb565Data, _pendingLeftDisplay, rgb565Data.Length);
                    _hasLeftDisplayUpdate = true;
                }
            }
            else
            {
                lock (_rightDisplayLock)
                {
                    Array.Copy(rgb565Data, _pendingRightDisplay, rgb565Data.Length);
                    _hasRightDisplayUpdate = true;
                }
            }
            _commandsQueuedThisFrame++;
        }

        // Write Texture2D using background thread (converts to RGB888, Rust does RGB888 → RGB565)
        public void WriteDisplayTexture(MaschineMK3Native.DisplayId displayId, Texture2D texture, bool useDirtyDetection = false)
        {
            if (texture == null || texture.width != 480 || texture.height != 272)
            {
                Debug.LogError($"WriteDisplayTexture: Invalid texture (must be 480x272)");
                return;
            }

            // Convert Texture2D to Color32[] to byte[] RGB888
            Color32[] pixels = texture.GetPixels32();
            byte[] rgb888 = new byte[480 * 272 * 3];

            // Simple RGB conversion (DLL handles Y-flip and RGB888 → RGB565x conversion)
            for (int i = 0; i < pixels.Length; i++)
            {
                rgb888[i * 3] = pixels[i].r;
                rgb888[i * 3 + 1] = pixels[i].g;
                rgb888[i * 3 + 2] = pixels[i].b;
            }

            // Double-buffered write - lock only for buffer swap, not USB transfer
            if (displayId == MaschineMK3Native.DisplayId.Left)
            {
                lock (_leftDisplayLock)
                {
                    Array.Copy(rgb888, _pendingLeftDisplay, rgb888.Length);
                    _hasLeftDisplayUpdate = true;
                    _useLeftDirtyDetection = useDirtyDetection;
                }
            }
            else
            {
                lock (_rightDisplayLock)
                {
                    Array.Copy(rgb888, _pendingRightDisplay, rgb888.Length);
                    _hasRightDisplayUpdate = true;
                    _useRightDirtyDetection = useDirtyDetection;
                }
            }
            _commandsQueuedThisFrame++;
        }

        // Write Texture2D region - SYNCHRONOUS (called directly, not via background thread)
        // Region updates are typically small and need immediate application
        public void WriteDisplayTextureRegion(MaschineMK3Native.DisplayId displayId, uint x, uint y, Texture2D texture)
        {
            if (_deviceHandle == IntPtr.Zero || texture == null)
            {
                Debug.LogError("WriteDisplayTextureRegion: Device not connected or texture is null");
                return;
            }

            uint width = (uint)texture.width;
            uint height = (uint)texture.height;

            if (x >= 480 || y >= 272 || x + width > 480 || y + height > 272)
            {
                Debug.LogError($"WriteDisplayTextureRegion: Region out of bounds ({x},{y} + {width}x{height})");
                return;
            }

            if (enableDebugLogging)
            {
                Debug.Log($"[HW] WriteDisplayTextureRegion: {displayId} ({x},{y}) {width}x{height} = {width * height} pixels, {width * height * 3} bytes RGB888");
            }

            // Convert Texture2D to Color32[] to byte[] RGB888
            Color32[] pixels = texture.GetPixels32();
            byte[] rgb888 = new byte[width * height * 3];

            // Simple RGB conversion (DLL handles Y-flip and RGB888 → RGB565x conversion)
            for (int i = 0; i < pixels.Length; i++)
            {
                rgb888[i * 3] = pixels[i].r;
                rgb888[i * 3 + 1] = pixels[i].g;
                rgb888[i * 3 + 2] = pixels[i].b;
            }

            // Call directly with USB lock (synchronous for immediate region updates)
            lock (_usbLock)
            {
                int result = mk3_write_display_region_rgb888(
                    _deviceHandle,
                    (uint)displayId,
                    x, y,
                    width, height,
                    rgb888,
                    (uint)rgb888.Length
                );

                if (result != MK3_SUCCESS)
                {
                    Debug.LogError($"[HW] WriteDisplayTextureRegion FAILED: error code {result}");
                }
                else if (enableDebugLogging)
                {
                    Debug.Log($"[HW] WriteDisplayTextureRegion SUCCESS");
                }
            }
        }

        // Diagnostic methods

        [ContextMenu("Debug: Show Thread Status")]
        public void ShowThreadStatus()
        {
            Debug.Log($"=== MaschineMK3HardwareService Status ===");
            Debug.Log($"Connected: {IsConnected}");
            Debug.Log($"Display Available: {IsDisplayAvailable}");
            Debug.Log($"Threads Running: {_running}");
            Debug.Log($"Input Thread Alive: {_inputThread?.IsAlive ?? false}");
            Debug.Log($"Output Thread Alive: {_outputThread?.IsAlive ?? false}");
            Debug.Log($"Polling Interval: {pollingIntervalMs}ms");
            Debug.Log($"Input Queue Size: {_inputQueue.ApproximateCount}");
            Debug.Log($"LED Queue Size: {_ledQueue.ApproximateCount}");
            Debug.Log($"Pending Left Display: {_hasLeftDisplayUpdate}");
            Debug.Log($"Pending Right Display: {_hasRightDisplayUpdate}");
            Debug.Log($"Events This Frame: {_eventsProcessedThisFrame}");
            Debug.Log($"Commands This Frame: {_commandsQueuedThisFrame}");
        }
    }
}
