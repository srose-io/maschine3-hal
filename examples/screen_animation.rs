use maschine3_hal::MaschineMK3;
use std::io::Result;
use std::time::Instant;

const EACH_WIDTH: u16 = 480;
const EACH_HEIGHT: u16 = 272;
const XPOS: u16 = 0;
const YPOS: u16 = 0;
const PIXEL_LENGTH: usize = 2; // RGB565 = 2 bytes per pixel
const HEADER_LENGTH: usize = 16;
const COMMAND_LENGTH: usize = 4;

fn rgb565x(red: u8, green: u8, blue: u8) -> u16 {
    // Convert to RGB444 with corrected channels (4 bits each)
    let r5 = (red >> 3) as u16;
    let g3high = (green >> 5) as u16;
    let glow = ((green >> 2) & 7) as u16;
    let b5 = (blue >> 3) as u16;

    // Pack 565 bits with offset: GGGB BBBB RRRR RGGG
    (glow << 13) | (b5 << 8) | (r5 << 3) | g3high
}

// fn rgb_split_green(red: u8, green: u8, blue: u8) -> u16 {
//     let corrected_r = blue;
//     let corrected_g = red;
//     let corrected_b = green;

//     // Pack as: GGGB BBBB RRRR GGGG (with 6-bit green split 3+3)
//     let r4 = (corrected_r >> 4) as u16; // Red: 4 bits
//     let b5 = (corrected_b >> 3) as u16; // Blue: 5 bits
//     let g_low = (corrected_g >> 5) as u16; // Green low: 3 bits (from MSB)
//     let g_high = (corrected_g >> 3) & 0x7; // Green high: 3 bits (from middle)

//     (g_low << 13) | (b5 << 8) | (r4 << 4) | (g_high << 1)
// }

fn fill_header(buf: &mut [u8], display_num: u8, x: u16, y: u16, width: u16, height: u16) {
    buf[0] = 0x84; // Command type
    buf[1] = 0x00; // Always 0
    buf[2] = display_num; // Display ID (0=left, 1=right)
    buf[3] = 0x60; // Always 0x60
    // bytes 4-7 are 0x00

    // Coordinates at offsets 8-15
    buf[8] = (x >> 8) as u8; // X MSB
    buf[9] = (x & 0xff) as u8; // X LSB
    buf[10] = (y >> 8) as u8; // Y MSB  
    buf[11] = (y & 0xff) as u8; // Y LSB
    buf[12] = (width >> 8) as u8; // Width MSB
    buf[13] = (width & 0xff) as u8; // Width LSB
    buf[14] = (height >> 8) as u8; // Height MSB
    buf[15] = (height & 0xff) as u8; // Height LSB
}

fn fill_transmit_command(buf: &mut [u8], num_pixels: u32, offset: usize) {
    let half_pixels = num_pixels / 2;

    // Command 0x00 = transmit pixels
    buf[offset] = 0x00;
    buf[offset + 1] = (half_pixels >> 16) as u8; // MSB  
    buf[offset + 2] = (half_pixels >> 8) as u8; // Middle
    buf[offset + 3] = (half_pixels & 0xff) as u8; // LSB
}

fn create_single_row_packet(display_num: u8, time: f32) -> Vec<u8> {
    // Create pixel buffer for entire display
    let num_pixels = (EACH_WIDTH as usize) * (EACH_HEIGHT as usize);
    let mut pixel_buffer = vec![0u8; num_pixels * PIXEL_LENGTH];

    for y in 0..EACH_HEIGHT {
        for x in 0..EACH_WIDTH {
            let pixel_idx = (y as usize * EACH_WIDTH as usize + x as usize) * 2;

            // Create horizontal gradient with dithering
            let mut x_progress = x as f32 / EACH_WIDTH as f32;
            let mut y_progress = y as f32 / EACH_HEIGHT as f32;
            x_progress = (x_progress).min(1.0);
            y_progress = (y_progress + time.sin()).min(1.0);

            // Simple 2x2 Bayer dither to reduce banding
            let dither_matrix = [[0.0, 0.5], [0.75, 0.25]];
            let dither_value = dither_matrix[(y % 2) as usize][(x % 2) as usize];
            let dither_amount = 8.0; // Adjust strength (0-8 works well)

            let red_float = x_progress * 255.0 + (dither_value - 0.375) * dither_amount;
            let blue_float =
                y_progress * y_progress * 255.0 + (dither_value - 0.375) * dither_amount;

            let red = red_float.clamp(0.0, 255.0) as u8;
            let blue = blue_float.clamp(0.0, 255.0) as u8;
            let color_rgb444 = rgb565x(red, blue, 0); // Pure red channel animation

            pixel_buffer[pixel_idx] = (color_rgb444 & 0xff) as u8; // LSB
            pixel_buffer[pixel_idx + 1] = (color_rgb444 >> 8) as u8; // MSB
        }
    }

    // Create packet with header + single transmit command + data + blit + end
    let packet_size =
        HEADER_LENGTH + COMMAND_LENGTH + (num_pixels * PIXEL_LENGTH) + COMMAND_LENGTH * 2;
    let mut packet = vec![0u8; packet_size];
    let mut offset = 0;

    // Fill header for entire display
    fill_header(
        &mut packet[offset..],
        display_num,
        XPOS,
        YPOS,
        EACH_WIDTH,
        EACH_HEIGHT, // full height
    );
    offset += HEADER_LENGTH;

    // Add single transmit command for entire display
    fill_transmit_command(&mut packet, num_pixels as u32, offset);
    offset += COMMAND_LENGTH;

    // Copy pixel data for entire display
    packet[offset..offset + (num_pixels * PIXEL_LENGTH)].copy_from_slice(&pixel_buffer);
    offset += num_pixels * PIXEL_LENGTH;

    // Add blit command (0x03)
    packet[offset] = 0x03;
    packet[offset + 1] = 0x00;
    packet[offset + 2] = 0x00;
    packet[offset + 3] = 0x00;
    offset += COMMAND_LENGTH;

    // Add end command (0x40)
    packet[offset] = 0x40;
    packet[offset + 1] = 0x00;
    packet[offset + 2] = 0x00;
    packet[offset + 3] = 0x00;

    packet
}

fn paint_single_frame(device: &MaschineMK3, packet: &[u8]) -> Result<()> {
    if let Err(e) = device.send_raw_data(packet) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to send packet: {}", e),
        ));
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("🌈 Multi-Row Command Test - Rolling Red Gradient");

    // Initialize device
    let device = match MaschineMK3::new() {
        Ok(dev) => dev,
        Err(e) => {
            eprintln!("Failed to initialize MK3 device: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Device not found",
            ));
        }
    };

    println!("Device initialized successfully");
    println!("🔴 Testing multiple row commands in single packet...");
    println!("Press Ctrl+C to stop");

    let start_time = Instant::now();
    let mut frame_count = 0;

    loop {
        let elapsed = start_time.elapsed().as_secs_f32();

        // Generate frame with multiple row commands in one packet
        let packet = create_single_row_packet(0, elapsed);

        // Paint frame
        if let Err(e) = paint_single_frame(&device, &packet) {
            eprintln!("Failed to paint frame: {}", e);
            break;
        }

        frame_count += 1;

        // Print frame rate every 30 frames
        if frame_count % 30 == 0 {
            let fps = frame_count as f32 / elapsed;
            println!(
                "📊 Frame {}: {:.1} FPS - Time: {:.1}s",
                frame_count, fps, elapsed
            );
        }

        // Stop after 200 frames
        if frame_count >= 200 {
            let final_fps = frame_count as f32 / elapsed;
            println!(
                "🏁 Test complete! Final: {:.1} FPS over {} frames",
                final_fps, frame_count
            );
            break;
        }
    }

    Ok(())
}
