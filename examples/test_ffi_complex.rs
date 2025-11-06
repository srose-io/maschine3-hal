use maschine3_hal::MaschineMK3;
use std::thread;
use std::time::Duration;

extern "C" {
    fn mk3_new() -> *mut MaschineMK3;
    fn mk3_free(device: *mut MaschineMK3);
    fn mk3_write_display_rgb888_dirty(
        device: *mut MaschineMK3,
        display_id: u32,
        rgb888_data: *const u8,
        data_len: u32,
    ) -> i32;
    fn mk3_is_display_available(device: *const MaschineMK3) -> i32;
}

fn main() {
    println!("🧪 Complex FFI Dirty Region Test");
    println!("   Testing multiple UI elements at different positions");

    unsafe {
        let device = mk3_new();
        if device.is_null() {
            println!("❌ Failed to create device");
            return;
        }

        if mk3_is_display_available(device) != 1 {
            println!("❌ Display not available");
            mk3_free(device);
            return;
        }

        const WIDTH: usize = 480;
        const HEIGHT: usize = 272;
        const BUFFER_SIZE: usize = WIDTH * HEIGHT * 3;
        let mut framebuffer = vec![0u8; BUFFER_SIZE];

        println!("\n📺 Initial clear");
        mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
        thread::sleep(Duration::from_millis(300));

        // Test 1: Top bar (near Y=0)
        println!("\n🔸 Test 1: Top title bar at Y=20");
        draw_rect(&mut framebuffer, 50, 20, 380, 30, 100, 100, 150);
        mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
        thread::sleep(Duration::from_secs(1));

        // Test 2: Progress bar near top (Y=60)
        println!("\n🔸 Test 2: Progress bar #1 at Y=60");
        draw_rect(&mut framebuffer, 50, 60, 300, 30, 80, 80, 80);
        mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
        thread::sleep(Duration::from_millis(500));

        println!("   Animating progress bar #1...");
        for i in 0..=5 {
            let fill_width = 60 * i;
            draw_rect(&mut framebuffer, 50, 60, 300, 30, 80, 80, 80); // Background
            draw_rect(&mut framebuffer, 50, 60, fill_width, 30, 0, 255, 255); // Cyan fill
            mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
            thread::sleep(Duration::from_millis(200));
        }

        // Test 3: Progress bar in middle (Y=120)
        println!("\n🔸 Test 3: Progress bar #2 at Y=120");
        draw_rect(&mut framebuffer, 100, 120, 280, 30, 80, 80, 80);
        mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
        thread::sleep(Duration::from_millis(500));

        println!("   Animating progress bar #2...");
        for i in 0..=5 {
            let fill_width = 56 * i;
            draw_rect(&mut framebuffer, 100, 120, 280, 30, 80, 80, 80); // Background
            draw_rect(&mut framebuffer, 100, 120, fill_width, 30, 255, 0, 255); // Magenta fill
            mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
            thread::sleep(Duration::from_millis(200));
        }

        // Test 4: Progress bar near bottom (Y=200)
        println!("\n🔸 Test 4: Progress bar #3 at Y=200");
        draw_rect(&mut framebuffer, 150, 200, 200, 30, 80, 80, 80);
        mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
        thread::sleep(Duration::from_millis(500));

        println!("   Animating progress bar #3...");
        for i in 0..=(5*40) {
            let fill_width = i;
            draw_rect(&mut framebuffer, 150, 200, 200, 30, 80, 80, 80); // Background
            draw_rect(&mut framebuffer, 150, 200, fill_width, 30, 255, 255, 0); // Yellow fill
            mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
            thread::sleep(Duration::from_millis(1));
        }

        // Test 5: Side panels
        println!("\n🔸 Test 5: Left side panel at X=10");
        draw_rect(&mut framebuffer, 10, 80, 30, 150, 150, 100, 100);
        mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
        thread::sleep(Duration::from_millis(500));

        println!("\n🔸 Test 6: Right side panel at X=440");
        draw_rect(&mut framebuffer, 440, 80, 30, 150, 100, 150, 100);
        mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
        thread::sleep(Duration::from_millis(500));

        // Test 7: Bottom status bar
        println!("\n🔸 Test 7: Bottom status bar at Y=240");
        draw_rect(&mut framebuffer, 50, 240, 380, 20, 150, 150, 100);
        mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
        thread::sleep(Duration::from_secs(1));


        framebuffer = vec![0u8; BUFFER_SIZE];
        println!("\n📺 Initial clear");
        mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
        thread::sleep(Duration::from_millis(300));

        println!("\n🔸 Test 6: Right side panel at X=440");
        draw_rect(&mut framebuffer, 440, 80, 30, 150, 100, 150, 100);
        mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
        thread::sleep(Duration::from_millis(500));

        // Test 8: Indicator dots at various Y positions
        println!("\n🔸 Test 8: Indicator dots at different Y levels");
        let dot_positions = [
            (60, 100, 255, 0, 0),     // Red - near top
            (60, 130, 0, 255, 0),     // Green - middle-upper
            (60, 160, 0, 0, 255),     // Blue - middle
            (60, 190, 255, 255, 0),   // Yellow - middle-lower
            (60, 220, 255, 0, 255),   // Magenta - near bottom
        ];

        for (x, y, r, g, b) in dot_positions {
            println!("   Drawing dot at ({}, {}) color ({},{},{})", x, y, r, g, b);
            draw_rect(&mut framebuffer, x, y, 15, 15, r, g, b);
            let result = mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
            thread::sleep(Duration::from_millis(300));
        }

                // Test 7: Bottom status bar
        println!("\n🔸 Test 7: Bottom status bar at Y=240");
        draw_rect(&mut framebuffer, 50, 240, 380, 20, 150, 150, 100);
        mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
        thread::sleep(Duration::from_secs(1));

        // Test 9: Vertical gradient to check Y-coordinate mapping
        println!("\n🔸 Test 9: Vertical gradient (Y-coordinate test)");
        framebuffer.fill(0);
        for y in 0..HEIGHT {
            let color_val = ((y as f32 / HEIGHT as f32) * 255.0) as u8;
            for x in 200..280 {
                let idx = (y * WIDTH + x) * 3;
                framebuffer[idx] = color_val;
                framebuffer[idx + 1] = color_val;
                framebuffer[idx + 2] = color_val;
            }
        }
        println!("   Sending gradient...");
        let result = mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
        if result != 0 {
            println!("   ❌ Failed: error {}", result);
        }
        thread::sleep(Duration::from_secs(2));

        // Test 10: Multiple simultaneous updates
        println!("\n🔸 Test 10: Multiple regions updating simultaneously");
        framebuffer.fill(0);
        for frame in 0..10 {
            // Top bar
            let top_width = 50 + frame * 30;
            draw_rect(&mut framebuffer, 50, 30, top_width, 20, 255, 0, 0);

            // Middle bar
            let mid_width = 50 + (9 - frame) * 30;
            draw_rect(&mut framebuffer, 50, 126, mid_width, 20, 0, 255, 0);

            // Bottom bar
            let bot_width = 50 + frame * 30;
            draw_rect(&mut framebuffer, 50, 222, bot_width, 20, 0, 0, 255);

            mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
            thread::sleep(Duration::from_millis(150));
        }

        println!("\n👀 VISUAL INSPECTION CHECKLIST:");
        println!("   ✅ All 3 progress bars should be aligned with their gray backgrounds");
        println!("   ✅ Colored dots should form a vertical line");
        println!("   ✅ Gradient should go from black (top) to white (bottom)");
        println!("   ✅ Multiple bars in test 10 should animate smoothly at their positions");
        println!("   ❌ If ANY element is offset vertically, note which Y position is affected");
        thread::sleep(Duration::from_secs(3));

        mk3_free(device);
        println!("\n✅ Complex test complete!");
    }
}

fn draw_rect(fb: &mut [u8], x: usize, y: usize, w: usize, h: usize, r: u8, g: u8, b: u8) {
    const WIDTH: usize = 480;
    const HEIGHT: usize = 272;

    for dy in 0..h {
        for dx in 0..w {
            let px = x + dx;
            let py = y + dy;
            if px < WIDTH && py < HEIGHT {
                let idx = (py * WIDTH + px) * 3;
                fb[idx] = r;
                fb[idx + 1] = g;
                fb[idx + 2] = b;
            }
        }
    }
}
