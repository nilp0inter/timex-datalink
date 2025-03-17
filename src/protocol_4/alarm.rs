//! Alarm implementation for Protocol 4
//!
//! This module handles alarms for Timex Datalink watches.

use std::time::SystemTime;
use crate::PacketGenerator;
use chrono::{DateTime, Utc, Timelike};
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
        // Constants from Ruby implementation
        const CPACKET_ALARM: u8 = 0x50;
        
        // Extract hour and minute from SystemTime 
        // Convert SystemTime to chrono::DateTime to easily get hour and minute
        let duration_since_epoch = self.time
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        let datetime = DateTime::<Utc>::from_timestamp(
            duration_since_epoch.as_secs() as i64, 
            0
        ).expect("Invalid timestamp");
        
        let hour = datetime.hour() as u8;
        let minute = datetime.minute() as u8;
        
        // Create the raw packet without CRC
        let mut raw_packet = Vec::with_capacity(16); // Approximate capacity
        raw_packet.push(CPACKET_ALARM);  // Alarm command
        raw_packet.push(self.number);    // Alarm number
        raw_packet.push(hour);           // Hour
        raw_packet.push(minute);         // Minute
        raw_packet.push(0);              // Two zeros as per Ruby implementation
        raw_packet.push(0);
        
        // Add message characters - make sure to get the full 8 characters
        // We need to ensure we're getting all 8 characters (padded if necessary)
        for &byte in self.message.as_array() {
            raw_packet.push(byte);
        }
        
        // Add audible flag
        raw_packet.push(if self.audible { 1 } else { 0 });
        
        // Apply CRC wrapping
        use crate::helpers::crc_packets_wrapper::wrap_packets_with_crc;
        wrap_packets_with_crc(vec![raw_packet])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    // Helper function to create a SystemTime with just hour and minute
    fn system_time_from_time(hour: u32, min: u32) -> SystemTime {
        // For alarms, we only care about hour and minute, so use a fixed date
        // Using a fixed date (2000-01-01) as the base date
        let naive_dt = chrono::NaiveDate::from_ymd_opt(2000, 1, 1)
            .unwrap()
            .and_hms_opt(hour, min, 0)
            .unwrap();
        
        let dt = Utc.from_utc_datetime(&naive_dt);
        
        // Convert to system time
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(dt.timestamp() as u64)
    }

    #[test]
    fn test_alarm_basic() {
        let alarm = Alarm {
            number: 1,
            audible: true,
            time: system_time_from_time(9, 0),
            message: CharString::new("Wake up", false),
        };

        // From golden fixture: alarm_basic.jsonl
        #[rustfmt::skip]
        let expected = vec![vec![18,80,1,9,0,0,0,32,10,20,14,36,30,25,36,1,32,240]];

        assert_eq!(alarm.packets(), expected);
    }

    #[test]
    fn test_alarm_silent() {
        let alarm = Alarm {
            number: 3,
            audible: false,
            time: system_time_from_time(9, 10),
            message: CharString::new("Get up", false),
        };

        // From golden fixture: alarm_silent.jsonl
        #[rustfmt::skip]
        let expected = vec![vec![18,80,3,9,10,0,0,16,14,29,36,30,25,36,36,0,191,169]];

        assert_eq!(alarm.packets(), expected);
    }
}
