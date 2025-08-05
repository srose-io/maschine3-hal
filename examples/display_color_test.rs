use mk3_hal::MaschineMK3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Display Color Breakthrough Test");
    println!("==================================");
    
    let device = MaschineMK3::new()?;
    println!("✅ Connected: {}", device.device_info()?);

    // Test with pure colors in different bit patterns
    println!("\n🧪 Test: Pure color patterns with different intensities");
    
    let color_tests = vec![
        ("Black", 0x0000),
        ("Dark Red", 0x8000),  // Half intensity red
        ("Full Red", 0xF800),
        ("Dark Green", 0x0400), // Half intensity green
        ("Full Green", 0x07E0),
        ("Dark Blue", 0x0010),  // Half intensity blue
        ("Full Blue", 0x001F),
        ("Yellow", 0xFFE0),     // Red + Green
        ("Magenta", 0xF81F),    // Red + Blue
        ("Cyan", 0x07FF),       // Green + Blue
    ];
    
    for (i, (name, color)) in color_tests.iter().enumerate() {
        println!("📤 Test {}: {} (0x{:04X})", i + 1, name, color);
        
        let mut packet = Vec::new();
        
        // Header Part 1 (16 bytes)
        packet.extend_from_slice(&[
            0x84, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        
        // Header Part 2 - 20x20 pixel squares in a grid pattern
        let x = ((i % 4) * 25) as u16;
        let y = ((i / 4) * 25) as u16;
        packet.extend_from_slice(&[
            (x >> 8) as u8, (x & 0xFF) as u8,      // X position
            (y >> 8) as u8, (y & 0xFF) as u8,      // Y position  
            0x00, 0x14,                             // Width: 20
            0x00, 0x14,                             // Height: 20
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        
        // Command: Repeat pixels (20x20 = 400 pixels / 2 = 200 pairs)
        packet.extend_from_slice(&[
            0x01,        // Command 0x01: Repeat pixels
            0x00, 0x00, 0xC8,  // Count: 200
        ]);
        
        // Two pixels with test color - try both byte orders
        let color_le = [(*color & 0xFF) as u8, (*color >> 8) as u8];  // Little endian
        packet.extend_from_slice(&color_le);
        packet.extend_from_slice(&color_le);
        
        // Add 0x03 command
        packet.extend_from_slice(&[0x03, 0x00, 0x00, 0x00]);
        
        // End transmission
        packet.extend_from_slice(&[0x40, 0x00, 0x00, 0x00]);
        
        match device.write_display(&packet) {
            Ok(_) => println!("   ✅ Sent {} bytes", packet.len()),
            Err(e) => println!("   ❌ Failed: {}", e),
        }
        
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    
    println!("\n🏁 Color test complete!");
    println!("💡 Check left display for a colorful grid pattern");
    println!("❓ Did you see any colors other than white?");
    
    Ok(())
}
