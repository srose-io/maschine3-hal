use mk3_hal::{MaschineMK3, MK3Error};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 MK3 Simple Protocol Test - Testing Discovered Protocol");
    println!("📝 Based on packet analysis from Wireshark captures");
    
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

    println!("\n🎯 Testing discovered 32-byte simple protocol...");
    
    // Test 1: Draw a red rectangle on left screen
    println!("\n🟥 Test 1: Red rectangle on left screen (Y=100, X=50, W=20, H=1)");
    let red_rect_left = create_simple_packet(
        0,      // Left display
        100,    // Y coordinate  
        50,     // X coordinate
        20,     // Width
        0xF800, // Red color (RGB565)
    );
    
    println!("📤 Sending packet: {} bytes", red_rect_left.len());
    print_packet_hex(&red_rect_left);
    device.write_display(&red_rect_left)?;
    std::thread::sleep(Duration::from_secs(2));

    // Test 2: Draw a green rectangle on right screen  
    println!("\n🟩 Test 2: Green rectangle on right screen (Y=150, X=100, W=30, H=1)");
    let green_rect_right = create_simple_packet(
        1,      // Right display
        150,    // Y coordinate
        100,    // X coordinate
        30,     // Width
        0x07E0, // Green color (RGB565)
    );
    
    println!("📤 Sending packet: {} bytes", green_rect_right.len());
    print_packet_hex(&green_rect_right);
    device.write_display(&green_rect_right)?;
    std::thread::sleep(Duration::from_secs(2));

    // Test 3: Draw a blue rectangle on left screen
    println!("\n🟦 Test 3: Blue rectangle on left screen (Y=200, X=200, W=50, H=1)");
    let blue_rect_left = create_simple_packet(
        0,      // Left display
        200,    // Y coordinate
        200,    // X coordinate
        50,     // Width
        0x001F, // Blue color (RGB565)
    );
    
    println!("📤 Sending packet: {} bytes", blue_rect_left.len());
    print_packet_hex(&blue_rect_left);
    device.write_display(&blue_rect_left)?;
    std::thread::sleep(Duration::from_secs(2));

    // Test 4: Draw a yellow progress bar on right screen (like in capture)
    println!("\n🟨 Test 4: Yellow progress bar on right screen (Y=338, X=2, W=10)");
    let yellow_bar_right = create_simple_packet(
        1,      // Right display  
        338,    // Y coordinate (from capture: 0x0152 = 338)
        2,      // X coordinate (from capture)
        10,     // Width (from capture)
        0x81F8, // Color from capture
    );
    
    println!("📤 Sending packet: {} bytes", yellow_bar_right.len());
    print_packet_hex(&yellow_bar_right);
    device.write_display(&yellow_bar_right)?;
    std::thread::sleep(Duration::from_secs(3));

    println!("\n✅ Simple protocol test complete!");
    println!("💡 If you see colored rectangles on the displays, the protocol works!");
    println!("❓ If nothing appears, check USB interface access or driver setup");
    
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
        0x01, 0x00, 0x00, width,         // Height (1) and repeat width
        (color & 0xFF) as u8, (color >> 8) as u8,      // Color (little endian)
        0x00, 0x00, 0x03, 0x00,          // RLE count (3)
        0x00, 0x00, 0x40, 0x00,          // End command
        display_id, 0x00                  // Display ID terminator
    ]
}

/// Print packet data in hex format for debugging
fn print_packet_hex(packet: &[u8]) {
    print!("   Hex: ");
    for (i, byte) in packet.iter().enumerate() {
        if i > 0 && i % 16 == 0 {
            print!("\n        ");
        }
        print!("{:02x} ", byte);
    }
    println!();
}
