use mk3_hal::MaschineMK3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Display Header Fix Test");
    println!("==========================");
    
    let device = MaschineMK3::new()?;
    println!("✅ Connected: {}", device.device_info()?);

    println!("\n🧪 Testing corrected header format...");
    
    let mut packet = Vec::new();
    
    // Header Part 1 (16 bytes) - CORRECTED BASED ON DOCS
    // Headers 1-2: 0x84, 0x00 
    // Headers 3-4: Display ID (0x00=left), 0x60
    // Headers 5-8: All 0x00
    packet.extend_from_slice(&[
        0x84, 0x00,  // Header 1-2
        0x00, 0x60,  // Header 3-4: Left display (0x00), then 0x60
        0x00, 0x00,  // Header 5-6
        0x00, 0x00,  // Header 7-8  
        0x00, 0x00,  // Header 9-10
        0x00, 0x00,  // Header 11-12
        0x00, 0x00,  // Header 13-14
        0x00, 0x00,  // Header 15-16
    ]);
    
    // Header Part 2 (16 bytes) - Coordinates in big-endian (MSB first)
    // Small test area: 10x10 at position (50, 50)
    packet.extend_from_slice(&[
        0x00, 0x32,  // X start: 50 (MSB, LSB)
        0x00, 0x32,  // Y start: 50 (MSB, LSB)
        0x00, 0x0A,  // Width: 10 (MSB, LSB)
        0x00, 0x0A,  // Height: 10 (MSB, LSB)
        0x00, 0x00,  // Padding
        0x00, 0x00,  // Padding
        0x00, 0x00,  // Padding
        0x00, 0x00,  // Padding
    ]);
    
    println!("📦 Testing with Command 0x00 (direct pixel data)");
    
    // Command 0x00: Direct pixel transmission
    // 10x10 = 100 pixels, but command works in pairs, so 50 pairs
    packet.extend_from_slice(&[
        0x00,        // Command 0x00: Direct pixels
        0x00, 0x00, 0x32,  // 50 pairs (24-bit, MSB first)
    ]);
    
    // Add 50 pairs of pixels (100 pixels total) - alternating red and blue
    for i in 0..50 {
        if i % 2 == 0 {
            // Red pixel pair
            packet.extend_from_slice(&[0x00, 0xF8, 0x00, 0xF8]); // Red, Red
        } else {
            // Blue pixel pair  
            packet.extend_from_slice(&[0x1F, 0x00, 0x1F, 0x00]); // Blue, Blue
        }
    }
    
    // Command 0x03: Blit (required to actually display)
    packet.extend_from_slice(&[0x03, 0x00, 0x00, 0x00]);
    
    // Command 0x40: End transmission
    packet.extend_from_slice(&[0x40, 0x00, 0x00, 0x00]);
    
    println!("📦 Packet size: {} bytes", packet.len());
    
    match device.write_display(&packet) {
        Ok(_) => println!("✅ Header fix packet sent!"),
        Err(e) => println!("❌ Failed: {}", e),
    }
    
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    println!("\n🧪 Testing pure black square (to rule out white override)");
    
    let mut black_packet = Vec::new();
    
    // Same corrected headers
    black_packet.extend_from_slice(&packet[..32]);
    
    // Command 0x01: Repeat pixels (easier for solid color)
    black_packet.extend_from_slice(&[
        0x01,        // Command 0x01: Repeat pixels
        0x00, 0x00, 0x32,  // 50 repetitions
    ]);
    
    // Two black pixels (RGB565: 0x0000)
    black_packet.extend_from_slice(&[
        0x00, 0x00,  // First pixel: Black
        0x00, 0x00,  // Second pixel: Black
    ]);
    
    // Blit and end
    black_packet.extend_from_slice(&[0x03, 0x00, 0x00, 0x00]);
    black_packet.extend_from_slice(&[0x40, 0x00, 0x00, 0x00]);
    
    match device.write_display(&black_packet) {
        Ok(_) => println!("✅ Black square sent!"),
        Err(e) => println!("❌ Failed: {}", e),
    }
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    
    println!("\n🏁 Header fix test complete!");
    println!("💡 Look for a 10x10 square at position (50,50):");
    println!("   - First test: Red/blue checkerboard pattern");
    println!("   - Second test: Pure black square");
    
    Ok(())
}
