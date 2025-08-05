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
    pub browser_plugin: RgbColor,
    pub group_a: RgbColor,
    pub group_b: RgbColor,
    pub group_c: RgbColor,
    pub group_d: RgbColor,
    pub group_e: RgbColor,
    pub group_f: RgbColor,
    pub group_g: RgbColor,
    pub group_h: RgbColor,
    pub nav_up: RgbColor,
    pub nav_left: RgbColor,
    pub nav_right: RgbColor,
    pub nav_down: RgbColor,
}

/// State of pad and touch strip LEDs (Type 0x81 packet)
#[derive(Debug, Clone, Default)]
pub struct PadLedState {
    pub touch_strip_leds: [RgbColor; 25], // 25 RGB LEDs on touch strip
    pub pad_leds: [RgbColor; 16],         // 16 RGB pad LEDs
}

impl ButtonLedState {
    /// Convert to Type 0x80 packet (62 bytes)
    pub fn to_packet(&self) -> Vec<u8> {
        let mut packet = vec![0u8; 62];
        packet[0] = 0x80; // Packet type

        // Single-color LEDs (according to documentation order)
        packet[1] = self.channel_midi;
        packet[2] = self.plugin_instance;
        packet[3] = self.arranger;
        packet[4] = self.mixer;
        packet[5] = self.browser_plugin.r; // RGB LED - using only red for now
        packet[6] = self.sampler;
        packet[7] = self.arrow_left;
        packet[8] = self.arrow_right;
        packet[9] = self.file_save;
        packet[10] = self.settings;
        packet[11] = self.macro_set;
        packet[12] = self.display_button_1;
        packet[13] = self.display_button_2;
        packet[14] = self.display_button_3;
        packet[15] = self.display_button_4;
        packet[16] = self.display_button_5;
        packet[17] = self.display_button_6;
        packet[18] = self.display_button_7;
        packet[19] = self.display_button_8;
        packet[20] = self.volume;
        packet[21] = self.swing;
        packet[22] = self.note_repeat;
        packet[23] = self.tempo;
        packet[24] = self.lock;
        packet[25] = self.pitch;
        packet[26] = self.mod_;
        packet[27] = self.perform;
        packet[28] = self.notes;

        // Group RGB LEDs (simplified - need proper RGB mapping)
        packet[29] = self.group_a.r;
        packet[30] = self.group_b.r;
        packet[31] = self.group_c.r;
        packet[32] = self.group_d.r;
        packet[33] = self.group_e.r;
        packet[34] = self.group_f.r;
        packet[35] = self.group_g.r;
        packet[36] = self.group_h.r;

        packet[37] = self.restart;
        packet[38] = self.erase;
        packet[39] = self.tap;
        packet[40] = self.follow;
        packet[41] = self.play;
        packet[42] = self.rec;
        packet[43] = self.stop;
        packet[44] = self.shift;
        packet[45] = self.fixed_vel;
        packet[46] = self.pad_mode;
        packet[47] = self.keyboard;
        packet[48] = self.chords;
        packet[49] = self.step;
        packet[50] = self.scene;
        packet[51] = self.pattern;
        packet[52] = self.events;
        packet[53] = self.variation;
        packet[54] = self.duplicate;
        packet[55] = self.select;
        packet[56] = self.solo;
        packet[57] = self.mute;

        // Navigation RGB LEDs
        packet[58] = self.nav_up.r;
        packet[59] = self.nav_left.r;
        packet[60] = self.nav_right.r;
        packet[61] = self.nav_down.r;

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
                packet[i + 1] = led.r; // Simplified - should be proper RGB encoding
            }
        }

        // Pad LEDs (16 RGB, bytes 27-42, simplified to single byte per LED)
        for (i, led) in self.pad_leds.iter().enumerate() {
            if i + 27 < packet.len() {
                packet[i + 27] = led.r; // Simplified - should be proper RGB encoding
            }
        }

        packet
    }
}

/// RGB565 pixel format for displays
#[derive(Debug, Clone, Copy, Default)]
pub struct Rgb565 {
    pub value: u16,
}

impl Rgb565 {
pub fn new(r: u8, g: u8, b: u8) -> Self {
let r5 = (r >> 3) as u16;
let g6 = (g >> 2) as u16;
let b5 = (b >> 3) as u16;
Self {
value: (r5 << 11) | (g6 << 5) | b5,
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
    pub fn checkerboard(width: u16, height: u16, square_size: u16, color1: Rgb565, color2: Rgb565) -> Vec<Rgb565> {
        let mut pixels = Vec::with_capacity((width * height) as usize);
        
        for y in 0..height {
            for x in 0..width {
                let checker_x = (x / square_size) % 2;
                let checker_y = (y / square_size) % 2;
                let color = if (checker_x + checker_y) % 2 == 0 { color1 } else { color2 };
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
                let v2 = ((fx * 8.0 + fy * 6.0 + time * 1.5).sin() + (fx * 4.0 + time * 2.0).cos()) / 2.0;
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
    /// Unknown command (probably blit)
    Unknown,
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

    pub fn finish(&mut self) {
        self.commands.push(DisplayCommand::EndTransmission);
    }

    /// Build the complete display packet
    pub fn to_packet(&self) -> Vec<u8> {
        let mut packet = Vec::new();

        // Header Part 1 (16 bytes)
        packet.extend_from_slice(&[
            0x84,
            0x00, // Header 1-2
            self.display_id,
            0x00, // Header 3-4 (display selection)
            0x60,
            0x00, // Header 5-6
            0x00,
            0x00, // Header 7-8
            0x00,
            0x00, // Header 9-10
            0x00,
            0x00, // Header 11-12
            0x00,
            0x00, // Header 13-14
            0x00,
            0x00, // Header 15-16
        ]);

        // Header Part 2 (16 bytes) - coordinates and dimensions
        packet.extend_from_slice(&[
            (self.x_start >> 8) as u8,
            (self.x_start & 0xFF) as u8,
            (self.y_start >> 8) as u8,
            (self.y_start & 0xFF) as u8,
            (self.width >> 8) as u8,
            (self.width & 0xFF) as u8,
            (self.height >> 8) as u8,
            (self.height & 0xFF) as u8,
            0x00,
            0x00,
            0x00,
            0x00, // Padding
            0x00,
            0x00,
            0x00,
            0x00, // Padding
        ]);

        // Add commands
        for command in &self.commands {
            match command {
                DisplayCommand::TransmitPixels { pixels } => {
                    let pixel_count = pixels.len() as u32;
                    packet.push(0x00); // Command code
                    packet.push((pixel_count >> 16) as u8);
                    packet.push((pixel_count >> 8) as u8);
                    packet.push((pixel_count & 0xFF) as u8);

                    // Add pixel data
                    for pixel in pixels {
                        packet.push((pixel.value & 0xFF) as u8);
                        packet.push((pixel.value >> 8) as u8);
                    }

                    // Pad to 4-byte boundary
                    while packet.len() % 4 != 0 {
                        packet.push(0x00);
                    }
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
                DisplayCommand::Unknown => {
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
