use mk3_hal::MaschineMK3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Debug Color Format Issues");
    println!("============================");
    
    let device = MaschineMK3::new()?;
    println!("✅ Connected: {}", device.device_info()?);

    // Test 1: Try without the 0x03 blit command
    println!("\n🧪 Test 1: No blit command");
    
    let mut no_blit_packet = Vec::new();
    
    // Working headers
    no_blit_packet.extend_from_slice(&[
        0x84, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
    
    // Small area at (10, 10)
    no_blit_packet.extend_from_slice(&[
        0x00, 0x0A,  // X: 10
        0x00, 0x0A,  // Y: 10
        0x00, 0x0A,  // Width: 10
        0x00, 0x0A,  // Height: 10
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
    
    // Pure red with repeat command
    no_blit_packet.extend_from_slice(&[
        0x01,        // Command 0x01: Repeat pixels
        0x00, 0x00, 0x32,  // Count: 50
    ]);
    
    // Red pixels (try both byte orders)
    no_blit_packet.extend_from_slice(&[
        0x00, 0xF8,  // Red in little-endian
        0x00, 0xF8,  // Red in little-endian
    ]);
    
    // Skip the 0x03 blit command, go straight to end
    no_blit_packet.extend_from_slice(&[0x40, 0x00, 0x00, 0x00]);
    
    match device.write_display(&no_blit_packet) {
        Ok(_) => println!("✅ No-blit red sent"),
        Err(e) => println!("❌ Failed: {}", e),
    }
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    
    // Test 2: Try RGB888 format (3 bytes per pixel)
    println!("\n🧪 Test 2: RGB888 format test");
    
    let mut rgb888_packet = Vec::new();
    
    // Same headers
    rgb888_packet.extend_from_slice(&[
        0x84, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
    
    // 5x5 area
    rgb888_packet.extend_from_slice(&[
        0x00, 0x14,  // X: 20
        0x00, 0x0A,  // Y: 10  
        0x00, 0x05,  // Width: 5
        0x00, 0x05,  // Height: 5
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
    
    // Test if it expects RGB888 - 25 pixels = 75 bytes
    rgb888_packet.extend_from_slice(&[
        0x00,        // Command 0x00: Direct pixels
        0x00, 0x00, 0x4B,  // 75 bytes (25 pixels * 3 bytes each)
    ]);
    
    // 25 pixels in RGB888 format (R, G, B bytes) - pure red
    for _ in 0..25 {
        rgb888_packet.extend_from_slice(&[0xFF, 0x00, 0x00]); // Red
    }
    
    // Pad to 4-byte boundary if needed
    rgb888_packet.extend_from_slice(&[0x00]); // 75 + 1 = 76 bytes (4-byte aligned)
    
    rgb888_packet.extend_from_slice(&[0x03, 0x00, 0x00, 0x00]);
    rgb888_packet.extend_from_slice(&[0x40, 0x00, 0x00, 0x00]);
    
    match device.write_display(&rgb888_packet) {
        Ok(_) => println!("✅ RGB888 test sent"),
        Err(e) => println!("❌ RGB888 failed: {}", e),
    }
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    
    // Test 3: Try extreme bit patterns to see what happens
    println!("\n🧪 Test 3: Extreme bit patterns");
    
    let bit_tests = vec![
        ("All zeros", [0x00, 0x00]),
        ("All ones", [0xFF, 0xFF]),
        ("Alternating 0xAA", [0xAA, 0xAA]),
        ("Alternating 0x55", [0x55, 0x55]),
        ("0x01 pattern", [0x01, 0x01]),
        ("0x80 pattern", [0x80, 0x80]),
    ];
    
    for (i, (desc, pattern)) in bit_tests.iter().enumerate() {
        println!("   Testing: {}", desc);
        
        let mut pattern_packet = Vec::new();
        
        // Headers
        pattern_packet.extend_from_slice(&[
            0x84, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        
        // Different positions
        let x = 30 + (i * 12) as u16;
        pattern_packet.extend_from_slice(&[
            (x >> 8) as u8, (x & 0xFF) as u8,  // X position
            0x00, 0x14,                         // Y: 20
            0x00, 0x08,                         // Width: 8
            0x00, 0x08,                         // Height: 8
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        
        pattern_packet.extend_from_slice(&[
            0x01,        // Command 0x01: Repeat pixels
            0x00, 0x00, 0x20,  // Count: 32
        ]);
        
        pattern_packet.extend_from_slice(pattern);
        pattern_packet.extend_from_slice(pattern);
        
        pattern_packet.extend_from_slice(&[0x03, 0x00, 0x00, 0x00]);
        pattern_packet.extend_from_slice(&[0x40, 0x00, 0x00, 0x00]);
        
        match device.write_display(&pattern_packet) {
            Ok(_) => {},
            Err(e) => println!("   ❌ Failed: {}", e),
        }
        
        std::thread::sleep(std::time::Duration::from_millis(400));
    }
    
    println!("\n🏁 Debug tests complete!");
    println!("💡 Check for:");
    println!("   - Red square at (10,10) - no blit command");
    println!("   - Red square at (20,10) - RGB888 format");
    println!("   - Pattern squares at y=20 - different bit patterns");
    println!("\n❓ Are ALL these still white, or do you see any differences?");
    
    Ok(())
}
