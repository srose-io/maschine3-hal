use maschine3_hal::{MaschineMK3, MK3Error};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Maschine MK3 Partial Update Test");
    println!("   Testing region updates with progress bar simulation");

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

    if !device.is_display_available() {
        println!("❌ Display interface not available");
        return Ok(());
    }

    const WIDTH: usize = 480;
    const HEIGHT: usize = 272;
    const DISPLAY_ID: u8 = 0; // Left display

    println!("\n📺 Step 1: Clearing screen to black...");
    let mut full_frame = vec![0u8; WIDTH * HEIGHT * 3]; // Black RGB888
    device.write_display_framebuffer_rgb888(DISPLAY_ID, &full_frame)?;
    thread::sleep(Duration::from_millis(500));
    println!("   ✅ Screen cleared");

    println!("\n📊 Step 2: Drawing progress bar background...");

    // Progress bar dimensions
    let bar_x = 50;
    let bar_y = 100;
    let bar_width = 380;
    let bar_height = 40;

    // Draw white background for progress bar area on full frame
    for y in bar_y..(bar_y + bar_height) {
        for x in bar_x..(bar_x + bar_width) {
            let idx = (y * WIDTH + x) * 3;
            full_frame[idx] = 80;     // R
            full_frame[idx + 1] = 80; // G
            full_frame[idx + 2] = 80; // B
        }
    }

    // Add some text labels using simple pixel patterns
    // "Progress Bar Test" at top
    draw_text(&mut full_frame, WIDTH, 140, 50, "PROGRESS BAR TEST");

    device.write_display_framebuffer_rgb888(DISPLAY_ID, &full_frame)?;
    thread::sleep(Duration::from_millis(500));
    println!("   ✅ Progress bar background drawn");

    println!("\n🔄 Step 3: Animating progress bar using partial updates...");
    println!("   Testing Y-axis orientation and size accuracy");

    // Progress bar fill dimensions (slightly smaller to see border)
    let fill_x = bar_x + 5;
    let fill_y = bar_y + 5;
    let fill_height = bar_height - 10;
    let fill_max_width = bar_width - 10;

    // Animate progress from 0% to 100%
    let steps = 20;
    for step in 0..=steps {
        let progress = step as f32 / steps as f32;
        let fill_width = (fill_max_width as f32 * progress) as usize;

        if fill_width == 0 {
            continue;
        }

        println!("   Progress: {:.0}% (width: {} pixels)", progress * 100.0, fill_width);

        // Create cyan fill region
        let mut region_data = vec![0u8; fill_width * fill_height * 3];
        for pixel in 0..(fill_width * fill_height) {
            let idx = pixel * 3;
            region_data[idx] = 0;       // R
            region_data[idx + 1] = 255; // G (cyan)
            region_data[idx + 2] = 255; // B (cyan)
        }

        // Send partial update
        device.write_display_region_rgb888(
            DISPLAY_ID,
            fill_x as u16,
            fill_y as u16,
            fill_width as u16,
            fill_height as u16,
            &region_data,
        )?;

        thread::sleep(Duration::from_millis(150));
    }

    println!("   ✅ Progress bar animation complete");

    println!("\n📏 Step 4: Testing exact positioning with grid pattern...");
    thread::sleep(Duration::from_secs(1));

    // Clear screen
    full_frame.fill(0);
    device.write_display_framebuffer_rgb888(DISPLAY_ID, &full_frame)?;
    thread::sleep(Duration::from_millis(300));

    // Draw grid pattern at specific coordinates to test alignment
    let test_regions = [
        // (x, y, width, height, color_name, r, g, b)
        (0, 0, 100, 50, "Top-Left Red", 255, 0, 0),
        (380, 0, 100, 50, "Top-Right Green", 0, 255, 0),
        (0, 222, 100, 50, "Bottom-Left Blue", 0, 0, 255),
        (380, 222, 100, 50, "Bottom-Right Yellow", 255, 255, 0),
        (190, 111, 100, 50, "Center Magenta", 255, 0, 255),
    ];

    for (x, y, w, h, name, r, g, b) in test_regions {
        println!("   Drawing {} at ({}, {}) size {}x{}", name, x, y, w, h);

        let mut region = vec![0u8; w * h * 3];
        for pixel in 0..(w * h) {
            let idx = pixel * 3;
            region[idx] = r;
            region[idx + 1] = g;
            region[idx + 2] = b;
        }

        device.write_display_region_rgb888(DISPLAY_ID, x as u16, y as u16, w as u16, h as u16, &region)?;
        thread::sleep(Duration::from_millis(300));
    }

    println!("   ✅ Grid pattern test complete");

    println!("\n🎯 Step 5: Testing edge cases...");
    thread::sleep(Duration::from_secs(1));

    // Test 1: Single pixel update
    println!("   Test 5a: Single pixel at (240, 136) - white");
    let single_pixel = vec![255u8, 255, 255];
    device.write_display_region_rgb888(DISPLAY_ID, 240, 136, 1, 1, &single_pixel)?;
    thread::sleep(Duration::from_millis(500));

    // Test 2: Horizontal line
    println!("   Test 5b: Horizontal line at y=136, x=100-380 - cyan");
    let h_line = vec![0u8, 255, 255].repeat(280);
    device.write_display_region_rgb888(DISPLAY_ID, 100, 136, 280, 1, &h_line)?;
    thread::sleep(Duration::from_millis(500));

    // Test 3: Vertical line
    println!("   Test 5c: Vertical line at x=240, y=50-222 - magenta");
    let mut v_line = vec![0u8; 172 * 3];
    for i in 0..172 {
        v_line[i * 3] = 255;     // R
        v_line[i * 3 + 1] = 0;   // G
        v_line[i * 3 + 2] = 255; // B
    }
    device.write_display_region_rgb888(DISPLAY_ID, 240, 50, 1, 172, &v_line)?;
    thread::sleep(Duration::from_millis(500));

    println!("   ✅ Edge cases test complete");

    println!("\n📐 Step 6: Size accuracy test with reference markers...");
    thread::sleep(Duration::from_secs(1));

    // Clear screen
    full_frame.fill(0);
    device.write_display_framebuffer_rgb888(DISPLAY_ID, &full_frame)?;
    thread::sleep(Duration::from_millis(300));

    // Draw exact 100x100 squares using both full frame and partial updates
    println!("   Drawing reference 100x100 square using full frame (white)");
    for y in 50..150 {
        for x in 50..150 {
            let idx = (y * WIDTH + x) * 3;
            full_frame[idx] = 255;
            full_frame[idx + 1] = 255;
            full_frame[idx + 2] = 255;
        }
    }
    device.write_display_framebuffer_rgb888(DISPLAY_ID, &full_frame)?;
    thread::sleep(Duration::from_secs(1));

    println!("   Drawing 100x100 square using partial update (cyan) - should overlap exactly");
    let square = vec![0u8, 255, 255].repeat(100 * 100);
    device.write_display_region_rgb888(DISPLAY_ID, 50, 50, 100, 100, &square)?;
    thread::sleep(Duration::from_secs(2));

    // If they don't overlap exactly, we'll see white edges

    println!("\n✅ All partial update tests complete!");
    println!("\n🔍 Visual inspection guide:");
    println!("   - If you see white edges around cyan square: SIZE MISMATCH");
    println!("   - If cyan square is offset from where white was: POSITION MISMATCH");
    println!("   - If progress bar filled from top instead of left: Y-AXIS INVERTED");
    println!("   - If colored corners aren't in corners: COORDINATE SYSTEM WRONG");

    thread::sleep(Duration::from_secs(3));

    Ok(())
}

// Simple text drawing helper (very basic, just for labels)
fn draw_text(buffer: &mut [u8], width: usize, x: usize, y: usize, text: &str) {
    // Very simple: just draw a line of pixels for each character
    for (i, _ch) in text.chars().enumerate() {
        let char_x = x + i * 8;
        if char_x + 5 >= width {
            break;
        }

        // Draw a simple 5x7 filled rectangle per character
        for dy in 0..7 {
            for dx in 0..5 {
                let px = char_x + dx;
                let py = y + dy;
                if py * width + px < buffer.len() / 3 {
                    let idx = (py * width + px) * 3;
                    buffer[idx] = 255;
                    buffer[idx + 1] = 255;
                    buffer[idx + 2] = 255;
                }
            }
        }
    }
}
