#[cfg_attr(not(windows), path = "can/socketcan.rs")]
#[cfg_attr(windows, path = "can/windowscan.rs")]
pub mod canutil;
