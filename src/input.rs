use crate::error::{MK3Error, Result};

/// Represents the state of all buttons on the Maschine MK3
#[derive(Debug, Clone, Default)]
pub struct ButtonState {
    // Transport controls
    pub play: bool,
    pub rec: bool,
    pub stop: bool,
    pub restart: bool,
    pub erase: bool,
    pub tap: bool,
    pub follow: bool,
    
    // Group buttons A-H
    pub group_a: bool,
    pub group_b: bool,
    pub group_c: bool,
    pub group_d: bool,
    pub group_e: bool,
    pub group_f: bool,
    pub group_g: bool,
    pub group_h: bool,
    
    // Mode buttons
    pub notes: bool,
    pub volume: bool,
    pub swing: bool,
    pub tempo: bool,
    pub note_repeat: bool,
    pub lock: bool,
    pub pad_mode: bool,
    pub keyboard: bool,
    pub chords: bool,
    pub step: bool,
    pub fixed_vel: bool,
    pub scene: bool,
    pub pattern: bool,
    pub events: bool,
    
    // Navigation
    pub variation: bool,
    pub duplicate: bool,
    pub select: bool,
    pub solo: bool,
    pub mute: bool,
    pub pitch: bool,
    pub mod_: bool,
    pub perform: bool,
    
    // Other controls
    pub shift: bool,
    pub encoder_push: bool,
    pub encoder_up: bool,
    pub encoder_down: bool,
    pub encoder_left: bool,
    pub encoder_right: bool,
    
    // Display buttons
    pub display_button_1: bool,
    pub display_button_2: bool,
    pub display_button_3: bool,
    pub display_button_4: bool,
    pub display_button_5: bool,
    pub display_button_6: bool,
    pub display_button_7: bool,
    pub display_button_8: bool,
    
    // System buttons
    pub channel_midi: bool,
    pub arranger: bool,
    pub browser_plugin: bool,
    pub arrow_left: bool,
    pub arrow_right: bool,
    pub file_save: bool,
    pub settings: bool,
    pub macro_: bool,
    pub plugin: bool,
    pub mixer: bool,
    pub sampling: bool,
    pub auto: bool,
    
    // Hardware status
    pub pedal_connected: bool,
    pub microphone_connected: bool,
}

/// Represents the state of all knobs on the Maschine MK3
#[derive(Debug, Clone, Default)]
pub struct KnobState {
    pub knob_1: u16,      // 10-bit resolution (0-1023)
    pub knob_2: u16,
    pub knob_3: u16,
    pub knob_4: u16,
    pub knob_5: u16,
    pub knob_6: u16,
    pub knob_7: u16,
    pub knob_8: u16,
    pub main_encoder: u8,  // 4-bit resolution (0-15)
    
    // Touch detection
    pub knob_1_touched: bool,
    pub knob_2_touched: bool,
    pub knob_3_touched: bool,
    pub knob_4_touched: bool,
    pub knob_5_touched: bool,
    pub knob_6_touched: bool,
    pub knob_7_touched: bool,
    pub knob_8_touched: bool,
    pub main_knob_touched: bool,
}

/// Represents touch strip data
#[derive(Debug, Clone, Default)]
pub struct TouchStripState {
    pub finger_1: TouchData,
    pub finger_2: TouchData,
}

#[derive(Debug, Clone, Default)]
pub struct TouchData {
    pub data_a: u8,
    pub data_b: u8,
    pub data_c: u8,
    pub data_d: u8,
}

/// Represents audio controls
#[derive(Debug, Clone, Default)]
pub struct AudioState {
    pub mic_gain: u16,
    pub headphone_volume: u16,
    pub master_volume: u16,
}

/// Complete input state from Type 0x01 packets (buttons/knobs)
#[derive(Debug, Clone, Default)]
pub struct InputState {
    pub buttons: ButtonState,
    pub knobs: KnobState,
    pub touch_strip: TouchStripState,
    pub audio: AudioState,
}

/// Individual pad hit data
#[derive(Debug, Clone)]
pub struct PadHit {
    pub pad_number: u8,    // 0-15, numbered from top-right to bottom-left
    pub data_a: u8,        // Not yet reverse engineered
    pub data_b: u8,        // Not yet reverse engineered
}

/// Represents pad input from Type 0x02 packets
#[derive(Debug, Clone, Default)]
pub struct PadState {
    pub hits: Vec<PadHit>,
}

impl InputState {
    /// Parse a Type 0x01 packet (42 bytes) into button/knob state
    pub fn from_button_packet(data: &[u8]) -> Result<Self> {
        if data.len() < 42 {
            return Err(MK3Error::InvalidPacket);
        }
        
        if data[0] != 0x01 {
            return Err(MK3Error::InvalidPacket);
        }
        
        let mut state = InputState::default();
        
        // Parse byte 1 - Encoder and system controls
        state.buttons.encoder_push = (data[1] & 0x01) != 0;
        state.buttons.pedal_connected = (data[1] & 0x02) != 0;
        state.buttons.encoder_up = (data[1] & 0x04) != 0;
        state.buttons.encoder_right = (data[1] & 0x08) != 0;
        state.buttons.encoder_down = (data[1] & 0x10) != 0;
        state.buttons.encoder_left = (data[1] & 0x20) != 0;
        state.buttons.shift = (data[1] & 0x40) != 0;
        state.buttons.display_button_8 = (data[1] & 0x80) != 0;
        
        // Parse byte 2 - Group buttons A-H
        state.buttons.group_a = (data[2] & 0x01) != 0;
        state.buttons.group_b = (data[2] & 0x02) != 0;
        state.buttons.group_c = (data[2] & 0x04) != 0;
        state.buttons.group_d = (data[2] & 0x08) != 0;
        state.buttons.group_e = (data[2] & 0x10) != 0;
        state.buttons.group_f = (data[2] & 0x20) != 0;
        state.buttons.group_g = (data[2] & 0x40) != 0;
        state.buttons.group_h = (data[2] & 0x80) != 0;
        
        // Parse byte 3 - Mode buttons
        state.buttons.notes = (data[3] & 0x01) != 0;
        state.buttons.volume = (data[3] & 0x02) != 0;
        state.buttons.swing = (data[3] & 0x04) != 0;
        state.buttons.tempo = (data[3] & 0x08) != 0;
        state.buttons.note_repeat = (data[3] & 0x10) != 0;
        state.buttons.lock = (data[3] & 0x20) != 0;
        
        // Parse byte 4 - More mode buttons
        state.buttons.pad_mode = (data[4] & 0x01) != 0;
        state.buttons.keyboard = (data[4] & 0x02) != 0;
        state.buttons.chords = (data[4] & 0x04) != 0;
        state.buttons.step = (data[4] & 0x08) != 0;
        state.buttons.fixed_vel = (data[4] & 0x10) != 0;
        state.buttons.scene = (data[4] & 0x20) != 0;
        state.buttons.pattern = (data[4] & 0x40) != 0;
        state.buttons.events = (data[4] & 0x80) != 0;
        
        // Parse byte 5 - Control buttons
        state.buttons.microphone_connected = (data[5] & 0x01) != 0;
        state.buttons.variation = (data[5] & 0x02) != 0;
        state.buttons.duplicate = (data[5] & 0x04) != 0;
        state.buttons.select = (data[5] & 0x08) != 0;
        state.buttons.solo = (data[5] & 0x10) != 0;
        state.buttons.mute = (data[5] & 0x20) != 0;
        state.buttons.pitch = (data[5] & 0x40) != 0;
        state.buttons.mod_ = (data[5] & 0x80) != 0;
        
        // Parse byte 6 - Transport controls
        state.buttons.perform = (data[6] & 0x01) != 0;
        state.buttons.restart = (data[6] & 0x02) != 0;
        state.buttons.erase = (data[6] & 0x04) != 0;
        state.buttons.tap = (data[6] & 0x08) != 0;
        state.buttons.follow = (data[6] & 0x10) != 0;
        state.buttons.play = (data[6] & 0x20) != 0;
        state.buttons.rec = (data[6] & 0x40) != 0;
        state.buttons.stop = (data[6] & 0x80) != 0;
        
        // Parse byte 7 - More system buttons
        state.buttons.macro_ = (data[7] & 0x01) != 0;
        state.buttons.settings = (data[7] & 0x02) != 0;
        state.buttons.arrow_right = (data[7] & 0x04) != 0;
        state.buttons.sampling = (data[7] & 0x08) != 0;
        state.buttons.mixer = (data[7] & 0x10) != 0;
        state.buttons.plugin = (data[7] & 0x20) != 0;
        
        // Parse byte 8 - More system buttons
        state.buttons.channel_midi = (data[8] & 0x01) != 0;
        state.buttons.arranger = (data[8] & 0x02) != 0;
        state.buttons.browser_plugin = (data[8] & 0x04) != 0;
        state.buttons.arrow_left = (data[8] & 0x08) != 0;
        state.buttons.file_save = (data[8] & 0x10) != 0;
        state.buttons.auto = (data[8] & 0x20) != 0;
        
        // Parse byte 9 - Display buttons
        state.buttons.display_button_1 = (data[9] & 0x01) != 0;
        state.buttons.display_button_2 = (data[9] & 0x02) != 0;
        state.buttons.display_button_3 = (data[9] & 0x04) != 0;
        state.buttons.display_button_4 = (data[9] & 0x08) != 0;
        state.buttons.display_button_5 = (data[9] & 0x10) != 0;
        state.buttons.display_button_6 = (data[9] & 0x20) != 0;
        state.buttons.display_button_7 = (data[9] & 0x40) != 0;
        state.knobs.main_knob_touched = (data[9] & 0x80) != 0;
        
        // Parse byte 10 - Knob touch detection
        state.knobs.knob_8_touched = (data[10] & 0x01) != 0;
        state.knobs.knob_7_touched = (data[10] & 0x02) != 0;
        state.knobs.knob_6_touched = (data[10] & 0x04) != 0;
        state.knobs.knob_5_touched = (data[10] & 0x08) != 0;
        state.knobs.knob_4_touched = (data[10] & 0x10) != 0;
        state.knobs.knob_3_touched = (data[10] & 0x20) != 0;
        state.knobs.knob_2_touched = (data[10] & 0x40) != 0;
        state.knobs.knob_1_touched = (data[10] & 0x80) != 0;
        
        // Parse byte 11 - Main encoder position (4-bit)
        state.knobs.main_encoder = data[11] & 0x0F;
        
        // Parse knob positions (10-bit each, 2 bytes per knob)
        state.knobs.knob_1 = ((data[13] as u16 & 0x03) << 8) | (data[12] as u16);
        state.knobs.knob_2 = ((data[15] as u16 & 0x03) << 8) | (data[14] as u16);
        state.knobs.knob_3 = ((data[17] as u16 & 0x03) << 8) | (data[16] as u16);
        state.knobs.knob_4 = ((data[19] as u16 & 0x03) << 8) | (data[18] as u16);
        state.knobs.knob_5 = ((data[21] as u16 & 0x03) << 8) | (data[20] as u16);
        state.knobs.knob_6 = ((data[23] as u16 & 0x03) << 8) | (data[22] as u16);
        state.knobs.knob_7 = ((data[25] as u16 & 0x03) << 8) | (data[24] as u16);
        state.knobs.knob_8 = ((data[27] as u16 & 0x03) << 8) | (data[26] as u16);
        
        // Parse touch strip data (bytes 28-35)
        state.touch_strip.finger_1.data_a = data[28];
        state.touch_strip.finger_1.data_b = data[29];
        state.touch_strip.finger_1.data_c = data[30];
        state.touch_strip.finger_1.data_d = data[31];
        state.touch_strip.finger_2.data_a = data[32];
        state.touch_strip.finger_2.data_b = data[33];
        state.touch_strip.finger_2.data_c = data[34];
        state.touch_strip.finger_2.data_d = data[35];
        
        // Parse audio controls (bytes 36-41)
        state.audio.mic_gain = ((data[37] as u16) << 8) | (data[36] as u16);
        state.audio.headphone_volume = ((data[39] as u16) << 8) | (data[38] as u16);
        state.audio.master_volume = ((data[41] as u16) << 8) | (data[40] as u16);
        
        Ok(state)
    }
}

impl PadState {
    /// Parse a Type 0x02 packet (up to 64 bytes) into pad hits
    pub fn from_pad_packet(data: &[u8]) -> Result<Self> {
        if data.is_empty() || data[0] != 0x02 {
            return Err(MK3Error::InvalidPacket);
        }
        
        let mut hits = Vec::new();
        let mut offset = 1;
        
        // Parse pad hits in groups of 3 bytes (pad_number, data_a, data_b)
        while offset + 2 < data.len() {
            let pad_number = data[offset];
            let data_a = data[offset + 1];
            let data_b = data[offset + 2];
            
            // Check if this is a valid pad hit (pad numbers 0-15)
            if pad_number <= 15 {
                hits.push(PadHit {
                    pad_number,
                    data_a,
                    data_b,
                });
            } else {
                // End of pad data
                break;
            }
            
            offset += 3;
        }
        
        Ok(PadState { hits })
    }
}
