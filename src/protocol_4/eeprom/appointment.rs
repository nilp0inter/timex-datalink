//! Appointment implementation for Protocol 4 EEPROM
//!
//! This module handles appointments stored in the watch's EEPROM.

use std::time::SystemTime;

/// Appointment structure for Protocol 4 EEPROM
pub struct Appointment {
    /// Time of appointment
    pub time: SystemTime,
    
    /// Appointment message text (will be truncated to fit watch requirements)
    pub message: String,
}