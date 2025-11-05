//! # maschine3-hal
//! 
//! Hardware abstraction layer for Native Instruments Maschine MK3 controller.
//! 
//! This crate provides low-level USB communication with the Maschine MK3, handling:
//! - Button, pad, knob, and touch strip input events
//! - LED control for buttons and pads with full color support
//! - Display graphics output (480x272 RGB565)
//! 
//! ## Platform Support
//! 
//! ### Windows
//! - Windows 10 or later
//! - WinUSB driver installed (replaces Native Instruments driver)
//! - Uses HID API for optimal compatibility
//!
//! ### Linux  
//! - Linux kernel 2.6.x or later
//! - Proper udev rules for device access (see LINUX_SETUP.md)
//! - Direct USB communication for better performance
//! 
//! ## Quick Start
//! 
//! ```no_run
//! use maschine3_hal::{MaschineMK3, MaschineLEDColor, InputEvent};
//! 
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Connect to device
//! let mut device = MaschineMK3::new()?;
//! 
//! // Monitor input events
//! let events = device.poll_input_events()?;
//! for event in events {
//!     match event {
//!         InputEvent::PadEvent { pad_number, event_type: maschine3_hal::PadEventType::Hit, value } => {
//!             println!("Pad {} hit with velocity {}", pad_number, value);
//!             device.set_pad_led(pad_number, MaschineLEDColor::red(true))?;
//!         }
//!         _ => {}
//!     }
//! }
//! # Ok(())
//! # }
//! ```

pub mod device;
pub mod error;
pub mod input;
pub mod output;

// FFI module for Unity/C interop
pub mod ffi;

pub use device::MaschineMK3;
pub use error::MK3Error;
pub use input::{
    AudioState, ButtonState, InputElement, InputEvent, InputState, InputTracker, KnobState,
    PadEvent, PadEventType, PadState, TouchStripState,
};
pub use output::{
    ButtonLedState, DisplayGraphics, DisplayPacket, LedBrightness, MaschineLEDColor, PadLedState,
    Rgb565, RgbColor,
};