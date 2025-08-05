use mk3_hal::MaschineMK3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Display Protocol Debug Test v2");
    println!("==================================");
    
    let device = MaschineMK3::new()?;
    println!("✅ Connected: {}", device.device_info()?);

    // Test 1: Try different RGB565 byte orders
    println!("\n🧪 Test 1: Different RGB565 byte orders");
    
    let tests = vec![
        ("Little-endian 0xF800 (red)", vec![0x00, 0xF8]),
        ("Big-endian 0xF800 (red)", vec![0xF8, 0x00]),
        ("Little-endian 0x07E0 (green)", vec![0xE0, 0x07]),
        ("Big-endian 0x07E0 (green)", vec![0x07, 0xE0]),
        ("Little-endian 0x001F (blue)", vec![0x1F, 0x00]),
        ("Big-endian 0x001F (blue)", vec![0x00, 0x1F]),
    ];
    
    for (i, (desc, pixel_bytes)) in tests.iter().enumerate() {
        println!("📤 Test {}: {}", i + 1, desc);
        
        let mut packet = Vec::new();
        
        // Header Part 1 (16 bytes)
        packet.extend_from_slice(&[
            0x84, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        
        // Header Part 2 (16 bytes) - 5x5 pixel area at different positions
        let x = (i * 10) as u16;
        let y = (i * 10) as u16;
        packet.extend_from_slice(&[
            (x >> 8) as u8, (x & 0xFF) as u8,      // X position
            (y >> 8) as u8, (y & 0xFF) as u8,      // Y position
            0x00, 0x05,                             // Width: 5
            0x00, 0x05,                             // Height: 5
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        
        // Command: Repeat pixels (5x5 = 25 pixels, but repeat command uses pairs)
        packet.extend_from_slice(&[
            0x01,        // Command 0x01: Repeat pixels
            0x00, 0x00, 0x0C,  // Count: 12 (covers 24 pixels, close to 25)
        ]);
        
        // Two pixels with the test color
        packet.extend_from_slice(&pixel_bytes);
        packet.extend_from_slice(&pixel_bytes);
        
        // Add 0x03 command (might be required for blitting)
        packet.extend_from_slice(&[0x03, 0x00, 0x00, 0x00]);
        
        // End transmission
        packet.extend_from_slice(&[0x40, 0x00, 0x00, 0x00]);
        
        match device.write_display(&packet) {
            Ok(_) => println!("   ✅ Sent {} bytes", packet.len()),
            Err(e) => println!("   ❌ Failed: {}", e),
        }
        
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
    
    println!("\n🧪 Test 2: Full screen color");
    
    // Try filling the entire screen with a bright color
    let mut full_packet = Vec::new();
    
    // Header Part 1
    full_packet.extend_from_slice(&[
        0x84, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
    
    // Header Part 2 - Full screen (480x272)
    full_packet.extend_from_slice(&[
        0x00, 0x00,  // X: 0
        0x00, 0x00,  // Y: 0
        0x01, 0xE0,  // Width: 480
        0x01, 0x10,  // Height: 272
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
    
    // Repeat command for full screen (480 * 272 / 2 = 65280 pairs)
    full_packet.extend_from_slice(&[
        0x01,        // Command 0x01: Repeat pixels
        0x00, 0xFF, 0x00,  // Count: 65280 (might be too big, let's try smaller)
    ]);
    
    // Bright white pixels (RGB565: 0xFFFF)
    full_packet.extend_from_slice(&[
        0xFF, 0xFF,  // First pixel: White
        0xFF, 0xFF,  // Second pixel: White
    ]);
    
    // Add 0x03 command
    full_packet.extend_from_slice(&[0x03, 0x00, 0x00, 0x00]);
    
    // End transmission
    full_packet.extend_from_slice(&[0x40, 0x00, 0x00, 0x00]);
    
    println!("📤 Sending full screen white ({} bytes)", full_packet.len());
    match device.write_display(&full_packet) {
        Ok(_) => println!("✅ Full screen packet sent"),
        Err(e) => println!("❌ Full screen failed: {}", e),
    }
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    
    println!("\n🏁 Test complete!");
    println!("💡 Check displays for colored squares or full white screen");
    
    Ok(())
}
