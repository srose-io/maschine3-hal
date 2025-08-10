use mk3_hal::{InputEvent, MK3Error, MaschineMK3};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎛️  Simple Async Maschine MK3 Monitor");
    println!("======================================");

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

    println!("\n🔍 Starting async monitoring...");
    println!("   Press Ctrl+C to stop\n");

    // Use the callback-based API with LED feedback
    device.start_input_monitoring(move |event| {
        println!("{}", event.description());
        
        // Provide visual feedback
        match event {
            InputEvent::ButtonPressed(btn) => {
                // Light up button when pressed (this won't work because we can't move device into closure)
                // We'll show a better example in simple_test.rs
                println!("  💡 {} button pressed!", btn.name());
            }
            InputEvent::PadEvent { pad_number, event_type: mk3_hal::PadEventType::Hit, value } => {
                println!("  🥁 Pad {} hit with velocity {}!", pad_number + 1, value);
            }
            _ => {}
        }
    })?;

    // Keep the program running
    println!("✨ Monitoring started! Interact with your device...");
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}