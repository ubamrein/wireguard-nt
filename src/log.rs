use crate::wireguard_nt_raw;
use log::*;
use widestring::U16CStr;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Sets the logger wireguard will use when logging. Maps to the wireguardSetLogger C function
pub fn set_logger(
    wireguard: &Arc<wireguard_nt_raw::wireguard>,
    f: wireguard_nt_raw::WIREGUARD_LOGGER_CALLBACK,
) {
    unsafe { wireguard.WireGuardSetLogger(f) };
}

/// What level of logging this adapter is using
pub enum AdapterLoggingLevel {
    /// No messages are logged
    Off,

    /// All messages are logged
    On,

    /// All messaged are logged and the adapter id prefixes the log message
    OnWithPrefix,
}

static SET_LOGGER: AtomicBool = AtomicBool::new(false);

/// The logger that is active by default. Logs messages to the log crate
pub extern "C" fn default_logger(
    level: wireguard_nt_raw::WIREGUARD_LOGGER_LEVEL,
    _timestamp: wireguard_nt_raw::DWORD64,
    message: *const wireguard_nt_raw::WCHAR,
) {
    if message.is_null() {
        return;
    }
    //WireGuard will always give us a valid UTF16 null terminated string
    let msg = unsafe { U16CStr::from_ptr_str(message) };
    let utf8_msg = msg.to_string_lossy();
    match level {
        wireguard_nt_raw::WIREGUARD_LOGGER_LEVEL_WIREGUARD_LOG_INFO => {
            info!("wireguard: {}", utf8_msg)
        }
        wireguard_nt_raw::WIREGUARD_LOGGER_LEVEL_WIREGUARD_LOG_WARN => {
            warn!("wireguard: {}", utf8_msg)
        }
        wireguard_nt_raw::WIREGUARD_LOGGER_LEVEL_WIREGUARD_LOG_ERR => {
            error!("wireguard: {}", utf8_msg)
        }
        _ => error!("wireguard: {} (with invalid log level {})", utf8_msg, level),
    }
}

pub(crate) fn set_default_logger_if_unset(wireguard: &Arc<wireguard_nt_raw::wireguard>) {
    if !SET_LOGGER.load(Ordering::Relaxed) {
        set_logger(wireguard, Some(default_logger));
        SET_LOGGER.store(true, Ordering::Relaxed);
    }
}
