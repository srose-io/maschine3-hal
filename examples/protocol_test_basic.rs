use mk3_hal::{MaschineMK3, MK3Error};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📍 MK3 Basic Position Test - One Rectangle at a Time");
    
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

    println!("\n🎯 Testing ONE rectangle on right screen...");
    println!("   Expected: RED rectangle at TOP-LEFT corner");
    println!("   Position: X=50, Y=50");
    println!("   Size: 100 pixels wide, 50 pixels tall");
    
    // Clear right screen first
    println!("\n⚫ Step 1: Clear right screen (black)");
    let clear_packet = create_simple_packet(1, 0, 0, 255, 100, rgb565_corrected(0, 0, 0));
    device.write_display(&clear_packet)?;
    
    println!("   Waiting 3 seconds...");
    std::thread::sleep(Duration::from_secs(3));
    
    // Draw ONE red rectangle
    println!("\n🔴 Step 2: Draw RED rectangle");
    println!("   X=50, Y=50, Width=100, Height=50");
    
    let red_packet = create_simple_packet(
        1,    // Right screen
        50,   // Y = 50 pixels from top
        50,   // X = 50 pixels from left  
        100,  // Width = 100 pixels
        50,   // Height = 50 pixels
        rgb565_corrected(255, 0, 0)  // RED color
    );
    
    print_packet_debug(&red_packet);
    device.write_display(&red_packet)?;
    
    println!("\n✅ Test complete!");
    println!("📷 Please take screenshot and verify:");
    println!("   ✓ Rectangle appears on RIGHT screen");
    println!("   ✓ Rectangle is RED color");  
    println!("   ✓ Rectangle is near TOP-LEFT area");
    println!("   ✓ Rectangle is roughly 100x50 pixels");
    
    // Wait so you can take screenshot
    std::thread::sleep(Duration::from_secs(10));
    
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

/// Create simple packet - using exact format from working captures
fn create_simple_packet(display_id: u8, y: u16, x: u8, width: u8, height: u8, color: u16) -> Vec<u8> {
    vec![
        0x84, 0x00, display_id, 0x60,    // Header (confirmed working)
        0x00, 0x00, 0x00, 0x00,          // Padding
        (y & 0xFF) as u8, (y >> 8) as u8, height, 0x00,  // Y coord (little endian) + height
        0x00, x, 0x00, width,            // X coord + width
        0x01, 0x00, 0x00, width,         // ?? + width again
        (color & 0xFF) as u8, (color >> 8) as u8,        // Color (little endian)
        0x00, 0x00, 0x03, 0x00,          // RLE/repeat count
        0x00, 0x00, 0x40, 0x00,          // End command
        display_id, 0x00                  // Display ID terminator
    ]
}

/// Debug print packet contents
fn print_packet_debug(packet: &[u8]) {
    println!("   📦 Packet (32 bytes):");
    for (i, chunk) in packet.chunks(8).enumerate() {
        print!("      {:2}: ", i * 8);
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        println!();
    }
}
