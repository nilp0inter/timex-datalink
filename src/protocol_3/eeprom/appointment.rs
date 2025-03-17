//! Appointment implementation for Protocol 3 EEPROM
//!
//! This module handles appointment data for Timex Datalink watches.

use std::time::SystemTime;
use chrono::{DateTime, Utc, Datelike, Timelike};
use crate::char_encoders::EepromString;
use crate::protocol_3::eeprom::EepromModel;

/// Appointment structure for Protocol 3
///
/// This represents an appointment to be stored in the watch's EEPROM.
pub struct Appointment {
    /// Time of the appointment
    pub time: SystemTime,
    
    /// Message text for the appointment (max 31 chars)
    pub message: String,
}

impl Appointment {
    /// Create a new Appointment
    pub fn new(time: SystemTime, message: String) -> Self {
        Appointment { time, message }
    }
    
    // Helper to convert the time to 15-minute intervals
    fn time_15m(&self) -> u8 {
        let duration = self.time
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
            
        let dt = DateTime::<Utc>::from_timestamp(
            duration.as_secs() as i64,
            0
        ).expect("Invalid timestamp");
        
        (dt.hour() * 4 + dt.minute() / 15) as u8
    }
}

impl EepromModel for Appointment {
    fn packet(&self) -> Vec<u8> {
        let duration = self.time
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
            
        let dt = DateTime::<Utc>::from_timestamp(
            duration.as_secs() as i64,
            0
        ).expect("Invalid timestamp");
        
        // Encode the appointment
        let month = dt.month() as u8;
        let day = dt.day() as u8;
        let time_15m = self.time_15m();
        
        // Encode the message
        let message_bytes = EepromString::new(&self.message).as_bytes().to_vec();
        
        // Combine the data
        let mut data = Vec::with_capacity(3 + message_bytes.len());
        data.push(month);
        data.push(day);
        data.push(time_15m);
        data.extend(message_bytes);
        
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
    fn system_time_from_date(year: i32, month: u32, day: u32, hour: u32, min: u32) -> SystemTime {
        let naive_dt = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, min, 0)
            .unwrap();
        
        let dt = Utc.from_utc_datetime(&naive_dt);
        
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(dt.timestamp() as u64)
    }
    
    #[test]
    fn test_appointment_packet() {
        let appointment = Appointment::new(
            system_time_from_date(2022, 10, 31, 19, 0),
            "Scare the neighbors".to_string()
        );
        
        // The exact bytes depend on the EepromString encoding, but we can at least
        // check the first few bytes which we know the exact values of
        let packet = appointment.packet();
        
        // Check the structure (don't check the encoded message since it's tested in the EepromString tests)
        assert!(!packet.is_empty());
        assert_eq!(packet[0], packet.len() as u8); // Length byte is correct
        assert_eq!(packet[1], 10); // Month - October
        assert_eq!(packet[2], 31); // Day - 31
        assert_eq!(packet[3], 19 * 4); // Time - 19:00 = 19*4 + 0 = 76
    }
}