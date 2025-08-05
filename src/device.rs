use crate::error::{MK3Error, Result};
use crate::output::{DisplayPacket, Rgb565};
use rusb::{Context, Device, DeviceHandle, UsbContext};
use std::time::Duration;

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
    context: Context,
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

        Ok(Self {
            device_handle,
            context,
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
        let timeout = Duration::from_millis(100);
        self.device_handle
            .write_interrupt(OUTPUT_ENDPOINT, data, timeout)?;
        Ok(())
    }

    /// Write display data to the device
    pub fn write_display(&self, data: &[u8]) -> Result<()> {
        let timeout = Duration::from_millis(1000); // Longer timeout for display data
        self.device_handle
            .write_bulk(DISPLAY_ENDPOINT, data, timeout)?;
        Ok(())
    }

    /// Write a display packet to a specific display
    pub fn write_display_packet(&self, packet: &DisplayPacket) -> Result<()> {
        let data = packet.to_packet();
        self.write_display(&data)
    }

    /// Clear a display (fill with black)
    pub fn clear_display(&self, display_id: u8) -> Result<()> {
        let mut packet = DisplayPacket::new(display_id, 0, 0, 480, 272);

        // Use repeat command for efficiency - fill entire screen with black
        packet.add_repeat(Rgb565::black(), Rgb565::black(), (480 * 272) / 2);
        packet.finish();

        self.write_display_packet(&packet)
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
}

impl Drop for MaschineMK3 {
    fn drop(&mut self) {
        // Release interfaces on cleanup
        let _ = self.device_handle.release_interface(HID_INTERFACE);
        let _ = self.device_handle.release_interface(DISPLAY_INTERFACE);
    }
}
