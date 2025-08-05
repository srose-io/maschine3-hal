use mk3_hal::{MaschineLEDColor, RgbColor};

fn main() {
    println!("🎨 Maschine MK3 Color Mapping Test");

    // Test predefined colors
    println!("\n📋 Predefined colors:");
    let colors = [
        ("Red", MaschineLEDColor::red(true)),
        ("Orange", MaschineLEDColor::orange(true)),
        ("Yellow", MaschineLEDColor::yellow(true)),
        ("Green", MaschineLEDColor::green(true)),
        ("Cyan", MaschineLEDColor::cyan(true)),
        ("Blue", MaschineLEDColor::blue(true)),
        ("Purple", MaschineLEDColor::purple(true)),
        ("Magenta", MaschineLEDColor::magenta(true)),
        ("Pink", MaschineLEDColor::pink(true)),
        ("White", MaschineLEDColor::white(true)),
        ("Black", MaschineLEDColor::black()),
    ];

    for (name, color) in colors {
        let led_value = color.to_led_value();
        let (r, g, b) = color.to_rgb();
        println!(
            "   {:<8} -> index: {}, bright: {}, LED value: {}, RGB: ({}, {}, {})",
            name, color.index, color.bright, led_value, r, g, b
        );
    }

    // Test RGB to Maschine color conversion
    println!("\n🔄 RGB to Maschine color conversion:");
    let test_colors = [
        (255, 0, 0),     // Pure red
        (255, 128, 0),   // Orange-ish
        (255, 255, 0),   // Yellow
        (0, 255, 0),     // Green
        (0, 255, 255),   // Cyan
        (0, 0, 255),     // Blue
        (255, 0, 255),   // Magenta
        (128, 64, 192),  // Purple-ish
        (64, 64, 64),    // Dark gray
        (192, 192, 192), // Light gray
        (0, 0, 0),       // Black
    ];

    for (r, g, b) in test_colors {
        let rgb = RgbColor::new(r, g, b);
        let maschine_color = MaschineLEDColor::from(rgb);
        let led_value = maschine_color.to_led_value();
        let (mr, mg, mb) = maschine_color.to_rgb();

        println!(
            "   RGB({}, {}, {}) -> index: {}, bright: {}, LED: {}, mapped RGB: ({}, {}, {})",
            r, g, b, maschine_color.index, maschine_color.bright, led_value, mr, mg, mb
        );
    }

    // Test the C# formula edge cases
    println!("\n🧮 LED value formula test:");
    for index in 0..17 {
        for bright in [false, true] {
            let color = MaschineLEDColor::new(index, bright);
            let led_value = color.to_led_value();
            println!(
                "   Index: {}, Bright: {} -> LED value: {}",
                index, bright, led_value
            );
        }
    }
}
