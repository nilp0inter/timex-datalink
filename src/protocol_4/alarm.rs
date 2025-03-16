//! Alarm implementation for Protocol 4
//!
//! This module handles alarms for Timex Datalink watches.

use std::time::SystemTime;
use crate::PacketGenerator;

use crate::char_encoders::CharString;

/// Alarm structure for Protocol 4
pub struct Alarm {
    /// Alarm number (from 1 to 5)
    pub number: u8,
    
    /// Whether the alarm makes a sound
    pub audible: bool,
    
    /// Time of alarm
    pub time: SystemTime,
    
    /// Alarm message text (max 8 characters)
    pub message: CharString<8>,
}

impl PacketGenerator for Alarm {
    fn packets(&self) -> Vec<Vec<u8>> {
        todo!()
    }
}