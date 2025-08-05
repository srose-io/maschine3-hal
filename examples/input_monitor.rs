use mk3_hal::{MaschineMK3, MK3Error, InputState, PadState};
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

    let mut last_button_state: Option<InputState> = None;
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
                                // Check for changes since last state
                                let mut changes = Vec::new();
                                
                                if let Some(ref last) = last_button_state {
                                    check_button_changes(&input, last, &mut changes);
                                    check_knob_changes(&input, last, &mut changes);
                                }
                                
                                if !changes.is_empty() || last_button_state.is_none() {
                                    any_activity = true;
                                    print_button_state(&input, &changes, frame_count);
                                }
                                
                                last_button_state = Some(input);
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

fn print_button_state(input: &InputState, changes: &[String], frame: u32) {
    println!("\n🎛️  BUTTON/KNOB STATE [Frame {}] ===========================", frame);
    
    // Transport Controls
    let transport_buttons = [
        ("▶️  Play", input.buttons.play),
        ("⏺️  Rec", input.buttons.rec),
        ("⏹️  Stop", input.buttons.stop),
        ("⏮️  Restart", input.buttons.restart),
        ("⏏️  Erase", input.buttons.erase),
        ("👆 Tap", input.buttons.tap),
        ("👣 Follow", input.buttons.follow),
    ];
    
    let active_transport: Vec<&str> = transport_buttons
        .iter()
        .filter(|(_, pressed)| *pressed)
        .map(|(name, _)| *name)
        .collect();
    
    if !active_transport.is_empty() {
        println!("  🚦 Transport: {}", active_transport.join(", "));
    }
    
    // Group Buttons
    let group_buttons = [
        ("A", input.buttons.group_a),
        ("B", input.buttons.group_b),
        ("C", input.buttons.group_c),
        ("D", input.buttons.group_d),
        ("E", input.buttons.group_e),
        ("F", input.buttons.group_f),
        ("G", input.buttons.group_g),
        ("H", input.buttons.group_h),
    ];
    
    let active_groups: Vec<&str> = group_buttons
        .iter()
        .filter(|(_, pressed)| *pressed)
        .map(|(name, _)| *name)
        .collect();
    
    if !active_groups.is_empty() {
        println!("  🅰️  Groups: {}", active_groups.join(", "));
    }
    
    // Mode Buttons
    let mode_buttons = [
        ("Notes", input.buttons.notes),
        ("Volume", input.buttons.volume),
        ("Swing", input.buttons.swing),
        ("Tempo", input.buttons.tempo),
        ("Note Repeat", input.buttons.note_repeat),
        ("Lock", input.buttons.lock),
        ("Pad Mode", input.buttons.pad_mode),
        ("Keyboard", input.buttons.keyboard),
        ("Chords", input.buttons.chords),
        ("Step", input.buttons.step),
        ("Fixed Vel", input.buttons.fixed_vel),
        ("Scene", input.buttons.scene),
        ("Pattern", input.buttons.pattern),
        ("Events", input.buttons.events),
    ];
    
    let active_modes: Vec<&str> = mode_buttons
        .iter()
        .filter(|(_, pressed)| *pressed)
        .map(|(name, _)| *name)
        .collect();
    
    if !active_modes.is_empty() {
        println!("  🎵 Modes: {}", active_modes.join(", "));
    }
    
    // Control Buttons
    let control_buttons = [
        ("Variation", input.buttons.variation),
        ("Duplicate", input.buttons.duplicate),
        ("Select", input.buttons.select),
        ("Solo", input.buttons.solo),
        ("Mute", input.buttons.mute),
        ("Pitch", input.buttons.pitch),
        ("Mod", input.buttons.mod_),
        ("Perform", input.buttons.perform),
        ("Shift", input.buttons.shift),
    ];
    
    let active_controls: Vec<&str> = control_buttons
        .iter()
        .filter(|(_, pressed)| *pressed)
        .map(|(name, _)| *name)
        .collect();
    
    if !active_controls.is_empty() {
        println!("  🎚️  Controls: {}", active_controls.join(", "));
    }
    
    // Display Buttons
    let display_buttons = [
        ("DB1", input.buttons.display_button_1),
        ("DB2", input.buttons.display_button_2),
        ("DB3", input.buttons.display_button_3),
        ("DB4", input.buttons.display_button_4),
        ("DB5", input.buttons.display_button_5),
        ("DB6", input.buttons.display_button_6),
        ("DB7", input.buttons.display_button_7),
        ("DB8", input.buttons.display_button_8),
    ];
    
    let active_display: Vec<&str> = display_buttons
        .iter()
        .filter(|(_, pressed)| *pressed)
        .map(|(name, _)| *name)
        .collect();
    
    if !active_display.is_empty() {
        println!("  📱 Display: {}", active_display.join(", "));
    }
    
    // Encoder
    if input.buttons.encoder_push || input.buttons.encoder_up || input.buttons.encoder_down || input.buttons.encoder_left || input.buttons.encoder_right {
        let mut encoder_actions = Vec::new();
        if input.buttons.encoder_push { encoder_actions.push("Push"); }
        if input.buttons.encoder_up { encoder_actions.push("Up"); }
        if input.buttons.encoder_down { encoder_actions.push("Down"); }
        if input.buttons.encoder_left { encoder_actions.push("Left"); }
        if input.buttons.encoder_right { encoder_actions.push("Right"); }
        println!("  🕹️  Encoder: {} (pos: {})", encoder_actions.join("+"), input.knobs.main_encoder);
    }
    
    // Knobs (only show if touched or changed significantly)
    let knob_values = [
        ("K1", input.knobs.knob_1, input.knobs.knob_1_touched),
        ("K2", input.knobs.knob_2, input.knobs.knob_2_touched),
        ("K3", input.knobs.knob_3, input.knobs.knob_3_touched),
        ("K4", input.knobs.knob_4, input.knobs.knob_4_touched),
        ("K5", input.knobs.knob_5, input.knobs.knob_5_touched),
        ("K6", input.knobs.knob_6, input.knobs.knob_6_touched),
        ("K7", input.knobs.knob_7, input.knobs.knob_7_touched),
        ("K8", input.knobs.knob_8, input.knobs.knob_8_touched),
    ];
    
    let touched_knobs: Vec<String> = knob_values
        .iter()
        .filter(|(_, _, touched)| *touched)
        .map(|(name, value, _)| format!("{}:{}", name, value))
        .collect();
    
    if !touched_knobs.is_empty() {
        println!("  🎛️  Knobs: {} (touched)", touched_knobs.join(", "));
    }
    
    let changed_knobs: Vec<String> = knob_values
        .iter()
        .filter(|(_, _, touched)| !*touched)
        .filter(|(name, _, _)| changes.iter().any(|c| c.starts_with(name)))
        .map(|(name, value, _)| format!("{}:{}", name, value))
        .collect();
        
    if !changed_knobs.is_empty() {
        println!("  🎛️  Knobs: {} (changed)", changed_knobs.join(", "));
    }
    
    // Audio Controls
    if input.audio.mic_gain > 0 || input.audio.headphone_volume > 0 || input.audio.master_volume > 0 {
        println!("  🔊 Audio: Mic:{} HP:{} Master:{}", 
                 input.audio.mic_gain, input.audio.headphone_volume, input.audio.master_volume);
    }
    
    // Touch Strip
    if input.touch_strip.finger_1.data_a > 0 || input.touch_strip.finger_2.data_a > 0 {
        println!("  👆 Touch Strip: F1({},{},{},{}) F2({},{},{},{})",
                 input.touch_strip.finger_1.data_a, input.touch_strip.finger_1.data_b, 
                 input.touch_strip.finger_1.data_c, input.touch_strip.finger_1.data_d,
                 input.touch_strip.finger_2.data_a, input.touch_strip.finger_2.data_b,
                 input.touch_strip.finger_2.data_c, input.touch_strip.finger_2.data_d);
    }
    
    // Hardware Status
    if input.buttons.pedal_connected || input.buttons.microphone_connected {
        let mut status = Vec::new();
        if input.buttons.pedal_connected { status.push("Pedal"); }
        if input.buttons.microphone_connected { status.push("Microphone"); }
        println!("  🔌 Connected: {}", status.join(", "));
    }
    
    // Show specific changes
    if !changes.is_empty() {
        println!("  📝 Changes: {}", changes.join(", "));
    }
}

fn print_pad_state(pads: &PadState, frame: u32) {
    println!("\n🥁 PAD HITS [Frame {}] ===============================", frame);
    
    for (i, hit) in pads.hits.iter().enumerate() {
        let pad_name = match hit.pad_number {
            0..=15 => format!("Pad #{} ({})", hit.pad_number + 1, get_pad_position(hit.pad_number)),
            _ => format!("Unknown Pad {}", hit.pad_number),
        };
        
        println!("  {} - Velocity: {} Pressure: {} (raw: {}, {})", 
                 pad_name, hit.data_a, hit.data_b, hit.data_a, hit.data_b);
        
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
        0 => "TR",    // Top Right
        1 => "T2",
        2 => "T3", 
        3 => "TL",    // Top Left
        4 => "2R",
        5 => "22",
        6 => "23",
        7 => "2L",
        8 => "3R",
        9 => "32",
        10 => "33",
        11 => "3L",
        12 => "BR",   // Bottom Right
        13 => "B2",
        14 => "B3",
        15 => "BL",   // Bottom Left
        _ => "??",
    }
}

fn check_button_changes(current: &InputState, last: &InputState, changes: &mut Vec<String>) {
    // Check transport buttons
    if current.buttons.play != last.buttons.play {
        changes.push(format!("Play: {}", if current.buttons.play { "ON" } else { "OFF" }));
    }
    if current.buttons.rec != last.buttons.rec {
        changes.push(format!("Rec: {}", if current.buttons.rec { "ON" } else { "OFF" }));
    }
    if current.buttons.stop != last.buttons.stop {
        changes.push(format!("Stop: {}", if current.buttons.stop { "ON" } else { "OFF" }));
    }
    
    // Check group buttons
    let groups = [
        ("Group A", current.buttons.group_a, last.buttons.group_a),
        ("Group B", current.buttons.group_b, last.buttons.group_b),
        ("Group C", current.buttons.group_c, last.buttons.group_c),
        ("Group D", current.buttons.group_d, last.buttons.group_d),
    ];
    
    for (name, current_state, last_state) in groups {
        if current_state != last_state {
            changes.push(format!("{}: {}", name, if current_state { "ON" } else { "OFF" }));
        }
    }
    
    // Add more button checks as needed...
}

fn check_knob_changes(current: &InputState, last: &InputState, changes: &mut Vec<String>) {
    let knobs = [
        ("K1", current.knobs.knob_1, last.knobs.knob_1),
        ("K2", current.knobs.knob_2, last.knobs.knob_2),
        ("K3", current.knobs.knob_3, last.knobs.knob_3),
        ("K4", current.knobs.knob_4, last.knobs.knob_4),
        ("K5", current.knobs.knob_5, last.knobs.knob_5),
        ("K6", current.knobs.knob_6, last.knobs.knob_6),
        ("K7", current.knobs.knob_7, last.knobs.knob_7),
        ("K8", current.knobs.knob_8, last.knobs.knob_8),
    ];
    
    for (name, current_val, last_val) in knobs {
        if (current_val as i32 - last_val as i32).abs() > 5 {
            changes.push(format!("{}: {} -> {}", name, last_val, current_val));
        }
    }
    
    if current.knobs.main_encoder != last.knobs.main_encoder {
        changes.push(format!("Main Encoder: {} -> {}", last.knobs.main_encoder, current.knobs.main_encoder));
    }
}
