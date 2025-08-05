use crate::error::{MK3Error, Result};
use crate::output::{DisplayPacket, Rgb565};
use crate::{ButtonLedState, PadLedState};
use rusb::{Context, Device, DeviceHandle, UsbContext};
use std::time::Duration;

#[cfg(target_os = "windows")]
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

pub struct MaschineMK3 {
    device_handle: DeviceHandle<Context>,
    pub context: Context,
    #[cfg(target_os = "windows")]
    hid_device: Option<HidDevice>,
    #[cfg(target_os = "windows")]
    _hid_api: Option<HidApi>,
}

impl MaschineMK3 {
    /// Find and connect to the first available Maschine MK3 device
    pub fn new() -> Result<Self> {
        let context = Context::new()?;
        let device = Self::find_device(&context)?;
        let mut device_handle = device.open()?;

        // Debug: print device configuration info
        Self::debug_device_info(&device)?;

        // Enable automatic kernel driver detachment (skip on Windows where it's not supported)
        #[cfg(not(target_os = "windows"))]
        device_handle.set_auto_detach_kernel_driver(true)?;

        // Try to detach kernel drivers and claim interfaces
        Self::claim_interface_with_detach(&mut device_handle, HID_INTERFACE)?;

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

        // Try to open HID device for LED communication on Windows
        #[cfg(target_os = "windows")]
        let (hid_device, hid_api) = {
            match HidApi::new() {
                Ok(api) => {
                    let devices = api.device_list();
                    let mut hid_dev = None;
                    
                    for device_info in devices {
                        if device_info.vendor_id() == VENDOR_ID && device_info.product_id() == PRODUCT_ID {
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
            #[cfg(target_os = "windows")]
            hid_device,
            #[cfg(target_os = "windows")]
            _hid_api: hid_api,
        })
    }

    /// Detach kernel driver and claim interface
    fn claim_interface_with_detach(
        handle: &mut DeviceHandle<Context>,
        interface: u8,
    ) -> Result<()> {
        println!("🔧 Attempting to claim interface {}", interface);

        // On Windows, try direct claim first (kernel driver detachment not supported)
        #[cfg(target_os = "windows")]
        {
            match handle.claim_interface(interface) {
                Ok(()) => {
                    println!("✅ Successfully claimed interface {}", interface);
                    return Ok(());
                }
                Err(e) => {
                    println!("❌ Failed to claim interface {}: {:?}", interface, e);
                    return Err(MK3Error::Usb(e));
                }
            }
        }

        // On non-Windows systems, try to detach kernel driver first
        #[cfg(not(target_os = "windows"))]
        {
            // Try to detach kernel driver (ignore errors)
            let _ = handle.detach_kernel_driver(interface);

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
    pub fn read_input(&self) -> Result<Vec<u8>> {
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
    pub fn write_leds(&self, data: &[u8]) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            // On Windows, use HID API for LED communication (interface 4 requires HID driver)
            if let Some(ref hid_dev) = self.hid_device {
                match hid_dev.write(data) {
                    Ok(_) => return Ok(()),
                    Err(e) => {
                        eprintln!("HID LED write failed: {}", e);
                        return Err(MK3Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)));
                    }
                }
            }
        }
        
        // Fallback to USB interrupt transfer (for non-Windows or if HID failed)
        let timeout = Duration::from_millis(100);
        match self.device_handle
            .write_interrupt(OUTPUT_ENDPOINT, data, timeout) {
            Ok(_) => Ok(()),
            Err(e) => Err(MK3Error::Usb(e))
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
}

impl Drop for MaschineMK3 {
    fn drop(&mut self) {
        // Release interfaces on cleanup
        let _ = self.device_handle.release_interface(HID_INTERFACE);
        let _ = self.device_handle.release_interface(DISPLAY_INTERFACE);
    }
}
