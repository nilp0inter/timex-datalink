//! Appointment implementation for Protocol 4 EEPROM
//!
//! This module handles appointments stored in the watch's EEPROM.

use std::time::SystemTime;
use chrono::{Datelike, Timelike};
use crate::char_encoders::EepromString;

/// Appointment structure for Protocol 4 EEPROM
pub struct Appointment {
    /// Time of appointment
    pub time: SystemTime,
    
    /// Appointment message text (EEPROM encoded, max 31 characters)
    pub message: EepromString,
}

impl Appointment {
    /// Create the packet for an appointment, similar to Ruby's packet method
    /// 
    /// This returns the raw packet bytes without the length prefix
    fn packet_content(&self) -> Vec<u8> {
        // Extract time components from SystemTime
        let duration_since_epoch = self.time
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        
        let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(
            duration_since_epoch.as_secs() as i64,
            0
        ).expect("Invalid timestamp");
        
        let month = datetime.month() as u8;
        let day = datetime.day() as u8;
        let hour = datetime.hour() as u8;
        let minute = datetime.minute() as u8;
        
        // Calculate time_15m (each 15-minute slot of the day)
        let time_15m = hour * 4 + minute / 15;
        
        // Create the packet
        let mut packet = Vec::new();
        packet.push(month);
        packet.push(day);
        packet.push(time_15m);
        packet.extend_from_slice(self.message.as_bytes());
        
        packet
    }
    
    /// Create the full packet including length prefix (LengthPacketWrapper in Ruby)
    pub fn packet(&self) -> Vec<u8> {
        let content = self.packet_content();
        let mut result = Vec::with_capacity(content.len() + 1);
        
        // Add length byte (content length + 1 for the length byte itself)
        result.push((content.len() + 1) as u8);
        result.extend(content);
        
        result
    }
}
