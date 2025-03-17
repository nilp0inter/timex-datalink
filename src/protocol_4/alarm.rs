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
        let expected = vec![vec![
            18, 80, 1, 9, 0, 0, 0, 32, 10, 20, 14, 36, 30, 25, 36, 1, 32, 240
        ]];

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
        let expected = vec![vec![
            18, 80, 3, 9, 10, 0, 0, 16, 14, 29, 36, 30, 25, 36, 36, 0, 191, 169
        ]];

        assert_eq!(alarm.packets(), expected);
    }
}