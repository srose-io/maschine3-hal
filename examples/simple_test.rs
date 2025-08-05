use mk3_hal::{
    ButtonLedState, InputState, MK3Error, MaschineLEDColor, MaschineMK3, PadLedState, PadState,
};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 Maschine MK3 Simple Test");

    let device = match MaschineMK3::new() {
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

    println!("\n🧪 Test 1: Input Parsing (10 seconds)");
    println!("   Press buttons and hit pads!");

    let start_time = std::time::Instant::now();
    let mut button_events = 0;
    let mut pad_events = 0;

    while start_time.elapsed() < Duration::from_secs(10) {
        match device.read_input() {
            Ok(data) if !data.is_empty() => {
                match data[0] {
                    0x01 if data.len() >= 42 => {
                        // Parse button packet
                        match InputState::from_button_packet(&data) {
                            Ok(input) => {
                                button_events += 1;
                                if button_events % 20 == 1 {
                                    println!(
                                        "   📊 Button state - knob1: {}, play: {}, group_a: {}",
                                        input.knobs.knob_1,
                                        input.buttons.play,
                                        input.buttons.group_a
                                    );
                                }

                                // Test specific button presses
                                if input.buttons.play {
                                    println!("   ▶️  PLAY button detected!");
                                }
                                if input.buttons.group_a {
                                    println!("   🅰️  Group A detected!");
                                }
                            }
                            Err(e) => println!("   ❌ Button parse error: {}", e),
                        }
                    }
                    0x02 => {
                        // Parse pad packet
                        match PadState::from_pad_packet(&data) {
                            Ok(pads) => {
                                if !pads.hits.is_empty() {
                                    pad_events += 1;
                                    println!(
                                        "   🥁 Pads hit: {:?}",
                                        pads.hits.iter().map(|h| h.pad_number).collect::<Vec<_>>()
                                    );
                                }
                            }
                            Err(e) => println!("   ❌ Pad parse error: {}", e),
                        }
                    }
                    _ => {} // Unknown packet type
                }
            }
            Ok(_) => {} // No data
            Err(e) => println!("   ❌ Read error: {}", e),
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    println!(
        "   ✅ Input test complete. Button events: {}, Pad events: {}",
        button_events, pad_events
    );

    println!("\n🌈 Test 2: LED Control");

    // Test basic LED functionality
    let mut button_leds = ButtonLedState::default();
    button_leds.play = 127; // Bright
    button_leds.group_a = MaschineLEDColor::red(true);
    button_leds.group_b = MaschineLEDColor::green(true);

    let button_packet = button_leds.to_packet();
    println!(
        "   💡 Writing button LEDs ({} bytes)...",
        button_packet.len()
    );
    device.write_button_leds(&button_leds)?;

    std::thread::sleep(Duration::from_secs(2));

    // Test pad LEDs
    let mut pad_leds = PadLedState::default();
    for i in 0..4 {
        pad_leds.pad_leds[i] = MaschineLEDColor::blue(true);
    }

    let pad_packet = pad_leds.to_packet();
    println!("   🔵 Writing pad LEDs ({} bytes)...", pad_packet.len());
    device.write_pad_leds(&pad_leds)?;

    std::thread::sleep(Duration::from_secs(2));

    // Turn off LEDs
    println!("   🔄 Turning off LEDs...");
    device.write_button_leds(&ButtonLedState::default())?;
    device.write_pad_leds(&PadLedState::default())?;

    println!("   ✅ LED test complete");

    println!("\n🎉 All tests completed successfully!");
    Ok(())
}
