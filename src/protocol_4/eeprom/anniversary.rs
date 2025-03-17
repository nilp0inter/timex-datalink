//! Anniversary implementation for Protocol 4 EEPROM
//!
//! This module handles anniversaries stored in the watch's EEPROM.

use std::time::SystemTime;

use crate::char_encoders::EepromString;

/// Anniversary structure for Protocol 4 EEPROM
pub struct Anniversary {
    /// Time of anniversary
    pub time: SystemTime,
    
    /// Anniversary text (EEPROM encoded, max 31 characters)
    pub anniversary: EepromString,
}
