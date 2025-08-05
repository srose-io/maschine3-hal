use mk3_hal::{MaschineMK3, MK3Error};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 MK3 Exact Format Test - Using Working Capture Format");
    
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

    println!("\n🎯 Testing with EXACT format from working capture...");
    
    // Test 1: Replicate the exact working capture first
    println!("\n📋 Test 1: Exact replica of working capture");
    println!("   Right screen, Y=338, X=2, W=10, H=1, Color=0x81f8");
    
    let exact_replica = create_exact_replica();
    print_packet_hex("Exact replica", &exact_replica);
    device.write_display(&exact_replica)?;
    std::thread::sleep(Duration::from_secs(3));
    
    // Test 2: Try different Y positions using correct format
    println!("\n📍 Test 2: Different Y positions");
    
    let y_positions = [50, 100, 150, 200, 250];
    for (i, &y) in y_positions.iter().enumerate() {
        println!("   Position {}: Y={}, X=50", i + 1, y);
        
        let packet = create_corrected_packet(
            1,    // Right screen
            y,    // Y position
            50,   // X position
            20,   // Width
            1,    // Height (start with 1 like working capture)
            rgb565_corrected(255, 0, 0)  // Red
        );
        
        device.write_display(&packet)?;
        std::thread::sleep(Duration::from_millis(800));
    }
    
    println!("\n✅ Test complete - check if Y positions make sense now!");
    std::thread::sleep(Duration::from_secs(5));
    
    Ok(())
}

/// Create exact replica of working capture packet
/// Working: 84 00 01 60 00 00 00 00 00 52 01 00 00 02 00 0a 01 00 00 0a f8 81 00 00 03 00 00 00 40 00 01 00
fn create_exact_replica() -> Vec<u8> {
    vec![
        0x84, 0x00, 0x01, 0x60, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x52, 0x01, 0x00, 0x00, 0x02, 0x00, 0x0a,
        0x01, 0x00, 0x00, 0x0a, 0xf8, 0x81, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x00, 0x40, 0x00, 0x01, 0x00
    ]
}

/// Create packet using corrected format based on working capture analysis
/// Format: 84 00 [DISP] 60 00 00 00 00 | 00 [Y_LO] [Y_HI] 00 | 00 [X] 00 [W] | 01 00 00 [W] | [COLOR_LO] [COLOR_HI] 00 00 | 03 00 00 00 | 40 00 [DISP] 00
fn create_corrected_packet(display_id: u8, y: u16, x: u8, width: u8, height: u8, color: u16) -> Vec<u8> {
    vec![
        // Header (8 bytes)
        0x84, 0x00, display_id, 0x60, 
        0x00, 0x00, 0x00, 0x00,
        
        // Coordinates (8 bytes) - CORRECTED based on working capture
        0x00, (y & 0xFF) as u8, (y >> 8) as u8, 0x00,  // Y coordinate with padding
        0x00, x, 0x00, width,                           // X and width
        
        // Size info (8 bytes)
        height, 0x00, 0x00, width,  // Height and width repeat
        
        // Color (4 bytes)
        (color & 0xFF) as u8, (color >> 8) as u8, 0x00, 0x00,
        
        // RLE and terminator (4 bytes)
        0x03, 0x00, 0x00, 0x00,
        
        // End command (4 bytes)
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

/// Debug print packet
fn print_packet_hex(name: &str, packet: &[u8]) {
    println!("   📦 {} ({} bytes):", name, packet.len());
    for (i, chunk) in packet.chunks(8).enumerate() {
        print!("      {:2}: ", i * 8);
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        println!();
    }
}
