use crate::error::{MK3Error, Result};
use crate::input::{InputState, PadState};
use crate::output::{ButtonLedState, PadLedState, DisplayPacket, Rgb565};
use hidapi::{HidApi, HidDevice};

/// Native Instruments Maschine MK3 USB constants
const VENDOR_ID: u16 = 0x17CC;
const PRODUCT_ID: u16 = 0x1600;

pub struct MaschineMK3Hid {
    hid_device: HidDevice,
    _hid_api: HidApi,
}

impl MaschineMK3Hid {
    /// Find and connect to the first available Maschine MK3 device using HID API
    pub fn new() -> Result<Self> {
        let hid_api = HidApi::new()
            .map_err(|e| MK3Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        // List all HID devices to find the MK3
        let devices = hid_api.device_list();
        let mut mk3_device_info = None;

        for device_info in devices {
            if device_info.vendor_id() == VENDOR_ID && device_info.product_id() == PRODUCT_ID {
                // Look for the HID interface (usually interface 4)
                if device_info.interface_number() == 4 {
                    mk3_device_info = Some(device_info);
                    break;
                }
                // Fallback: take any MK3 device if no interface 4 found
                if mk3_device_info.is_none() {
                    mk3_device_info = Some(device_info);
                }
            }
        }

        let device_info = mk3_device_info.ok_or(MK3Error::DeviceNotFound)?;
        let hid_device = device_info
            .open_device(&hid_api)
            .map_err(|e| MK3Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        Ok(Self {
            hid_device,
            _hid_api: hid_api,
        })
    }

    /// Read raw input data from the device
    pub fn read_input_raw(&self) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; 64]; // Max packet size

        match self.hid_device.read_timeout(&mut buffer, 100) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    buffer.truncate(bytes_read);
                    Ok(buffer)
                } else {
                    Ok(Vec::new()) // No data available
                }
            }
            Err(e) => Err(MK3Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e,
            ))),
        }
    }

    /// Read and parse input state (buttons/knobs)
    pub fn read_input_state(&self) -> Result<Option<InputState>> {
        let data = self.read_input_raw()?;
        if data.is_empty() {
            return Ok(None);
        }

        match data[0] {
            0x01 => Ok(Some(InputState::from_button_packet(&data)?)),
            _ => Ok(None), // Unknown packet type or pad data
        }
    }

    /// Read and parse pad state
    pub fn read_pad_state(&self) -> Result<Option<PadState>> {
        let data = self.read_input_raw()?;
        if data.is_empty() {
            return Ok(None);
        }

        match data[0] {
            0x02 => Ok(Some(PadState::from_pad_packet(&data)?)),
            _ => Ok(None), // Unknown packet type or button data
        }
    }

    /// Write raw LED data to the device
    pub fn write_leds_raw(&self, data: &[u8]) -> Result<()> {
        self.hid_device
            .write(data)
            .map_err(|e| MK3Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        Ok(())
    }

    /// Write button LED state
    pub fn write_button_leds(&self, state: &ButtonLedState) -> Result<()> {
        let packet = state.to_packet();
        self.write_leds_raw(&packet)
    }

    /// Write pad LED state
    pub fn write_pad_leds(&self, state: &PadLedState) -> Result<()> {
        let packet = state.to_packet();
        self.write_leds_raw(&packet)
    }

    /// Write display data to the device
    pub fn write_display_raw(&self, data: &[u8]) -> Result<()> {
        // Note: Display uses a different endpoint that might not work through HID
        // This is experimental - may need direct USB access
        self.hid_device.write(data)
            .map_err(|e| MK3Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        Ok(())
    }

    /// Write a display packet to a specific display
    pub fn write_display_packet(&self, packet: &DisplayPacket) -> Result<()> {
        let data = packet.to_packet();
        self.write_display_raw(&data)
    }

    /// Clear a display (fill with black)
    pub fn clear_display(&self, display_id: u8) -> Result<()> {
        let mut packet = DisplayPacket::new(display_id, 0, 0, 480, 272);
        
        // Fill with black pixels
        let black_pixels = vec![Rgb565::black(); 480 * 272];
        packet.add_pixels(black_pixels);
        packet.finish();
        
        self.write_display_packet(&packet)
    }

    /// Fill display with a solid color
    pub fn fill_display(&self, display_id: u8, color: Rgb565) -> Result<()> {
        let mut packet = DisplayPacket::new(display_id, 0, 0, 480, 272);
        
        // Use repeat command for efficiency
        packet.add_repeat(color, color, 480 * 272 / 2);
        packet.finish();
        
        self.write_display_packet(&packet)
    }

    /// Draw a rectangle on the display
    pub fn draw_rect(&self, display_id: u8, x: u16, y: u16, width: u16, height: u16, color: Rgb565) -> Result<()> {
        let mut packet = DisplayPacket::new(display_id, x, y, width, height);
        
        // Fill rectangle with color
        let pixels = vec![color; (width as usize) * (height as usize)];
        packet.add_pixels(pixels);
        packet.finish();
        
        self.write_display_packet(&packet)
    }

    /// Draw a simple pattern or image to display
    pub fn draw_pattern(&self, display_id: u8, pattern: &[Rgb565], width: u16, height: u16, x: u16, y: u16) -> Result<()> {
        if pattern.len() != (width as usize * height as usize) {
            return Err(MK3Error::InvalidPacket);
        }
        
        let mut packet = DisplayPacket::new(display_id, x, y, width, height);
        packet.add_pixels(pattern.to_vec());
        packet.finish();
        
        self.write_display_packet(&packet)
    }

    /// Get device information for debugging
    pub fn device_info(&self) -> Result<String> {
        let info = self
            .hid_device
            .get_device_info()
            .map_err(|e| MK3Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        let manufacturer = info.manufacturer_string().unwrap_or("Unknown");
        let product = info.product_string().unwrap_or("Unknown");

        Ok(format!(
            "Maschine MK3 (HID) - Manufacturer: {}, Product: {}, VID: 0x{:04X}, PID: 0x{:04X}",
            manufacturer,
            product,
            info.vendor_id(),
            info.product_id()
        ))
    }
}
