//! Detect if the program is running under Windows Subsystem for Linux
//!
//! # Usage
//! ```
//! wsl::is_wsl()
//! ```
//!
#![deny(clippy::all)]
#![deny(missing_docs)]

/// Test if the program is running under WSL
#[cfg(target_os = "linux")]
pub fn is_wsl() -> bool {
    if let Ok(b) = std::fs::read("/proc/sys/kernel/osrelease") {
        if let Ok(s) = std::str::from_utf8(&b) {
            let a = s.to_ascii_lowercase();
            return a.contains("microsoft") || a.contains("wsl");
        }
    }
    false
}

/// Test if the program is running under WSL
#[cfg(not(target_os = "linux"))]
pub fn is_wsl() -> bool {
    false
}
