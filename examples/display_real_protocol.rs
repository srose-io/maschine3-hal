use mk3_hal::MaschineMK3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Real Protocol Structure Test");
    println!("===============================");
    
    let device = MaschineMK3::new()?;
    println!("✅ Connected: {}", device.device_info()?);

    println!("\n🧪 Testing with REAL packet structure from capture...");
    
    let mut packet = Vec::new();
    
    // Header Part 1 (16 bytes) - EXACT structure from real packet at offset 0x27
    packet.extend_from_slice(&[
        0x84, 0x00,  // Header 1-2: Always 0x84, 0x00
        0x00,        // Header 3: Display ID (0x00 = left)
        0x60,        // Header 4: Always 0x60  
        0x00, 0x00,  // Header 5-6: Always 0x00
        0x00, 0x00,  // Header 7-8: Always 0x00
        0x00, 0x00,  // Header 9-10: Always 0x00
        0x00, 0x00,  // Header 11-12: Always 0x00
        0x00, 0x00,  // Header 13-14: Always 0x00
        0x00, 0x00,  // Header 15-16: Always 0x00
    ]);
    
    // Header Part 2 (16 bytes) - EXACT structure from real packet
    packet.extend_from_slice(&[
        0x00, 0x00,  // X start: 0
        0x00, 0x00,  // Y start: 0
        0x01, 0xE0,  // Width: 480 (full screen width)
        0x01, 0x10,  // Height: 272 (full screen height)
        0x01, 0x00,  // Unknown fields from real packet
        0x70, 0xBE,  // Unknown fields from real packet
        0x00, 0x00,  // Padding
        0x00, 0x00,  // Padding
    ]);
    
    println!("📦 Testing with small red square first...");
    
    // Let's start small but use the real structure - 100x100 red square
    let mut small_packet = packet.clone();
    // Modify dimensions for small test
    small_packet[24] = 0x00; // Width MSB
    small_packet[25] = 0x64; // Width LSB (100)
    small_packet[26] = 0x00; // Height MSB  
    small_packet[27] = 0x64; // Height LSB (100)
    
    // Command 0x01: Repeat pixels (100x100 = 10000 pixels / 2 = 5000 pairs)
    small_packet.extend_from_slice(&[
        0x01,        // Command 0x01: Repeat pixels
        0x00, 0x13, 0x88,  // Count: 5000 pairs (24-bit big-endian)
    ]);
    
    // Two red pixels (RGB565: 0xF800 in little-endian)
    small_packet.extend_from_slice(&[
        0x00, 0xF8,  // First pixel: Red
        0x00, 0xF8,  // Second pixel: Red
    ]);
    
    // Command 0x03: Blit
    small_packet.extend_from_slice(&[0x03, 0x00, 0x00, 0x00]);
    
    // Command 0x40: End transmission
    small_packet.extend_from_slice(&[0x40, 0x00, 0x00, 0x00]);
    
    println!("📦 Packet size: {} bytes", small_packet.len());
    
    match device.write_display(&small_packet) {
        Ok(_) => println!("✅ Real structure red square sent!"),
        Err(e) => println!("❌ Failed: {}", e),
    }
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    
    // Test 2: Try pure black to see if it's different
    println!("\n🧪 Testing black square with real structure...");
    
    let mut black_packet = small_packet.clone();
    // Replace the pixel data with black
    let black_start = black_packet.len() - 12; // 4 bytes pixels + 8 bytes commands
    black_packet[black_start] = 0x00;     // Black pixel 1 LSB
    black_packet[black_start + 1] = 0x00; // Black pixel 1 MSB
    black_packet[black_start + 2] = 0x00; // Black pixel 2 LSB  
    black_packet[black_start + 3] = 0x00; // Black pixel 2 MSB
    
    match device.write_display(&black_packet) {
        Ok(_) => println!("✅ Real structure black square sent!"),
        Err(e) => println!("❌ Failed: {}", e),
    }
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    
    // Test 3: Try blue to confirm color capability
    println!("\n🧪 Testing blue square with real structure...");
    
    let mut blue_packet = small_packet.clone();
    let blue_start = blue_packet.len() - 12;
    blue_packet[blue_start] = 0x1F;     // Blue pixel 1 LSB (0x001F)
    blue_packet[blue_start + 1] = 0x00; // Blue pixel 1 MSB
    blue_packet[blue_start + 2] = 0x1F; // Blue pixel 2 LSB
    blue_packet[blue_start + 3] = 0x00; // Blue pixel 2 MSB
    
    match device.write_display(&blue_packet) {
        Ok(_) => println!("✅ Real structure blue square sent!"),
        Err(e) => println!("❌ Failed: {}", e),
    }
    
    println!("\n🏁 Real protocol test complete!");
    println!("💡 Look for a 100x100 square in top-left corner:");
    println!("   - First: Red square");
    println!("   - Second: Black square (should be visible if not all white)");
    println!("   - Third: Blue square");
    println!("\n❓ Did you see any actual colored squares this time?");
    
    Ok(())
}
