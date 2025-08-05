use mk3_hal::{MK3Error, MaschineMK3};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Maschine MK3 Input Debug Tool");

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

    println!("\n🎛️  Press buttons, turn knobs, hit pads - showing raw data:");
    println!("   Press Ctrl+C to stop\n");

    loop {
        match device.read_input() {
            Ok(data) if !data.is_empty() => {
                match data[0] {
                    0x01 => {
                        println!(
                            "📊 Button/Knob packet ({}B): {:?}",
                            data.len(),
                            &data[..std::cmp::min(12, data.len())]
                        );
                        if data.len() >= 42 {
                            // Show some key button bytes
                            println!(
                                "   Buttons: byte1={:08b} byte2={:08b} byte6={:08b}",
                                data[1], data[2], data[6]
                            );
                            if data[6] & 0x20 != 0 {
                                println!("   ▶️  PLAY pressed!");
                            }
                            if data[2] & 0x01 != 0 {
                                println!("   🅰️  Group A pressed!");
                            }
                        }
                    }
                    0x02 => {
                        println!(
                            "🥁 Pad packet ({}B): {:?}",
                            data.len(),
                            &data[..std::cmp::min(20, data.len())]
                        );

                        // Debug pad parsing
                        let mut offset = 1;
                        let mut pad_count = 0;
                        print!("   Pads: ");

                        while offset + 2 < data.len() && pad_count < 10 {
                            let pad_num = data[offset];
                            let data_a = data[offset + 1];
                            let data_b = data[offset + 2];

                            // Stop if we hit obvious padding or invalid data
                            if pad_num > 50 || (pad_num == 0 && data_a == 0 && data_b == 0) {
                                break;
                            }

                            print!("pad{}({},{}) ", pad_num, data_a, data_b);
                            offset += 3;
                            pad_count += 1;
                        }
                        println!();
                    }
                    other => {
                        println!(
                            "❓ Unknown packet type 0x{:02X} ({}B): {:?}",
                            other,
                            data.len(),
                            &data[..std::cmp::min(8, data.len())]
                        );
                    }
                }
            }
            Ok(_) => {
                // No data - just continue
            }
            Err(e) => {
                println!("❌ Read error: {}", e);
            }
        }

        std::thread::sleep(Duration::from_millis(50));
    }
}
