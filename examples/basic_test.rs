use mk3_hal::{
    ButtonLedState, InputState, MK3Error, MaschineLEDColor, MaschineMK3, PadLedState, PadState,
    RgbColor,
};
use std::time::Duration;

/// Basic test application demonstrating all MK3 functionality
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 Maschine MK3 HAL - Comprehensive Test");
    println!("⚠️  Make sure to close any NI software first!\n");

    // Connect to device
    let device = match MaschineMK3::new() {
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
    let mut button_count = 0;
    let mut pad_count = 0;

    while start_time.elapsed() < Duration::from_secs(10) {
        // Read button/knob state
        if let Ok(data) = device.read_input() {
            if !data.is_empty() && data[0] == 0x01 && data.len() >= 42 {
                if let Ok(input) = InputState::from_button_packet(&data) {
                    button_count += 1;
                    if button_count % 50 == 1 {
                        // Print every 50th update
                        println!("   📊 Buttons active: {}", count_active_buttons(&input));
                        println!(
                            "   🎛️  Knob 1: {} (touched: {})",
                            input.knobs.knob_1, input.knobs.knob_1_touched
                        );
                        if input.buttons.play {
                            println!("   ▶️  PLAY button pressed!");
                        }
                        if input.buttons.group_a {
                            println!("   🅰️  Group A pressed!");
                        }
                    }
                }
            }
        }

        // Read pad state
        if let Ok(data) = device.read_input() {
            if !data.is_empty() && data[0] == 0x02 {
                if let Ok(pads) = PadState::from_pad_packet(&data) {
                    pad_count += 1;
                    if !pads.hits.is_empty() {
                        println!(
                            "   🥁 Pad hits: {:?}",
                            pads.hits.iter().map(|h| h.pad_number).collect::<Vec<_>>()
                        );
                    }
                }
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    println!(
        "   ✅ Input test complete. Button packets: {}, Pad packets: {}\n",
        button_count, pad_count
    );

    // Test 2: LED Control
    println!("🌈 Test 2: LED Control");

    // Test button LEDs
    println!("   💡 Testing button LEDs...");
    let mut button_leds = ButtonLedState::default();
    button_leds.play = 127; // Bright
    button_leds.rec = 64; // Medium
    button_leds.stop = 32; // Dim
    button_leds.group_a = MaschineLEDColor::red(true);
    button_leds.group_b = MaschineLEDColor::green(true);
    button_leds.group_c = MaschineLEDColor::blue(true);

    device.write_leds(&button_leds.to_packet())?;
    std::thread::sleep(Duration::from_secs(2));

    // Test pad LEDs
    println!("   🟡 Testing pad LEDs...");
    let mut pad_leds = PadLedState::default();
    for i in 0..16 {
        pad_leds.pad_leds[i] = match i % 4 {
            0 => MaschineLEDColor::red(true),
            1 => MaschineLEDColor::green(true),
            2 => MaschineLEDColor::blue(true),
            _ => MaschineLEDColor::white(true),
        };
    }

    // Rainbow touch strip
    for i in 0..25 {
        let hue = (i as f32 / 25.0) * 360.0;
        pad_leds.touch_strip_leds[i] = MaschineLEDColor::from_rgb_color(hsv_to_rgb(hue, 1.0, 1.0));
    }

    device.write_leds(&pad_leds.to_packet())?;
    std::thread::sleep(Duration::from_secs(3));

    // Turn off LEDs
    println!("   🔄 Turning off LEDs...");
    device.write_button_leds(&ButtonLedState::default())?;
    device.write_pad_leds(&PadLedState::default())?;

    println!("   ✅ LED test complete\n");

    // Test 3: Interactive mode
    println!("🎮 Test 3: Interactive Mode (30 seconds)");
    println!("   Press buttons to light up LEDs!");
    println!("   Hit pads to light them up!");

    let start_time = std::time::Instant::now();
    let mut current_button_leds = ButtonLedState::default();
    let mut current_pad_leds = PadLedState::default();

    while start_time.elapsed() < Duration::from_secs(30) {
        let mut led_update_needed = false;

        // React to button presses
        if let Ok(data) = device.read_input() {
            if !data.is_empty() && data[0] == 0x01 && data.len() >= 42 {
                if let Ok(input) = InputState::from_button_packet(&data) {
                    // Light up transport buttons
                    current_button_leds.play = if input.buttons.play { 127 } else { 0 };
                    current_button_leds.rec = if input.buttons.rec { 127 } else { 0 };
                    current_button_leds.stop = if input.buttons.stop { 127 } else { 0 };

                    // Light up group buttons with colors
                    current_button_leds.group_a = if input.buttons.group_a {
                        MaschineLEDColor::red(true)
                    } else {
                        MaschineLEDColor::black()
                    };
                    current_button_leds.group_b = if input.buttons.group_b {
                        MaschineLEDColor::green(true)
                    } else {
                        MaschineLEDColor::black()
                    };
                    current_button_leds.group_c = if input.buttons.group_c {
                        MaschineLEDColor::blue(true)
                    } else {
                        MaschineLEDColor::black()
                    };
                    current_button_leds.group_d = if input.buttons.group_d {
                        MaschineLEDColor::white(true)
                    } else {
                        MaschineLEDColor::black()
                    };

                    led_update_needed = true;
                }
            }
        }

        // React to pad hits
        if let Ok(data) = device.read_input() {
            if !data.is_empty() && data[0] == 0x02 {
                if let Ok(pads) = PadState::from_pad_packet(&data) {
                    for hit in &pads.hits {
                        if hit.pad_number < 16 {
                            // Light up the hit pad with random color
                            current_pad_leds.pad_leds[hit.pad_number as usize] =
                                MaschineLEDColor::from_rgb_color(RgbColor::new(
                                    ((hit.pad_number.wrapping_mul(17)) % 255) as u8,
                                    ((hit.pad_number.wrapping_mul(31)) % 255) as u8,
                                    ((hit.pad_number.wrapping_mul(47)) % 255) as u8,
                                ));
                            led_update_needed = true;
                        }
                    }
                }
            }
        }

        // Update LEDs if needed
        if led_update_needed {
            device.write_button_leds(&current_button_leds)?;
            device.write_pad_leds(&current_pad_leds)?;
        }

        // // Fade pad LEDs
        // for led in &mut current_pad_leds.pad_leds {
        //     led.index = led.index.saturating_sub(2);
        // }

        std::thread::sleep(Duration::from_millis(50));
    }

    // Final cleanup
    println!("\n🧹 Cleaning up...");
    device.write_button_leds(&ButtonLedState::default())?;
    device.write_pad_leds(&PadLedState::default())?;

    println!("✅ All tests completed successfully!");
    println!("🎉 Maschine MK3 HAL is working!");

    Ok(())
}

fn count_active_buttons(input: &mk3_hal::InputState) -> usize {
    let mut count = 0;
    if input.buttons.play {
        count += 1;
    }
    if input.buttons.rec {
        count += 1;
    }
    if input.buttons.stop {
        count += 1;
    }
    if input.buttons.group_a {
        count += 1;
    }
    if input.buttons.group_b {
        count += 1;
    }
    if input.buttons.group_c {
        count += 1;
    }
    if input.buttons.group_d {
        count += 1;
    }
    if input.buttons.shift {
        count += 1;
    }
    // Add more as needed...
    count
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> RgbColor {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
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

    RgbColor::new(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}
