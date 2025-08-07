use maschine3_hal::{ButtonLedState, MK3Error, MaschineLEDColor, MaschineMK3, PadLedState, RgbColor};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌈 Maschine MK3 LED Animation Test");

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

    println!("\n🎬 Starting LED animations - watch your device!");
    println!("   Press Ctrl+C to stop\n");

    // Animation 1: Transport button chase
    println!("🚦 Animation 1: Transport button chase");
    for i in 0..20 {
        let mut leds = ButtonLedState::default();
        match i % 4 {
            0 => leds.play = 127,
            1 => leds.rec = 127,
            2 => leds.stop = 127,
            3 => leds.restart = 127,
            _ => {}
        }
        device.write_button_leds(&leds)?;
        std::thread::sleep(Duration::from_millis(200));
    }

    // Animation 2: Group button rainbow
    println!("🌈 Animation 2: Group button rainbow");
    for i in 0..30 {
        let mut leds = ButtonLedState::default();
        let hue = (i * 12) % 360;
        let color = hsv_to_rgb(hue as f32, 1.0, 1.0);

        leds.group_a = MaschineLEDColor::from_rgb_color(color);
        leds.group_b =
            MaschineLEDColor::from_rgb_color(hsv_to_rgb((hue + 45) as f32 % 360.0, 1.0, 1.0));
        leds.group_c =
            MaschineLEDColor::from_rgb_color(hsv_to_rgb((hue + 90) as f32 % 360.0, 1.0, 1.0));
        leds.group_d =
            MaschineLEDColor::from_rgb_color(hsv_to_rgb((hue + 135) as f32 % 360.0, 1.0, 1.0));
        leds.group_e =
            MaschineLEDColor::from_rgb_color(hsv_to_rgb((hue + 180) as f32 % 360.0, 1.0, 1.0));
        leds.group_f =
            MaschineLEDColor::from_rgb_color(hsv_to_rgb((hue + 225) as f32 % 360.0, 1.0, 1.0));
        leds.group_g =
            MaschineLEDColor::from_rgb_color(hsv_to_rgb((hue + 270) as f32 % 360.0, 1.0, 1.0));
        leds.group_h =
            MaschineLEDColor::from_rgb_color(hsv_to_rgb((hue + 315) as f32 % 360.0, 1.0, 1.0));

        device.write_button_leds(&leds)?;
        std::thread::sleep(Duration::from_millis(100));
    }

    // Animation 3: Pad matrix sweep
    println!("🔥 Animation 3: Pad matrix sweep");
    for i in 0..32 {
        let mut pad_leds = PadLedState::default();

        // Light up pads in a wave pattern
        for pad in 0..16 {
            let distance = ((pad as i32 - (i / 2) as i32).abs()) as f32;
            let brightness = (1.0 - (distance / 8.0)).max(0.0);

            pad_leds.pad_leds[pad] = MaschineLEDColor::from_rgb_color(RgbColor::new(
                (255.0 * brightness) as u8,
                (128.0 * brightness) as u8,
                (64.0 * brightness) as u8,
            ));
        }

        device.write_pad_leds(&pad_leds)?;
        std::thread::sleep(Duration::from_millis(100));
    }

    // Animation 4: Touch strip chase
    println!("✨ Animation 4: Touch strip chase");
    for i in 0..50 {
        let mut pad_leds = PadLedState::default();

        // Create a moving dot on touch strip
        let pos = i % 25;
        for j in 0..25 {
            if j == pos {
                pad_leds.touch_strip_leds[j] = MaschineLEDColor::white(true);
            } else if (j as i32 - pos as i32).abs() == 1 {
                pad_leds.touch_strip_leds[j] =
                    MaschineLEDColor::from_rgb_color(RgbColor::new(128, 128, 128));
            } else if (j as i32 - pos as i32).abs() == 2 {
                pad_leds.touch_strip_leds[j] =
                    MaschineLEDColor::from_rgb_color(RgbColor::new(64, 64, 64));
            } else {
                pad_leds.touch_strip_leds[j] = MaschineLEDColor::black();
            }
        }

        device.write_pad_leds(&pad_leds)?;
        std::thread::sleep(Duration::from_millis(80));
    }

    // Animation 5: All buttons pulse
    println!("💗 Animation 5: Button pulse");
    for i in 0..30 {
        let brightness = ((i as f32 * 0.2).sin() * 127.0 + 127.0) as u8;
        let mut leds = ButtonLedState::default();

        // Pulse all single-color LEDs
        leds.play = brightness;
        leds.rec = brightness;
        leds.stop = brightness;
        leds.volume = brightness;
        leds.swing = brightness;
        leds.tempo = brightness;
        leds.notes = brightness;

        device.write_button_leds(&leds)?;
        std::thread::sleep(Duration::from_millis(100));
    }

    // Animation 6: Pad checkerboard
    println!("🏁 Animation 6: Pad checkerboard");
    for i in 0..20 {
        let mut pad_leds = PadLedState::default();

        for pad in 0..16 {
            let row = pad / 4;
            let col = pad % 4;
            let is_even_frame = (i % 2) == 0;
            let is_checker = ((row + col) % 2) == 0;

            if is_even_frame == is_checker {
                pad_leds.pad_leds[pad] = MaschineLEDColor::red(true);
            } else {
                pad_leds.pad_leds[pad] = MaschineLEDColor::blue(true);
            }
        }

        device.write_pad_leds(&pad_leds)?;
        std::thread::sleep(Duration::from_millis(300));
    }

    // Animation 7: Everything party mode!
    println!("🎉 Animation 7: PARTY MODE!");
    for i in 0..100 {
        let mut button_leds = ButtonLedState::default();
        let mut pad_leds = PadLedState::default();

        // Random-ish colors for everything
        let time = i as f32 * 0.1;

        // Rainbow groups
        for group in 0..8 {
            let hue = (time * 60.0 + group as f32 * 45.0) % 360.0;
            let color = hsv_to_rgb(hue, 1.0, 1.0);
            match group {
                0 => button_leds.group_a = MaschineLEDColor::from_rgb_color(color),
                1 => button_leds.group_b = MaschineLEDColor::from_rgb_color(color),
                2 => button_leds.group_c = MaschineLEDColor::from_rgb_color(color),
                3 => button_leds.group_d = MaschineLEDColor::from_rgb_color(color),
                4 => button_leds.group_e = MaschineLEDColor::from_rgb_color(color),
                5 => button_leds.group_f = MaschineLEDColor::from_rgb_color(color),
                6 => button_leds.group_g = MaschineLEDColor::from_rgb_color(color),
                7 => button_leds.group_h = MaschineLEDColor::from_rgb_color(color),
                _ => {}
            }
        }

        // Pulsing transport
        let pulse = ((time * 3.0).sin() * 127.0 + 127.0) as u8;
        button_leds.play = pulse;
        button_leds.rec = 255 - pulse;
        button_leds.stop = pulse / 2;

        // Rainbow pads
        for pad in 0..16 {
            let hue = (time * 80.0 + pad as f32 * 22.5) % 360.0;
            pad_leds.pad_leds[pad] = MaschineLEDColor::from_rgb_color(hsv_to_rgb(hue, 1.0, 0.8));
        }

        // Touch strip wave
        for led in 0..25 {
            let wave = ((time * 2.0 + led as f32 * 0.5).sin() + 1.0) / 2.0;
            let hue = (time * 50.0 + led as f32 * 14.4) % 360.0;
            let color = hsv_to_rgb(hue, 1.0, wave as f32);
            pad_leds.touch_strip_leds[led] = MaschineLEDColor::from_rgb_color(color);
        }

        device.write_button_leds(&button_leds)?;
        device.write_pad_leds(&pad_leds)?;
        std::thread::sleep(Duration::from_millis(50));
    }

    // Clean up - turn everything off
    println!("\n🧹 Cleaning up...");
    device.write_button_leds(&ButtonLedState::default())?;
    device.write_pad_leds(&PadLedState::default())?;

    println!("✅ LED animation test complete!");
    println!("💡 If you saw the LEDs animate, the output system is working perfectly!");

    Ok(())
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
