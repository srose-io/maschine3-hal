use mk3_hal::MaschineMK3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Back to Working Headers + Color Experiments");
    println!("===============================================");
    
    let device = MaschineMK3::new()?;
    println!("✅ Connected: {}", device.device_info()?);

    // Test 1: Verify we can still get white with the original working headers
    println!("\n🧪 Test 1: Confirm white still works");
    
    let mut white_packet = Vec::new();
    
    // EXACT header from display_test2.rs that produced white
    white_packet.extend_from_slice(&[
        0x84, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
    
    // Small 10x10 area at (10, 10)
    white_packet.extend_from_slice(&[
        0x00, 0x0A,  // X: 10
        0x00, 0x0A,  // Y: 10
        0x00, 0x0A,  // Width: 10
        0x00, 0x0A,  // Height: 10
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
    
    // Command 0x01 with white pixels (this worked before)
    white_packet.extend_from_slice(&[
        0x01,        // Command 0x01: Repeat pixels
        0x00, 0x00, 0x32,  // Count: 50 (covers 100 pixels)
    ]);
    
    // White pixels (RGB565: 0xFFFF) in little-endian
    white_packet.extend_from_slice(&[
        0xFF, 0xFF,  // First pixel: White
        0xFF, 0xFF,  // Second pixel: White  
    ]);
    
    // Blit and end
    white_packet.extend_from_slice(&[0x03, 0x00, 0x00, 0x00]);
    white_packet.extend_from_slice(&[0x40, 0x00, 0x00, 0x00]);
    
    match device.write_display(&white_packet) {
        Ok(_) => println!("✅ White test sent"),
        Err(e) => println!("❌ White test failed: {}", e),
    }
    
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Test 2: Try pure black with same structure
    println!("\n🧪 Test 2: Pure black with working headers");
    
    let mut black_packet = Vec::new();
    black_packet.extend_from_slice(&white_packet[..40]); // Copy headers and position
    
    // Same command structure but black pixels
    black_packet.extend_from_slice(&[
        0x01,        // Command 0x01: Repeat pixels
        0x00, 0x00, 0x32,  // Count: 50
    ]);
    
    // Black pixels (RGB565: 0x0000)
    black_packet.extend_from_slice(&[
        0x00, 0x00,  // First pixel: Black
        0x00, 0x00,  // Second pixel: Black
    ]);
    
    black_packet.extend_from_slice(&[0x03, 0x00, 0x00, 0x00]);
    black_packet.extend_from_slice(&[0x40, 0x00, 0x00, 0x00]);
    
    match device.write_display(&black_packet) {
        Ok(_) => println!("✅ Black test sent"),
        Err(e) => println!("❌ Black test failed: {}", e),
    }
    
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Test 3: Try different RGB565 interpretations
    println!("\n🧪 Test 3: RGB565 format experiments");
    
    let color_tests = vec![
        ("Red 0xF800 LE", [0x00, 0xF8]),     // Little-endian
        ("Red 0xF800 BE", [0xF8, 0x00]),     // Big-endian  
        ("Green 0x07E0 LE", [0xE0, 0x07]),   // Little-endian
        ("Green 0x07E0 BE", [0x07, 0xE0]),   // Big-endian
        ("Blue 0x001F LE", [0x1F, 0x00]),    // Little-endian
        ("Blue 0x001F BE", [0x00, 0x1F]),    // Big-endian
    ];
    
    for (i, (desc, pixel_bytes)) in color_tests.iter().enumerate() {
        println!("   Testing: {}", desc);
        
        let mut test_packet = Vec::new();
        test_packet.extend_from_slice(&white_packet[..32]); // Copy headers
        
        // Different position for each test
        let x = 20 + (i * 15) as u16;
        test_packet.extend_from_slice(&[
            (x >> 8) as u8, (x & 0xFF) as u8,  // X position
            0x00, 0x14,                         // Y: 20
            0x00, 0x08,                         // Width: 8
            0x00, 0x08,                         // Height: 8
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        
        test_packet.extend_from_slice(&[
            0x01,        // Command 0x01: Repeat pixels
            0x00, 0x00, 0x20,  // Count: 32 (64 pixels for 8x8)
        ]);
        
        test_packet.extend_from_slice(pixel_bytes);
        test_packet.extend_from_slice(pixel_bytes);
        
        test_packet.extend_from_slice(&[0x03, 0x00, 0x00, 0x00]);
        test_packet.extend_from_slice(&[0x40, 0x00, 0x00, 0x00]);
        
        match device.write_display(&test_packet) {
            Ok(_) => {},
            Err(e) => println!("   ❌ Failed: {}", e),
        }
        
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
    
    println!("\n🏁 Tests complete!");
    println!("💡 Check display for:");
    println!("   - White square at (10,10)");
    println!("   - Black square at (10,10) - should be visible if not all white");
    println!("   - Colored squares in a row at y=20");
    
    Ok(())
}
