use mk3_hal::{MK3Error, MaschineMK3};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎛️  Maschine MK3 Input Monitor");
    println!("=====================================");

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

    println!("\n🔍 Monitoring all input - interact with your device!");
    println!("   Press Ctrl+C to stop\n");

    // Simple polling mode example
    let mut frame_count = 0;

    loop {
        frame_count += 1;
        
        // Single call to get all events - framework handles packet parsing and tracking
        let events = device.poll_input_events()?;
        
        if !events.is_empty() {
            println!(
                "\n⚡ INPUT EVENTS [Frame {}] ================================",
                frame_count
            );
            
            for event in events {
                println!("  {}", event.description());
            }
        } else if frame_count % 200 == 0 {
            // Heartbeat every few seconds when no activity
            print!("💓");
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}

// Alternative async/callback example (commented out)
#[allow(dead_code)]
fn async_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎛️  Maschine MK3 Input Monitor (Async Mode)");
    println!("============================================");

    let mut device = MaschineMK3::new()?;
    println!("✅ Connected: {}", device.device_info()?);

    // Start monitoring with callback
    device.start_input_monitoring(|event| {
        println!("{}", event.description());
    })?;

    println!("\n🔍 Monitoring all input - interact with your device!");
    println!("   Press Ctrl+C to stop\n");

    // Keep the program running
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}