use maschine3_hal::{InputEvent, MK3Error, MaschineLEDColor, MaschineMK3};
use std::time::Duration;

/// Interactive pad lighting demo
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Maschine MK3 Interactive Pad Lighting");
    println!("⚠️  Make sure to close any NI software first!\n");

    // Connect to device
    let mut device = match MaschineMK3::new() {
        Ok(device) => {
            println!("✅ Connected: {}", device.device_info()?);
            device
        }
        Err(MK3Error::DeviceNotFound) => {
            println!("❌ No Maschine MK3 found. Check connection.");
            return Ok(());
        }
        Err(e) => {
            println!("❌ Connection error: {}", e);
            return Ok(());
        }
    };

    println!("\n🎮 Interactive Mode");
    println!("   Press buttons to light them up!");
    println!("   Hit pads to light them up with velocity-based brightness!");
    println!("   Press Ctrl+C to exit\n");

    // Clear all LEDs initially
    device.clear_all_leds()?;

    let start_time = std::time::Instant::now();

    loop {
        let events = device.poll_input_events()?;

        for event in events {
            match event {
                InputEvent::ButtonPressed(element) => {
                    println!("🔽 {} pressed", element.name());

                    if element.has_color() {
                        device.set_button_led_color(element, MaschineLEDColor::red(true))?;
                    } else {
                        device.set_button_led(element, 127)?;
                    }
                }
                InputEvent::ButtonReleased(element) => {
                    println!("🔼 {} released", element.name());

                    if element.has_color() {
                        device.set_button_led_color(element, MaschineLEDColor::black())?;
                    } else {
                        device.set_button_led(element, 0)?;
                    }
                }

                InputEvent::PadEvent {
                    pad_number,
                    event_type: maschine3_hal::PadEventType::Hit,
                    value,
                } => {
                    println!("🥁 Pad {} hit (velocity: {})", pad_number + 1, value);

                    // Light up pad with color based on velocity (12-bit value scaled)
                    let high_velocity = value > 2048; // Half of 4095 max
                    let color = match pad_number % 8 {
                        0 => MaschineLEDColor::red(high_velocity),
                        1 => MaschineLEDColor::green(high_velocity),
                        2 => MaschineLEDColor::blue(high_velocity),
                        3 => MaschineLEDColor::white(high_velocity),
                        4 => MaschineLEDColor::red(!high_velocity),
                        5 => MaschineLEDColor::green(!high_velocity),
                        6 => MaschineLEDColor::blue(!high_velocity),
                        _ => MaschineLEDColor::white(!high_velocity),
                    };

                    device.set_pad_led(pad_number, color)?;
                }
                
                InputEvent::PadEvent {
                    pad_number,
                    event_type: maschine3_hal::PadEventType::HitRelease | maschine3_hal::PadEventType::TouchRelease,
                    ..
                } => {
                    println!("🔼 Pad {} released", pad_number + 1);
                    device.set_pad_led(pad_number, MaschineLEDColor::black())?;
                }
                
                InputEvent::PadEvent {
                    pad_number,
                    event_type: maschine3_hal::PadEventType::Aftertouch,
                    value,
                } => {
                    // Update brightness based on aftertouch pressure
                    let high_pressure = value > 2048;
                    let color = MaschineLEDColor::white(high_pressure);
                    device.set_pad_led(pad_number, color)?;
                }

                _ => {
                    // Other events like knob changes - just log occasionally
                    if start_time.elapsed().as_millis() % 1000 < 100 {
                        println!("🎛️ {}", event.description());
                    }
                }
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}
