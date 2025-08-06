use crate::error::{MK3Error, Result};
use std::collections::HashMap;

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
    pub knob_1: u16, // 10-bit resolution (0-1023)
    pub knob_2: u16,
    pub knob_3: u16,
    pub knob_4: u16,
    pub knob_5: u16,
    pub knob_6: u16,
    pub knob_7: u16,
    pub knob_8: u16,
    pub main_encoder: u8, // 4-bit resolution (0-15)

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

/// Enumeration of all input elements for event-based input
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputElement {
    // Buttons
    Play,
    Rec,
    Stop,
    Restart,
    Erase,
    Tap,
    Follow,
    GroupA,
    GroupB,
    GroupC,
    GroupD,
    GroupE,
    GroupF,
    GroupG,
    GroupH,
    Notes,
    Volume,
    Swing,
    Tempo,
    NoteRepeat,
    Lock,
    PadMode,
    Keyboard,
    Chords,
    Step,
    FixedVel,
    Scene,
    Pattern,
    Events,
    Variation,
    Duplicate,
    Select,
    Solo,
    Mute,
    Pitch,
    Mod,
    Perform,
    Shift,
    EncoderPush,
    EncoderUp,
    EncoderDown,
    EncoderLeft,
    EncoderRight,
    DisplayButton1,
    DisplayButton2,
    DisplayButton3,
    DisplayButton4,
    DisplayButton5,
    DisplayButton6,
    DisplayButton7,
    DisplayButton8,
    ChannelMidi,
    Arranger,
    BrowserPlugin,
    ArrowLeft,
    ArrowRight,
    FileSave,
    Settings,
    Macro,
    Plugin,
    Mixer,
    Sampling,
    Auto,
    PedalConnected,
    MicrophoneConnected,
    // Knobs
    Knob1,
    Knob2,
    Knob3,
    Knob4,
    Knob5,
    Knob6,
    Knob7,
    Knob8,
    MainEncoder,
    // Touch detection
    Knob1Touched,
    Knob2Touched,
    Knob3Touched,
    Knob4Touched,
    Knob5Touched,
    Knob6Touched,
    Knob7Touched,
    Knob8Touched,
    MainKnobTouched,
    // Audio controls
    MicGain,
    HeadphoneVolume,
    MasterVolume,
}

impl InputElement {
    /// Get the display name for this input element
    pub fn name(&self) -> &'static str {
        match self {
            InputElement::Play => "Play",
            InputElement::Rec => "Rec",
            InputElement::Stop => "Stop",
            InputElement::Restart => "Restart",
            InputElement::Erase => "Erase",
            InputElement::Tap => "Tap",
            InputElement::Follow => "Follow",
            InputElement::GroupA => "Group A",
            InputElement::GroupB => "Group B",
            InputElement::GroupC => "Group C",
            InputElement::GroupD => "Group D",
            InputElement::GroupE => "Group E",
            InputElement::GroupF => "Group F",
            InputElement::GroupG => "Group G",
            InputElement::GroupH => "Group H",
            InputElement::Notes => "Notes",
            InputElement::Volume => "Volume",
            InputElement::Swing => "Swing",
            InputElement::Tempo => "Tempo",
            InputElement::NoteRepeat => "Note Repeat",
            InputElement::Lock => "Lock",
            InputElement::PadMode => "Pad Mode",
            InputElement::Keyboard => "Keyboard",
            InputElement::Chords => "Chords",
            InputElement::Step => "Step",
            InputElement::FixedVel => "Fixed Vel",
            InputElement::Scene => "Scene",
            InputElement::Pattern => "Pattern",
            InputElement::Events => "Events",
            InputElement::Variation => "Variation",
            InputElement::Duplicate => "Duplicate",
            InputElement::Select => "Select",
            InputElement::Solo => "Solo",
            InputElement::Mute => "Mute",
            InputElement::Pitch => "Pitch",
            InputElement::Mod => "Mod",
            InputElement::Perform => "Perform",
            InputElement::Shift => "Shift",
            InputElement::EncoderPush => "Encoder Push",
            InputElement::EncoderUp => "Encoder Up",
            InputElement::EncoderDown => "Encoder Down",
            InputElement::EncoderLeft => "Encoder Left",
            InputElement::EncoderRight => "Encoder Right",
            InputElement::DisplayButton1 => "Display 1",
            InputElement::DisplayButton2 => "Display 2",
            InputElement::DisplayButton3 => "Display 3",
            InputElement::DisplayButton4 => "Display 4",
            InputElement::DisplayButton5 => "Display 5",
            InputElement::DisplayButton6 => "Display 6",
            InputElement::DisplayButton7 => "Display 7",
            InputElement::DisplayButton8 => "Display 8",
            InputElement::ChannelMidi => "Channel/MIDI",
            InputElement::Arranger => "Arranger",
            InputElement::BrowserPlugin => "Browser/Plugin",
            InputElement::ArrowLeft => "Arrow Left",
            InputElement::ArrowRight => "Arrow Right",
            InputElement::FileSave => "File/Save",
            InputElement::Settings => "Settings",
            InputElement::Macro => "Macro",
            InputElement::Plugin => "Plugin",
            InputElement::Mixer => "Mixer",
            InputElement::Sampling => "Sampling",
            InputElement::Auto => "Auto",
            InputElement::PedalConnected => "Pedal Connected",
            InputElement::MicrophoneConnected => "Microphone Connected",
            InputElement::Knob1 => "Knob 1",
            InputElement::Knob2 => "Knob 2",
            InputElement::Knob3 => "Knob 3",
            InputElement::Knob4 => "Knob 4",
            InputElement::Knob5 => "Knob 5",
            InputElement::Knob6 => "Knob 6",
            InputElement::Knob7 => "Knob 7",
            InputElement::Knob8 => "Knob 8",
            InputElement::MainEncoder => "Main Encoder",
            InputElement::Knob1Touched => "Knob 1 Touch",
            InputElement::Knob2Touched => "Knob 2 Touch",
            InputElement::Knob3Touched => "Knob 3 Touch",
            InputElement::Knob4Touched => "Knob 4 Touch",
            InputElement::Knob5Touched => "Knob 5 Touch",
            InputElement::Knob6Touched => "Knob 6 Touch",
            InputElement::Knob7Touched => "Knob 7 Touch",
            InputElement::Knob8Touched => "Knob 8 Touch",
            InputElement::MainKnobTouched => "Main Knob Touch",
            InputElement::MicGain => "Mic Gain",
            InputElement::HeadphoneVolume => "Headphone Volume",
            InputElement::MasterVolume => "Master Volume",
        }
    }

    pub fn has_color(&self) -> bool {
        match self {
            InputElement::GroupA => true,
            InputElement::GroupB => true,
            InputElement::GroupC => true,
            InputElement::GroupD => true,
            InputElement::GroupE => true,
            InputElement::GroupF => true,
            InputElement::GroupG => true,
            InputElement::GroupH => true,
            InputElement::BrowserPlugin => true,
            InputElement::EncoderUp => true,
            InputElement::EncoderLeft => true,
            InputElement::EncoderRight => true,
            InputElement::EncoderDown => true,
            _ => false,
        }
    }
}

/// Input event types
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    ButtonPressed(InputElement),
    ButtonReleased(InputElement),
    ButtonHeld(InputElement),
    KnobChanged {
        element: InputElement,
        value: u16,
        delta: i32,
    },
    AudioChanged {
        element: InputElement,
        value: u16,
        delta: i32,
    },
    PadHit {
        pad_number: u8,
        velocity: u8,
        pressure: u8,
    },
}

/// Input change tracker for delta detection
#[derive(Debug, Clone)]
pub struct InputTracker {
    previous_state: Option<InputState>,
    held_buttons: HashMap<InputElement, u32>, // frame counter for held buttons
    frame_count: u32,
    is_first_update: bool,
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
    pub pad_number: u8, // 0-15, numbered from top-right to bottom-left
    pub data_a: u8,     // Not yet reverse engineered
    pub data_b: u8,     // Not yet reverse engineered
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

    /// Get the value of a specific button
    pub fn get_button(&self, element: &InputElement) -> bool {
        match element {
            InputElement::Play => self.buttons.play,
            InputElement::Rec => self.buttons.rec,
            InputElement::Stop => self.buttons.stop,
            InputElement::Restart => self.buttons.restart,
            InputElement::Erase => self.buttons.erase,
            InputElement::Tap => self.buttons.tap,
            InputElement::Follow => self.buttons.follow,
            InputElement::GroupA => self.buttons.group_a,
            InputElement::GroupB => self.buttons.group_b,
            InputElement::GroupC => self.buttons.group_c,
            InputElement::GroupD => self.buttons.group_d,
            InputElement::GroupE => self.buttons.group_e,
            InputElement::GroupF => self.buttons.group_f,
            InputElement::GroupG => self.buttons.group_g,
            InputElement::GroupH => self.buttons.group_h,
            InputElement::Notes => self.buttons.notes,
            InputElement::Volume => self.buttons.volume,
            InputElement::Swing => self.buttons.swing,
            InputElement::Tempo => self.buttons.tempo,
            InputElement::NoteRepeat => self.buttons.note_repeat,
            InputElement::Lock => self.buttons.lock,
            InputElement::PadMode => self.buttons.pad_mode,
            InputElement::Keyboard => self.buttons.keyboard,
            InputElement::Chords => self.buttons.chords,
            InputElement::Step => self.buttons.step,
            InputElement::FixedVel => self.buttons.fixed_vel,
            InputElement::Scene => self.buttons.scene,
            InputElement::Pattern => self.buttons.pattern,
            InputElement::Events => self.buttons.events,
            InputElement::Variation => self.buttons.variation,
            InputElement::Duplicate => self.buttons.duplicate,
            InputElement::Select => self.buttons.select,
            InputElement::Solo => self.buttons.solo,
            InputElement::Mute => self.buttons.mute,
            InputElement::Pitch => self.buttons.pitch,
            InputElement::Mod => self.buttons.mod_,
            InputElement::Perform => self.buttons.perform,
            InputElement::Shift => self.buttons.shift,
            InputElement::EncoderPush => self.buttons.encoder_push,
            InputElement::EncoderUp => self.buttons.encoder_up,
            InputElement::EncoderDown => self.buttons.encoder_down,
            InputElement::EncoderLeft => self.buttons.encoder_left,
            InputElement::EncoderRight => self.buttons.encoder_right,
            InputElement::DisplayButton1 => self.buttons.display_button_1,
            InputElement::DisplayButton2 => self.buttons.display_button_2,
            InputElement::DisplayButton3 => self.buttons.display_button_3,
            InputElement::DisplayButton4 => self.buttons.display_button_4,
            InputElement::DisplayButton5 => self.buttons.display_button_5,
            InputElement::DisplayButton6 => self.buttons.display_button_6,
            InputElement::DisplayButton7 => self.buttons.display_button_7,
            InputElement::DisplayButton8 => self.buttons.display_button_8,
            InputElement::ChannelMidi => self.buttons.channel_midi,
            InputElement::Arranger => self.buttons.arranger,
            InputElement::BrowserPlugin => self.buttons.browser_plugin,
            InputElement::ArrowLeft => self.buttons.arrow_left,
            InputElement::ArrowRight => self.buttons.arrow_right,
            InputElement::FileSave => self.buttons.file_save,
            InputElement::Settings => self.buttons.settings,
            InputElement::Macro => self.buttons.macro_,
            InputElement::Plugin => self.buttons.plugin,
            InputElement::Mixer => self.buttons.mixer,
            InputElement::Sampling => self.buttons.sampling,
            InputElement::Auto => self.buttons.auto,
            InputElement::PedalConnected => self.buttons.pedal_connected,
            InputElement::MicrophoneConnected => self.buttons.microphone_connected,
            InputElement::Knob1Touched => self.knobs.knob_1_touched,
            InputElement::Knob2Touched => self.knobs.knob_2_touched,
            InputElement::Knob3Touched => self.knobs.knob_3_touched,
            InputElement::Knob4Touched => self.knobs.knob_4_touched,
            InputElement::Knob5Touched => self.knobs.knob_5_touched,
            InputElement::Knob6Touched => self.knobs.knob_6_touched,
            InputElement::Knob7Touched => self.knobs.knob_7_touched,
            InputElement::Knob8Touched => self.knobs.knob_8_touched,
            InputElement::MainKnobTouched => self.knobs.main_knob_touched,
            _ => false, // Non-button elements
        }
    }

    /// Get the value of a specific knob or audio control
    pub fn get_value(&self, element: &InputElement) -> u16 {
        match element {
            InputElement::Knob1 => self.knobs.knob_1,
            InputElement::Knob2 => self.knobs.knob_2,
            InputElement::Knob3 => self.knobs.knob_3,
            InputElement::Knob4 => self.knobs.knob_4,
            InputElement::Knob5 => self.knobs.knob_5,
            InputElement::Knob6 => self.knobs.knob_6,
            InputElement::Knob7 => self.knobs.knob_7,
            InputElement::Knob8 => self.knobs.knob_8,
            InputElement::MainEncoder => self.knobs.main_encoder as u16,
            InputElement::MicGain => self.audio.mic_gain,
            InputElement::HeadphoneVolume => self.audio.headphone_volume,
            InputElement::MasterVolume => self.audio.master_volume,
            _ => 0, // Non-value elements
        }
    }

    /// Get all currently active (pressed) buttons
    pub fn get_active_buttons(&self) -> Vec<InputElement> {
        let all_buttons = [
            InputElement::Play,
            InputElement::Rec,
            InputElement::Stop,
            InputElement::Restart,
            InputElement::Erase,
            InputElement::Tap,
            InputElement::Follow,
            InputElement::GroupA,
            InputElement::GroupB,
            InputElement::GroupC,
            InputElement::GroupD,
            InputElement::GroupE,
            InputElement::GroupF,
            InputElement::GroupG,
            InputElement::GroupH,
            InputElement::Notes,
            InputElement::Volume,
            InputElement::Swing,
            InputElement::Tempo,
            InputElement::NoteRepeat,
            InputElement::Lock,
            InputElement::PadMode,
            InputElement::Keyboard,
            InputElement::Chords,
            InputElement::Step,
            InputElement::FixedVel,
            InputElement::Scene,
            InputElement::Pattern,
            InputElement::Events,
            InputElement::Variation,
            InputElement::Duplicate,
            InputElement::Select,
            InputElement::Solo,
            InputElement::Mute,
            InputElement::Pitch,
            InputElement::Mod,
            InputElement::Perform,
            InputElement::Shift,
            InputElement::EncoderPush,
            InputElement::EncoderUp,
            InputElement::EncoderDown,
            InputElement::EncoderLeft,
            InputElement::EncoderRight,
            InputElement::DisplayButton1,
            InputElement::DisplayButton2,
            InputElement::DisplayButton3,
            InputElement::DisplayButton4,
            InputElement::DisplayButton5,
            InputElement::DisplayButton6,
            InputElement::DisplayButton7,
            InputElement::DisplayButton8,
            InputElement::ChannelMidi,
            InputElement::Arranger,
            InputElement::BrowserPlugin,
            InputElement::ArrowLeft,
            InputElement::ArrowRight,
            InputElement::FileSave,
            InputElement::Settings,
            InputElement::Macro,
            InputElement::Plugin,
            InputElement::Mixer,
            InputElement::Sampling,
            InputElement::Auto,
            InputElement::PedalConnected,
            InputElement::MicrophoneConnected,
            InputElement::Knob1Touched,
            InputElement::Knob2Touched,
            InputElement::Knob3Touched,
            InputElement::Knob4Touched,
            InputElement::Knob5Touched,
            InputElement::Knob6Touched,
            InputElement::Knob7Touched,
            InputElement::Knob8Touched,
            InputElement::MainKnobTouched,
        ];

        all_buttons
            .into_iter()
            .filter(|element| self.get_button(element))
            .collect()
    }

    /// Get all currently touched knobs with their values
    pub fn get_active_knobs(&self) -> Vec<(InputElement, u16)> {
        let knob_touch_pairs = [
            (InputElement::Knob1, InputElement::Knob1Touched),
            (InputElement::Knob2, InputElement::Knob2Touched),
            (InputElement::Knob3, InputElement::Knob3Touched),
            (InputElement::Knob4, InputElement::Knob4Touched),
            (InputElement::Knob5, InputElement::Knob5Touched),
            (InputElement::Knob6, InputElement::Knob6Touched),
            (InputElement::Knob7, InputElement::Knob7Touched),
            (InputElement::Knob8, InputElement::Knob8Touched),
            (InputElement::MainEncoder, InputElement::MainKnobTouched),
        ];

        knob_touch_pairs
            .into_iter()
            .filter(|(_, touch_element)| self.get_button(touch_element))
            .map(|(knob_element, _)| (knob_element.clone(), self.get_value(&knob_element)))
            .collect()
    }

    /// Get all non-zero audio control values with their elements
    pub fn get_active_audio(&self) -> Vec<(InputElement, u16)> {
        let audio_elements = [
            InputElement::MicGain,
            InputElement::HeadphoneVolume,
            InputElement::MasterVolume,
        ];

        audio_elements
            .into_iter()
            .map(|element| (element.clone(), self.get_value(&element)))
            .filter(|(_, value)| *value > 0)
            .collect()
    }

    /// Get touch strip data if any finger is active
    pub fn get_touch_strip_data(&self) -> Option<((u8, u8, u8, u8), (u8, u8, u8, u8))> {
        if self.touch_strip.finger_1.data_a > 0 || self.touch_strip.finger_2.data_a > 0 {
            Some((
                (
                    self.touch_strip.finger_1.data_a,
                    self.touch_strip.finger_1.data_b,
                    self.touch_strip.finger_1.data_c,
                    self.touch_strip.finger_1.data_d,
                ),
                (
                    self.touch_strip.finger_2.data_a,
                    self.touch_strip.finger_2.data_b,
                    self.touch_strip.finger_2.data_c,
                    self.touch_strip.finger_2.data_d,
                ),
            ))
        } else {
            None
        }
    }
}

impl InputTracker {
    pub fn new() -> Self {
        Self {
            previous_state: None,
            held_buttons: HashMap::new(),
            frame_count: 0,
            is_first_update: true,
        }
    }

    /// Update the tracker with a new input state and return all events
    pub fn update(&mut self, current_state: InputState) -> Vec<InputEvent> {
        let mut events = Vec::new();
        self.frame_count += 1;

        let prev_state = self.previous_state.take().unwrap_or_default();

        // Check button events
        Self::check_button_events_static(
            &mut events,
            &prev_state,
            &current_state,
            &mut self.held_buttons,
            self.frame_count,
        );

        // Check knob/value events - but skip on first update to avoid spurious events from initial hardware state
        if !self.is_first_update {
            Self::check_value_events_static(&mut events, &prev_state, &current_state);
        }

        self.previous_state = Some(current_state);
        self.is_first_update = false;
        events
    }

    /// Update the tracker with pad events and return them as InputEvents
    pub fn update_pads(&mut self, pad_state: PadState) -> Vec<InputEvent> {
        pad_state
            .hits
            .into_iter()
            .map(|hit| InputEvent::PadHit {
                pad_number: hit.pad_number,
                velocity: hit.data_a,
                pressure: hit.data_b,
            })
            .collect()
    }
}

impl InputEvent {
    /// Get a human-readable description of this input event
    pub fn description(&self) -> String {
        match self {
            InputEvent::ButtonPressed(element) => format!("{} pressed", element.name()),
            InputEvent::ButtonReleased(element) => format!("{} released", element.name()),
            InputEvent::ButtonHeld(element) => format!("{} held", element.name()),
            InputEvent::KnobChanged {
                element,
                value,
                delta,
            } => {
                format!("{} → {} (Δ{})", element.name(), value, delta)
            }
            InputEvent::AudioChanged {
                element,
                value,
                delta,
            } => {
                format!("{} → {} (Δ{})", element.name(), value, delta)
            }
            InputEvent::PadHit {
                pad_number,
                velocity,
                pressure,
            } => {
                format!(
                    "Pad {} ({}) - velocity:{} pressure:{}",
                    pad_number + 1,
                    Self::get_pad_position(*pad_number),
                    velocity,
                    pressure
                )
            }
        }
    }

    /// Get the grid position name for a pad number (0-15)
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
}

impl InputTracker {
    fn check_button_events_static(
        events: &mut Vec<InputEvent>,
        prev: &InputState,
        current: &InputState,
        held_buttons: &mut HashMap<InputElement, u32>,
        frame_count: u32,
    ) {
        let button_elements = [
            InputElement::Play,
            InputElement::Rec,
            InputElement::Stop,
            InputElement::Restart,
            InputElement::Erase,
            InputElement::Tap,
            InputElement::Follow,
            InputElement::GroupA,
            InputElement::GroupB,
            InputElement::GroupC,
            InputElement::GroupD,
            InputElement::GroupE,
            InputElement::GroupF,
            InputElement::GroupG,
            InputElement::GroupH,
            InputElement::Notes,
            InputElement::Volume,
            InputElement::Swing,
            InputElement::Tempo,
            InputElement::NoteRepeat,
            InputElement::Lock,
            InputElement::PadMode,
            InputElement::Keyboard,
            InputElement::Chords,
            InputElement::Step,
            InputElement::FixedVel,
            InputElement::Scene,
            InputElement::Pattern,
            InputElement::Events,
            InputElement::Variation,
            InputElement::Duplicate,
            InputElement::Select,
            InputElement::Solo,
            InputElement::Mute,
            InputElement::Pitch,
            InputElement::Mod,
            InputElement::Perform,
            InputElement::Shift,
            InputElement::EncoderPush,
            InputElement::EncoderUp,
            InputElement::EncoderDown,
            InputElement::EncoderLeft,
            InputElement::EncoderRight,
            InputElement::DisplayButton1,
            InputElement::DisplayButton2,
            InputElement::DisplayButton3,
            InputElement::DisplayButton4,
            InputElement::DisplayButton5,
            InputElement::DisplayButton6,
            InputElement::DisplayButton7,
            InputElement::DisplayButton8,
            InputElement::ChannelMidi,
            InputElement::Arranger,
            InputElement::BrowserPlugin,
            InputElement::ArrowLeft,
            InputElement::ArrowRight,
            InputElement::FileSave,
            InputElement::Settings,
            InputElement::Macro,
            InputElement::Plugin,
            InputElement::Mixer,
            InputElement::Sampling,
            InputElement::Auto,
            InputElement::PedalConnected,
            InputElement::MicrophoneConnected,
            InputElement::Knob1Touched,
            InputElement::Knob2Touched,
            InputElement::Knob3Touched,
            InputElement::Knob4Touched,
            InputElement::Knob5Touched,
            InputElement::Knob6Touched,
            InputElement::Knob7Touched,
            InputElement::Knob8Touched,
            InputElement::MainKnobTouched,
        ];

        for element in &button_elements {
            let prev_pressed = prev.get_button(element);
            let current_pressed = current.get_button(element);

            match (prev_pressed, current_pressed) {
                (false, true) => {
                    events.push(InputEvent::ButtonPressed(element.clone()));
                    held_buttons.insert(element.clone(), frame_count);
                }
                (true, false) => {
                    events.push(InputEvent::ButtonReleased(element.clone()));
                    held_buttons.remove(element);
                }
                (true, true) => {
                    if let Some(held_since) = held_buttons.get(element) {
                        if frame_count - held_since > 30 {
                            // ~0.5 seconds at 60fps
                            events.push(InputEvent::ButtonHeld(element.clone()));
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn check_value_events_static(
        events: &mut Vec<InputEvent>,
        prev: &InputState,
        current: &InputState,
    ) {
        let knob_elements = [
            InputElement::Knob1,
            InputElement::Knob2,
            InputElement::Knob3,
            InputElement::Knob4,
            InputElement::Knob5,
            InputElement::Knob6,
            InputElement::Knob7,
            InputElement::Knob8,
            InputElement::MainEncoder,
        ];

        let audio_elements = [
            InputElement::MicGain,
            InputElement::HeadphoneVolume,
            InputElement::MasterVolume,
        ];

        for element in &knob_elements {
            let prev_value = prev.get_value(element);
            let current_value = current.get_value(element);

            if prev_value != current_value {
                let delta = current_value as i32 - prev_value as i32;
                events.push(InputEvent::KnobChanged {
                    element: element.clone(),
                    value: current_value,
                    delta,
                });
            }
        }

        for element in &audio_elements {
            let prev_value = prev.get_value(element);
            let current_value = current.get_value(element);

            if prev_value != current_value {
                let delta = current_value as i32 - prev_value as i32;
                events.push(InputEvent::AudioChanged {
                    element: element.clone(),
                    value: current_value,
                    delta,
                });
            }
        }
    }

    /// Check if a button was just pressed this frame
    pub fn was_pressed(&self, element: &InputElement) -> bool {
        if let Some(ref current) = self.previous_state {
            current.get_button(element) && !self.held_buttons.contains_key(element)
        } else {
            false
        }
    }

    /// Check if a button is currently held
    pub fn is_held(&self, element: &InputElement) -> bool {
        self.held_buttons.contains_key(element)
    }

    /// Check if a button was just released this frame
    pub fn was_released(&self, element: &InputElement) -> bool {
        if let Some(ref current) = self.previous_state {
            !current.get_button(element) && self.held_buttons.contains_key(element)
        } else {
            false
        }
    }
}

impl Default for InputTracker {
    fn default() -> Self {
        Self::new()
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

            // Skip empty/padding entries (all zeros)
            if pad_number == 0 && data_a == 0 && data_b == 0 {
                offset += 3;
                continue;
            }

            if pad_number == 0 && data_a == 0 && data_b == 0 {
                offset += 3;
                continue;
            }

            //Debug the data thats coming in for pad hits, show everything coming in
            //Show in binary, pad the bits out to 8 bits
            println!(
                "Pad Hit: {:08b}, {:08b}, {:08b}",
                pad_number, data_a, data_b
            );

            //pad_number = pad_number.saturating_sub(1);
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
