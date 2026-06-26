//! USB support.
//!
//! Exactly one USB role may be selected for an OTG peripheral.

#[cfg(all(feature = "usb-device", feature = "usb-host"))]
compile_error!("Select either `usb-device` or `usb-host`, not both.");

#[cfg(not(any(feature = "usb-device", feature = "usb-host")))]
compile_error!("Select one USB role: `usb-device` or `usb-host`.");

#[cfg(feature = "usb-device")]
#[path = "main/device.rs"]
mod implementation;

#[cfg(feature = "usb-host")]
#[path = "main/host.rs"]
mod implementation;

pub use implementation::*;
