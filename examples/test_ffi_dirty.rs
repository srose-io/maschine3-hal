use maschine3_hal::MaschineMK3;
use std::thread;
use std::time::Duration;

// Import the FFI functions
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
    println!("🧪 FFI Dirty Region Test (Unity Simulation)");
    println!("   Using mk3_write_display_rgb888_dirty through FFI");

    unsafe {
        // Create device using FFI (like Unity does)
        let device = mk3_new();
        if device.is_null() {
            println!("❌ Failed to create device");
            return;
        }
        println!("✅ Device created via FFI");

        // Check display availability
        if mk3_is_display_available(device) != 1 {
            println!("❌ Display not available");
            mk3_free(device);
            return;
        }
        println!("✅ Display available");

        const WIDTH: usize = 480;
        const HEIGHT: usize = 272;
        const DISPLAY_ID: u32 = 0;
        const BUFFER_SIZE: usize = WIDTH * HEIGHT * 3;

        // Create framebuffer (like Unity does)
        let mut framebuffer = vec![0u8; BUFFER_SIZE];

        println!("\n📺 Step 1: Clear screen");
        let result = mk3_write_display_rgb888_dirty(
            device,
            DISPLAY_ID,
            framebuffer.as_ptr(),
            BUFFER_SIZE as u32,
        );
        if result != 0 {
            println!("❌ Failed to write: error {}", result);
            mk3_free(device);
            return;
        }
        thread::sleep(Duration::from_millis(500));

        println!("\n📊 Step 2: Draw progress bar background");
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

        let result = mk3_write_display_rgb888_dirty(
            device,
            DISPLAY_ID,
            framebuffer.as_ptr(),
            BUFFER_SIZE as u32,
        );
        if result != 0 {
            println!("❌ Failed to write: error {}", result);
            mk3_free(device);
            return;
        }
        thread::sleep(Duration::from_secs(1));

        println!("\n🔄 Step 3: Animate progress bar");
        for step in 0..=10 {
            let progress = step as f32 / 10.0;
            let fill_width = (bar_width as f32 * progress) as usize;

            println!("   Progress: {:.0}%", progress * 100.0);

            // Clear to gray
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

            let result = mk3_write_display_rgb888_dirty(
                device,
                DISPLAY_ID,
                framebuffer.as_ptr(),
                BUFFER_SIZE as u32,
            );
            if result != 0 {
                println!("❌ Failed to write: error {}", result);
                mk3_free(device);
                return;
            }
            thread::sleep(Duration::from_millis(300));
        }

        println!("\n👀 VISUAL INSPECTION:");
        println!("   This uses EXACTLY the same FFI calls as Unity!");
        println!("   ✅ CORRECT: Cyan bar inside gray box");
        println!("   ❌ WRONG: Cyan bar above/below gray box");
        thread::sleep(Duration::from_secs(3));

        // Cleanup
        mk3_free(device);
        println!("\n✅ Test complete");
    }
}
