use mk3_hal::{MaschineMK3, MK3Error};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 MK3 Correct Colors Test - Using Discovered Channel Mapping");
    
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

    println!("\n🎯 Testing with corrected color mapping...");
    println!("   Discovery: RED→BLUE, GREEN→RED, BLUE→GREEN");
    
    // Test corrected colors
    let colors = [
        ("Corrected RED", rgb565_corrected(255, 0, 0)),      // Should show red
        ("Corrected GREEN", rgb565_corrected(0, 255, 0)),    // Should show green  
        ("Corrected BLUE", rgb565_corrected(0, 0, 255)),     // Should show blue
        ("Corrected YELLOW", rgb565_corrected(255, 255, 0)), // Should show yellow
        ("Corrected CYAN", rgb565_corrected(0, 255, 255)),   // Should show cyan
        ("Corrected MAGENTA", rgb565_corrected(255, 0, 255)), // Should show magenta
        ("WHITE", 0xFFFF),                                   // Should show white
        ("BLACK", 0x0000),                                   // Should show black
    ];

    for (name, color) in colors.iter() {
        println!("\n🎯 Testing: {} (0x{:04X})", name, color);
        
        // Draw a large rectangle in center of right screen
        let packet = create_test_packet(1, 100, 150, 180, 72, *color); // Right screen, large rect
        device.write_display(&packet)?;
        
        std::thread::sleep(Duration::from_secs(4));
        
        // Clear with black
        let clear_packet = create_test_packet(1, 100, 150, 180, 72, 0x0000);
        device.write_display(&clear_packet)?;
        std::thread::sleep(Duration::from_millis(500));
    }

    println!("\n✅ Corrected color test complete!");
    
    Ok(())
}

/// Convert RGB to corrected RGB565 format for MK3
/// Based on discovery: RED→BLUE, GREEN→RED, BLUE→GREEN channels
fn rgb565_corrected(r: u8, g: u8, b: u8) -> u16 {
    // Rotate channels: R→B, G→R, B→G
    let corrected_r = b; // Red channel gets blue input
    let corrected_g = r; // Green channel gets red input  
    let corrected_b = g; // Blue channel gets green input
    
    // Convert to RGB565
    let r5 = (corrected_r >> 3) as u16;
    let g6 = (corrected_g >> 2) as u16;
    let b5 = (corrected_b >> 3) as u16;
    
    (r5 << 11) | (g6 << 5) | b5
}

/// Create a test packet with specific dimensions
fn create_test_packet(display_id: u8, y: u16, x: u8, width: u8, height: u8, color: u16) -> Vec<u8> {
    vec![
        0x84, 0x00, display_id, 0x60,    // Header
        0x00, 0x00, 0x00, 0x00,          // Padding
        (y & 0xFF) as u8, (y >> 8) as u8, height, 0x00,  // Y coordinate and height
        0x00, x, 0x00, width,            // X and width
        0x01, 0x00, 0x00, width,         // Repeat info
        (color & 0xFF) as u8, (color >> 8) as u8,      // Color (little endian)
        0x00, 0x00, 0x03, 0x00,          // RLE count
        0x00, 0x00, 0x40, 0x00,          // End command
        display_id, 0x00                  // Display ID terminator
    ]
}
