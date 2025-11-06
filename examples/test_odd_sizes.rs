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
    println!("🧪 Testing Odd Dimension Sizes");

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

        // Clear screen
        println!("\n📺 Clearing screen");
        mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
        thread::sleep(Duration::from_millis(500));

        // Test different sizes that had issues
        let test_sizes = [
            (3, "3x3 (9 pixels - odd)"),
            (4, "4x4 (16 pixels - even)"),
            (5, "5x5 (25 pixels - odd)"),
            (6, "6x6 (36 pixels - even)"),
            (7, "7x7 (49 pixels - odd)"),
            (8, "8x8 (64 pixels - even)"),
            (15, "15x15 (225 pixels - odd)"),
            (16, "16x16 (256 pixels - even)"),
        ];

        let mut x_pos = 50;
        let y_pos = 100;

        for (size, desc) in test_sizes {
            println!("\n🔸 Testing {} at ({}, {})", desc, x_pos, y_pos);

            // Draw square
            for dy in 0..size {
                for dx in 0..size {
                    let px = x_pos + dx;
                    let py = y_pos + dy;
                    if px < WIDTH && py < HEIGHT {
                        let idx = (py * WIDTH + px) * 3;
                        framebuffer[idx] = 255;     // Red
                        framebuffer[idx + 1] = 0;
                        framebuffer[idx + 2] = 0;
                    }
                }
            }

            let result = mk3_write_display_rgb888_dirty(device, 0, framebuffer.as_ptr(), BUFFER_SIZE as u32);
            if result != 0 {
                println!("   ❌ FAILED with error code: {}", result);
                println!("   Size: {}x{} = {} pixels", size, size, size * size);
                println!("   Half pixels: {}", (size * size + 1) / 2);
                println!("   RGB565 bytes: {}", size * size * 2);
            } else {
                println!("   ✅ Success!");
            }

            x_pos += size + 10;
            thread::sleep(Duration::from_millis(500));
        }

        println!("\n👀 Check display: All red squares should be visible");
        println!("   If any are missing, that size failed");
        thread::sleep(Duration::from_secs(5));

        mk3_free(device);
        println!("\n✅ Test complete");
    }
}
