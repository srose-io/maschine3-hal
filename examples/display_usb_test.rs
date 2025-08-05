use mk3_hal::{MaschineMK3, MK3Error, Rgb565, DisplayGraphics};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🖥️  Maschine MK3 Display Test (Direct USB)");
    println!("⚠️  This requires administrator privileges and closing NI software!");
    
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
            println!("   Try running as administrator and ensure NI software is closed");
            return Ok(());
        }
    };

    println!("\n🎬 Starting display tests using direct USB bulk endpoint...");

    // Test 1: Clear displays
    println!("\n🖤 Test 1: Clear both displays");
    match device.clear_display(0) {
        Ok(_) => println!("   ✅ Left display cleared"),
        Err(e) => println!("   ❌ Left display error: {}", e),
    }
    
    match device.clear_display(1) {
        Ok(_) => println!("   ✅ Right display cleared"),
        Err(e) => println!("   ❌ Right display error: {}", e),
    }
    std::thread::sleep(Duration::from_secs(2));

    // Test 2: Fill with solid colors
    println!("🔴 Test 2: Fill with solid colors");
    match device.fill_display(0, Rgb565::red()) {
        Ok(_) => println!("   ✅ Left display filled red"),
        Err(e) => println!("   ❌ Left display error: {}", e),
    }
    
    match device.fill_display(1, Rgb565::blue()) {
        Ok(_) => println!("   ✅ Right display filled blue"),
        Err(e) => println!("   ❌ Right display error: {}", e),
    }
    std::thread::sleep(Duration::from_secs(2));

    // Test 3: Draw rectangles
    println!("🟨 Test 3: Draw rectangles");
    let _ = device.clear_display(0);
    let _ = device.clear_display(1);
    
    // Draw some colored rectangles
    match device.draw_rect(0, 50, 50, 100, 50, Rgb565::yellow()) {
        Ok(_) => println!("   ✅ Yellow rectangle on left"),
        Err(e) => println!("   ❌ Rectangle error: {}", e),
    }
    
    match device.draw_rect(1, 100, 60, 120, 100, Rgb565::magenta()) {
        Ok(_) => println!("   ✅ Magenta rectangle on right"),
        Err(e) => println!("   ❌ Rectangle error: {}", e),
    }
    std::thread::sleep(Duration::from_secs(2));

    // Test 4: Small pattern test  
    println!("🔍 Test 4: Small test pattern");
    let _ = device.clear_display(0);
    let _ = device.clear_display(1);
    
    // Create a small 100x100 test pattern
    let test_pattern = DisplayGraphics::checkerboard(100, 100, 10, Rgb565::white(), Rgb565::green());
    
    match device.draw_pattern(0, &test_pattern, 100, 100, 50, 50) {
        Ok(_) => println!("   ✅ Test pattern on left display"),
        Err(e) => println!("   ❌ Pattern error: {}", e),
    }
    
    match device.draw_pattern(1, &test_pattern, 100, 100, 190, 86) {
        Ok(_) => println!("   ✅ Test pattern on right display"),
        Err(e) => println!("   ❌ Pattern error: {}", e),
    }
    std::thread::sleep(Duration::from_secs(3));

    // Test 5: Gradient test (smaller to avoid timeout)
    println!("🌈 Test 5: Gradient test");
    let gradient = DisplayGraphics::gradient(200, 150, Rgb565::red(), Rgb565::yellow());
    
    match device.draw_pattern(0, &gradient, 200, 150, 140, 60) {
        Ok(_) => println!("   ✅ Gradient on left display"),
        Err(e) => println!("   ❌ Gradient error: {}", e),
    }
    std::thread::sleep(Duration::from_secs(2));

    // Test 6: Animation test (simple)
    println!("✨ Test 6: Simple animation (10 frames)");
    for i in 0..10 {
        let hue = (i as f32 * 36.0) % 360.0; // Change hue each frame
        let color = Rgb565::from_hsv(hue, 1.0, 1.0);
        
        if let Ok(_) = device.fill_display(0, color) {
            print!(".");
        } else {
            print!("!");
        }
        std::io::Write::flush(&mut std::io::stdout()).ok();
        
        std::thread::sleep(Duration::from_millis(200));
    }
    println!(" Done!");

    // Cleanup
    println!("\n🧹 Cleaning up...");
    let _ = device.clear_display(0);
    let _ = device.clear_display(1);
    
    println!("✅ Display test complete!");
    println!("💡 Check your Maschine MK3 displays for any graphics that appeared!");
    println!("🔧 If displays stayed black, the protocol implementation may need adjustment");
    
    Ok(())
}
