pub mod device;
pub mod error;
pub mod input;
pub mod output;

#[cfg(windows)]
pub mod ni_ipc;

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
