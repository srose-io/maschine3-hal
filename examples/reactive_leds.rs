use mk3_hal::{InputEvent, InputElement, MK3Error, MaschineLEDColor, MaschineMK3};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Maschine MK3 Reactive LEDs Example");
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

    println!("\n🔍 Press buttons and hit pads to see LED feedback!");
    println!("   Group buttons will light up when pressed");
    println!("   Pads will flash when hit");
    println!("   Press Ctrl+C to stop\n");

    // Clear all LEDs initially
    device.clear_all_leds()?;

    loop {
        let events = device.poll_input_events()?;
        
        for event in events {
            match event {
                InputEvent::ButtonPressed(element) => {
                    println!("🔽 {} pressed", element.name());
                    
                    // Light up group buttons when pressed
                    match element {
                        InputElement::GroupA => device.set_button_led_color(element, MaschineLEDColor::red(true))?,
                        InputElement::GroupB => device.set_button_led_color(element, MaschineLEDColor::green(true))?,
                        InputElement::GroupC => device.set_button_led_color(element, MaschineLEDColor::blue(true))?,
                        InputElement::GroupD => device.set_button_led_color(element, MaschineLEDColor::white(true))?,
                        InputElement::GroupE => device.set_button_led_color(element, MaschineLEDColor::red(false))?,
                        InputElement::GroupF => device.set_button_led_color(element, MaschineLEDColor::green(false))?,
                        InputElement::GroupG => device.set_button_led_color(element, MaschineLEDColor::blue(false))?,
                        InputElement::GroupH => device.set_button_led_color(element, MaschineLEDColor::white(false))?,
                        InputElement::Play => device.set_button_led(element, 127)?,
                        _ => {},
                    }
                }
                InputEvent::ButtonReleased(element) => {
                    println!("🔼 {} released", element.name());
                    
                    // Turn off group buttons when released
                    match element {
                        InputElement::GroupA | InputElement::GroupB | InputElement::GroupC | InputElement::GroupD |
                        InputElement::GroupE | InputElement::GroupF | InputElement::GroupG | InputElement::GroupH => {
                            device.set_button_led_color(element, MaschineLEDColor::black())?;
                        }
                        InputElement::Play => device.set_button_led(element, 0)?,
                        _ => {},
                    }
                }
                InputEvent::PadEvent { pad_number, event_type: mk3_hal::PadEventType::Hit, value } => {
                    println!("🥁 Pad {} hit (velocity: {})", pad_number + 1, value);
                    
                    // Flash pad with brightness based on velocity (12-bit scale)
                    let brightness = value > 2048;
                    let color = match pad_number % 4 {
                        0 => MaschineLEDColor::red(brightness),
                        1 => MaschineLEDColor::green(brightness),
                        2 => MaschineLEDColor::blue(brightness),
                        _ => MaschineLEDColor::white(brightness),
                    };
                    
                    device.set_pad_led(pad_number, color)?;
                    
                    // Schedule pad to turn off after a short delay
                    // (In a real app you might use a timer/scheduler)
                    std::thread::spawn(move || {
                        std::thread::sleep(Duration::from_millis(200));
                        // Note: We can't access device from here in this simple example
                        // In a real app you'd use channels or shared state
                    });
                }
                _ => {}
            }
        }
        
        std::thread::sleep(Duration::from_millis(10));
    }
}