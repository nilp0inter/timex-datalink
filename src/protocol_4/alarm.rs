//! Alarm implementation for Protocol 4
//!
//! This module handles alarms for Timex Datalink watches.

use std::time::SystemTime;

/// Alarm structure for Protocol 4
pub struct Alarm {
    /// Alarm number (from 1 to 5)
    pub number: u8,
    
    /// Whether the alarm makes a sound
    pub audible: bool,
    
    /// Time of alarm
    pub time: SystemTime,
    
    /// Alarm message text
    pub message: String,
}