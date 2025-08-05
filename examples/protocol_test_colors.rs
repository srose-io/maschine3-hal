use mk3_hal::{MK3Error, MaschineMK3};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 MK3 Color Debug Test - Finding Correct RGB565 Format");

    let device = match MaschineMK3::new() {
        Ok(device) => {
            println!("✅ Connected: {}", device.device_info()?);
            device
        }
        Err(MK3Error::DeviceNotFound) => {
            println!("❌ No Maschine MK3 found");
            return Ok(());
        }
        Err(e) => {
            println!("❌ Connection error: {}", e);
            return Ok(());
        }
    };

    println!("\n🔍 Testing different color encodings...");

    // Test pure colors with different encodings
    let colors = [
        ("Pure RED (std RGB565)", 0xF800),   // Standard: R=31, G=0, B=0
        ("Pure RED (swapped)", 0x001F),      // Swapped: might be BGR
        ("Pure GREEN (std RGB565)", 0x07E0), // Standard: R=0, G=63, B=0
        ("Pure GREEN (alt)", 0x07C0),        // Alternative green
        ("Pure BLUE (std RGB565)", 0x001F),  // Standard: R=0, G=0, B=31
        ("Pure BLUE (swapped)", 0xF800),     // Swapped
        ("WHITE", 0xFFFF),                   // All bits set
        ("BLACK", 0x0000),                   // All bits clear
    ];

    for (name, color) in colors.iter() {
        println!("\n🎯 Testing: {} (0x{:04X})", name, color);

        // Draw a large rectangle in center of right screen
        let packet = create_test_packet(1, 136, 120, 100, 32, *color); // Right screen, centered
        device.write_display(&packet)?;

        std::thread::sleep(Duration::from_secs(5));
    }

    println!("\n✅ Color test complete! Check which colors appeared correctly.");

    Ok(())
}

/// Create a test packet with specific dimensions for color testing
fn create_test_packet(display_id: u8, y: u16, x: u8, width: u8, height: u8, color: u16) -> Vec<u8> {
    vec![
        0x84,
        0x00,
        display_id,
        0x60, // Header
        0x00,
        0x00,
        0x00,
        0x00, // Padding
        (y & 0xFF) as u8,
        (y >> 8) as u8,
        height,
        0x00, // Y coordinate and height
        0x00,
        x,
        0x00,
        width, // X and width
        0x01,
        0x00,
        0x00,
        width, // Height and repeat width
        (color & 0xFF) as u8,
        (color >> 8) as u8, // Color (little endian)
        0x00,
        0x00,
        0x03,
        0x00, // RLE count
        0x00,
        0x00,
        0x40,
        0x00, // End command
        display_id,
        0x00, // Display ID terminator
    ]
}
