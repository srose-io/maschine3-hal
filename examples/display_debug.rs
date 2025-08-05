use mk3_hal::{MaschineMK3, MK3Error, Rgb565};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Display Protocol Debug Test");
    println!("===============================");
    
    let device = match MaschineMK3::new() {
        Ok(device) => {
            println!("✅ Connected: {}", device.device_info()?);
            device
        }
        Err(e) => {
            println!("❌ Connection error: {}", e);
            return Ok(());
        }
    };

    println!("\n🧪 Testing minimal display packet...");
    
    // Create a minimal test packet based on docs
    // Header Part 1: [0x84, 0x00, display_id, 0x60, 0x00, 0x00, 0x00, 0x00, ...]
    let mut minimal_packet = Vec::new();
    
    // Header Part 1 (16 bytes) - Fixed values from docs
    minimal_packet.extend_from_slice(&[
        0x84, 0x00,  // Header 1-2: Always 0x84, 0x00
        0x00, 0x00,  // Header 3-4: Display ID (0x00 = left)
        0x60, 0x00,  // Header 5-6: Always 0x60, 0x00  
        0x00, 0x00,  // Header 7-8: Always 0x00, 0x00
        0x00, 0x00,  // Header 9-10: Always 0x00, 0x00
        0x00, 0x00,  // Header 11-12: Always 0x00, 0x00
        0x00, 0x00,  // Header 13-14: Always 0x00, 0x00
        0x00, 0x00,  // Header 15-16: Always 0x00, 0x00
    ]);
    
    // Header Part 2 (16 bytes) - Coordinates and dimensions
    // Let's try a small 10x10 pixel area at (0,0)
    minimal_packet.extend_from_slice(&[
        0x00, 0x00,  // X start MSB, LSB (0)
        0x00, 0x00,  // Y start MSB, LSB (0)  
        0x00, 0x0A,  // Width MSB, LSB (10)
        0x00, 0x0A,  // Height MSB, LSB (10)
        0x00, 0x00,  // Padding
        0x00, 0x00,  // Padding
        0x00, 0x00,  // Padding
        0x00, 0x00,  // Padding
    ]);
    
    // Command: Fill with solid color using repeat command (0x01)
    // Repeat 50 times (10x10 / 2 pixels per repeat)
    minimal_packet.extend_from_slice(&[
        0x01,        // Command 0x01: Repeat pixels
        0x00, 0x00, 0x32,  // Count: 50 (24-bit, MSB first)
    ]);
    
    // Two pixels: both red (RGB565: 0xF800)
    minimal_packet.extend_from_slice(&[
        0x00, 0xF8,  // First pixel: Red (RGB565 little-endian)
        0x00, 0xF8,  // Second pixel: Red (RGB565 little-endian)
    ]);
    
    // End transmission command
    minimal_packet.extend_from_slice(&[
        0x40, 0x00, 0x00, 0x00,  // Command 0x40: End of data
    ]);
    
    println!("📦 Minimal packet size: {} bytes", minimal_packet.len());
    println!("📦 Packet hex: {:02x?}", &minimal_packet[..minimal_packet.len().min(64)]);
    
    match device.write_display(&minimal_packet) {
        Ok(_) => println!("✅ Minimal packet sent successfully!"),
        Err(e) => println!("❌ Failed to send minimal packet: {}", e),
    }
    
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    println!("\n🧪 Testing with 0x03 blit command...");
    
    // Try adding a 0x03 command to blit the data
    let mut blit_packet = Vec::new();
    
    // Same header as before
    blit_packet.extend_from_slice(&minimal_packet[..32]);
    
    // Same repeat command
    blit_packet.extend_from_slice(&minimal_packet[32..44]);
    
    // Add 0x03 command before end transmission
    blit_packet.extend_from_slice(&[
        0x03, 0x00, 0x00, 0x00,  // Command 0x03: Unknown (probably blit)
    ]);
    
    // End transmission
    blit_packet.extend_from_slice(&[
        0x40, 0x00, 0x00, 0x00,  // Command 0x40: End of data
    ]);
    
    println!("📦 Blit packet size: {} bytes", blit_packet.len());
    
    match device.write_display(&blit_packet) {
        Ok(_) => println!("✅ Blit packet sent successfully!"),
        Err(e) => println!("❌ Failed to send blit packet: {}", e),
    }
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    
    println!("\n🏁 Debug test complete!");
    println!("💡 Check the left display for a small red square in the top-left corner");
    
    Ok(())
}
