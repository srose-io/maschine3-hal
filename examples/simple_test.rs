use mk3_hal::{
    InputEvent, InputElement, MK3Error, MaschineLEDColor, MaschineMK3,
};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 Maschine MK3 Simple Test");

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

    println!("\n🧪 Test 1: Input Events (10 seconds)");
    println!("   Press buttons and hit pads!");

    let start_time = std::time::Instant::now();
    let mut event_count = 0;

    while start_time.elapsed() < Duration::from_secs(10) {
        let events = device.poll_input_events()?;
        for event in events {
            event_count += 1;
            match event {
                InputEvent::ButtonPressed(element) => {
                    println!("   ▶️  {} button pressed!", element.name());
                }
                InputEvent::PadEvent { pad_number, event_type: mk3_hal::PadEventType::Hit, value } => {
                    println!("   🥁 Pad {} hit with velocity {}", pad_number + 1, value);
                }
                InputEvent::KnobChanged { element, value, .. } => {
                    println!("   🎛️  {} changed to {}", element.name(), value);
                }
                _ => {
                    if event_count % 10 == 1 {
                        println!("   📊 Event: {}", event.description());
                    }
                }
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    println!(
        "   ✅ Input test complete. Total events: {}",
        event_count
    );

    println!("\n🌈 Test 2: New LED API");

    // Test new stateful LED API
    println!("   💡 Setting individual button LEDs...");
    device.set_button_led(InputElement::Play, 127)?; // Brightness-based LED
    device.set_button_led_color(InputElement::GroupA, MaschineLEDColor::red(true))?;
    device.set_button_led_color(InputElement::GroupB, MaschineLEDColor::green(true))?;

    std::thread::sleep(Duration::from_secs(2));

    // Test pad LEDs
    println!("   🔵 Setting individual pad LEDs...");
    for i in 0..4 {
        device.set_pad_led(i, MaschineLEDColor::blue(true))?;
    }

    std::thread::sleep(Duration::from_secs(2));

    // Test bulk operations
    println!("   🌈 Setting all pads to rainbow colors...");
    let colors = [
        MaschineLEDColor::red(true),
        MaschineLEDColor::green(true),
        MaschineLEDColor::blue(true),
        MaschineLEDColor::white(true),
    ];
    
    for i in 0..16 {
        device.set_pad_led(i, colors[(i as usize) % colors.len()])?;
    }

    std::thread::sleep(Duration::from_secs(2));

    // Turn off LEDs using new API
    println!("   🔄 Turning off all LEDs...");
    device.clear_all_leds()?;

    println!("   ✅ LED test complete");

    println!("\n🎉 All tests completed successfully!");
    Ok(())
}
