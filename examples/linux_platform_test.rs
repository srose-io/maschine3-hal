use maschine3_hal::{
    InputEvent, InputElement, MK3Error, MaschineLEDColor, MaschineMK3,
};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🐧 Maschine MK3 Linux Platform Test");
    println!("   This example demonstrates Linux-specific features and capabilities.");

    let mut device = match MaschineMK3::new() {
        Ok(device) => {
            println!("✅ Connected: {}", device.device_info()?);
            device
        }
        Err(MK3Error::DeviceNotFound) => {
            println!("❌ No Maschine MK3 found");
            println!("   💡 Make sure:");
            println!("      - Device is connected via USB");
            println!("      - udev rules are installed (see LINUX_SETUP.md)");
            println!("      - You're in the 'audio' group");
            println!("      - You've logged out/in after group membership change");
            return Ok(());
        }
        Err(e) => {
            println!("❌ Connection error: {}", e);
            println!("   💡 This might be a permission issue. Check LINUX_SETUP.md");
            return Ok(());
        }
    };

    println!("\n🔧 Platform Information:");
    println!("   OS: {}", std::env::consts::OS);
    println!("   Architecture: {}", std::env::consts::ARCH);
    
    #[cfg(unix)]
    {
        println!("   Communication: Direct USB (Linux native)");
        println!("   Driver detachment: Automatic");
        println!("   Performance: Optimized for low latency");
    }
    
    #[cfg(windows)]
    {
        println!("   Communication: HID API (Windows compatibility)");
        println!("   Driver detachment: Manual via Zadig");
        println!("   Performance: Stable with good compatibility");
    }

    println!("\n🧪 Test 1: Direct USB Communication Performance");
    println!("   Testing input latency and throughput...");

    let start_time = std::time::Instant::now();
    let mut total_events = 0;
    let mut max_events_per_poll = 0;
    
    // Test for 5 seconds to measure performance
    while start_time.elapsed() < Duration::from_secs(5) {
        let poll_start = std::time::Instant::now();
        let events = device.poll_input_events()?;
        let poll_duration = poll_start.elapsed();
        
        let event_count = events.len();
        total_events += event_count;
        max_events_per_poll = max_events_per_poll.max(event_count);
        
        for event in events {
            match event {
                InputEvent::PadEvent { pad_number, event_type: maschine3_hal::PadEventType::Hit, value } => {
                    println!("   🥁 Pad {} hit (velocity: {}, poll time: {:?})", 
                             pad_number + 1, value, poll_duration);
                    
                    // Light up the pad that was hit
                    device.set_pad_led(pad_number, MaschineLEDColor::green(true))?;
                }
                InputEvent::ButtonPressed(element) => {
                    println!("   ▶️  {} pressed (poll time: {:?})", 
                             element.name(), poll_duration);
                }
                _ => {}
            }
        }
        
        std::thread::sleep(Duration::from_millis(1)); // Minimal sleep for high polling rate
    }

    println!("   📊 Performance Results:");
    println!("      Total events: {}", total_events);
    println!("      Max events per poll: {}", max_events_per_poll);
    println!("      Average events per second: {:.1}", 
             total_events as f64 / 5.0);

    println!("\n🌈 Test 2: LED Performance Test");
    println!("   Testing LED update rates...");

    let led_start = std::time::Instant::now();
    let mut led_updates = 0;

    // Animate LEDs for 3 seconds to test update performance
    while led_start.elapsed() < Duration::from_secs(3) {
        let time = led_start.elapsed().as_secs_f32();
        
        // Create a moving wave pattern
        for pad in 0..16 {
            let intensity = ((time * 2.0 + pad as f32 * 0.4).sin() * 0.5 + 0.5) * 255.0;
            let color = MaschineLEDColor::new(
                intensity as u8,
                true
            );
            device.set_pad_led(pad, color)?;
            led_updates += 1;
        }
        
        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    println!("   💡 LED Updates: {} ({:.1}/sec)", 
             led_updates, led_updates as f64 / 3.0);

    println!("\n🎛️  Test 3: Group Button RGB LEDs");
    println!("   Testing RGB group buttons (Linux optimized path)...");

    let rgb_colors = [
        MaschineLEDColor::red(true),
        MaschineLEDColor::green(true),
        MaschineLEDColor::blue(true),
        MaschineLEDColor::yellow(true),
        MaschineLEDColor::magenta(true),
        MaschineLEDColor::cyan(true),
        MaschineLEDColor::white(true),
        MaschineLEDColor::orange(true),
    ];

    let group_buttons = [
        InputElement::GroupA,
        InputElement::GroupB,
        InputElement::GroupC,
        InputElement::GroupD,
        InputElement::GroupE,
        InputElement::GroupF,
        InputElement::GroupG,
        InputElement::GroupH,
    ];

    for (i, button) in group_buttons.iter().enumerate() {
        device.set_button_led_color(button.clone(), rgb_colors[i])?;
        std::thread::sleep(Duration::from_millis(200));
    }

    std::thread::sleep(Duration::from_secs(1));

    println!("\n🧹 Cleanup: Turning off all LEDs...");
    device.clear_all_leds()?;

    println!("\n✅ Linux platform test completed successfully!");
    println!("   💡 For optimal performance on Linux:");
    println!("      - Ensure proper udev rules are installed");
    println!("      - Consider real-time kernel for audio applications");
    println!("      - Monitor system resources during intensive operations");

    Ok(())
}