use mk3_hal::{MK3Error, MaschineMK3};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Maschine MK3 HAL - Phase 1 Test");
    println!("⚠️  Make sure to close any NI software (Maschine, Traktor, etc.)");

    println!("\n🔍 Trying HID API first (recommended for Windows)...");
    match MaschineMK3::new() {
        Ok(device) => {
            println!("✅ Successfully connected via HID API!");
            println!("{}", device.device_info()?);

            // Test basic input reading
            println!("\n🎛️  Testing input reading - press some buttons/knobs NOW...");
            for i in 0..50 {
                match device.read_raw_input() {
                    Ok(data) if !data.is_empty() => {
                        println!(
                            "Input #{}: {} bytes - {:?}",
                            i + 1,
                            data.len(),
                            &data[..std::cmp::min(8, data.len())]
                        );
                    }
                    Ok(_) => {
                        print!(".");
                        std::io::Write::flush(&mut std::io::stdout()).ok();
                    }
                    Err(e) => println!("Read error: {}", e),
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            println!("\n✅ HID API test complete!");
            return Ok(());
        }
        Err(e) => {
            println!("⚠️  HID API failed: {}", e);
        }
    }

    println!("\n🔍 Falling back to direct USB (rusb)...");
    match MaschineMK3::new() {
        Ok(device) => {
            println!("✅ Successfully connected to device!");
            println!("{}", device.device_info()?);

            // Test basic input reading
            println!("\n🎛️  Testing input reading (press Ctrl+C to stop)...");
            for i in 0..10 {
                match device.read_raw_input() {
                    Ok(data) if !data.is_empty() => {
                        println!(
                            "Input #{}: {} bytes - {:?}",
                            i + 1,
                            data.len(),
                            &data[..std::cmp::min(8, data.len())]
                        );
                    }
                    Ok(_) => {
                        print!(".");
                        std::io::Write::flush(&mut std::io::stdout()).ok();
                    }
                    Err(e) => println!("Read error: {}", e),
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            println!("\n✅ Phase 1 complete!");
        }
        Err(MK3Error::DeviceNotFound) => {
            println!("❌ No Maschine MK3 device found");
            println!("   Make sure your device is connected and try again");
        }
        Err(e) => {
            println!("❌ Error connecting to device: {}", e);
        }
    }

    Ok(())
}
