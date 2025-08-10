use mk3_hal::{InputElement, InputEvent, MK3Error, MaschineLEDColor, MaschineMK3, RgbColor};
use std::time::Duration;

/// Basic test application demonstrating all MK3 functionality
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 Maschine MK3 HAL - Comprehensive Test");
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

    println!("\n🧪 Starting test sequence...\n");

    // Test 1: Input monitoring
    println!("🎛️  Test 1: Input Monitoring (10 seconds)");
    println!("   Press buttons, turn knobs, hit pads!");

    let start_time = std::time::Instant::now();
    let mut event_count = 0;

    while start_time.elapsed() < Duration::from_secs(10) {
        let events = device.poll_input_events()?;

        for event in events {
            event_count += 1;
            match &event {
                InputEvent::ButtonPressed(element) => {
                    println!("   🔽 {} pressed", element.name());
                }
                InputEvent::ButtonReleased(element) => {
                    println!("   🔼 {} released", element.name());
                }
                InputEvent::PadEvent {
                    pad_number,
                    event_type: mk3_hal::PadEventType::Hit,
                    value,
                } => {
                    println!("   🥁 Pad {} hit (velocity: {})", pad_number + 1, value);
                }
                InputEvent::KnobChanged { element, value, .. } => {
                    println!("   🎛️  {} → {}", element.name(), value);
                }
                _ => {
                    if event_count % 20 == 1 {
                        println!("   📊 {}", event.description());
                    }
                }
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    println!("   ✅ Input test complete. Total events: {}\n", event_count);

    // Test 2: LED Control with new API
    println!("🌈 Test 2: New LED API");

    // Test button LEDs
    println!("   💡 Testing individual button LEDs...");
    device.set_button_led(InputElement::Play, 127)?; // Bright
    device.set_button_led_color(InputElement::GroupA, MaschineLEDColor::red(true))?;
    device.set_button_led_color(InputElement::GroupB, MaschineLEDColor::green(true))?;
    device.set_button_led_color(InputElement::GroupC, MaschineLEDColor::blue(true))?;
    device.set_button_led_color(InputElement::GroupD, MaschineLEDColor::white(true))?;

    std::thread::sleep(Duration::from_secs(2));

    // Test pad LEDs
    println!("   🟡 Testing individual pad LEDs...");
    for i in 0..16 {
        let color = match i % 4 {
            0 => MaschineLEDColor::red(true),
            1 => MaschineLEDColor::green(true),
            2 => MaschineLEDColor::blue(true),
            _ => MaschineLEDColor::white(true),
        };
        device.set_pad_led(i, color)?;
    }

    std::thread::sleep(Duration::from_secs(3));

    // Turn off LEDs using new API
    println!("   🔄 Turning off all LEDs...");
    device.clear_all_leds()?;

    println!("   ✅ LED test complete\n");

    // Test 3: Interactive mode with reactive LEDs
    println!("🎮 Test 3: Interactive Mode (30 seconds)");
    println!("   Press buttons to light up LEDs!");
    println!("   Hit pads to light them up!");

    let start_time = std::time::Instant::now();

    while start_time.elapsed() < Duration::from_secs(30) {
        let events = device.poll_input_events()?;

        for event in events {
            match event {
                InputEvent::ButtonPressed(element) => {
                    // Light up button when pressed
                    match element {
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
                        InputElement::Play => device.set_button_led(element, 127)?,
                        _ => {}
                    }
                }
                InputEvent::ButtonReleased(element) => {
                    // Turn off button when released
                    match element {
                        InputElement::GroupA
                        | InputElement::GroupB
                        | InputElement::GroupC
                        | InputElement::GroupD => {
                            device.set_button_led_color(element, MaschineLEDColor::black())?;
                        }
                        InputElement::Play => device.set_button_led(element, 0)?,
                        _ => {}
                    }
                }
                InputEvent::PadEvent {
                    pad_number,
                    event_type: mk3_hal::PadEventType::Hit,
                    value,
                } => {
                    // Flash pad based on velocity (12-bit scale)
                    let brightness = value > 2048;
                    let color = match pad_number % 4 {
                        0 => MaschineLEDColor::red(brightness),
                        1 => MaschineLEDColor::green(brightness),
                        2 => MaschineLEDColor::blue(brightness),
                        _ => MaschineLEDColor::white(brightness),
                    };
                    device.set_pad_led(pad_number, color)?;
                }
                _ => {}
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    // Clean up
    device.clear_all_leds()?;
    println!("   ✅ Interactive test complete\n");

    // Test 4: Display (if available)
    println!("📺 Test 4: Display Test");
    println!("   Clearing display with solid color...");

    match device.clear_display(0, 255, 0, 0) {
        Ok(()) => {
            println!("   ✅ Display cleared to red");
            std::thread::sleep(Duration::from_secs(2));

            device.clear_display(0, 0, 0, 0)?;
            println!("   ✅ Display turned off");
        }
        Err(e) => {
            println!("   ⚠️  Display test failed: {} (WinUSB driver needed?)", e);
        }
    }

    println!("\n🎉 All tests completed!");
    Ok(())
}

// Helper function for HSV to RGB conversion
#[allow(dead_code)]
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> RgbColor {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    RgbColor {
        r: ((r_prime + m) * 255.0) as u8,
        g: ((g_prime + m) * 255.0) as u8,
        b: ((b_prime + m) * 255.0) as u8,
    }
}
