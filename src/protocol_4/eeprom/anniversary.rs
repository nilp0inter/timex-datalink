//! Anniversary implementation for Protocol 4 EEPROM
//!
//! This module handles anniversaries stored in the watch's EEPROM.

use std::time::SystemTime;
use chrono::Datelike;
use crate::char_encoders::EepromString;

/// Anniversary structure for Protocol 4 EEPROM
pub struct Anniversary {
    /// Time of anniversary
    pub time: SystemTime,
    
    /// Anniversary text (EEPROM encoded, max 31 characters)
    pub anniversary: EepromString,
}

impl Anniversary {
    /// Create the packet for an anniversary, similar to Ruby's packet method
    /// 
    /// This returns the raw packet bytes without the length prefix
    fn packet_content(&self) -> Vec<u8> {
        // Convert SystemTime to chrono::DateTime for easier month/day extraction
        // We need to handle dates that may be before the Unix epoch
        let datetime = match self.time.duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => {
                chrono::DateTime::<chrono::Utc>::from_timestamp(
                    duration.as_secs() as i64,
                    0
                ).expect("Invalid positive timestamp")
            },
            Err(_) => {
                // For dates before Unix epoch (1970-01-01)
                // We need to handle this differently
                // First convert to a duration before epoch
                let duration_before_epoch = std::time::UNIX_EPOCH
                    .duration_since(self.time)
                    .expect("Time error");
                
                // Create a DateTime by subtracting from epoch
                let seconds_before_epoch = -(duration_before_epoch.as_secs() as i64);
                chrono::DateTime::<chrono::Utc>::from_timestamp(
                    seconds_before_epoch,
                    0
                ).expect("Invalid negative timestamp")
            }
        };
        
        let month = datetime.month() as u8;
        let day = datetime.day() as u8;
        
        // Combine month, day, and anniversary text
        let mut packet = Vec::new();
        packet.push(month);
        packet.push(day);
        packet.extend_from_slice(self.anniversary.as_bytes());
        
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
