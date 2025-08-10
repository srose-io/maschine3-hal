//! # maschine3-hal
//! 
//! Hardware abstraction layer for Native Instruments Maschine MK3 controller (Windows only).
//! 
//! This crate provides low-level USB communication with the Maschine MK3, handling:
//! - Button, pad, knob, and touch strip input events
//! - LED control for buttons and pads with full color support
//! - Display graphics output (480x272 RGB565)
//! 
//! ## Platform Requirements
//! 
//! - Windows 10 or later
//! - WinUSB driver installed (replaces Native Instruments driver)
//! 
//! ## Quick Start
//! 
//! ```no_run
//! use maschine_mk3::{MaschineMK3, MaschineLEDColor, InputEvent};
//! 
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Connect to device
//! let mut device = MaschineMK3::new()?;
//! 
//! // Monitor input events
//! let events = device.poll_input_events()?;
//! for event in events {
//!     match event {
//!         InputEvent::PadHit { pad_number, velocity } => {
//!             println!("Pad {} hit with velocity {}", pad_number, velocity);
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