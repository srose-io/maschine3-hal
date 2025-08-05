use mk3_hal::{MaschineMK3, MK3Error};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 MK3 Perfect Full Screen Fill Test");
    
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

    println!("\n🖥️  Testing perfect full screen fills...");
    
    // Test sequence with proper solid fills
    let tests = [
        ("🔴 Solid RED", 255, 0, 0),
        ("🟢 Solid GREEN", 0, 255, 0),
        ("🔵 Solid BLUE", 0, 0, 255),
        ("🟡 Solid YELLOW", 255, 255, 0),
        ("🟣 Solid MAGENTA", 255, 0, 255),
        ("🟦 Solid CYAN", 0, 255, 255),
        ("⚪ Solid WHITE", 255, 255, 255),
        ("⚫ Solid BLACK", 0, 0, 0),
    ];

    for (name, r, g, b) in tests.iter() {
        println!("\n{} - Left Screen", name);
        fill_screen_solid(&device, 0, *r, *g, *b)?;
        std::thread::sleep(Duration::from_secs(2));

        println!("{} - Right Screen", name);
        fill_screen_solid(&device, 1, *r, *g, *b)?;
        std::thread::sleep(Duration::from_secs(2));
    }

    println!("\n✅ Perfect fill test complete!");
    println!("🎊 MK3 Display Protocol Successfully Reverse Engineered!");
    
    Ok(())
}

/// Fill entire screen with solid color using optimal strategy
/// Screen resolution: 480x272 pixels
fn fill_screen_solid(device: &MaschineMK3, display_id: u8, r: u8, g: u8, b: u8) -> Result<(), Box<dyn std::error::Error>> {
    let color = rgb565_corrected(r, g, b);
    
    // Strategy: Use tall strips to minimize packet count
    // Each strip: 240 pixels wide, 32 pixels tall
    const STRIP_HEIGHT: u8 = 32;
    const STRIP_WIDTH: u8 = 240;
    const SCREEN_HEIGHT: u16 = 272;
    
    // Cover width with two strips: 0-239, 240-479
    let x_positions = [0u8, 240u8];
    
    for y in (0..SCREEN_HEIGHT).step_by(STRIP_HEIGHT as usize) {
        let actual_height = if y + STRIP_HEIGHT as u16 > SCREEN_HEIGHT {
            (SCREEN_HEIGHT - y) as u8  // Last strip might be shorter
        } else {
            STRIP_HEIGHT
        };
        
        for &x in x_positions.iter() {
            let width = if x == 240 { 240 } else { STRIP_WIDTH }; // Second strip covers remaining 240 pixels
            
            let packet = create_fill_packet(display_id, y, x, width, actual_height, color);
            device.write_display(&packet)?;
        }
        
        // Small delay to avoid overwhelming device
        std::thread::sleep(Duration::from_millis(1));
    }
    
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

/// Create optimized fill packet with proper height
fn create_fill_packet(display_id: u8, y: u16, x: u8, width: u8, height: u8, color: u16) -> Vec<u8> {
    vec![
        0x84, 0x00, display_id, 0x60,    // Header
        0x00, 0x00, 0x00, 0x00,          // Padding
        (y & 0xFF) as u8, (y >> 8) as u8, height, 0x00,  // Y coordinate and height
        0x00, x, 0x00, width,            // X and width
        0x01, 0x00, 0x00, width,         // Repeat info
        (color & 0xFF) as u8, (color >> 8) as u8,      // Color
        0x00, 0x00, 0x03, 0x00,          // RLE count
        0x00, 0x00, 0x40, 0x00,          // End command
        display_id, 0x00                  // Display ID terminator
    ]
}
