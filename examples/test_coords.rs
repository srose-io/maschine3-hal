use maschine3_hal::{MaschineMK3, MK3Error};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Coordinate System Test");
    println!("   Drawing labeled boxes to identify coordinate issues");

    let device = match MaschineMK3::new() {
        Ok(device) => device,
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
    const DISPLAY_ID: u8 = 0;

    println!("\n📺 Clearing screen...");
    let mut full_frame = vec![0u8; WIDTH * HEIGHT * 3];
    device.write_display_framebuffer_rgb888(DISPLAY_ID, &full_frame)?;
    thread::sleep(Duration::from_millis(500));

    // Test specific coordinates that should be easy to identify
    let tests = [
        // (x, y, width, height, name, r, g, b)
        (10, 10, 50, 30, "TOP-LEFT (10,10)", 255, 0, 0),
        (420, 10, 50, 30, "TOP-RIGHT (420,10)", 0, 255, 0),
        (10, 232, 50, 30, "BOTTOM-LEFT (10,232)", 0, 0, 255),
        (420, 232, 50, 30, "BOTTOM-RIGHT (420,232)", 255, 255, 0),
        (215, 121, 50, 30, "CENTER (215,121)", 255, 0, 255),
    ];

    for (x, y, w, h, name, r, g, b) in tests {
        println!("\n🔸 Drawing: {}", name);
        println!("   Coordinates: x={}, y={}, width={}, height={}", x, y, w, h);

        let mut region = vec![0u8; (w * h * 3) as usize];
        for pixel in 0..(w * h) {
            let idx = (pixel * 3) as usize;
            region[idx] = r;
            region[idx + 1] = g;
            region[idx + 2] = b;
        }

        device.write_display_region_rgb888(
            DISPLAY_ID,
            x,
            y,
            w,
            h,
            &region,
        )?;

        thread::sleep(Duration::from_secs(1));
    }

    println!("\n👀 VISUAL INSPECTION:");
    println!("   ✅ CORRECT: Boxes appear in their labeled corners/center");
    println!("   ❌ WRONG: Boxes appear elsewhere");
    println!("\n   If boxes are offset, measure the offset in pixels!");
    thread::sleep(Duration::from_secs(5));

    println!("\n🧪 Test 2: Progress bar at known position");
    full_frame.fill(0);
    device.write_display_framebuffer_rgb888(DISPLAY_ID, &full_frame)?;
    thread::sleep(Duration::from_millis(300));

    // Draw background bar using full frame
    let bar_x = 90;
    let bar_y = 116;
    let bar_w = 300;
    let bar_h = 40;

    println!("\n📊 Drawing progress bar background (full frame method)");
    println!("   Position: x={}, y={}", bar_x, bar_y);
    println!("   Size: {}×{}", bar_w, bar_h);

    for y in bar_y..(bar_y + bar_h) {
        for x in bar_x..(bar_x + bar_w) {
            let idx = (y * WIDTH + x) * 3;
            full_frame[idx] = 80;
            full_frame[idx + 1] = 80;
            full_frame[idx + 2] = 80;
        }
    }
    device.write_display_framebuffer_rgb888(DISPLAY_ID, &full_frame)?;
    thread::sleep(Duration::from_secs(1));

    // Draw fill using partial update at SAME coordinates
    let fill_w = 200;
    println!("\n📊 Drawing progress bar fill (partial update method)");
    println!("   Position: x={}, y={}", bar_x, bar_y);
    println!("   Size: {}×{}", fill_w, bar_h);

    let mut fill_region = vec![0u8; fill_w * bar_h * 3];
    for pixel in 0..(fill_w * bar_h) {
        let idx = pixel * 3;
        fill_region[idx] = 0;
        fill_region[idx + 1] = 255;
        fill_region[idx + 2] = 255;
    }

    device.write_display_region_rgb888(
        DISPLAY_ID,
        bar_x as u16,
        bar_y as u16,
        fill_w as u16,
        bar_h as u16,
        &fill_region,
    )?;

    println!("\n👀 VISUAL INSPECTION:");
    println!("   ✅ CORRECT: Cyan fill is INSIDE gray box, aligned at left edge");
    println!("   ❌ WRONG: Cyan fill is ABOVE, BELOW, or SHIFTED from gray box");
    println!("\n   Measure the offset if misaligned!");
    thread::sleep(Duration::from_secs(5));

    Ok(())
}
