//! Anniversary implementation for Protocol 4 EEPROM
//!
//! This module handles anniversaries stored in the watch's EEPROM.

use std::time::SystemTime;

/// Anniversary structure for Protocol 4 EEPROM
pub struct Anniversary {
    /// Time of anniversary
    pub time: SystemTime,
    
    /// Anniversary text (will be truncated to fit watch requirements)
    pub anniversary: String,
}