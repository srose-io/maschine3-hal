use mk3_hal::{InputElement, InputEvent, MK3Error, MaschineLEDColor, MaschineMK3};
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

                    // Light up different buttons with different colors
                    match element {
                        // Transport buttons (brightness-based)
                        InputElement::Play => device.set_button_led(element, 127)?,
                        InputElement::Rec => device.set_button_led(element, 127)?,
                        InputElement::Stop => device.set_button_led(element, 127)?,

                        // Group buttons (color-based)
                        InputElement::GroupA => {
                            device.set_button_led_color(element, MaschineLEDColor::red(true))?
                        }
                        InputElement::GroupB => {
                            device.set_button_led_color(element, MaschineLEDColor::green(true))?
                        }
                        InputElement::GroupC => {
                            device.set_button_led_color(element, MaschineLEDColor::blue(true))?
                        }
                        InputElement::GroupD => {
                            device.set_button_led_color(element, MaschineLEDColor::white(true))?
                        }
                        InputElement::GroupE => {
                            device.set_button_led_color(element, MaschineLEDColor::red(false))?
                        }
                        InputElement::GroupF => {
                            device.set_button_led_color(element, MaschineLEDColor::green(false))?
                        }
                        InputElement::GroupG => {
                            device.set_button_led_color(element, MaschineLEDColor::blue(false))?
                        }
                        InputElement::GroupH => {
                            device.set_button_led_color(element, MaschineLEDColor::white(false))?
                        }

                        _ => {}
                    }
                }

                InputEvent::ButtonReleased(element) => {
                    println!("🔼 {} released", element.name());

                    // Turn off button when released
                    match element {
                        // Transport buttons
                        InputElement::Play | InputElement::Rec | InputElement::Stop => {
                            device.set_button_led(element, 0)?;
                        }

                        // Group buttons
                        InputElement::GroupA
                        | InputElement::GroupB
                        | InputElement::GroupC
                        | InputElement::GroupD
                        | InputElement::GroupE
                        | InputElement::GroupF
                        | InputElement::GroupG
                        | InputElement::GroupH => {
                            device.set_button_led_color(element, MaschineLEDColor::black())?;
                        }

                        _ => {}
                    }
                }

                InputEvent::PadHit {
                    pad_number,
                    velocity,
                    ..
                } => {
                    println!("🥁 Pad {} hit (velocity: {})", pad_number + 1, velocity);

                    // Light up pad with color and brightness based on velocity
                    let high_velocity = velocity > 100;
                    // let color = match pad_number % 8 {
                    //     0 => MaschineLEDColor::red(high_velocity),
                    //     1 => MaschineLEDColor::green(high_velocity),
                    //     2 => MaschineLEDColor::blue(high_velocity),
                    //     3 => MaschineLEDColor::white(high_velocity),
                    //     4 => MaschineLEDColor::red(!high_velocity),
                    //     5 => MaschineLEDColor::green(!high_velocity),
                    //     6 => MaschineLEDColor::blue(!high_velocity),
                    //     _ => MaschineLEDColor::white(!high_velocity),
                    // };
                    let color = MaschineLEDColor::white(true);

                    device.set_pad_led(pad_number, color)?;

                    // Schedule pad to fade after a moment (in a real app you'd use a proper timer)
                    // For now, just keep the pad lit
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
