use maschine3_hal::{MaschineMK3, MK3Error};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Dirty Region Detection Progress Bar Test");
    println!("   Simulating Unity's rendering approach");

    let mut device = match MaschineMK3::new() {
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
    const DISPLAY_ID: u8 = 0;

    // Create a persistent frame buffer (like Unity does)
    let mut framebuffer = vec![0u8; WIDTH * HEIGHT * 3];

    println!("\n📺 Step 1: Clear screen (first dirty update)");
    // First call initializes dirty tracking
    device.write_display_framebuffer_rgb888_dirty(DISPLAY_ID, &framebuffer)?;
    thread::sleep(Duration::from_millis(500));

    println!("\n📊 Step 2: Draw progress bar background into framebuffer");
    let bar_x = 90;
    let bar_y = 116;
    let bar_width = 300;
    let bar_height = 40;

    // Draw gray background
    for y in bar_y..(bar_y + bar_height) {
        for x in bar_x..(bar_x + bar_width) {
            let idx = (y * WIDTH + x) * 3;
            framebuffer[idx] = 80;
            framebuffer[idx + 1] = 80;
            framebuffer[idx + 2] = 80;
        }
    }

    println!("   Sending via dirty detection...");
    device.write_display_framebuffer_rgb888_dirty(DISPLAY_ID, &framebuffer)?;
    thread::sleep(Duration::from_secs(1));

    println!("\n🔄 Step 3: Animate progress bar (10 steps)");
    println!("   This simulates Unity rendering frames with changing progress");

    for step in 0..=10 {
        let progress = step as f32 / 10.0;
        let fill_width = (bar_width as f32 * progress) as usize;

        println!("   Progress: {:.0}% (width: {}px)", progress * 100.0, fill_width);

        // Clear previous fill to gray first
        for y in bar_y..(bar_y + bar_height) {
            for x in bar_x..(bar_x + bar_width) {
                let idx = (y * WIDTH + x) * 3;
                framebuffer[idx] = 80;
                framebuffer[idx + 1] = 80;
                framebuffer[idx + 2] = 80;
            }
        }

        // Draw cyan fill
        for y in bar_y..(bar_y + bar_height) {
            for x in bar_x..(bar_x + fill_width) {
                let idx = (y * WIDTH + x) * 3;
                framebuffer[idx] = 0;
                framebuffer[idx + 1] = 255;
                framebuffer[idx + 2] = 255;
            }
        }

        // Send via dirty detection
        device.write_display_framebuffer_rgb888_dirty(DISPLAY_ID, &framebuffer)?;
        thread::sleep(Duration::from_millis(300));
    }

    println!("\n👀 VISUAL INSPECTION:");
    println!("   ✅ CORRECT: Cyan bar fills LEFT to RIGHT inside gray box");
    println!("   ❌ WRONG: Cyan bar is ABOVE gray box or misaligned");
    thread::sleep(Duration::from_secs(2));

    println!("\n🧪 Step 4: Test with smaller incremental changes");
    println!("   Adding small UI elements to test dirty region accuracy");

    // Add a title bar at top
    for y in 10..30 {
        for x in 90..390 {
            let idx = (y * WIDTH + x) * 3;
            framebuffer[idx] = 100;
            framebuffer[idx + 1] = 100;
            framebuffer[idx + 2] = 150;
        }
    }
    println!("   Added title bar");
    device.write_display_framebuffer_rgb888_dirty(DISPLAY_ID, &framebuffer)?;
    thread::sleep(Duration::from_millis(500));

    // Add indicator dots (small changes)
    let dot_positions = [(100, 170), (140, 170), (180, 170), (220, 170)];
    for (i, (dot_x, dot_y)) in dot_positions.iter().enumerate() {
        // Draw 10x10 colored dot
        for dy in 0..10 {
            for dx in 0..10 {
                let y = dot_y + dy;
                let x = dot_x + dx;
                let idx = (y * WIDTH + x) * 3;
                framebuffer[idx] = if i % 2 == 0 { 255 } else { 0 };
                framebuffer[idx + 1] = if i == 1 { 255 } else { 0 };
                framebuffer[idx + 2] = if i == 2 { 255 } else { 0 };
            }
        }
        println!("   Added dot {} at ({}, {})", i + 1, dot_x, dot_y);
        device.write_display_framebuffer_rgb888_dirty(DISPLAY_ID, &framebuffer)?;
        thread::sleep(Duration::from_millis(300));
    }

    println!("\n👀 Check if title bar and dots are positioned correctly!");
    thread::sleep(Duration::from_secs(2));

    println!("\n🧪 Step 5: Test partial clear");
    // Clear just the progress bar area
    for y in bar_y..(bar_y + bar_height) {
        for x in bar_x..(bar_x + bar_width) {
            let idx = (y * WIDTH + x) * 3;
            framebuffer[idx] = 0;
            framebuffer[idx + 1] = 0;
            framebuffer[idx + 2] = 0;
        }
    }
    println!("   Clearing progress bar area");
    device.write_display_framebuffer_rgb888_dirty(DISPLAY_ID, &framebuffer)?;
    thread::sleep(Duration::from_secs(1));

    println!("\n🧪 Step 6: Redraw progress bar at 75%");
    let fill_width = (bar_width as f32 * 0.75) as usize;

    // Gray background
    for y in bar_y..(bar_y + bar_height) {
        for x in bar_x..(bar_x + bar_width) {
            let idx = (y * WIDTH + x) * 3;
            framebuffer[idx] = 80;
            framebuffer[idx + 1] = 80;
            framebuffer[idx + 2] = 80;
        }
    }

    // Cyan fill (75%)
    for y in bar_y..(bar_y + bar_height) {
        for x in bar_x..(bar_x + fill_width) {
            let idx = (y * WIDTH + x) * 3;
            framebuffer[idx] = 0;
            framebuffer[idx + 1] = 255;
            framebuffer[idx + 2] = 255;
        }
    }

    println!("   Redrawing at 75%");
    device.write_display_framebuffer_rgb888_dirty(DISPLAY_ID, &framebuffer)?;

    println!("\n👀 Final check: Progress bar should be at 75%, aligned correctly");
    thread::sleep(Duration::from_secs(3));

    println!("\n✅ Test complete!");
    println!("\n📊 Summary:");
    println!("   - This test simulates Unity's approach exactly");
    println!("   - Full framebuffer is maintained and modified");
    println!("   - Only changed regions are sent via dirty detection");
    println!("   - If progress bar is misaligned, the issue is in dirty region flip logic");

    Ok(())
}
