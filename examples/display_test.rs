use mk3_hal::{MaschineMK3Hid, MK3Error, Rgb565, DisplayGraphics};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🖥️  Maschine MK3 Display Test");
    
    let device = match MaschineMK3Hid::new() {
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

    println!("\n🎬 Starting display tests...");
    println!("⚠️  Note: Display output may not work through HID - needs direct USB");

    // Test 1: Clear displays
    println!("\n🖤 Test 1: Clear both displays");
    device.clear_display(0)?; // Left display
    device.clear_display(1)?; // Right display
    std::thread::sleep(Duration::from_secs(2));

    // Test 2: Fill with solid colors
    println!("🔴 Test 2: Fill with solid colors");
    device.fill_display(0, Rgb565::red())?;   // Left = Red
    device.fill_display(1, Rgb565::blue())?;  // Right = Blue
    std::thread::sleep(Duration::from_secs(2));

    // Test 3: Draw rectangles
    println!("🟨 Test 3: Draw rectangles");
    device.clear_display(0)?;
    device.clear_display(1)?;
    
    // Draw some colored rectangles
    device.draw_rect(0, 50, 50, 100, 50, Rgb565::yellow())?;   // Left display
    device.draw_rect(0, 200, 100, 80, 80, Rgb565::green())?;
    device.draw_rect(1, 100, 60, 120, 100, Rgb565::magenta())?; // Right display
    device.draw_rect(1, 300, 150, 60, 60, Rgb565::cyan())?;
    std::thread::sleep(Duration::from_secs(2));

    // Test 4: Gradient patterns
    println!("🌈 Test 4: Gradient patterns");
    let gradient_left = DisplayGraphics::gradient(480, 272, Rgb565::red(), Rgb565::yellow());
    let gradient_right = DisplayGraphics::gradient(480, 272, Rgb565::blue(), Rgb565::cyan());
    
    device.draw_pattern(0, &gradient_left, 480, 272, 0, 0)?;
    device.draw_pattern(1, &gradient_right, 480, 272, 0, 0)?;
    std::thread::sleep(Duration::from_secs(3));

    // Test 5: Rainbow pattern
    println!("🏳️‍🌈 Test 5: Rainbow pattern");
    let rainbow = DisplayGraphics::rainbow(480, 272);
    device.draw_pattern(0, &rainbow, 480, 272, 0, 0)?;
    device.draw_pattern(1, &rainbow, 480, 272, 0, 0)?;
    std::thread::sleep(Duration::from_secs(3));

    // Test 6: Checkerboard
    println!("🏁 Test 6: Checkerboard pattern");
    let checkerboard = DisplayGraphics::checkerboard(480, 272, 20, Rgb565::white(), Rgb565::black());
    device.draw_pattern(0, &checkerboard, 480, 272, 0, 0)?;
    device.draw_pattern(1, &checkerboard, 480, 272, 0, 0)?;
    std::thread::sleep(Duration::from_secs(2));

    // Test 7: Animated plasma
    println!("✨ Test 7: Animated plasma (10 seconds)");
    for i in 0..100 {
        let time = i as f32 * 0.1;
        let plasma = DisplayGraphics::plasma(480, 272, time);
        
        device.draw_pattern(0, &plasma, 480, 272, 0, 0)?;
        // Mirror on right display
        device.draw_pattern(1, &plasma, 480, 272, 0, 0)?;
        
        std::thread::sleep(Duration::from_millis(100));
        
        if i % 20 == 0 {
            println!("   Frame {}/100", i);
        }
    }

    // Test 8: Small pattern test
    println!("🔍 Test 8: Small pattern in corner");
    device.clear_display(0)?;
    device.clear_display(1)?;
    
    // Create a small 64x64 test pattern
    let small_pattern = DisplayGraphics::checkerboard(64, 64, 8, Rgb565::red(), Rgb565::green());
    device.draw_pattern(0, &small_pattern, 64, 64, 10, 10)?;      // Top-left
    device.draw_pattern(0, &small_pattern, 64, 64, 400, 200)?;    // Bottom-right
    device.draw_pattern(1, &small_pattern, 64, 64, 208, 104)?;    // Center
    std::thread::sleep(Duration::from_secs(3));

    // Cleanup
    println!("\n🧹 Cleaning up...");
    device.clear_display(0)?;
    device.clear_display(1)?;
    
    println!("✅ Display test complete!");
    println!("💡 If you saw any graphics on the displays, the system is working!");
    println!("❓ If displays stayed black, the HID interface might not support display data");
    println!("   (Would need direct USB bulk endpoint access)");
    
    Ok(())
}
