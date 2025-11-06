use crate::error::{MK3Error, Result};
use crate::input::{InputElement, InputEvent, InputState, InputTracker, PadState};
use crate::output::{DisplayPacket, MaschineLEDColor, Rgb565};
use crate::{ButtonLedState, PadLedState};
use rusb::{Context, Device, DeviceHandle, UsbContext};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[cfg(windows)]
use hidapi::{HidApi, HidDevice};

/// Native Instruments Maschine MK3 USB constants
const VENDOR_ID: u16 = 0x17CC;
const PRODUCT_ID: u16 = 0x1600;

/// USB Interface and Endpoint constants
const HID_INTERFACE: u8 = 4;
const DISPLAY_INTERFACE: u8 = 5; // Back to original - Interface 5 with WinUSB
const INPUT_ENDPOINT: u8 = 0x83;
const OUTPUT_ENDPOINT: u8 = 0x03;
const DISPLAY_ENDPOINT: u8 = 0x04; // Original endpoint 0x04 from interface 5

/// Convert standard RGB888 (0-255) to Maschine's custom RGB565x format
///
/// The Maschine MK3 uses a non-standard RGB565 variant with bit packing: GGGB BBBB RRRR RGGG
#[inline]
fn rgb888_to_rgb565x(red: u8, green: u8, blue: u8) -> u16 {
    let r5 = (red >> 3) as u16;
    let g3high = (green >> 5) as u16;
    let glow = ((green >> 2) & 7) as u16;
    let b5 = (blue >> 3) as u16;

    (glow << 13) | (b5 << 8) | (r5 << 3) | g3high
}

/// Convert RGB888 buffer to RGB565x buffer (with optional vertical flip)
///
/// # Parameters
/// - `rgb888_data`: RGB888 data (3 bytes per pixel: R, G, B)
/// - `rgb565x_out`: Output buffer for RGB565x data (2 bytes per pixel, little-endian)
/// - `width`: Image width in pixels
/// - `height`: Image height in pixels
/// - `flip_y`: Flip vertically (needed for Unity textures which are upside-down)
fn convert_rgb888_to_rgb565x(rgb888_data: &[u8], rgb565x_out: &mut [u8], width: usize, height: usize, flip_y: bool) {
    assert!(rgb888_data.len() % 3 == 0, "RGB888 data must be multiple of 3 bytes");
    assert!(rgb565x_out.len() == (rgb888_data.len() / 3) * 2, "Output buffer size mismatch");
    assert!(rgb888_data.len() == width * height * 3, "Data size doesn't match width × height");

    for y in 0..height {
        for x in 0..width {
            // Source pixel index (in RGB888 buffer)
            let src_y = if flip_y { height - 1 - y } else { y };
            let src_idx = (src_y * width + x) * 3;

            // Destination pixel index (in RGB565x buffer)
            let dst_idx = (y * width + x) * 2;

            let r = rgb888_data[src_idx];
            let g = rgb888_data[src_idx + 1];
            let b = rgb888_data[src_idx + 2];

            let rgb565x = rgb888_to_rgb565x(r, g, b);

            // Store as little-endian
            rgb565x_out[dst_idx] = (rgb565x & 0xFF) as u8;
            rgb565x_out[dst_idx + 1] = (rgb565x >> 8) as u8;
        }
    }
}

/// Main interface for communicating with a Maschine MK3 controller.
/// 
/// Provides methods for reading input events and controlling LEDs/display.
/// 
/// # Example
/// 
/// ```no_run
/// use maschine3_hal::MaschineMK3;
/// 
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut device = MaschineMK3::new()?;
/// let events = device.poll_input_events()?;
/// # Ok(())
/// # }
/// ```
pub struct MaschineMK3 {
    device_handle: DeviceHandle<Context>,
    pub context: Context,
    #[cfg(windows)]
    hid_device: Option<HidDevice>,
    #[cfg(windows)]
    _hid_api: Option<HidApi>,

    // LED state management
    current_button_leds: ButtonLedState,
    current_pad_leds: PadLedState,
    led_state_dirty: bool,

    // Input monitoring
    input_tracker: InputTracker,
    input_thread: Option<JoinHandle<()>>,
    input_stop_signal: Arc<Mutex<bool>>,
    input_event_receiver: Option<Receiver<InputEvent>>,

    // Display interface status
    display_interface_available: bool,

    // Display state tracking for dirty region detection
    display_state: [Option<Vec<u8>>; 2], // Track RGB888 state for displays 0 and 1
}

impl MaschineMK3 {
    /// Connect to the first available Maschine MK3 device.
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - No Maschine MK3 device is found
    /// - USB interfaces cannot be claimed
    /// - Device communication fails
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use maschine3_hal::MaschineMK3;
    /// 
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut device = MaschineMK3::new()?;
    /// println!("Connected to Maschine MK3");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Self> {
        let context = Context::new()?;
        let device = Self::find_device(&context)?;
        let mut device_handle = device.open()?;

        // Debug: print device configuration info
        Self::debug_device_info(&device)?;

        // Platform-specific interface claiming
        #[cfg(windows)]
        {
            // Windows doesn't support automatic kernel driver detachment
            Self::claim_interface_with_detach(&mut device_handle, HID_INTERFACE)?;
        }

        #[cfg(unix)]
        {
            // Linux: detach kernel drivers and claim interfaces
            Self::detach_and_claim_interface(&mut device_handle, HID_INTERFACE)?;
        }

        // Platform-specific display interface handling
        #[cfg(windows)]
        let display_interface_available = {
            // On Windows, try to claim display interface but don't fail if it doesn't work
            match Self::claim_interface_with_detach(&mut device_handle, DISPLAY_INTERFACE) {
                Ok(()) => {
                    println!(
                        "✅ Display interface {} claimed successfully",
                        DISPLAY_INTERFACE
                    );
                    true
                }
                Err(e) => {
                    println!(
                        "⚠️  Could not claim display interface {}: {}",
                        DISPLAY_INTERFACE, e
                    );
                    println!("   Trying alternative interface 3...");

                    // Try Interface 3 as backup
                    match Self::claim_interface_with_detach(&mut device_handle, 3) {
                        Ok(()) => {
                            println!("✅ Alternative interface 3 claimed successfully");
                            // Update display endpoint to use Interface 3's bulk endpoint
                            println!("   📝 Note: Using endpoint 0x02 instead of 0x04");
                            true
                        }
                        Err(e2) => {
                            println!("⚠️  Alternative interface 3 also failed: {}", e2);
                            println!("   💡 Consider installing WinUSB driver using Zadig");
                            println!("   💡 Or use HID-only mode for input/LEDs");
                            false
                        }
                    }
                }
            }
        };

        #[cfg(unix)]
        let display_interface_available = {
            // On Linux, try to claim display interface
            match Self::detach_and_claim_interface(&mut device_handle, DISPLAY_INTERFACE) {
                Ok(()) => {
                    println!(
                        "✅ Display interface {} claimed successfully",
                        DISPLAY_INTERFACE
                    );
                    true
                }
                Err(e) => {
                    println!(
                        "⚠️  Could not claim display interface {}: {}",
                        DISPLAY_INTERFACE, e
                    );
                    println!("   💡 Check udev rules and user permissions");
                    false
                }
            }
        };

        // Platform-specific HID device initialization
        #[cfg(windows)]
        let (hid_device, hid_api) = {
            match HidApi::new() {
                Ok(api) => {
                    let devices = api.device_list();
                    let mut hid_dev = None;

                    for device_info in devices {
                        if device_info.vendor_id() == VENDOR_ID
                            && device_info.product_id() == PRODUCT_ID
                        {
                            if device_info.interface_number() == 4 {
                                match device_info.open_device(&api) {
                                    Ok(dev) => {
                                        hid_dev = Some(dev);
                                        break;
                                    }
                                    Err(_) => {
                                        // Silently continue to next device
                                    }
                                }
                            }
                        }
                    }

                    (hid_dev, Some(api))
                }
                Err(_) => {
                    // HID API not available, fall back to USB only
                    (None, None)
                }
            }
        };

        Ok(Self {
            device_handle,
            context,
            #[cfg(windows)]
            hid_device,
            #[cfg(windows)]
            _hid_api: hid_api,

            // Initialize LED state management
            current_button_leds: ButtonLedState::default(),
            current_pad_leds: PadLedState::default(),
            led_state_dirty: false,

            // Initialize input monitoring
            input_tracker: InputTracker::new(),
            input_thread: None,
            input_stop_signal: Arc::new(Mutex::new(false)),
            input_event_receiver: None,

            // Display interface status
            display_interface_available,

            // Initialize display state tracking
            display_state: [None, None],
        })
    }

    /// Windows-specific: Claim interface without kernel driver detachment
    #[cfg(windows)]
    fn claim_interface_with_detach(
        handle: &mut DeviceHandle<Context>,
        interface: u8,
    ) -> Result<()> {
        println!("🔧 Attempting to claim interface {}", interface);

        // Windows doesn't support kernel driver detachment
        match handle.claim_interface(interface) {
            Ok(()) => {
                println!("✅ Successfully claimed interface {}", interface);
                Ok(())
            }
            Err(e) => {
                println!("❌ Failed to claim interface {}: {:?}", interface, e);
                Err(MK3Error::Usb(e))
            }
        }
    }

    /// Linux-specific: Detach kernel driver and claim interface
    #[cfg(unix)]
    fn detach_and_claim_interface(
        handle: &mut DeviceHandle<Context>,
        interface: u8,
    ) -> Result<()> {
        println!("🔧 Attempting to detach kernel driver and claim interface {}", interface);

        // Try to detach kernel driver if it's attached
        match handle.kernel_driver_active(interface) {
            Ok(true) => {
                println!("📤 Detaching kernel driver from interface {}", interface);
                match handle.detach_kernel_driver(interface) {
                    Ok(()) => println!("✅ Kernel driver detached from interface {}", interface),
                    Err(e) => {
                        println!("⚠️  Failed to detach kernel driver: {:?}", e);
                        // Continue anyway - might still work
                    }
                }
            }
            Ok(false) => {
                println!("✅ No kernel driver attached to interface {}", interface);
            }
            Err(e) => {
                println!("⚠️  Could not check kernel driver status: {:?}", e);
                // Continue anyway
            }
        }

        // Claim the interface
        match handle.claim_interface(interface) {
            Ok(()) => {
                println!("✅ Successfully claimed interface {}", interface);
                Ok(())
            }
            Err(e) => {
                println!("❌ Failed to claim interface {}: {:?}", interface, e);
                Err(MK3Error::Usb(e))
            }
        }
    }

    /// Find the first Maschine MK3 device
    fn find_device(context: &Context) -> Result<Device<Context>> {
        let devices = context.devices()?;

        for device in devices.iter() {
            let device_desc = device.device_descriptor()?;

            if device_desc.vendor_id() == VENDOR_ID && device_desc.product_id() == PRODUCT_ID {
                return Ok(device);
            }
        }

        Err(MK3Error::DeviceNotFound)
    }

    /// Debug device configuration information
    fn debug_device_info(device: &Device<Context>) -> Result<()> {
        let device_desc = device.device_descriptor()?;
        println!(
            "📱 Device found: VID:0x{:04X} PID:0x{:04X}",
            device_desc.vendor_id(),
            device_desc.product_id()
        );

        let config_desc = device.config_descriptor(0)?;
        println!(
            "🔧 Configuration: {} interfaces",
            config_desc.num_interfaces()
        );

        for interface in config_desc.interfaces() {
            println!("   Interface {}", interface.number());

            for interface_desc in interface.descriptors() {
                println!(
                    "     Class: 0x{:02X}, Subclass: 0x{:02X}, Protocol: 0x{:02X}",
                    interface_desc.class_code(),
                    interface_desc.sub_class_code(),
                    interface_desc.protocol_code()
                );

                for endpoint in interface_desc.endpoint_descriptors() {
                    println!(
                        "       Endpoint: 0x{:02X} ({:?})",
                        endpoint.address(),
                        endpoint.transfer_type()
                    );
                }
            }
        }
        Ok(())
    }

    /// Read input data from the device
    fn read_input(&self) -> Result<Vec<u8>> {
        self.read_input_with_timeout(Duration::from_millis(100))
    }

    /// Read input with custom timeout (for performance-critical applications)
    fn read_input_with_timeout(&self, timeout: Duration) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; 64]; // Max packet size

        match self
            .device_handle
            .read_interrupt(INPUT_ENDPOINT, &mut buffer, timeout)
        {
            Ok(bytes_read) => {
                buffer.truncate(bytes_read);
                Ok(buffer)
            }
            Err(rusb::Error::Timeout) => Ok(Vec::new()), // No data available
            Err(e) => Err(MK3Error::Usb(e)),
        }
    }

    /// Write LED data to the device
    fn write_leds(&self, data: &[u8]) -> Result<()> {
        #[cfg(windows)]
        {
            // Windows: Use HID API for LED communication (interface 4 requires HID driver)
            if let Some(ref hid_dev) = self.hid_device {
                match hid_dev.write(data) {
                    Ok(_) => return Ok(()),
                    Err(e) => {
                        eprintln!("HID LED write failed: {}", e);
                        return Err(MK3Error::Io(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            e,
                        )));
                    }
                }
            }

            // Fallback to USB interrupt transfer if HID failed
            let timeout = Duration::from_millis(100);
            match self
                .device_handle
                .write_interrupt(OUTPUT_ENDPOINT, data, timeout)
            {
                Ok(_) => Ok(()),
                Err(e) => Err(MK3Error::Usb(e)),
            }
        }

        #[cfg(unix)]
        {
            // Linux: Use direct USB interrupt transfer
            let timeout = Duration::from_millis(100);
            match self
                .device_handle
                .write_interrupt(OUTPUT_ENDPOINT, data, timeout)
            {
                Ok(_) => Ok(()),
                Err(e) => Err(MK3Error::Usb(e)),
            }
        }
    }

    /// Write display data to the device
    pub fn write_display(&self, data: &[u8]) -> Result<()> {
        if !self.display_interface_available {
            eprintln!("⚠️  Display interface not available - write ignored");
            eprintln!("   Check Unity console for display interface claim status");
            return Err(MK3Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Display interface not claimed",
            )));
        }

        // Optimized timeout: 261KB at USB 2.0 High Speed (480 Mbps theoretical)
        // Realistic bulk transfer: ~30-40 MB/s = ~7-9ms for 261KB
        // Add safety margin: 50ms timeout is plenty
        let timeout = Duration::from_millis(50);
        self.device_handle
            .write_bulk(DISPLAY_ENDPOINT, data, timeout)?;
        Ok(())
    }

    /// Check if the display interface is available for writing
    pub fn is_display_available(&self) -> bool {
        self.display_interface_available
    }
    /// Write RGB565 framebuffer to display with proper packet format (full screen)
    /// framebuffer_data should be 480×272×2 bytes of RGB565 data
    /// display_id: 0 = Left display, 1 = Right display
    pub fn write_display_framebuffer(&self, display_id: u8, framebuffer_data: &[u8]) -> Result<()> {
        const WIDTH: u16 = 480;
        const HEIGHT: u16 = 272;
        self.write_display_region(display_id, 0, 0, WIDTH, HEIGHT, framebuffer_data)
    }

    /// Write RGB565 data to a specific region of the display
    ///
    /// # Parameters
    /// - `display_id`: 0 = Left display, 1 = Right display
    /// - `x`: Starting X coordinate (0-479)
    /// - `y`: Starting Y coordinate (0-271)
    /// - `width`: Region width in pixels
    /// - `height`: Region height in pixels
    /// - `region_data`: RGB565 pixel data (width × height × 2 bytes)
    ///
    /// # Performance
    /// Smaller regions = faster transfers. For example:
    /// - Full screen (480×272): ~33ms
    /// - Half screen (480×136): ~17ms
    /// - Quarter screen (240×136): ~8ms
    ///
    /// # Example
    /// ```no_run
    /// // Update only top-left 100x100 region
    /// let region_pixels = vec![0u8; 100 * 100 * 2]; // Your RGB565 data
    /// device.write_display_region(0, 0, 0, 100, 100, &region_pixels)?;
    /// ```
    pub fn write_display_region(
        &self,
        display_id: u8,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        region_data: &[u8],
    ) -> Result<()> {
        // Validate parameters
        if display_id > 1 {
            return Err(MK3Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("display_id must be 0 (left) or 1 (right), got {}", display_id),
            )));
        }

        const MAX_WIDTH: u16 = 480;
        const MAX_HEIGHT: u16 = 272;

        if x >= MAX_WIDTH || y >= MAX_HEIGHT {
            return Err(MK3Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Region start ({}, {}) out of bounds (max: {}, {})", x, y, MAX_WIDTH - 1, MAX_HEIGHT - 1),
            )));
        }

        if x + width > MAX_WIDTH || y + height > MAX_HEIGHT {
            return Err(MK3Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Region ({}x{} at {},{}) exceeds display bounds ({}x{})",
                    width, height, x, y, MAX_WIDTH, MAX_HEIGHT),
            )));
        }

        let pixel_count = (width as usize) * (height as usize);
        let expected_size = pixel_count * 2; // RGB565 = 2 bytes per pixel

        if region_data.len() != expected_size {
            return Err(MK3Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Region data must be {} bytes ({}x{} pixels), got {}",
                    expected_size, width, height, region_data.len()),
            )));
        }

        const HEADER_LEN: usize = 16;
        const CMD_LEN: usize = 4;
        // Check if we need padding for odd pixel counts
        let needs_padding = pixel_count % 2 == 1;
        let padding_bytes = if needs_padding { 2 } else { 0 }; // One RGB565 pixel
        let packet_size = HEADER_LEN + CMD_LEN + expected_size + padding_bytes + CMD_LEN * 2;
        let mut packet = vec![0u8; packet_size];
        let mut offset = 0;

        // Header with region coordinates
        packet[0] = 0x84; packet[1] = 0x00; packet[2] = display_id; packet[3] = 0x60;
        packet[8] = (x >> 8) as u8; packet[9] = (x & 0xFF) as u8;           // X start
        packet[10] = (y >> 8) as u8; packet[11] = (y & 0xFF) as u8;         // Y start
        packet[12] = (width >> 8) as u8; packet[13] = (width & 0xFF) as u8; // Width
        packet[14] = (height >> 8) as u8; packet[15] = (height & 0xFF) as u8; // Height
        offset += HEADER_LEN;

        // Transmit command
        // Device expects count in pixel pairs (half_pixels)
        let padded_pixel_count = if needs_padding { pixel_count + 1 } else { pixel_count };
        let half_pixels = (padded_pixel_count as u32) / 2;
        packet[offset] = 0x00;
        packet[offset + 1] = (half_pixels >> 16) as u8;
        packet[offset + 2] = (half_pixels >> 8) as u8;
        packet[offset + 3] = (half_pixels & 0xFF) as u8;
        offset += CMD_LEN;

        // Pixel data
        packet[offset..offset + expected_size].copy_from_slice(region_data);
        offset += expected_size;

        // Pad with one black pixel (RGB565: 0x0000) if pixel count was odd
        if needs_padding {
            packet[offset] = 0;
            packet[offset + 1] = 0;
            offset += 2;
        }

        // Blit command
        packet[offset] = 0x03; offset += CMD_LEN;

        // End command
        packet[offset] = 0x40;

        self.write_display(&packet)
    }

    /// Write RGB888 data to a region (convenience method with automatic conversion)
    ///
    /// This method accepts standard RGB888 data (3 bytes per pixel) and automatically
    /// converts it to the Maschine's RGB565x format before sending.
    ///
    /// # Parameters
    /// - `display_id`: 0 = Left display, 1 = Right display
    /// - `x`: Starting X coordinate (0-479)
    /// - `y`: Starting Y coordinate (0-271)
    /// - `width`: Region width in pixels
    /// - `height`: Region height in pixels
    /// - `rgb888_data`: Standard RGB888 pixel data (width × height × 3 bytes, R-G-B order)
    ///
    /// # Note
    /// This method does NOT flip the Y-axis. If you're using Unity textures, you need to
    /// flip them before extracting the region data.
    ///
    /// # Example
    /// ```no_run
    /// // Update 100x100 region with standard RGB data
    /// let mut rgb_data = vec![0u8; 100 * 100 * 3];
    /// // Fill with red
    /// for i in 0..(100 * 100) {
    ///     rgb_data[i * 3] = 255;     // Red
    ///     rgb_data[i * 3 + 1] = 0;   // Green
    ///     rgb_data[i * 3 + 2] = 0;   // Blue
    /// }
    /// device.write_display_region_rgb888(0, 0, 0, 100, 100, &rgb_data)?;
    /// ```
    pub fn write_display_region_rgb888(
        &self,
        display_id: u8,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        rgb888_data: &[u8],
    ) -> Result<()> {
        self.write_display_region_rgb888_internal(display_id, x, y, width, height, rgb888_data, false)
    }

    /// Internal method for RGB888 region writes with optional Y-flip control
    fn write_display_region_rgb888_internal(
        &self,
        display_id: u8,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        rgb888_data: &[u8],
        flip_y: bool,
    ) -> Result<()> {
        let pixel_count = (width as usize) * (height as usize);
        let expected_rgb888_size = pixel_count * 3;

        if rgb888_data.len() != expected_rgb888_size {
            return Err(MK3Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("RGB888 data must be {} bytes ({}x{} pixels × 3), got {}",
                    expected_rgb888_size, width, height, rgb888_data.len()),
            )));
        }

        // Convert RGB888 to RGB565x
        let mut rgb565x_data = vec![0u8; pixel_count * 2];
        convert_rgb888_to_rgb565x(
            rgb888_data,
            &mut rgb565x_data,
            width as usize,
            height as usize,
            flip_y
        );

        // Use existing RGB565x method
        self.write_display_region(display_id, x, y, width, height, &rgb565x_data)
    }

    /// Write RGB888 framebuffer to display (convenience method with automatic conversion)
    ///
    /// This method accepts standard RGB888 data for the full screen (480×272 pixels)
    /// and automatically converts it to the Maschine's RGB565x format.
    ///
    /// # Parameters
    /// - `display_id`: 0 = Left display, 1 = Right display
    /// - `rgb888_data`: Standard RGB888 framebuffer (480 × 272 × 3 = 391,680 bytes)
    ///
    /// # Note
    /// This method DOES flip the Y-axis for Unity texture compatibility.
    /// Unity textures have (0,0) at bottom-left, but the display has (0,0) at top-left.
    ///
    /// # Example
    /// ```no_run
    /// let mut frame = vec![0u8; 480 * 272 * 3];
    /// // Fill with blue
    /// for i in 0..(480 * 272) {
    ///     frame[i * 3] = 0;        // Red
    ///     frame[i * 3 + 1] = 0;    // Green
    ///     frame[i * 3 + 2] = 255;  // Blue
    /// }
    /// device.write_display_framebuffer_rgb888(0, &frame)?;
    /// ```
    pub fn write_display_framebuffer_rgb888(&self, display_id: u8, rgb888_data: &[u8]) -> Result<()> {
        const WIDTH: u16 = 480;
        const HEIGHT: u16 = 272;
        // Full frame updates use Y-flip for Unity compatibility
        self.write_display_region_rgb888_internal(display_id, 0, 0, WIDTH, HEIGHT, rgb888_data, true)
    }

    /// Write RGB888 framebuffer with dirty region detection
    ///
    /// This method tracks the current display state and only sends the minimal rectangular
    /// region that contains all changed pixels. This significantly reduces USB bandwidth
    /// for incremental updates.
    ///
    /// # Parameters
    /// - `display_id`: 0 = Left display, 1 = Right display
    /// - `rgb888_data`: Standard RGB888 framebuffer (480 × 272 × 3 = 391,680 bytes)
    ///
    /// # Performance
    /// - First call: sends full screen and saves state
    /// - Subsequent calls: detects changes and sends only the dirty rectangle
    /// - No changes: skips USB transfer entirely
    ///
    /// # Example
    /// ```no_run
    /// let mut frame = vec![0u8; 480 * 272 * 3];
    /// // First update: full screen
    /// device.write_display_framebuffer_rgb888_dirty(0, &frame)?;
    ///
    /// // Change only a small region
    /// for i in 0..100 {
    ///     frame[i * 3] = 255; // Change first 100 pixels to red
    /// }
    /// // Second update: only sends the changed region
    /// device.write_display_framebuffer_rgb888_dirty(0, &frame)?;
    /// ```
    pub fn write_display_framebuffer_rgb888_dirty(&mut self, display_id: u8, rgb888_data: &[u8]) -> Result<()> {
        const WIDTH: usize = 480;
        const HEIGHT: usize = 272;
        const EXPECTED_SIZE: usize = WIDTH * HEIGHT * 3;

        if display_id > 1 {
            return Err(MK3Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("display_id must be 0 (left) or 1 (right), got {}", display_id),
            )));
        }

        if rgb888_data.len() != EXPECTED_SIZE {
            return Err(MK3Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("RGB888 data must be {} bytes (480×272×3), got {}", EXPECTED_SIZE, rgb888_data.len()),
            )));
        }

        let display_idx = display_id as usize;

        // STEP 1: Flip Unity's Y-inverted data to display coordinate space
        // This way we store, compare, and send everything in the same space
        let mut flipped_data = vec![0u8; EXPECTED_SIZE];
        for y in 0..HEIGHT {
            let src_y = HEIGHT - 1 - y; // Flip Y
            let src_row_start = src_y * WIDTH * 3;
            let dst_row_start = y * WIDTH * 3;
            flipped_data[dst_row_start..dst_row_start + WIDTH * 3]
                .copy_from_slice(&rgb888_data[src_row_start..src_row_start + WIDTH * 3]);
        }

        // First call: no previous state, send full screen
        if self.display_state[display_idx].is_none() {
            self.display_state[display_idx] = Some(flipped_data.clone());
            return self.write_display_region_rgb888_internal(display_id, 0, 0, WIDTH as u16, HEIGHT as u16, &flipped_data, false);
        }

        let prev_state = self.display_state[display_idx].as_ref().unwrap();

        // STEP 2: Compare with previous state (both in display space now)
        let mut min_x = WIDTH;
        let mut min_y = HEIGHT;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut has_changes = false;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixel_idx = (y * WIDTH + x) * 3;

                // Compare RGB values
                if prev_state[pixel_idx] != flipped_data[pixel_idx]
                    || prev_state[pixel_idx + 1] != flipped_data[pixel_idx + 1]
                    || prev_state[pixel_idx + 2] != flipped_data[pixel_idx + 2]
                {
                    has_changes = true;
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
            }
        }

        // No changes detected
        if !has_changes {
            return Ok(());
        }

        // Expand to nearest 8-pixel block boundaries to avoid artifacts
        // This ensures we always send complete 8x8 aligned regions
        const BLOCK_SIZE: usize = 8;

        // Round down to 8-pixel boundary
        min_x = (min_x / BLOCK_SIZE) * BLOCK_SIZE;
        min_y = (min_y / BLOCK_SIZE) * BLOCK_SIZE;

        // Round up to 8-pixel boundary, clamped to screen size
        max_x = (((max_x + BLOCK_SIZE) / BLOCK_SIZE) * BLOCK_SIZE - 1).min(WIDTH - 1);
        max_y = (((max_y + BLOCK_SIZE) / BLOCK_SIZE) * BLOCK_SIZE - 1).min(HEIGHT - 1);

        // Calculate dirty rectangle dimensions (aligned to 8px blocks)
        let dirty_width = (max_x - min_x + 1) as u16;
        let dirty_height = (max_y - min_y + 1) as u16;

        // Enforce minimum region size to avoid protocol issues with small updates
        const MIN_REGION_SIZE: u16 = 8;

        // If region is too small, fall back to full frame update
        if dirty_width < MIN_REGION_SIZE || dirty_height < MIN_REGION_SIZE {
            self.display_state[display_idx] = Some(flipped_data.clone());
            return self.write_display_region_rgb888_internal(display_id, 0, 0, WIDTH as u16, HEIGHT as u16, &flipped_data, false);
        }

        // STEP 3: Extract dirty region (from flipped/display-space data)
        let mut dirty_region = Vec::with_capacity((dirty_width as usize * dirty_height as usize * 3) as usize);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pixel_idx = (y * WIDTH + x) * 3;
                dirty_region.push(flipped_data[pixel_idx]);
                dirty_region.push(flipped_data[pixel_idx + 1]);
                dirty_region.push(flipped_data[pixel_idx + 2]);
            }
        }

        // Update stored state (with flipped data)
        self.display_state[display_idx] = Some(flipped_data);

        // STEP 4: Send dirty region (no flip - already in display space)
        self.write_display_region_rgb888_internal(
            display_id,
            min_x as u16,
            min_y as u16,
            dirty_width,
            dirty_height,
            &dirty_region,
            false  // No flip - already in display coordinate space
        )
    }


    /// Write button LED state
    pub fn write_button_leds(&self, state: &ButtonLedState) -> Result<()> {
        let packet = state.to_packet();
        self.write_leds(&packet)
    }

    /// Write pad LED state
    pub fn write_pad_leds(&self, state: &PadLedState) -> Result<()> {
        let packet = state.to_packet();
        self.write_leds(&packet)
    }

    /// Write a display packet to a specific display
    pub fn write_display_packet(&self, packet: &DisplayPacket) -> Result<()> {
        let data = packet.to_packet();
        self.write_display(&data)
    }

    /// Send raw data directly to the device (for testing/debugging)
    pub fn send_raw_data(&self, data: &[u8]) -> Result<()> {
        let timeout = Duration::from_millis(50);

        // Try display endpoint first (bulk transfer)
        match self
            .device_handle
            .write_bulk(DISPLAY_ENDPOINT, data, timeout)
        {
            Ok(_) => {
                //println!("✅ Sent {} bytes via display endpoint (bulk)", data.len());
                Ok(())
            }
            Err(e) => {
                println!("⚠️  Display endpoint failed: {}, trying HID endpoint...", e);

                // Fallback to HID endpoint (interrupt transfer)
                match self
                    .device_handle
                    .write_interrupt(OUTPUT_ENDPOINT, data, timeout)
                {
                    Ok(_) => {
                        println!("✅ Sent {} bytes via HID endpoint (interrupt)", data.len());
                        Ok(())
                    }
                    Err(e2) => {
                        println!("❌ Both endpoints failed");
                        Err(MK3Error::Usb(e2))
                    }
                }
            }
        }
    }

    /// Get device information for debugging
    pub fn device_info(&self) -> Result<String> {
        let device = self.device_handle.device();
        let device_desc = device.device_descriptor()?;
        let handle = &self.device_handle;

        let manufacturer = handle
            .read_manufacturer_string_ascii(&device_desc)
            .unwrap_or_else(|_| "Unknown".to_string());

        let product = handle
            .read_product_string_ascii(&device_desc)
            .unwrap_or_else(|_| "Unknown".to_string());

        Ok(format!(
            "Maschine MK3 - Manufacturer: {}, Product: {}, VID: 0x{:04X}, PID: 0x{:04X}",
            manufacturer,
            product,
            device_desc.vendor_id(),
            device_desc.product_id()
        ))
    }

    /// Display dimensions
    pub const DISPLAY_WIDTH: u16 = 480;
    pub const DISPLAY_HEIGHT: u16 = 272;

    /// Send optimized full-screen image to display (30 FPS capable)
    pub fn send_display_image(&self, display_num: u8, pixels: Vec<Rgb565>) -> Result<()> {
        let num_pixels = Self::DISPLAY_WIDTH as usize * Self::DISPLAY_HEIGHT as usize;

        if pixels.len() != num_pixels {
            return Err(MK3Error::InvalidData(format!(
                "Expected {} pixels, got {}",
                num_pixels,
                pixels.len()
            )));
        }

        let packet = DisplayPacket::full_screen_optimized(display_num, pixels);
        self.send_raw_data(&packet.to_packet())
    }

    /// Send RGB888 image to display (converts to RGB565X)
    pub fn send_display_rgb888(&self, display_num: u8, rgb_data: &[u8]) -> Result<()> {
        let num_pixels = Self::DISPLAY_WIDTH as usize * Self::DISPLAY_HEIGHT as usize;

        if rgb_data.len() != num_pixels * 3 {
            return Err(MK3Error::InvalidData(format!(
                "Expected {} RGB bytes, got {}",
                num_pixels * 3,
                rgb_data.len()
            )));
        }

        // Convert RGB888 to RGB565X
        let mut pixels = Vec::with_capacity(num_pixels);
        for chunk in rgb_data.chunks_exact(3) {
            pixels.push(Rgb565::new(chunk[0], chunk[1], chunk[2]));
        }

        self.send_display_image(display_num, pixels)
    }

    /// Clear display with solid color
    pub fn clear_display(&self, display_num: u8, red: u8, green: u8, blue: u8) -> Result<()> {
        let num_pixels = Self::DISPLAY_WIDTH as usize * Self::DISPLAY_HEIGHT as usize;
        let color = Rgb565::new(red, green, blue);
        let pixels = vec![color; num_pixels];
        self.send_display_image(display_num, pixels)
    }

    // === Input Management ===

    /// Start monitoring input with a callback (non-blocking)
    pub fn start_input_monitoring<F>(&mut self, callback: F) -> Result<()>
    where
        F: Fn(InputEvent) + Send + 'static,
    {
        if self.input_thread.is_some() {
            return Err(MK3Error::InvalidData(
                "Input monitoring already running".to_string(),
            ));
        }

        let (sender, receiver) = mpsc::channel();
        self.input_event_receiver = Some(receiver);

        // Clone the device handle for the thread
        let device = self.device_handle.device();
        let mut thread_device_handle = device.open()?;
        
        #[cfg(windows)]
        Self::claim_interface_with_detach(&mut thread_device_handle, HID_INTERFACE)?;
        
        #[cfg(unix)]
        Self::detach_and_claim_interface(&mut thread_device_handle, HID_INTERFACE)?;

        let stop_signal = Arc::clone(&self.input_stop_signal);
        let mut tracker = InputTracker::new();

        let handle = thread::spawn(move || {
            loop {
                // Check stop signal
                if let Ok(stop) = stop_signal.lock() {
                    if *stop {
                        break;
                    }
                }

                // Read input from device
                let data = {
                    let mut buffer = vec![0u8; 64];
                    let timeout = Duration::from_millis(100);
                    match thread_device_handle.read_interrupt(INPUT_ENDPOINT, &mut buffer, timeout)
                    {
                        Ok(bytes_read) => {
                            buffer.truncate(bytes_read);
                            buffer
                        }
                        Err(rusb::Error::Timeout) => Vec::new(),
                        Err(_) => {
                            thread::sleep(Duration::from_millis(10));
                            continue;
                        }
                    }
                };

                if data.is_empty() {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }

                // Process packet and get events
                let events = match Self::process_input_packet(&mut tracker, &data) {
                    Ok(events) => events,
                    Err(_) => continue,
                };

                // Send events through callback and channel
                for event in events {
                    callback(event.clone());
                    let _ = sender.send(event);
                }

                thread::sleep(Duration::from_millis(10));
            }
        });

        self.input_thread = Some(handle);
        Ok(())
    }

    /// Stop input monitoring
    pub fn stop_input_monitoring(&mut self) -> Result<()> {
        if let Ok(mut stop) = self.input_stop_signal.lock() {
            *stop = true;
        }

        if let Some(handle) = self.input_thread.take() {
            handle.join().map_err(|_| {
                MK3Error::InvalidData("Failed to join monitoring thread".to_string())
            })?;
        }

        self.input_event_receiver = None;

        // Reset stop signal for future use
        if let Ok(mut stop) = self.input_stop_signal.lock() {
            *stop = false;
        }

        Ok(())
    }

    /// Poll for input events (blocking with timeout)
    pub fn poll_input_events(&mut self) -> Result<Vec<InputEvent>> {
        let data = self.read_input()?;

        if data.is_empty() {
            return Ok(Vec::new());
        }

        Self::process_input_packet(&mut self.input_tracker, &data)
    }

    /// Fast poll for input events with minimal timeout (1ms)
    /// Recommended for game loops and real-time applications
    pub fn poll_input_events_fast(&mut self) -> Result<Vec<InputEvent>> {
        let data = self.read_input_with_timeout(Duration::from_millis(1))?;

        if data.is_empty() {
            return Ok(Vec::new());
        }

        Self::process_input_packet(&mut self.input_tracker, &data)
    }

    /// Process a raw input packet and return events
    fn process_input_packet(tracker: &mut InputTracker, data: &[u8]) -> Result<Vec<InputEvent>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        match data[0] {
            0x01 if data.len() >= 42 => {
                let input_state = InputState::from_button_packet(data)?;
                Ok(tracker.update(input_state))
            }
            0x02 => {
                let pad_state = PadState::from_pad_packet(data)?;
                Ok(tracker.update_pads(pad_state))
            }
            _ => Ok(Vec::new()),
        }
    }

    // === LED Management ===

    /// Set individual button LED brightness
    pub fn set_button_led(&mut self, button: InputElement, brightness: u8) -> Result<()> {
        match button {
            InputElement::Play => self.current_button_leds.play = brightness,
            InputElement::Rec => self.current_button_leds.rec = brightness,
            InputElement::Stop => self.current_button_leds.stop = brightness,
            InputElement::Restart => self.current_button_leds.restart = brightness,
            InputElement::Erase => self.current_button_leds.erase = brightness,
            InputElement::Tap => self.current_button_leds.tap = brightness,
            InputElement::Follow => self.current_button_leds.follow = brightness,
            InputElement::ChannelMidi => self.current_button_leds.channel_midi = brightness,
            InputElement::Arranger => self.current_button_leds.arranger = brightness,
            InputElement::ArrowLeft => self.current_button_leds.arrow_left = brightness,
            InputElement::ArrowRight => self.current_button_leds.arrow_right = brightness,
            InputElement::FileSave => self.current_button_leds.file_save = brightness,
            InputElement::Settings => self.current_button_leds.settings = brightness,
            InputElement::Macro => self.current_button_leds.macro_set = brightness,
            InputElement::Auto => self.current_button_leds.auto = brightness,
            InputElement::Plugin => self.current_button_leds.plugin_instance = brightness,
            InputElement::Mixer => self.current_button_leds.mixer = brightness,
            InputElement::Sampling => self.current_button_leds.sampler = brightness,
            InputElement::Volume => self.current_button_leds.volume = brightness,
            InputElement::Swing => self.current_button_leds.swing = brightness,
            InputElement::NoteRepeat => self.current_button_leds.note_repeat = brightness,
            InputElement::Tempo => self.current_button_leds.tempo = brightness,
            InputElement::Lock => self.current_button_leds.lock = brightness,
            InputElement::Pitch => self.current_button_leds.pitch = brightness,
            InputElement::Mod => self.current_button_leds.mod_ = brightness,
            InputElement::Perform => self.current_button_leds.perform = brightness,
            InputElement::Notes => self.current_button_leds.notes = brightness,
            InputElement::Shift => self.current_button_leds.shift = brightness,
            InputElement::FixedVel => self.current_button_leds.fixed_vel = brightness,
            InputElement::PadMode => self.current_button_leds.pad_mode = brightness,
            InputElement::Keyboard => self.current_button_leds.keyboard = brightness,
            InputElement::Chords => self.current_button_leds.chords = brightness,
            InputElement::Step => self.current_button_leds.step = brightness,
            InputElement::Scene => self.current_button_leds.scene = brightness,
            InputElement::Pattern => self.current_button_leds.pattern = brightness,
            InputElement::Events => self.current_button_leds.events = brightness,
            InputElement::Variation => self.current_button_leds.variation = brightness,
            InputElement::Duplicate => self.current_button_leds.duplicate = brightness,
            InputElement::Select => self.current_button_leds.select = brightness,
            InputElement::Solo => self.current_button_leds.solo = brightness,
            InputElement::Mute => self.current_button_leds.mute = brightness,
            InputElement::DisplayButton1 => self.current_button_leds.display_button_1 = brightness,
            InputElement::DisplayButton2 => self.current_button_leds.display_button_2 = brightness,
            InputElement::DisplayButton3 => self.current_button_leds.display_button_3 = brightness,
            InputElement::DisplayButton4 => self.current_button_leds.display_button_4 = brightness,
            InputElement::DisplayButton5 => self.current_button_leds.display_button_5 = brightness,
            InputElement::DisplayButton6 => self.current_button_leds.display_button_6 = brightness,
            InputElement::DisplayButton7 => self.current_button_leds.display_button_7 = brightness,
            InputElement::DisplayButton8 => self.current_button_leds.display_button_8 = brightness,
            // For RGB LEDs, convert brightness to grayscale color
            InputElement::GroupA => {
                self.current_button_leds.group_a = MaschineLEDColor::from_brightness(brightness)
            }
            InputElement::GroupB => {
                self.current_button_leds.group_b = MaschineLEDColor::from_brightness(brightness)
            }
            InputElement::GroupC => {
                self.current_button_leds.group_c = MaschineLEDColor::from_brightness(brightness)
            }
            InputElement::GroupD => {
                self.current_button_leds.group_d = MaschineLEDColor::from_brightness(brightness)
            }
            InputElement::GroupE => {
                self.current_button_leds.group_e = MaschineLEDColor::from_brightness(brightness)
            }
            InputElement::GroupF => {
                self.current_button_leds.group_f = MaschineLEDColor::from_brightness(brightness)
            }
            InputElement::GroupG => {
                self.current_button_leds.group_g = MaschineLEDColor::from_brightness(brightness)
            }
            InputElement::GroupH => {
                self.current_button_leds.group_h = MaschineLEDColor::from_brightness(brightness)
            }
            InputElement::BrowserPlugin => {
                self.current_button_leds.browser_plugin =
                    MaschineLEDColor::from_brightness(brightness)
            }
            InputElement::EncoderUp => {
                self.current_button_leds.nav_up = MaschineLEDColor::from_brightness(brightness)
            }
            InputElement::EncoderLeft => {
                self.current_button_leds.nav_left = MaschineLEDColor::from_brightness(brightness)
            }
            InputElement::EncoderRight => {
                self.current_button_leds.nav_right = MaschineLEDColor::from_brightness(brightness)
            }
            InputElement::EncoderDown => {
                self.current_button_leds.nav_down = MaschineLEDColor::from_brightness(brightness)
            }
            _ => return Ok(()), // Elements that don't have LEDs
        }
        self.led_state_dirty = true;
        self.write_led_state()?;
        Ok(())
    }

    /// Set individual button LED color (for RGB LEDs only)
    pub fn set_button_led_color(
        &mut self,
        button: InputElement,
        color: MaschineLEDColor,
    ) -> Result<()> {
        match button {
            InputElement::GroupA => self.current_button_leds.group_a = color,
            InputElement::GroupB => self.current_button_leds.group_b = color,
            InputElement::GroupC => self.current_button_leds.group_c = color,
            InputElement::GroupD => self.current_button_leds.group_d = color,
            InputElement::GroupE => self.current_button_leds.group_e = color,
            InputElement::GroupF => self.current_button_leds.group_f = color,
            InputElement::GroupG => self.current_button_leds.group_g = color,
            InputElement::GroupH => self.current_button_leds.group_h = color,
            InputElement::BrowserPlugin => self.current_button_leds.browser_plugin = color,
            InputElement::EncoderUp => self.current_button_leds.nav_up = color,
            InputElement::EncoderLeft => self.current_button_leds.nav_left = color,
            InputElement::EncoderRight => self.current_button_leds.nav_right = color,
            InputElement::EncoderDown => self.current_button_leds.nav_down = color,
            _ => return Ok(()), // Elements that don't have RGB LEDs
        }
        self.led_state_dirty = true;
        self.write_led_state()?;
        Ok(())
    }

    /// Set individual pad LED color
    pub fn set_pad_led(&mut self, pad_number: u8, color: MaschineLEDColor) -> Result<()> {
        if pad_number > 15 {
            return Err(MK3Error::InvalidData("Pad number must be 0-15".to_string()));
        }

        let old_color = self.current_pad_leds.pad_leds[pad_number as usize];
        if old_color != color {
            self.current_pad_leds.pad_leds[pad_number as usize] = color;
            self.led_state_dirty = true;
            self.write_led_state()?;
        }
        Ok(())
    }

    /// Set all button LEDs to the same brightness
    pub fn set_all_button_leds(&mut self, brightness: u8) -> Result<()> {
        let mut changed = false;

        // Set all brightness-based LEDs
        if self.current_button_leds.play != brightness {
            self.current_button_leds.play = brightness;
            changed = true;
        }
        // Add more brightness-based buttons as needed

        if changed {
            self.led_state_dirty = true;
            self.write_led_state()?;
        }
        Ok(())
    }

    /// Set all pad LEDs to the same color
    pub fn set_all_pad_leds(&mut self, color: MaschineLEDColor) -> Result<()> {
        let mut changed = false;

        for i in 0..16 {
            if self.current_pad_leds.pad_leds[i] != color {
                self.current_pad_leds.pad_leds[i] = color;
                changed = true;
            }
        }

        if changed {
            self.led_state_dirty = true;
            self.write_led_state()?;
        }
        Ok(())
    }

    /// Turn off all LEDs (set to black/0 brightness)
    pub fn clear_all_leds(&mut self) -> Result<()> {
        self.current_button_leds = ButtonLedState::default();
        self.current_pad_leds = PadLedState::default();
        self.led_state_dirty = true;
        self.write_led_state()
    }

    /// Get current button LED brightness
    pub fn get_button_led_state(&self, button: InputElement) -> u8 {
        match button {
            InputElement::Play => self.current_button_leds.play,
            _ => 0,
        }
    }

    /// Get current pad LED color
    pub fn get_pad_led_color(&self, pad_number: u8) -> MaschineLEDColor {
        if pad_number > 15 {
            return MaschineLEDColor::black();
        }
        self.current_pad_leds.pad_leds[pad_number as usize]
    }

    /// Force send LED changes even if no changes detected
    pub fn flush_led_changes(&mut self) -> Result<()> {
        self.write_led_state()
    }

    /// Read raw input data (for debugging purposes)
    pub fn read_raw_input(&self) -> Result<Vec<u8>> {
        self.read_input()
    }

    // === Helper methods ===

    fn write_led_state(&mut self) -> Result<()> {
        let button_packet = self.current_button_leds.to_packet();
        self.write_led_data(&button_packet)?;

        let pad_packet = self.current_pad_leds.to_packet();
        self.write_led_data(&pad_packet)?;

        self.led_state_dirty = false;
        Ok(())
    }

    fn write_led_data(&self, data: &[u8]) -> Result<()> {
        #[cfg(windows)]
        {
            if let Some(ref hid_dev) = self.hid_device {
                match hid_dev.write(data) {
                    Ok(_) => return Ok(()),
                    Err(e) => {
                        eprintln!("HID LED write failed: {}", e);
                        return Err(MK3Error::Io(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            e,
                        )));
                    }
                }
            }

            let timeout = Duration::from_millis(100);
            match self
                .device_handle
                .write_interrupt(OUTPUT_ENDPOINT, data, timeout)
            {
                Ok(_) => Ok(()),
                Err(e) => Err(MK3Error::Usb(e)),
            }
        }

        #[cfg(unix)]
        {
            let timeout = Duration::from_millis(100);
            match self
                .device_handle
                .write_interrupt(OUTPUT_ENDPOINT, data, timeout)
            {
                Ok(_) => Ok(()),
                Err(e) => Err(MK3Error::Usb(e)),
            }
        }
    }
}

impl Drop for MaschineMK3 {
    fn drop(&mut self) {
        // Stop input monitoring
        let _ = self.stop_input_monitoring();

        // Release interfaces on cleanup
        let _ = self.device_handle.release_interface(HID_INTERFACE);
        let _ = self.device_handle.release_interface(DISPLAY_INTERFACE);
    }
}
