//! Appointment implementation for Protocol 4 EEPROM
//!
//! This module handles appointments stored in the watch's EEPROM.

use std::time::SystemTime;
use crate::PacketGenerator;

/// Appointment structure for Protocol 4 EEPROM
pub struct Appointment {
    /// Time of appointment
    pub time: SystemTime,
    
    /// Appointment message text (will be truncated to fit watch requirements)
    pub message: String,
}

impl PacketGenerator for Appointment {
    fn packets(&self) -> Vec<Vec<u8>> {
        todo!()
    }
}