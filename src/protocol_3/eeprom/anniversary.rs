//! Anniversary implementation for Protocol 3 EEPROM
//!
//! This module handles anniversary data for Timex Datalink watches.

use std::time::SystemTime;
use chrono::{DateTime, Utc, Datelike};
use crate::char_encoders::EepromString;
use crate::protocol_3::eeprom::EepromModel;

/// Anniversary structure for Protocol 3
///
/// This represents an anniversary to be stored in the watch's EEPROM.
pub struct Anniversary {
    /// Date of the anniversary
    pub time: SystemTime,
    
    /// Anniversary text (max 31 chars)
    pub anniversary: String,
}

impl Anniversary {
    /// Create a new Anniversary
    pub fn new(time: SystemTime, anniversary: String) -> Self {
        Anniversary { time, anniversary }
    }
}

impl EepromModel for Anniversary {
    fn packet(&self) -> Vec<u8> {
        let duration = self.time
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
            
        let dt = DateTime::<Utc>::from_timestamp(
            duration.as_secs() as i64,
            0
        ).expect("Invalid timestamp");
        
        // Encode the anniversary date
        let month = dt.month() as u8;
        let day = dt.day() as u8;
        
        // Encode the anniversary text
        let anniversary_bytes = EepromString::new(&self.anniversary).as_bytes().to_vec();
        
        // Combine the data
        let mut data = Vec::with_capacity(2 + anniversary_bytes.len());
        data.push(month);
        data.push(day);
        data.extend(anniversary_bytes);
        
        // Add packet length byte at the beginning
        let mut packet = Vec::with_capacity(data.len() + 1);
        packet.push((data.len() + 1) as u8); // +1 for the length byte itself
        packet.extend(data);
        
        packet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    
    // Helper function to create a SystemTime from date components
    fn system_time_from_date(year: i32, month: u32, day: u32) -> SystemTime {
        let naive_dt = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        
        let dt = Utc.from_utc_datetime(&naive_dt);
        
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(dt.timestamp() as u64)
    }
    
    #[test]
    fn test_anniversary_packet() {
        let anniversary = Anniversary::new(
            system_time_from_date(1985, 7, 3),
            "Release of Back to the Future".to_string()
        );
        
        // The exact bytes depend on the EepromString encoding, but we can at least
        // check the first few bytes which we know the exact values of
        let packet = anniversary.packet();
        
        // Check the structure (don't check the encoded message since it's tested in the EepromString tests)
        assert!(!packet.is_empty());
        assert_eq!(packet[0], packet.len() as u8); // Length byte is correct
        assert_eq!(packet[1], 7); // Month - July
        assert_eq!(packet[2], 3); // Day - 3
    }
}