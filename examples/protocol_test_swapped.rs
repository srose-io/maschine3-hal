use mk3_hal::{MaschineMK3, MK3Error};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 MK3 Coordinate Swap Test - Testing X/Y Field Hypothesis");
    
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

    println!("\n🔄 Hypothesis: Our X/Y fields are swapped!");
    println!("   Testing with corrected field interpretation...");
    
    // Clear screen first
    println!("\n⚫ Clearing screen");
    std::thread::sleep(Duration::from_secs(1));
    
    // Test 1: Vertical movement (changing what we NOW think is Y)
    println!("\n⬆️ Test 1: Vertical movement - changing 'true Y' field");
    
    let y_positions = [50, 100, 150, 200, 250];
    
    for (i, &y) in y_positions.iter().enumerate() {
        println!("   Line {}: X=100, Y={}", i + 1, y);
        
        let packet = create_corrected_packet(
            1,    // Right screen
            100,  // X position (what we previously thought was Y)
            y,    // Y position (what we previously thought was X) 
            20,   // Width
            rgb565_corrected(255, 0, 0)  // Red
        );
        
        device.write_display(&packet)?;
        std::thread::sleep(Duration::from_millis(800));
    }
    
    std::thread::sleep(Duration::from_secs(2));
    
    // Test 2: Horizontal movement (changing what we NOW think is X)
    println!("\n↔️ Test 2: Horizontal movement - changing 'true X' field");
    
    let x_positions = [50, 150, 250, 350, 450];
    
    for (i, &x) in x_positions.iter().enumerate() {
        println!("   Line {}: X={}, Y=100", i + 1, x);
        
        let packet = create_corrected_packet(
            1,    // Right screen
            x,    // X position 
            100,  // Y position 
            20,   // Width
            rgb565_corrected(0, 255, 0)  // Green
        );
        
        device.write_display(&packet)?;
        std::thread::sleep(Duration::from_millis(800));
    }
    
    println!("\n✅ Coordinate swap test complete!");
    println!("📋 Expected results:");
    println!("   Red lines: Should move VERTICALLY (up/down)");
    println!("   Green lines: Should move HORIZONTALLY (left/right)");
    
    std::thread::sleep(Duration::from_secs(5));
    
    Ok(())
}

/// Create packet with CORRECTED coordinate field interpretation
/// New hypothesis: First coordinate field is X, second is Y
fn create_corrected_packet(display_id: u8, x: u16, y: u8, width: u8, color: u16) -> Vec<u8> {
    vec![
        // Header (8 bytes)
        0x84, 0x00, display_id, 0x60, 
        0x00, 0x00, 0x00, 0x00,
        
        // Coordinates (8 bytes) - SWAPPED interpretation
        0x00, (x & 0xFF) as u8, (x >> 8) as u8, 0x00,  // X coordinate (was Y)
        0x00, y, 0x00, width,                           // Y coordinate (was X) and width
        
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
