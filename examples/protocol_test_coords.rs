use mk3_hal::{MaschineMK3, MK3Error};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📐 MK3 Coordinate System Discovery");
    
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

    println!("\n🔍 Exploring coordinate space from working Y=338...");
    
    // Test Y coordinates around the working value (338)
    println!("\n📍 Test 1: Y coordinates around working value 338");
    
    let y_tests = [
        300, 310, 320, 330, 338, 350, 360, 370, 380, 400
    ];
    
    for (i, &y) in y_tests.iter().enumerate() {
        println!("   Test {}: Y={}, X=10", i + 1, y);
        
        let packet = create_exact_format_packet(
            1,    // Right screen
            y,    // Y position
            10,   // X position (left side)
            15,   // Width
            rgb565_corrected(255, 0, 0)  // Red
        );
        
        device.write_display(&packet)?;
        std::thread::sleep(Duration::from_millis(600));
    }
    
    std::thread::sleep(Duration::from_secs(2));
    
    // Test X coordinates with known working Y
    println!("\n↔️ Test 2: X coordinates at working Y=338");
    
    let x_tests = [0, 20, 50, 100, 150, 200, 250];
    
    for (i, &x) in x_tests.iter().enumerate() {
        println!("   Test {}: Y=338, X={}", i + 1, x);
        
        let packet = create_exact_format_packet(
            1,    // Right screen
            338,  // Y position (known working)
            x,    // X position
            10,   // Width
            rgb565_corrected(0, 255, 0)  // Green
        );
        
        device.write_display(&packet)?;
        std::thread::sleep(Duration::from_millis(600));
    }
    
    std::thread::sleep(Duration::from_secs(2));
    
    // Test if Y coordinates go higher
    println!("\n⬆️ Test 3: Higher Y coordinates");
    
    let high_y_tests = [400, 450, 500, 550, 600];
    
    for (i, &y) in high_y_tests.iter().enumerate() {
        println!("   Test {}: Y={}, X=100", i + 1, y);
        
        let packet = create_exact_format_packet(
            1,    // Right screen
            y,    // Y position
            100,  // X position
            20,   // Width
            rgb565_corrected(0, 0, 255)  // Blue
        );
        
        device.write_display(&packet)?;
        std::thread::sleep(Duration::from_millis(600));
    }
    
    println!("\n✅ Coordinate exploration complete!");
    println!("📋 Results should show:");
    println!("   Red lines: Y coordinate range");
    println!("   Green lines: X coordinate range");  
    println!("   Blue lines: Higher Y coordinates");
    
    std::thread::sleep(Duration::from_secs(5));
    
    Ok(())
}

/// Create packet using exact format from working capture
fn create_exact_format_packet(display_id: u8, y: u16, x: u8, width: u8, color: u16) -> Vec<u8> {
    vec![
        // Header (8 bytes)
        0x84, 0x00, display_id, 0x60, 
        0x00, 0x00, 0x00, 0x00,
        
        // Coordinates (8 bytes) - exact format from working capture
        0x00, (y & 0xFF) as u8, (y >> 8) as u8, 0x00,  
        0x00, x, 0x00, width,                           
        
        // Size info (8 bytes)  
        0x01, 0x00, 0x00, width,  
        
        // Color (4 bytes)
        (color & 0xFF) as u8, (color >> 8) as u8, 0x00, 0x00,
        
        // RLE and terminator (8 bytes)
        0x03, 0x00, 0x00, 0x00,
        0x40, 0x00, display_id, 0x00
    ]
}

/// Convert RGB to corrected RGB565 format for MK3
fn rgb565_corrected(r: u8, g: u8, b: u8) -> u16 {
    let corrected_r = b;
    let corrected_g = r;
    let corrected_b = g;
    
    let r5 = (corrected_r >> 3) as u16;
    let g6 = (corrected_g >> 2) as u16;
    let b5 = (corrected_b >> 3) as u16;
    
    (r5 << 11) | (g6 << 5) | b5
}
