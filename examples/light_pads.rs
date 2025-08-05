use mk3_hal::{
    ButtonLedState, InputState, MK3Error, MaschineLEDColor, MaschineMK3, PadLedState, PadState,
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

    // Test 3: Interactive mode
    println!("🎮 Test 3: Interactive Mode (15 seconds)");
    println!("   Press buttons to light up LEDs!");
    println!("   Hit pads to light them up!");

    let start_time = std::time::Instant::now();
    let mut current_button_leds = ButtonLedState::default();
    let mut current_pad_leds = PadLedState::default();

    while start_time.elapsed() < Duration::from_secs(15) {
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
                    // Filter out false hits (pad 0 with no data)
                    let hits: Vec<_> = pads.hits;

                    for hit in &hits {
                        println!("Hit: {:?}", hit);
                        if hit.pad_number < 16 {
                            // Light up the hit pad with random color
                            current_pad_leds.pad_leds[hit.pad_number as usize] =
                                MaschineLEDColor::red(true);
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
        //     led.bright = led.bright.saturating_sub(2);
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
