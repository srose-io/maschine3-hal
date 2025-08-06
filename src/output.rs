/// LED brightness levels (0-127 for most LEDs)
pub type LedBrightness = u8;

/// RGB color for RGB LEDs
#[derive(Debug, Clone, Copy, Default)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }

    pub fn red() -> Self {
        Self::new(255, 0, 0)
    }

    pub fn green() -> Self {
        Self::new(0, 255, 0)
    }

    pub fn blue() -> Self {
        Self::new(0, 0, 255)
    }
}

/// Maschine MK3 color mapping based on the hardware color palette
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct MaschineLEDColor {
    pub index: u8,    // 0-16 color index
    pub bright: bool, // true for bright, false for dim
}

impl MaschineLEDColor {
    /// Standard Maschine color palette (17 colors from the grid)
    const PALETTE: [(u8, u8, u8); 17] = [
        (255, 0, 0),     // 0: Red
        (255, 165, 0),   // 1: Orange
        (255, 200, 0),   // 2: Orange-yellow
        (255, 255, 0),   // 3: Yellow
        (128, 255, 0),   // 4: Yellow-green
        (0, 255, 0),     // 5: Green
        (0, 255, 128),   // 6: Cyan-green
        (0, 255, 255),   // 7: Cyan
        (0, 128, 255),   // 8: Light blue
        (0, 0, 255),     // 9: Blue
        (128, 0, 255),   // 10: Purple
        (255, 0, 255),   // 11: Magenta
        (255, 0, 128),   // 12: Pink
        (255, 128, 255), // 13: Hot pink
        (64, 0, 128),    // 14: Dark purple
        (128, 128, 128), // 15: Gray
        (255, 255, 255), // 16: White
    ];

    pub fn from_rgb_color(color: RgbColor) -> Self {
        Self::from_rgb(color.r, color.g, color.b)
    }

    /// Create a new MaschineColor from RGB values
    /// Maps to the nearest color in the palette and determines brightness
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        // Handle black/off case
        if r == 0 && g == 0 && b == 0 {
            return MaschineLEDColor {
                index: 0,
                bright: false,
            };
        }

        // Find the closest color in the palette using Euclidean distance
        let mut best_distance = f32::MAX;
        let mut best_index = 0;

        for (index, &(pr, pg, pb)) in Self::PALETTE.iter().enumerate() {
            let distance = ((r as f32 - pr as f32).powi(2)
                + (g as f32 - pg as f32).powi(2)
                + (b as f32 - pb as f32).powi(2))
            .sqrt();

            if distance < best_distance {
                best_distance = distance;
                best_index = index;
            }
        }

        // Determine brightness based on the maximum RGB component rather than luminance
        // This ensures pure colors like red, green, blue are bright
        let max_component = r.max(g).max(b);
        let bright = max_component > 127;

        MaschineLEDColor {
            index: best_index as u8,
            bright,
        }
    }

    /// Create a MaschineColor with specific color index and brightness
    pub fn new(index: u8, bright: bool) -> Self {
        MaschineLEDColor {
            index: index.min(16), // Clamp to valid range (0-16)
            bright,
        }
    }

    /// Convert to the actual LED value using the Maschine mapping formula
    /// Port of the C# code for converting index + brightness to LED value
    pub fn to_led_value(&self) -> u8 {
        // Special case: black/off
        if self.index == 0 && !self.bright {
            return 0;
        }

        let mut basecolor = (self.index % 17) + 1;
        basecolor *= 2;
        let adjusted = basecolor - if !self.bright { 1 } else { 0 };

        let mut result = adjusted * 2 + 2;

        if result > 66 {
            result += 4;
        }

        result as u8
    }

    /// Predefined colors for common use
    pub fn red(bright: bool) -> Self {
        Self::new(0, bright)
    }
    pub fn orange(bright: bool) -> Self {
        Self::new(1, bright)
    }
    pub fn yellow(bright: bool) -> Self {
        Self::new(3, bright)
    }
    pub fn green(bright: bool) -> Self {
        Self::new(5, bright)
    }
    pub fn cyan(bright: bool) -> Self {
        Self::new(7, bright)
    }
    pub fn blue(bright: bool) -> Self {
        Self::new(9, bright)
    }
    pub fn purple(bright: bool) -> Self {
        Self::new(10, bright)
    }
    pub fn magenta(bright: bool) -> Self {
        Self::new(11, bright)
    }
    pub fn pink(bright: bool) -> Self {
        Self::new(12, bright)
    }
    pub fn white(bright: bool) -> Self {
        Self::new(16, bright)
    }
    pub fn black() -> Self {
        Self::new(0, false)
    }

    /// Create a grayscale color from brightness value (0-255)
    pub fn from_brightness(brightness: u8) -> Self {
        if brightness == 0 {
            Self::black()
        } else {
            // Use white with brightness control
            Self::new(16, brightness > 127)
        }
    }

    /// Get RGB values for this Maschine color (for preview/debugging)
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        // Special case: black/off
        if self.index == 0 && !self.bright {
            return (0, 0, 0);
        }

        let (r, g, b) = Self::PALETTE[self.index as usize % 17];
        if self.bright {
            (r, g, b)
        } else {
            // Dim version - reduce brightness by ~50%
            (r / 2, g / 2, b / 2)
        }
    }
}

impl From<RgbColor> for MaschineLEDColor {
    fn from(rgb: RgbColor) -> Self {
        Self::from_rgb(rgb.r, rgb.g, rgb.b)
    }
}

/// State of all button LEDs (Type 0x80 packet)
#[derive(Debug, Clone, Default)]
pub struct ButtonLedState {
    // Single-color LEDs
    pub channel_midi: LedBrightness,
    pub plugin_instance: LedBrightness,
    pub arranger: LedBrightness,
    pub mixer: LedBrightness,
    pub sampler: LedBrightness,
    pub arrow_left: LedBrightness,
    pub arrow_right: LedBrightness,
    pub file_save: LedBrightness,
    pub settings: LedBrightness,
    pub auto: LedBrightness,
    pub macro_set: LedBrightness,
    pub display_button_1: LedBrightness,
    pub display_button_2: LedBrightness,
    pub display_button_3: LedBrightness,
    pub display_button_4: LedBrightness,
    pub display_button_5: LedBrightness,
    pub display_button_6: LedBrightness,
    pub display_button_7: LedBrightness,
    pub display_button_8: LedBrightness,
    pub volume: LedBrightness,
    pub swing: LedBrightness,
    pub note_repeat: LedBrightness,
    pub tempo: LedBrightness,
    pub lock: LedBrightness,
    pub pitch: LedBrightness,
    pub mod_: LedBrightness,
    pub perform: LedBrightness,
    pub notes: LedBrightness,
    pub restart: LedBrightness,
    pub erase: LedBrightness,
    pub tap: LedBrightness,
    pub follow: LedBrightness,
    pub play: LedBrightness,
    pub rec: LedBrightness,
    pub stop: LedBrightness,
    pub shift: LedBrightness,
    pub fixed_vel: LedBrightness,
    pub pad_mode: LedBrightness,
    pub keyboard: LedBrightness,
    pub chords: LedBrightness,
    pub step: LedBrightness,
    pub scene: LedBrightness,
    pub pattern: LedBrightness,
    pub events: LedBrightness,
    pub variation: LedBrightness,
    pub duplicate: LedBrightness,
    pub select: LedBrightness,
    pub solo: LedBrightness,
    pub mute: LedBrightness,

    // RGB LEDs
    pub browser_plugin: MaschineLEDColor,
    pub group_a: MaschineLEDColor,
    pub group_b: MaschineLEDColor,
    pub group_c: MaschineLEDColor,
    pub group_d: MaschineLEDColor,
    pub group_e: MaschineLEDColor,
    pub group_f: MaschineLEDColor,
    pub group_g: MaschineLEDColor,
    pub group_h: MaschineLEDColor,
    pub nav_up: MaschineLEDColor,
    pub nav_left: MaschineLEDColor,
    pub nav_right: MaschineLEDColor,
    pub nav_down: MaschineLEDColor,
}

/// State of pad and touch strip LEDs (Type 0x81 packet)
#[derive(Debug, Clone, Default)]
pub struct PadLedState {
    pub touch_strip_leds: [MaschineLEDColor; 25], // 25 RGB LEDs on touch strip
    pub pad_leds: [MaschineLEDColor; 16],         // 16 RGB pad LEDs
}

impl ButtonLedState {
    /// Convert to Type 0x80 packet (62 bytes)
    pub fn to_packet(&self) -> Vec<u8> {
        let mut packet = vec![0u8; 63];
        packet[0] = 0x80; // Packet type

        // Single-color LEDs (according to documentation order)
        packet[1] = self.channel_midi;
        packet[2] = self.plugin_instance;
        packet[3] = self.arranger;
        packet[4] = self.mixer;
        packet[5] = self.browser_plugin.to_led_value(); // RGB LED - using only red for now
        packet[6] = self.sampler;
        packet[7] = self.arrow_left;
        packet[8] = self.arrow_right;
        packet[9] = self.file_save;
        packet[10] = self.settings;
        packet[11] = self.auto;
        packet[12] = self.macro_set;
        packet[13] = self.display_button_1;
        packet[14] = self.display_button_2;
        packet[15] = self.display_button_3;
        packet[16] = self.display_button_4;
        packet[17] = self.display_button_5;
        packet[18] = self.display_button_6;
        packet[19] = self.display_button_7;
        packet[20] = self.display_button_8;
        packet[21] = self.volume;
        packet[22] = self.swing;
        packet[23] = self.note_repeat;
        packet[24] = self.tempo;
        packet[25] = self.lock;
        packet[26] = self.pitch;
        packet[27] = self.mod_;
        packet[28] = self.perform;
        packet[29] = self.notes;

        // Group RGB LEDs (simplified - need proper RGB mapping)
        packet[30] = self.group_a.to_led_value();
        packet[31] = self.group_b.to_led_value();
        packet[32] = self.group_c.to_led_value();
        packet[33] = self.group_d.to_led_value();
        packet[34] = self.group_e.to_led_value();
        packet[35] = self.group_f.to_led_value();
        packet[36] = self.group_g.to_led_value();
        packet[37] = self.group_h.to_led_value();

        packet[38] = self.restart;
        packet[39] = self.erase;
        packet[40] = self.tap;
        packet[41] = self.follow;
        packet[42] = self.play;
        packet[43] = self.rec;
        packet[44] = self.stop;
        packet[45] = self.shift;
        packet[46] = self.fixed_vel;
        packet[47] = self.pad_mode;
        packet[48] = self.keyboard;
        packet[49] = self.chords;
        packet[50] = self.step;
        packet[51] = self.scene;
        packet[52] = self.pattern;
        packet[53] = self.events;
        packet[54] = self.variation;
        packet[55] = self.duplicate;
        packet[56] = self.select;
        packet[57] = self.solo;
        packet[58] = self.mute;

        // Navigation RGB LEDs
        packet[59] = self.nav_up.to_led_value();
        packet[60] = self.nav_left.to_led_value();
        packet[61] = self.nav_right.to_led_value();
        packet[62] = self.nav_down.to_led_value();

        packet
    }
}

impl PadLedState {
    /// Convert to Type 0x81 packet (42 bytes)
    pub fn to_packet(&self) -> Vec<u8> {
        let mut packet = vec![0u8; 42];
        packet[0] = 0x81; // Packet type

        // Touch strip LEDs (25 RGB, bytes 1-26, simplified to single byte per LED)
        for (i, led) in self.touch_strip_leds.iter().enumerate() {
            if i + 1 < packet.len() {
                packet[i + 1] = led.to_led_value();
            }
        }

        // Pad LEDs (16 RGB, bytes 27-42, simplified to single byte per LED)
        for (i, led) in self.pad_leds.iter().enumerate() {
            if i + 26 < packet.len() {
                packet[i + 26] = led.to_led_value();
            }
        }

        packet
    }
}

/// RGB565X pixel format for displays (CORRECTED)
#[derive(Debug, Clone, Copy, Default)]
pub struct Rgb565 {
    pub value: u16,
}

impl Rgb565 {
    /// Convert RGB color to MK3's custom RGB565X format with channel rotation
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        // Apply MK3 channel rotation: RED→BLUE, GREEN→RED, BLUE→GREEN
        let corrected_r = b; // Red channel gets blue input
        let corrected_g = r; // Green channel gets red input  
        let corrected_b = g; // Blue channel gets green input

        // Pack as: GGGB BBBB RRRR RGGG
        let r4 = (corrected_r >> 4) as u16; // Red high: 4 bits  
        let r1 = (corrected_r >> 3) & 0x1; // Red low: 1 bit
        let b5 = (corrected_b >> 3) as u16; // Blue: 5 bits
        let g_high = (corrected_g >> 5) as u16; // Green high: 3 bits
        let g_low = (corrected_g >> 3) & 0x7; // Green low: 3 bits

        Self {
            value: (g_high << 13) | (b5 << 8) | (r4 << 4) | ((r1 as u16) << 3) | (g_low as u16),
        }
    }

    pub fn from_rgb(color: RgbColor) -> Self {
        Self::new(color.r, color.g, color.b)
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }

    pub fn red() -> Self {
        Self::new(255, 0, 0)
    }

    pub fn green() -> Self {
        Self::new(0, 255, 0)
    }

    pub fn blue() -> Self {
        Self::new(0, 0, 255)
    }

    pub fn yellow() -> Self {
        Self::new(255, 255, 0)
    }

    pub fn magenta() -> Self {
        Self::new(255, 0, 255)
    }

    pub fn cyan() -> Self {
        Self::new(0, 255, 255)
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
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

        Self::new(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }
}

/// Helper functions for creating display patterns
pub struct DisplayGraphics;

impl DisplayGraphics {
    /// Create a gradient pattern
    pub fn gradient(width: u16, height: u16, color1: Rgb565, color2: Rgb565) -> Vec<Rgb565> {
        let mut pixels = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            let ratio = y as f32 / height as f32;
            let color = Self::lerp_color(color1, color2, ratio);
            for _ in 0..width {
                pixels.push(color);
            }
        }

        pixels
    }

    /// Create a rainbow pattern
    pub fn rainbow(width: u16, height: u16) -> Vec<Rgb565> {
        let mut pixels = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                let hue = ((x as f32 / width as f32) * 360.0) % 360.0;
                let sat = 1.0 - (y as f32 / height as f32) * 0.5;
                let val = 1.0;
                pixels.push(Rgb565::from_hsv(hue, sat, val));
            }
        }

        pixels
    }

    /// Create a checkerboard pattern
    pub fn checkerboard(
        width: u16,
        height: u16,
        square_size: u16,
        color1: Rgb565,
        color2: Rgb565,
    ) -> Vec<Rgb565> {
        let mut pixels = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                let checker_x = (x / square_size) % 2;
                let checker_y = (y / square_size) % 2;
                let color = if (checker_x + checker_y) % 2 == 0 {
                    color1
                } else {
                    color2
                };
                pixels.push(color);
            }
        }

        pixels
    }

    /// Create animated plasma effect
    pub fn plasma(width: u16, height: u16, time: f32) -> Vec<Rgb565> {
        let mut pixels = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                let fx = x as f32 / width as f32;
                let fy = y as f32 / height as f32;

                let v1 = (fx * 10.0 + time).sin();
                let v2 = ((fx * 8.0 + fy * 6.0 + time * 1.5).sin() + (fx * 4.0 + time * 2.0).cos())
                    / 2.0;
                let v3 = ((fx - 0.5).powi(2) + (fy - 0.5).powi(2)).sqrt() * 10.0 + time;
                let v = (v1 + v2 + v3.sin()) / 3.0;

                let hue = ((v + 1.0) / 2.0 * 360.0) % 360.0;
                pixels.push(Rgb565::from_hsv(hue, 1.0, 0.8));
            }
        }

        pixels
    }

    fn lerp_color(color1: Rgb565, color2: Rgb565, t: f32) -> Rgb565 {
        // Extract RGB components from RGB565
        let r1 = ((color1.value >> 11) & 0x1F) as f32 * 8.0;
        let g1 = ((color1.value >> 5) & 0x3F) as f32 * 4.0;
        let b1 = (color1.value & 0x1F) as f32 * 8.0;

        let r2 = ((color2.value >> 11) & 0x1F) as f32 * 8.0;
        let g2 = ((color2.value >> 5) & 0x3F) as f32 * 4.0;
        let b2 = (color2.value & 0x1F) as f32 * 8.0;

        let r = (r1 + (r2 - r1) * t) as u8;
        let g = (g1 + (g2 - g1) * t) as u8;
        let b = (b1 + (b2 - b1) * t) as u8;

        Rgb565::new(r, g, b)
    }
}

/// Display command for the MK3 displays
#[derive(Debug, Clone)]
pub enum DisplayCommand {
    /// Transmit pixels directly
    TransmitPixels { pixels: Vec<Rgb565> },
    /// Repeat two pixels n times
    RepeatPixels {
        pixel1: Rgb565,
        pixel2: Rgb565,
        count: u32,
    },
    /// Blit command (0x03)
    Blit,
    /// End of transmission
    EndTransmission,
}

/// Display packet builder for Type 0x84 packets
pub struct DisplayPacket {
    display_id: u8, // 0 = left, 1 = right
    x_start: u16,
    y_start: u16,
    width: u16,
    height: u16,
    commands: Vec<DisplayCommand>,
}

impl DisplayPacket {
    pub fn new(display_id: u8, x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            display_id,
            x_start: x,
            y_start: y,
            width,
            height,
            commands: Vec::new(),
        }
    }

    pub fn add_pixels(&mut self, pixels: Vec<Rgb565>) {
        self.commands
            .push(DisplayCommand::TransmitPixels { pixels });
    }

    pub fn add_repeat(&mut self, pixel1: Rgb565, pixel2: Rgb565, count: u32) {
        self.commands.push(DisplayCommand::RepeatPixels {
            pixel1,
            pixel2,
            count,
        });
    }

    pub fn add_blit(&mut self) {
        self.commands.push(DisplayCommand::Blit);
    }

    pub fn finish(&mut self) {
        self.commands.push(DisplayCommand::EndTransmission);
    }

    /// Create optimized full-screen packet (30 FPS capable)
    pub fn full_screen_optimized(display_id: u8, pixels: Vec<Rgb565>) -> Self {
        let mut packet = Self::new(display_id, 0, 0, 480, 272);
        packet.add_pixels(pixels);
        packet.add_blit();
        packet.finish();
        packet
    }

    /// Build the complete display packet (CORRECTED)
    pub fn to_packet(&self) -> Vec<u8> {
        let mut packet = Vec::new();

        // Header (16 bytes total) - CORRECTED FORMAT
        packet.extend_from_slice(&[
            0x84,
            0x00, // Packet type
            self.display_id,
            0x60, // Display ID and constant
            0x00,
            0x00,
            0x00,
            0x00,                        // Reserved bytes 4-7
            (self.x_start >> 8) as u8,   // X MSB
            (self.x_start & 0xFF) as u8, // X LSB
            (self.y_start >> 8) as u8,   // Y MSB
            (self.y_start & 0xFF) as u8, // Y LSB
            (self.width >> 8) as u8,     // Width MSB
            (self.width & 0xFF) as u8,   // Width LSB
            (self.height >> 8) as u8,    // Height MSB
            (self.height & 0xFF) as u8,  // Height LSB
        ]);

        // Add commands
        for command in &self.commands {
            match command {
                DisplayCommand::TransmitPixels { pixels } => {
                    let pixel_count = pixels.len() as u32;
                    let half_pixels = pixel_count / 2; // CORRECTED: Device expects pixel_count / 2
                    packet.push(0x00); // Command code
                    packet.push((half_pixels >> 16) as u8);
                    packet.push((half_pixels >> 8) as u8);
                    packet.push((half_pixels & 0xFF) as u8);

                    // Add pixel data (little-endian)
                    for pixel in pixels {
                        packet.push((pixel.value & 0xFF) as u8); // LSB first
                        packet.push((pixel.value >> 8) as u8); // MSB second
                    }

                    // No padding needed - data is already 2-byte aligned
                }
                DisplayCommand::RepeatPixels {
                    pixel1,
                    pixel2,
                    count,
                } => {
                    packet.push(0x01); // Command code
                    packet.push((*count >> 16) as u8);
                    packet.push((*count >> 8) as u8);
                    packet.push((*count & 0xFF) as u8);

                    // Add the two pixels to repeat
                    packet.push((pixel1.value & 0xFF) as u8);
                    packet.push((pixel1.value >> 8) as u8);
                    packet.push((pixel2.value & 0xFF) as u8);
                    packet.push((pixel2.value >> 8) as u8);
                }
                DisplayCommand::Blit => {
                    packet.extend_from_slice(&[0x03, 0x00, 0x00, 0x00]);
                }
                DisplayCommand::EndTransmission => {
                    packet.extend_from_slice(&[0x40, 0x00, 0x00, 0x00]);
                }
            }
        }

        packet
    }
}
