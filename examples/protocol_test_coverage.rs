use mk3_hal::{MK3Error, MaschineMK3};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📐 MK3 Coverage Test - Finding Correct Height/Fill Format");

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

    println!("\n📏 Testing different height values and coverage patterns...");

    // Clear screen first
    println!("\n⚫ Clearing right screen");
    let clear_packet = create_test_packet(1, 0, 0, 255, 255, rgb565_corrected(0, 0, 0));
    device.write_display(&clear_packet)?;
    std::thread::sleep(Duration::from_secs(1));

    // Test 1: Different height values for same rectangle
    let heights = [1];

    for (i, &height) in heights.iter().enumerate() {
        println!("\n📊 Test {}: Height = {} pixels", i + 1, height);

        let y = 10 + (i * 35) as u16; // Space them out vertically
        let packet = create_test_packet(
            1,                           // Right screen
            y,                           // Y position
            50,                          // X position
            200,                         // Width
            height,                      // Height to test
            rgb565_corrected(255, 0, 0), // Red
        );

        device.write_display(&packet)?;
        std::thread::sleep(Duration::from_millis(500));
    }

    std::thread::sleep(Duration::from_secs(3));

    // Test 2: Try overlapping fills to see if we can build solid areas
    println!("\n🔄 Test: Overlapping fills for solid coverage");

    // Clear left screen
    let clear_left = create_test_packet(0, 0, 0, 255, 255, rgb565_corrected(0, 0, 0));
    device.write_display(&clear_left)?;
    std::thread::sleep(Duration::from_millis(500));

    // Fill a 100x100 area with overlapping 1-pixel high strips
    let start_y = 50;
    let end_y = 150;
    let start_x = 100;
    let width = 100;

    for y in start_y..end_y {
        let packet = create_test_packet(
            0,                           // Left screen
            y,                           // Y position
            start_x,                     // X position
            width,                       // Width
            1,                           // Height = 1 pixel
            rgb565_corrected(0, 255, 0), // Green
        );
        device.write_display(&packet)?;

        // Small delay to avoid overwhelming device
        if y % 10 == 0 {
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    println!("\n✅ Coverage test complete! Check the results:");
    println!("   Right screen: Different height rectangles");
    println!("   Left screen: 100x100 solid green square (if overlapping works)");

    std::thread::sleep(Duration::from_secs(5));

    Ok(())
}

/// Convert RGB to corrected RGB565 format for MK3
fn rgb565_corrected(r: u8, g: u8, b: u8) -> u16 {
    // Rotate channels: R→B, G→R, B→G
    let corrected_r = b;
    let corrected_g = r;
    let corrected_b = g;

    // Convert to RGB565
    let r5 = (corrected_r >> 3) as u16;
    let g6 = (corrected_g >> 2) as u16;
    let b5 = (corrected_b >> 3) as u16;

    (r5 << 11) | (g6 << 5) | b5
}

/// Create a test packet - experimenting with height field
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
        0x00, // Y coordinate and height (trying height here)
        0x00,
        x,
        0x00,
        width, // X and width
        0x01,
        0x00,
        0x00,
        width, // Height field again?
        (color & 0xFF) as u8,
        (color >> 8) as u8, // Color
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
