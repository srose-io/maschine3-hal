use mk3_hal::{InputEvent, InputState, InputTracker, MK3Error, MaschineMK3, PadState};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎛️  Maschine MK3 Input Monitor");
    println!("=====================================");

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

    println!("\n🔍 Monitoring all input - interact with your device!");
    println!("   Press Ctrl+C to stop\n");

    let mut input_tracker = InputTracker::new();
    let mut frame_count = 0;

    loop {
        frame_count += 1;
        let mut any_activity = false;

        // Monitor button/knob input
        match device.read_input() {
            Ok(data) if !data.is_empty() => {
                match data[0] {
                    0x01 if data.len() >= 42 => {
                        // Parse button/knob packet
                        match InputState::from_button_packet(&data) {
                            Ok(input) => {
                                // Get input events from tracker
                                let events = input_tracker.update(input.clone());

                                if !events.is_empty() {
                                    any_activity = true;
                                    print_input_events(&events, frame_count);
                                    print_current_state(&input);
                                }
                            }
                            Err(e) => println!("❌ Button parse error: {}", e),
                        }
                    }
                    0x02 => {
                        // Parse pad packet
                        match PadState::from_pad_packet(&data) {
                            Ok(pads) => {
                                if !pads.hits.is_empty() {
                                    any_activity = true;
                                    print_pad_state(&pads, frame_count);
                                }
                            }
                            Err(e) => println!("❌ Pad parse error: {}", e),
                        }
                    }
                    other => {
                        println!("📦 Unknown packet type: 0x{:02X} ({}B)", other, data.len());
                    }
                }
            }
            Ok(_) => {
                // No data - just continue
            }
            Err(e) => {
                println!("❌ Read error: {}", e);
            }
        }

        // Print a heartbeat every few seconds when no activity
        if !any_activity && frame_count % 200 == 0 {
            print!("💓");
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}

fn print_input_events(events: &[InputEvent], frame: u32) {
    println!(
        "\n⚡ INPUT EVENTS [Frame {}] ================================",
        frame
    );

    for event in events {
        match event {
            InputEvent::ButtonPressed(element) => {
                println!("  🔽 {} PRESSED", element.name());
            }
            InputEvent::ButtonReleased(element) => {
                println!("  🔼 {} RELEASED", element.name());
            }
            InputEvent::ButtonHeld(element) => {
                println!("  ⏸️  {} HELD", element.name());
            }
            InputEvent::KnobChanged {
                element,
                value,
                delta,
            } => {
                println!("  🎛️  {} → {} (Δ{})", element.name(), value, delta);
            }
            InputEvent::AudioChanged {
                element,
                value,
                delta,
            } => {
                println!("  🔊 {} → {} (Δ{})", element.name(), value, delta);
            }
        }
    }
}

fn print_current_state(input: &InputState) {
    println!("\n🎛️  CURRENT STATE =======================================");

    // Active buttons
    let active_buttons = input.get_active_buttons();
    if !active_buttons.is_empty() {
        let button_names: Vec<&str> = active_buttons.iter().map(|b| b.name()).collect();
        println!("  🔘 Active: {}", button_names.join(", "));
    }

    // Active knobs
    let active_knobs = input.get_active_knobs();
    if !active_knobs.is_empty() {
        let knob_info: Vec<String> = active_knobs
            .iter()
            .map(|(element, value)| format!("{}:{}", element.name(), value))
            .collect();
        println!("  🎛️  Knobs: {}", knob_info.join(", "));
    }

    // Active audio controls
    let active_audio = input.get_active_audio();
    if !active_audio.is_empty() {
        let audio_info: Vec<String> = active_audio
            .iter()
            .map(|(element, value)| format!("{}:{}", element.name(), value))
            .collect();
        println!("  🔊 Audio: {}", audio_info.join(", "));
    }

    // Touch strip
    if let Some(((f1a, f1b, f1c, f1d), (f2a, f2b, f2c, f2d))) = input.get_touch_strip_data() {
        println!(
            "  👆 Touch Strip: F1({},{},{},{}) F2({},{},{},{})",
            f1a, f1b, f1c, f1d, f2a, f2b, f2c, f2d
        );
    }
}

fn print_pad_state(pads: &PadState, frame: u32) {
    println!(
        "\n🥁 PAD HITS [Frame {}] ===============================",
        frame
    );

    for (i, hit) in pads.hits.iter().enumerate() {
        let pad_name = match hit.pad_number {
            0..=15 => format!(
                "Pad #{} ({})",
                hit.pad_number + 1,
                get_pad_position(hit.pad_number)
            ),
            _ => format!("Unknown Pad {}", hit.pad_number),
        };

        println!(
            "  {} - Velocity: {} Pressure: {} (raw: {}, {})",
            pad_name, hit.data_a, hit.data_b, hit.data_a, hit.data_b
        );

        // Only show first 5 hits to avoid spam
        if i >= 4 {
            println!("  ... and {} more hits", pads.hits.len() - 5);
            break;
        }
    }
}

fn get_pad_position(pad_num: u8) -> &'static str {
    // Pads are numbered 0-15, from top-right to bottom-left
    match pad_num {
        0 => "TR", // Top Right
        1 => "T2",
        2 => "T3",
        3 => "TL", // Top Left
        4 => "2R",
        5 => "22",
        6 => "23",
        7 => "2L",
        8 => "3R",
        9 => "32",
        10 => "33",
        11 => "3L",
        12 => "BR", // Bottom Right
        13 => "B2",
        14 => "B3",
        15 => "BL", // Bottom Left
        _ => "??",
    }
}
