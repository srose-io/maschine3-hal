use maschine3_hal::{MaschineMK3, MK3Error};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Maschine MK3 Alignment Diagnostic Test");

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
    const DISPLAY_ID: u8 = 0;

    println!("\n🧪 Test 1: Single rectangle using full frame");
    let mut full_frame = vec![0u8; WIDTH * HEIGHT * 3];

    // Draw a white rectangle at (100, 100) size 280×72 using FULL FRAME method
    let test_x = 100;
    let test_y = 100;
    let test_w = 280;
    let test_h = 72;

    for y in test_y..(test_y + test_h) {
        for x in test_x..(test_x + test_w) {
            let idx = (y * WIDTH + x) * 3;
            full_frame[idx] = 255;
            full_frame[idx + 1] = 255;
            full_frame[idx + 2] = 255;
        }
    }

    device.write_display_framebuffer_rgb888(DISPLAY_ID, &full_frame)?;
    println!("   White box drawn at ({}, {}) size {}×{}", test_x, test_y, test_w, test_h);
    thread::sleep(Duration::from_secs(2));

    println!("\n🧪 Test 2: Overlay cyan rectangle using partial update at SAME coordinates");
    // Draw cyan rectangle at EXACT same coordinates using PARTIAL UPDATE
    let mut region_data = vec![0u8; test_w * test_h * 3];
    for i in 0..(test_w * test_h) {
        let idx = i * 3;
        region_data[idx] = 0;       // R
        region_data[idx + 1] = 255; // G
        region_data[idx + 2] = 255; // B
    }

    device.write_display_region_rgb888(
        DISPLAY_ID,
        test_x as u16,
        test_y as u16,
        test_w as u16,
        test_h as u16,
        &region_data,
    )?;
    println!("   Cyan box drawn at ({}, {}) size {}×{}", test_x, test_y, test_w, test_h);
    println!("   👀 If aligned correctly: Cyan should EXACTLY cover white");
    println!("   👀 If misaligned: White edges will be visible");
    thread::sleep(Duration::from_secs(3));

    println!("\n🧪 Test 3: Grid pattern to measure offset");
    full_frame.fill(0);
    device.write_display_framebuffer_rgb888(DISPLAY_ID, &full_frame)?;
    thread::sleep(Duration::from_millis(300));

    // Draw reference grid using full frame (10px squares every 50px)
    for grid_y in (0..HEIGHT).step_by(50) {
        for grid_x in (0..WIDTH).step_by(50) {
            for dy in 0..10 {
                for dx in 0..10 {
                    let y = grid_y + dy;
                    let x = grid_x + dx;
                    if y < HEIGHT && x < WIDTH {
                        let idx = (y * WIDTH + x) * 3;
                        full_frame[idx] = 100;
                        full_frame[idx + 1] = 100;
                        full_frame[idx + 2] = 100;
                    }
                }
            }
        }
    }
    device.write_display_framebuffer_rgb888(DISPLAY_ID, &full_frame)?;
    println!("   Gray grid drawn (reference)");
    thread::sleep(Duration::from_secs(1));

    // Overlay red squares using partial updates at SAME coordinates
    for grid_y in (50..HEIGHT).step_by(50) {
        for grid_x in (50..WIDTH).step_by(50) {
            let red_square = vec![255u8, 0, 0].repeat(10 * 10);
            device.write_display_region_rgb888(
                DISPLAY_ID,
                grid_x as u16,
                grid_y as u16,
                10,
                10,
                &red_square,
            )?;
            thread::sleep(Duration::from_millis(100));
        }
    }
    println!("   Red squares drawn at grid intersections");
    println!("   👀 Check if red squares are OFFSET from gray squares");
    thread::sleep(Duration::from_secs(3));

    println!("\n🧪 Test 4: Measure exact pixel offset");
    full_frame.fill(0);
    device.write_display_framebuffer_rgb888(DISPLAY_ID, &full_frame)?;
    thread::sleep(Duration::from_millis(300));

    // Draw coordinate axes using full frame
    let center_x = 240;
    let center_y = 136;

    // Horizontal line (white)
    for x in 0..WIDTH {
        let idx = (center_y * WIDTH + x) * 3;
        full_frame[idx] = 255;
        full_frame[idx + 1] = 255;
        full_frame[idx + 2] = 255;
    }

    // Vertical line (white)
    for y in 0..HEIGHT {
        let idx = (y * WIDTH + center_x) * 3;
        full_frame[idx] = 255;
        full_frame[idx + 1] = 255;
        full_frame[idx + 2] = 255;
    }

    device.write_display_framebuffer_rgb888(DISPLAY_ID, &full_frame)?;
    println!("   White cross drawn at center ({}, {})", center_x, center_y);
    thread::sleep(Duration::from_secs(1));

    // Draw red cross using partial update at SAME coordinates
    let h_line = vec![255u8, 0, 0].repeat(WIDTH);
    device.write_display_region_rgb888(DISPLAY_ID, 0, center_y as u16, WIDTH as u16, 1, &h_line)?;

    let mut v_line = vec![0u8; HEIGHT * 3];
    for i in 0..HEIGHT {
        v_line[i * 3] = 255;
        v_line[i * 3 + 1] = 0;
        v_line[i * 3 + 2] = 0;
    }
    device.write_display_region_rgb888(DISPLAY_ID, center_x as u16, 0, 1, HEIGHT as u16, &v_line)?;

    println!("   Red cross overlaid at center ({}, {})", center_x, center_y);
    println!("   👀 Measure pixel offset between white and red lines");
    thread::sleep(Duration::from_secs(3));

    println!("\n🧪 Test 5: Size verification with nested boxes");
    full_frame.fill(0);
    device.write_display_framebuffer_rgb888(DISPLAY_ID, &full_frame)?;
    thread::sleep(Duration::from_millis(300));

    // Draw nested boxes with known sizes
    let sizes = [(200, 150), (180, 130), (160, 110), (140, 90), (120, 70)];
    let colors = [
        (255, 255, 255), // White
        (255, 0, 0),     // Red
        (0, 255, 0),     // Green
        (0, 0, 255),     // Blue
        (255, 255, 0),   // Yellow
    ];

    for (i, ((w, h), (r, g, b))) in sizes.iter().zip(colors.iter()).enumerate() {
        let box_x = (WIDTH - w) / 2;
        let box_y = (HEIGHT - h) / 2;

        println!("   Box {}: {}×{} at ({}, {}) - {:?}", i + 1, w, h, box_x, box_y, (r, g, b));

        // Draw using partial update
        let mut box_data = vec![0u8; w * h * 3];
        // Only draw border (4px wide)
        for y in 0..*h {
            for x in 0..*w {
                if y < 4 || y >= h - 4 || x < 4 || x >= w - 4 {
                    let idx = (y * w + x) * 3;
                    box_data[idx] = *r;
                    box_data[idx + 1] = *g;
                    box_data[idx + 2] = *b;
                }
            }
        }

        device.write_display_region_rgb888(
            DISPLAY_ID,
            box_x as u16,
            box_y as u16,
            *w as u16,
            *h as u16,
            &box_data,
        )?;
        thread::sleep(Duration::from_millis(500));
    }

    println!("\n   👀 Check if boxes are concentric (same center point)");
    println!("   👀 Check if each box is exactly 20px smaller than previous");
    thread::sleep(Duration::from_secs(5));

    println!("\n✅ Diagnostic tests complete!");
    println!("\n📊 Analysis:");
    println!("   - Test 1-2: Check for position offset");
    println!("   - Test 3: Check for systematic X/Y offset in grid");
    println!("   - Test 4: Measure exact pixel offset in cross");
    println!("   - Test 5: Check for size scaling issues");

    Ok(())
}
