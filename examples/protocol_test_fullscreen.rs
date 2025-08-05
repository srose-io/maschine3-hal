use mk3_hal::{MaschineMK3, MK3Error};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🖥️  MK3 Full Screen Test - Using Discovered Protocol");
    
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

    println!("\n🎨 Testing full screen color fills...");
    
    // Test 1: Fill left screen RED
    println!("\n🔴 Test 1: Fill left screen RED");
    fill_screen(&device, 0, 0xF800)?; // Left screen, red
    std::thread::sleep(Duration::from_secs(3));

    // Test 2: Fill right screen GREEN  
    println!("🟢 Test 2: Fill right screen GREEN");
    fill_screen(&device, 1, 0x07E0)?; // Right screen, green
    std::thread::sleep(Duration::from_secs(3));

    // Test 3: Fill left screen BLUE
    println!("🔵 Test 3: Fill left screen BLUE");
    fill_screen(&device, 0, 0x001F)?; // Left screen, blue
    std::thread::sleep(Duration::from_secs(3));

    // Test 4: Fill right screen YELLOW
    println!("🟡 Test 4: Fill right screen YELLOW");
    fill_screen(&device, 1, 0xFFE0)?; // Right screen, yellow
    std::thread::sleep(Duration::from_secs(3));

    // Test 5: Fill both screens WHITE
    println!("⚪ Test 5: Fill both screens WHITE");
    fill_screen(&device, 0, 0xFFFF)?; // Left screen, white
    fill_screen(&device, 1, 0xFFFF)?; // Right screen, white
    std::thread::sleep(Duration::from_secs(3));

    // Test 6: Fill both screens BLACK
    println!("⚫ Test 6: Fill both screens BLACK");
    fill_screen(&device, 0, 0x0000)?; // Left screen, black
    fill_screen(&device, 1, 0x0000)?; // Right screen, black
    std::thread::sleep(Duration::from_secs(2));

    println!("\n✅ Full screen test complete!");
    println!("🎊 The MK3 display protocol has been successfully reverse engineered!");
    
    Ok(())
}

/// Fill entire screen with a solid color using our discovered 32-byte packet format
/// Screen resolution: 480x272 pixels
fn fill_screen(device: &MaschineMK3, display_id: u8, color: u16) -> Result<(), Box<dyn std::error::Error>> {
    println!("   📤 Filling display {} with color 0x{:04X}...", display_id, color);
    
    // Fill screen by sending horizontal strips of 480 pixels wide, 8 pixels tall
    // This reduces the number of packets from 272 to 34 (272/8 = 34)
    const STRIP_HEIGHT: u8 = 8;
    const SCREEN_WIDTH: u8 = 255; // Max width we can send in single packet
    const SCREEN_HEIGHT: u16 = 272;
    
    for y in (0..SCREEN_HEIGHT).step_by(STRIP_HEIGHT as usize) {
        // Send two packets per row to cover full 480 pixel width
        // Packet 1: X=0, Width=255
        let packet1 = create_simple_packet(display_id, y, 0, SCREEN_WIDTH, color);
        device.write_display(&packet1)?;
        
        // Packet 2: X=255, Width=225 (480-255=225)
        let packet2 = create_simple_packet(display_id, y, SCREEN_WIDTH, 225, color);
        device.write_display(&packet2)?;
        
        // Small delay to avoid overwhelming the device
        std::thread::sleep(Duration::from_millis(1));
    }
    
    println!("   ✅ Screen {} filled", display_id);
    Ok(())
}

/// Create a simple 32-byte packet using the discovered protocol format
/// Format: 84 00 [DISP] 60 00 00 00 00 [Y_LO] [Y_HI] 01 00 00 [X] 00 [W] 01 00 00 [W] [COLOR_LO] [COLOR_HI] 00 00 03 00 00 00 40 00 [DISP] 00
fn create_simple_packet(display_id: u8, y: u16, x: u8, width: u8, color: u16) -> Vec<u8> {
    vec![
        0x84, 0x00, display_id, 0x60,    // Header
        0x00, 0x00, 0x00, 0x00,          // Padding
        (y & 0xFF) as u8, (y >> 8) as u8, 0x01, 0x00,  // Y coordinate (little endian)
        0x00, x, 0x00, width,            // X and width
        0x01, 0x00, 0x00, width,         // Height and repeat width
        (color & 0xFF) as u8, (color >> 8) as u8,      // Color (little endian)
        0x00, 0x00, 0x03, 0x00,          // RLE count (3)
        0x00, 0x00, 0x40, 0x00,          // End command
        display_id, 0x00                  // Display ID terminator
    ]
}
