use mk3_hal::MaschineMK3;
use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

#[derive(Debug)]
struct DisplayPacketInfo {
    header_part1: [u8; 16],
    header_part2: [u8; 16],
    x_start: u16,
    y_start: u16,
    width: u16,
    height: u16,
    display_id: u8,
    commands_start: usize,
}

fn load_raw_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

fn parse_display_packet(data: &[u8]) -> Option<DisplayPacketInfo> {
    if data.len() < 32 {
        return None;
    }

    // Header Part 1 (16 bytes)
    let mut header_part1 = [0u8; 16];
    header_part1.copy_from_slice(&data[0..16]);

    // Header Part 2 (16 bytes)
    let mut header_part2 = [0u8; 16];
    header_part2.copy_from_slice(&data[16..32]);

    // Parse header according to WORKING JavaScript implementation
    // Header Part 1: bytes 0-15 (Header 3 is display ID)
    let display_id = data[2]; // Header 3

    // CORRECTED: Coordinates are at header offsets 8-15, not 16+!
    let x_start = ((data[8] as u16) << 8) | (data[9] as u16);   // offset 8-9
    let y_start = ((data[10] as u16) << 8) | (data[11] as u16); // offset 10-11
    let width = ((data[12] as u16) << 8) | (data[13] as u16);   // offset 12-13
    let height = ((data[14] as u16) << 8) | (data[15] as u16);  // offset 14-15

    println!("📊 Raw header bytes 0-31: {:02X?}", &data[0..32]);
    println!(
        "📊 Corrected coords: x={}, y={}, w={}, h={}",
        x_start, y_start, width, height
    );

    Some(DisplayPacketInfo {
        header_part1,
        header_part2,
        x_start,
        y_start,
        width,
        height,
        display_id,
        commands_start: 32,
    })
}

fn inject_test_pattern(data: &mut [u8], info: &DisplayPacketInfo) -> bool {
    println!("📝 Injecting test pattern into packet:");
    println!(
        "   Display: {} | Region: {}x{} at ({}, {})",
        info.display_id, info.width, info.height, info.x_start, info.y_start
    );

    let mut pos = info.commands_start;
    let data_len = data.len();

    while pos + 4 <= data_len {
        let cmd = data[pos];
        let param1 = data[pos + 1];
        let param2 = data[pos + 2];
        let param3 = data[pos + 3];

        match cmd {
            0x00 => {
                // Transmit pixels command
                let pixel_count =
                    ((param1 as u32) << 16) | ((param2 as u32) << 8) | (param3 as u32);
                let pixel_bytes = pixel_count * 2; // RGB565 = 2 bytes per pixel
                let data_end = pos + 4 + pixel_bytes as usize;

                if data_end <= data_len {
                    println!(
                        "   🎨 Found pixel data: {} pixels ({} bytes) at offset {}",
                        pixel_count,
                        pixel_bytes,
                        pos + 4
                    );

                    // Inject red/blue gradient pattern
                    for i in 0..pixel_count {
                        let pixel_offset = pos + 4 + (i as usize * 2);
                        if pixel_offset + 1 < data_len {
                            let ratio = i as f32 / pixel_count as f32;
                            // Create red-to-blue gradient
                            let red = ((1.0 - ratio) * 31.0) as u8; // 5-bit red
                            let blue = (ratio * 31.0) as u8; // 5-bit blue
                            let green = 32; // 6-bit green (middle value)

                            // Pack RGB565 using WORKING implementation format
                            let rgb565 = ((red as u16 & 0x1f) << 11) | 
                                    ((green as u16 & 0x3f) << 5) | 
                                        (blue as u16 & 0x1f);
                            
                             // Store as little-endian (LSB first, MSB second)
                             data[pixel_offset] = (rgb565 & 0xFF) as u8; // LSB
                             data[pixel_offset + 1] = (rgb565 >> 8) as u8; // MSB
                        }
                    }

                    // Align to 4-byte boundary
                    let aligned_end = (data_end + 3) & !3;
                    pos = aligned_end;
                } else {
                    break;
                }
            }
            0x01 => {
                // Repeat pixels command
                let repeat_count =
                    ((param1 as u32) << 16) | ((param2 as u32) << 8) | (param3 as u32);
                println!(
                    "   🔄 Found repeat command: {} repetitions at offset {}",
                    repeat_count,
                    pos + 4
                );

                // Inject alternating red/blue pattern
                if pos + 8 <= data_len {
                    // First pixel: red (RGB565: 11111 000000 00000 = 0xF800)
                    data[pos + 4] = 0x00;
                    data[pos + 5] = 0xF8;

                    // Second pixel: blue (RGB565: 00000 000000 11111 = 0x001F)
                    data[pos + 6] = 0x1F;
                    data[pos + 7] = 0x00;
                }
                pos += 8;
            }
            0x03 => {
                // Unknown command (probably blit)
                println!("   🔄 Found blit command at offset {}", pos);
                pos += 4;
            }
            0x40 => {
                // End of transmission
                println!("   ✅ End of transmission at offset {}", pos);
                break;
            }
            _ => {
                println!("   ❓ Unknown command 0x{:02X} at offset {}", cmd, pos);
                pos += 4;
            }
        }
    }

    true
}

fn send_raw_data_to_screen(
    device: &mut MaschineMK3,
    raw_data: &[u8],
    _display_id: u8,
) -> Result<()> {
    println!("Sending {} bytes of raw data", raw_data.len());

    // Print first 32 bytes for debugging
    println!(
        "First 32 bytes: {:02X?}",
        &raw_data[..std::cmp::min(32, raw_data.len())]
    );

    // Send raw data directly
    if let Err(e) = device.send_raw_data(raw_data) {
        eprintln!("Failed to send raw data: {}", e);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Send failed: {}", e),
        ));
    }

    Ok(())
}

fn main() -> Result<()> {
    println!("Loading Wireshark raw exports for pixel injection testing...");

    // Initialize device
    let mut device = match MaschineMK3::new() {
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

    // Process first.raw
    match load_raw_file("wireshark_dumps/first.raw") {
        Ok(mut data) => {
            println!("\n=== Processing first.raw ===");
            println!("File size: {} bytes", data.len());

            if let Some(info) = parse_display_packet(&data) {
                println!("📋 Parsed packet info: {:#?}", info);

                // println!("\n🔄 Sending original data first...");
                // if let Err(e) = send_raw_data_to_screen(&mut device, &data, info.display_id) {
                //     eprintln!("Failed to send original first.raw: {}", e);
                // } else {
                //     println!("✅ Original first.raw sent successfully");
                // }

                std::thread::sleep(std::time::Duration::from_millis(2000));

                println!("\n🎨 Injecting test pattern...");
                inject_test_pattern(&mut data, &info);

                if let Err(e) = send_raw_data_to_screen(&mut device, &data, info.display_id) {
                    eprintln!("Failed to send modified first.raw: {}", e);
                } else {
                    println!("✅ Modified first.raw sent successfully");
                }
            } else {
                eprintln!("Failed to parse first.raw packet");
            }
        }
        Err(e) => {
            eprintln!("Failed to load first.raw: {}", e);
        }
    }

    // Wait between files
    std::thread::sleep(std::time::Duration::from_millis(2000));

    println!("\n🎉 Pixel injection test complete");
    Ok(())
}
