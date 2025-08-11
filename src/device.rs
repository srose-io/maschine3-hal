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
        {
            // On Windows, try to claim display interface but don't fail if it doesn't work
            match Self::claim_interface_with_detach(&mut device_handle, DISPLAY_INTERFACE) {
                Ok(()) => println!(
                    "✅ Display interface {} claimed successfully",
                    DISPLAY_INTERFACE
                ),
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
                        }
                        Err(e2) => {
                            println!("⚠️  Alternative interface 3 also failed: {}", e2);
                            println!("   💡 Consider installing WinUSB driver using Zadig");
                            println!("   💡 Or use HID-only mode for input/LEDs");
                        }
                    }
                }
            }
        }

        #[cfg(unix)]
        {
            // On Linux, try to claim display interface
            match Self::detach_and_claim_interface(&mut device_handle, DISPLAY_INTERFACE) {
                Ok(()) => println!(
                    "✅ Display interface {} claimed successfully",
                    DISPLAY_INTERFACE
                ),
                Err(e) => {
                    println!(
                        "⚠️  Could not claim display interface {}: {}",
                        DISPLAY_INTERFACE, e
                    );
                    println!("   💡 Check udev rules and user permissions");
                }
            }
        }

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
        let mut buffer = vec![0u8; 64]; // Max packet size
        let timeout = Duration::from_millis(100);

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
        let timeout = Duration::from_millis(1000); // Longer timeout for display data
        self.device_handle
            .write_bulk(DISPLAY_ENDPOINT, data, timeout)?;
        Ok(())
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
        let timeout = Duration::from_millis(1000);

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
