//! Time implementation for Protocol 4
//!
//! This module handles time functionality for Timex Datalink watches.

use std::time::SystemTime;
use crate::PacketGenerator;

/// Date format options for Protocol 4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateFormat {
    /// Format: MM-DD-YY
    MonthDashDayDashYear,
    
    /// Format: DD-MM-YY
    DayDashMonthDashYear,
    
    /// Format: YY-MM-DD
    YearDashMonthDashDay,
    
    /// Format: MM.DD.YY
    MonthDotDayDotYear,
    
    /// Format: DD.MM.YY
    DayDotMonthDotYear,
    
    /// Format: YY.MM.DD
    YearDotMonthDotDay,
}

use crate::char_encoders::CharString;

/// Time structure for Protocol 4
pub struct Time {
    /// Time zone number (1 or 2)
    pub zone: u8,
    
    /// Whether to use 24 hour time format
    pub is_24h: bool,
    
    /// Date format to use
    pub date_format: DateFormat,
    
    /// System time to use
    pub time: SystemTime,
    
    /// Name of time zone (3 chars max)
    pub name: CharString<3>,
}

impl PacketGenerator for Time {
    fn packets(&self) -> Vec<Vec<u8>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    // Helper function to create a SystemTime from date components
    fn system_time_from_date(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> SystemTime {
        // Create a DateTime with chrono
        let naive_dt = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, min, sec)
            .unwrap();
        
        let dt = Utc.from_utc_datetime(&naive_dt);
        
        // Convert to system time
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(dt.timestamp() as u64)
    }

    #[test]
    fn test_time_12h() {
        let time = Time {
            zone: 1,
            is_24h: false,
            date_format: DateFormat::MonthDashDayDashYear,
            time: system_time_from_date(2022, 9, 5, 3, 39, 44),
            name: CharString::new("PDT", true),
        };

        // From golden fixture: time_12h.jsonl
        #[rustfmt::skip]
        let expected = vec![vec![
            17, 50, 1, 44, 3, 39, 9, 5, 22, 25, 13, 29, 0, 1, 0, 190, 59
        ]];

        assert_eq!(time.packets(), expected);
    }

    #[test]
    fn test_time_24h() {
        let time = Time {
            zone: 2,
            is_24h: true,
            date_format: DateFormat::MonthDashDayDashYear,
            time: system_time_from_date(2022, 9, 5, 11, 39, 44),
            name: CharString::new("GMT", true),
        };

        // From golden fixture: time_24h.jsonl
        #[rustfmt::skip]
        let expected = vec![vec![
            17, 50, 2, 44, 11, 39, 9, 5, 22, 16, 22, 29, 0, 2, 0, 118, 112
        ]];

        assert_eq!(time.packets(), expected);
    }
}