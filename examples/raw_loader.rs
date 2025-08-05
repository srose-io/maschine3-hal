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
    let x_start = ((data[8] as u16) << 8) | (data[9] as u16); // offset 8-9
    let y_start = ((data[10] as u16) << 8) | (data[11] as u16); // offset 10-11
    let width = ((data[12] as u16) << 8) | (data[13] as u16); // offset 12-13
    let height = ((data[14] as u16) << 8) | (data[15] as u16); // offset 14-15

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
