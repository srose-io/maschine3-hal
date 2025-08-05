use thiserror::Error;

#[derive(Error, Debug)]
pub enum MK3Error {
    #[error("USB error: {0}")]
    Usb(#[from] rusb::Error),

    #[error("Device not found")]
    DeviceNotFound,

    #[error("Invalid packet format")]
    InvalidPacket,

    #[error("Device disconnected")]
    DeviceDisconnected,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid data {0}")]
    InvalidData(String),
}

pub type Result<T> = std::result::Result<T, MK3Error>;
