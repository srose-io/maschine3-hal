//! FFI (Foreign Function Interface) layer for Unity and other C-compatible environments.
//!
//! This module provides C-compatible exports that can be called from Unity via P/Invoke.
//! All functions use `extern "C"` calling convention and return error codes instead of Results.

use crate::{MaschineMK3, MaschineLEDColor, InputEvent, PadEventType, InputElement};
use std::os::raw::{c_int, c_uint, c_ushort};
use std::ptr;
use std::slice;

// Error codes for FFI
pub const MK3_SUCCESS: c_int = 0;
pub const MK3_ERROR_NULL_POINTER: c_int = -1;
pub const MK3_ERROR_DEVICE_NOT_FOUND: c_int = -2;
pub const MK3_ERROR_USB_ERROR: c_int = -3;
pub const MK3_ERROR_TIMEOUT: c_int = -4;
pub const MK3_ERROR_COMMUNICATION: c_int = -5;
pub const MK3_ERROR_INVALID_PARAMETER: c_int = -6;
pub const MK3_ERROR_NO_EVENTS: c_int = -7;

// Event types for Unity
#[repr(C)]
pub enum EventType {
    ButtonPressed = 0,
    ButtonReleased = 1,
    ButtonHeld = 2,
    KnobChanged = 3,
    AudioChanged = 4,
    PadEvent = 5,
}

// Pad event types
#[repr(C)]
pub enum PadEvent {
    Hit = 0,
    TouchRelease = 1,
    HitRelease = 2,
    Aftertouch = 3,
}

// C-compatible input event structure
#[repr(C)]
pub struct CInputEvent {
    pub event_type: EventType,
    pub element_id: c_int,      // Button/Knob/Pad ID
    pub value: c_ushort,         // Velocity/Knob value/Pressure
    pub delta: c_int,            // For knobs/audio
    pub pad_event_type: PadEvent, // Only for pad events
}

// C-compatible RGB color structure
#[repr(C)]
pub struct CRgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Creates a new Maschine MK3 device instance.
///
/// # Returns
/// - A pointer to the device instance on success
/// - NULL on failure
///
/// # Safety
/// The caller must call `mk3_free()` to release the device when done.
#[no_mangle]
pub extern "C" fn mk3_new() -> *mut MaschineMK3 {
    match MaschineMK3::new() {
        Ok(device) => Box::into_raw(Box::new(device)),
        Err(_) => ptr::null_mut(),
    }
}

/// Frees a Maschine MK3 device instance.
///
/// # Safety
/// - `device` must be a valid pointer returned by `mk3_new()`
/// - `device` must not be used after this call
/// - Calling this function twice on the same pointer is undefined behavior
#[no_mangle]
pub unsafe extern "C" fn mk3_free(device: *mut MaschineMK3) {
    if !device.is_null() {
        drop(Box::from_raw(device));
    }
}

/// Polls for input events from the device (standard timeout: 100ms).
///
/// # Parameters
/// - `device`: Device instance pointer
/// - `events_out`: Output buffer for events
/// - `max_events`: Maximum number of events to read
/// - `events_read`: Output parameter for number of events actually read
///
/// # Returns
/// - `MK3_SUCCESS` on success
/// - `MK3_ERROR_NULL_POINTER` if any pointer is NULL
/// - `MK3_ERROR_COMMUNICATION` on USB communication error
/// - `MK3_ERROR_NO_EVENTS` if no events available
///
/// # Safety
/// - `device` must be a valid device pointer
/// - `events_out` must be a valid buffer of at least `max_events` size
/// - `events_read` must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn mk3_poll_events(
    device: *mut MaschineMK3,
    events_out: *mut CInputEvent,
    max_events: c_uint,
    events_read: *mut c_uint,
) -> c_int {
    if device.is_null() || events_out.is_null() || events_read.is_null() {
        return MK3_ERROR_NULL_POINTER;
    }

    let device = &mut *device;
    let events_buffer = slice::from_raw_parts_mut(events_out, max_events as usize);

    match device.poll_input_events() {
        Ok(events) => {
            let count = events.len().min(max_events as usize);
            *events_read = count as c_uint;

            if count == 0 {
                return MK3_ERROR_NO_EVENTS;
            }

            for (i, event) in events.iter().take(count).enumerate() {
                events_buffer[i] = convert_input_event(event);
            }

            MK3_SUCCESS
        }
        Err(_) => MK3_ERROR_COMMUNICATION,
    }
}

/// Fast poll for input events with minimal timeout (1ms).
/// Recommended for game loops and real-time applications like Unity.
///
/// # Parameters
/// - `device`: Device instance pointer
/// - `events_out`: Output buffer for events
/// - `max_events`: Maximum number of events to read
/// - `events_read`: Output parameter for number of events actually read
///
/// # Returns
/// - `MK3_SUCCESS` on success
/// - `MK3_ERROR_NULL_POINTER` if any pointer is NULL
/// - `MK3_ERROR_COMMUNICATION` on USB communication error
/// - `MK3_ERROR_NO_EVENTS` if no events available
///
/// # Safety
/// - `device` must be a valid device pointer
/// - `events_out` must be a valid buffer of at least `max_events` size
/// - `events_read` must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn mk3_poll_events_fast(
    device: *mut MaschineMK3,
    events_out: *mut CInputEvent,
    max_events: c_uint,
    events_read: *mut c_uint,
) -> c_int {
    if device.is_null() || events_out.is_null() || events_read.is_null() {
        return MK3_ERROR_NULL_POINTER;
    }

    let device = &mut *device;
    let events_buffer = slice::from_raw_parts_mut(events_out, max_events as usize);

    match device.poll_input_events_fast() {
        Ok(events) => {
            let count = events.len().min(max_events as usize);
            *events_read = count as c_uint;

            if count == 0 {
                return MK3_ERROR_NO_EVENTS;
            }

            for (i, event) in events.iter().take(count).enumerate() {
                events_buffer[i] = convert_input_event(event);
            }

            MK3_SUCCESS
        }
        Err(_) => MK3_ERROR_COMMUNICATION,
    }
}

/// Sets the color of a pad LED.
///
/// # Parameters
/// - `device`: Device instance pointer
/// - `pad_number`: Pad number (0-15)
/// - `color`: RGB color structure
/// - `bright`: Brightness flag (1 = bright, 0 = dim)
///
/// # Returns
/// - `MK3_SUCCESS` on success
/// - `MK3_ERROR_NULL_POINTER` if device is NULL
/// - `MK3_ERROR_INVALID_PARAMETER` if pad_number is out of range
/// - `MK3_ERROR_COMMUNICATION` on USB communication error
///
/// # Safety
/// - `device` must be a valid device pointer
#[no_mangle]
pub unsafe extern "C" fn mk3_set_pad_led(
    device: *mut MaschineMK3,
    pad_number: u8,
    color: CRgbColor,
    _bright: c_int,
) -> c_int {
    if device.is_null() {
        return MK3_ERROR_NULL_POINTER;
    }

    if pad_number >= 16 {
        return MK3_ERROR_INVALID_PARAMETER;
    }

    let device = &mut *device;
    let led_color = MaschineLEDColor::from_rgb(color.r, color.g, color.b);

    match device.set_pad_led(pad_number, led_color) {
        Ok(_) => MK3_SUCCESS,
        Err(_) => MK3_ERROR_COMMUNICATION,
    }
}

/// Sets the color of a button LED.
///
/// # Parameters
/// - `device`: Device instance pointer
/// - `button_id`: Button element ID (see InputElement enum mapping)
/// - `color`: RGB color structure
/// - `bright`: Brightness flag (1 = bright, 0 = dim)
///
/// # Returns
/// - `MK3_SUCCESS` on success
/// - `MK3_ERROR_NULL_POINTER` if device is NULL
/// - `MK3_ERROR_INVALID_PARAMETER` if button_id is invalid
/// - `MK3_ERROR_COMMUNICATION` on USB communication error
///
/// # Safety
/// - `device` must be a valid device pointer
#[no_mangle]
pub unsafe extern "C" fn mk3_set_button_led(
    device: *mut MaschineMK3,
    button_id: c_int,
    color: CRgbColor,
    _bright: c_int,
) -> c_int {
    if device.is_null() {
        return MK3_ERROR_NULL_POINTER;
    }

    let device = &mut *device;
    let led_color = MaschineLEDColor::from_rgb(color.r, color.g, color.b);

    // Convert button_id to InputElement
    let element = match convert_button_id(button_id) {
        Some(e) => e,
        None => return MK3_ERROR_INVALID_PARAMETER,
    };

    match device.set_button_led_color(element, led_color) {
        Ok(_) => MK3_SUCCESS,
        Err(_) => MK3_ERROR_COMMUNICATION,
    }
}

/// Writes a frame buffer to the device display.
///
/// # Parameters
/// - `device`: Device instance pointer
/// - `display_id`: Display ID (0 = Left display, 1 = Right display)
/// - `rgb565_data`: Pointer to RGB565 framebuffer data (480x272 pixels = 261,120 bytes)
/// - `data_len`: Length of the data buffer (should be 261120)
///
/// # Returns
/// - `MK3_SUCCESS` on success
/// - `MK3_ERROR_NULL_POINTER` if device or rgb565_data is NULL
/// - `MK3_ERROR_INVALID_PARAMETER` if data_len is incorrect or display_id > 1
/// - `MK3_ERROR_COMMUNICATION` on USB communication error
///
/// # Safety
/// - `device` must be a valid device pointer
/// - `rgb565_data` must point to valid RGB565 data of the specified length
#[no_mangle]
pub unsafe extern "C" fn mk3_write_display(
    device: *mut MaschineMK3,
    display_id: c_uint,
    rgb565_data: *const u8,
    data_len: c_uint,
) -> c_int {
    if device.is_null() || rgb565_data.is_null() {
        return MK3_ERROR_NULL_POINTER;
    }

    if display_id > 1 {
        return MK3_ERROR_INVALID_PARAMETER;
    }

    const EXPECTED_SIZE: usize = 480 * 272 * 2;
    if data_len as usize != EXPECTED_SIZE {
        return MK3_ERROR_INVALID_PARAMETER;
    }

    let device = &mut *device;
    let data = slice::from_raw_parts(rgb565_data, data_len as usize);

    match device.write_display_framebuffer(display_id as u8, data) {
        Ok(_) => MK3_SUCCESS,
        Err(_) => MK3_ERROR_COMMUNICATION,
    }
}

/// Flushes pending LED state changes to the device.
///
/// # Parameters
/// - `device`: Device instance pointer
///
/// # Returns
/// - `MK3_SUCCESS` on success
/// - `MK3_ERROR_NULL_POINTER` if device is NULL
/// - `MK3_ERROR_COMMUNICATION` on USB communication error
///
/// # Safety
/// - `device` must be a valid device pointer
#[no_mangle]
pub unsafe extern "C" fn mk3_flush_leds(device: *mut MaschineMK3) -> c_int {
    if device.is_null() {
        return MK3_ERROR_NULL_POINTER;
    }

    let device = &mut *device;

    match device.flush_led_changes() {
        Ok(_) => MK3_SUCCESS,
        Err(_) => MK3_ERROR_COMMUNICATION,
    }
}

/// Check if the display interface is available for writing.
///
/// # Parameters
/// - `device`: Device instance pointer
///
/// # Returns
/// - 1 if display is available
/// - 0 if display is not available
/// - -1 if device pointer is NULL
///
/// # Safety
/// - `device` must be a valid device pointer
#[no_mangle]
pub unsafe extern "C" fn mk3_is_display_available(device: *const MaschineMK3) -> c_int {
    if device.is_null() {
        return -1;
    }

    let device = &*device;

    if device.is_display_available() {
        1
    } else {
        0
    }
}

// Helper function to convert Rust InputEvent to C-compatible structure
fn convert_input_event(event: &InputEvent) -> CInputEvent {
    match event {
        InputEvent::ButtonPressed(element) => CInputEvent {
            event_type: EventType::ButtonPressed,
            element_id: input_element_to_id(element),
            value: 0,
            delta: 0,
            pad_event_type: PadEvent::Hit,
        },
        InputEvent::ButtonReleased(element) => CInputEvent {
            event_type: EventType::ButtonReleased,
            element_id: input_element_to_id(element),
            value: 0,
            delta: 0,
            pad_event_type: PadEvent::Hit,
        },
        InputEvent::ButtonHeld(element) => CInputEvent {
            event_type: EventType::ButtonHeld,
            element_id: input_element_to_id(element),
            value: 0,
            delta: 0,
            pad_event_type: PadEvent::Hit,
        },
        InputEvent::KnobChanged { element, value, delta } => CInputEvent {
            event_type: EventType::KnobChanged,
            element_id: input_element_to_id(element),
            value: *value,
            delta: *delta,
            pad_event_type: PadEvent::Hit,
        },
        InputEvent::AudioChanged { element, value, delta } => CInputEvent {
            event_type: EventType::AudioChanged,
            element_id: input_element_to_id(element),
            value: *value,
            delta: *delta,
            pad_event_type: PadEvent::Hit,
        },
        InputEvent::PadEvent { pad_number, event_type, value } => CInputEvent {
            event_type: EventType::PadEvent,
            element_id: *pad_number as c_int,
            value: *value,
            delta: 0,
            pad_event_type: match event_type {
                PadEventType::Hit => PadEvent::Hit,
                PadEventType::TouchRelease => PadEvent::TouchRelease,
                PadEventType::HitRelease => PadEvent::HitRelease,
                PadEventType::Aftertouch => PadEvent::Aftertouch,
            },
        },
    }
}

// Maps InputElement to a unique integer ID for FFI
fn input_element_to_id(element: &InputElement) -> c_int {
    match element {
        // Transport (0-6)
        InputElement::Play => 0,
        InputElement::Rec => 1,
        InputElement::Stop => 2,
        InputElement::Restart => 3,
        InputElement::Erase => 4,
        InputElement::Tap => 5,
        InputElement::Follow => 6,

        // Groups (7-14)
        InputElement::GroupA => 7,
        InputElement::GroupB => 8,
        InputElement::GroupC => 9,
        InputElement::GroupD => 10,
        InputElement::GroupE => 11,
        InputElement::GroupF => 12,
        InputElement::GroupG => 13,
        InputElement::GroupH => 14,

        // Knobs (15-23)
        InputElement::Knob1 => 15,
        InputElement::Knob2 => 16,
        InputElement::Knob3 => 17,
        InputElement::Knob4 => 18,
        InputElement::Knob5 => 19,
        InputElement::Knob6 => 20,
        InputElement::Knob7 => 21,
        InputElement::Knob8 => 22,
        InputElement::MainEncoder => 23,

        // Audio controls (24-26)
        InputElement::MicGain => 24,
        InputElement::HeadphoneVolume => 25,
        InputElement::MasterVolume => 26,

        // Mode buttons (27-40)
        InputElement::Notes => 27,
        InputElement::Volume => 28,
        InputElement::Swing => 29,
        InputElement::Tempo => 30,
        InputElement::NoteRepeat => 31,
        InputElement::Lock => 32,
        InputElement::PadMode => 33,
        InputElement::Keyboard => 34,
        InputElement::Chords => 35,
        InputElement::Step => 36,
        InputElement::FixedVel => 37,
        InputElement::Scene => 38,
        InputElement::Pattern => 39,
        InputElement::Events => 40,

        // Navigation (41-48)
        InputElement::Variation => 41,
        InputElement::Duplicate => 42,
        InputElement::Select => 43,
        InputElement::Solo => 44,
        InputElement::Mute => 45,
        InputElement::Pitch => 46,
        InputElement::Mod => 47,
        InputElement::Perform => 48,

        // Display buttons (49-56)
        InputElement::DisplayButton1 => 49,
        InputElement::DisplayButton2 => 50,
        InputElement::DisplayButton3 => 51,
        InputElement::DisplayButton4 => 52,
        InputElement::DisplayButton5 => 53,
        InputElement::DisplayButton6 => 54,
        InputElement::DisplayButton7 => 55,
        InputElement::DisplayButton8 => 56,

        // System buttons (57-68)
        InputElement::ChannelMidi => 57,
        InputElement::Arranger => 58,
        InputElement::BrowserPlugin => 59,
        InputElement::ArrowLeft => 60,
        InputElement::ArrowRight => 61,
        InputElement::FileSave => 62,
        InputElement::Settings => 63,
        InputElement::Macro => 64,
        InputElement::Plugin => 65,
        InputElement::Mixer => 66,
        InputElement::Sampling => 67,
        InputElement::Auto => 68,

        // Encoder directions (69-72)
        InputElement::EncoderPush => 69,
        InputElement::EncoderUp => 70,
        InputElement::EncoderDown => 71,
        InputElement::EncoderLeft => 72,
        InputElement::EncoderRight => 73,

        // Other (74-75)
        InputElement::Shift => 74,
        _ => 999, // Unknown
    }
}

// Converts button ID back to InputElement
fn convert_button_id(id: c_int) -> Option<InputElement> {
    match id {
        0 => Some(InputElement::Play),
        1 => Some(InputElement::Rec),
        2 => Some(InputElement::Stop),
        3 => Some(InputElement::Restart),
        4 => Some(InputElement::Erase),
        5 => Some(InputElement::Tap),
        6 => Some(InputElement::Follow),
        7 => Some(InputElement::GroupA),
        8 => Some(InputElement::GroupB),
        9 => Some(InputElement::GroupC),
        10 => Some(InputElement::GroupD),
        11 => Some(InputElement::GroupE),
        12 => Some(InputElement::GroupF),
        13 => Some(InputElement::GroupG),
        14 => Some(InputElement::GroupH),
        27 => Some(InputElement::Notes),
        28 => Some(InputElement::Volume),
        29 => Some(InputElement::Swing),
        30 => Some(InputElement::Tempo),
        31 => Some(InputElement::NoteRepeat),
        32 => Some(InputElement::Lock),
        33 => Some(InputElement::PadMode),
        34 => Some(InputElement::Keyboard),
        35 => Some(InputElement::Chords),
        36 => Some(InputElement::Step),
        37 => Some(InputElement::FixedVel),
        38 => Some(InputElement::Scene),
        39 => Some(InputElement::Pattern),
        40 => Some(InputElement::Events),
        41 => Some(InputElement::Variation),
        42 => Some(InputElement::Duplicate),
        43 => Some(InputElement::Select),
        44 => Some(InputElement::Solo),
        45 => Some(InputElement::Mute),
        46 => Some(InputElement::Pitch),
        47 => Some(InputElement::Mod),
        48 => Some(InputElement::Perform),
        49 => Some(InputElement::DisplayButton1),
        50 => Some(InputElement::DisplayButton2),
        51 => Some(InputElement::DisplayButton3),
        52 => Some(InputElement::DisplayButton4),
        53 => Some(InputElement::DisplayButton5),
        54 => Some(InputElement::DisplayButton6),
        55 => Some(InputElement::DisplayButton7),
        56 => Some(InputElement::DisplayButton8),
        57 => Some(InputElement::ChannelMidi),
        58 => Some(InputElement::Arranger),
        59 => Some(InputElement::BrowserPlugin),
        60 => Some(InputElement::ArrowLeft),
        61 => Some(InputElement::ArrowRight),
        62 => Some(InputElement::FileSave),
        63 => Some(InputElement::Settings),
        64 => Some(InputElement::Macro),
        65 => Some(InputElement::Plugin),
        66 => Some(InputElement::Mixer),
        67 => Some(InputElement::Sampling),
        68 => Some(InputElement::Auto),
        69 => Some(InputElement::EncoderPush),
        74 => Some(InputElement::Shift),
        _ => None,
    }
}
