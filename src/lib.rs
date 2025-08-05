pub mod device;
pub mod device_hid;
pub mod error;
pub mod input;
pub mod output;

#[cfg(windows)]
pub mod ni_ipc;

pub use device::MaschineMK3;
pub use device_hid::MaschineMK3Hid;
pub use error::MK3Error;
pub use input::{InputState, PadState, ButtonState, KnobState, TouchStripState, AudioState, PadHit};
pub use output::{ButtonLedState, PadLedState, DisplayPacket, DisplayGraphics, RgbColor, Rgb565, LedBrightness};
