//! Appointment implementation for Protocol 4 EEPROM
//!
//! This module handles appointments stored in the watch's EEPROM.

use std::time::SystemTime;
use crate::PacketGenerator;

use crate::char_encoders::EepromString;

/// Appointment structure for Protocol 4 EEPROM
pub struct Appointment {
    /// Time of appointment
    pub time: SystemTime,
    
    /// Appointment message text (EEPROM encoded, max 31 characters)
    pub message: EepromString,
}

impl PacketGenerator for Appointment {
    fn packets(&self) -> Vec<Vec<u8>> {
        todo!()
    }
}