//! Time implementation for Protocol 4
//!
//! This module handles time functionality for Timex Datalink watches.

use std::time::SystemTime;
use crate::PacketGenerator;
use chrono::{DateTime, Utc, Timelike, Datelike};

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
        // Define constants from Ruby implementation
        const CPACKET_TIME: u8 = 0x32;

        // Convert SystemTime to DateTime to extract components
        let duration_since_epoch = self.time
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        let datetime = DateTime::<Utc>::from_timestamp(
            duration_since_epoch.as_secs() as i64, 
            0
        ).expect("Invalid timestamp");

        // Extract time components
        let second = datetime.second() as u8;
        let hour = datetime.hour() as u8;
        let minute = datetime.minute() as u8;
        let month = datetime.month() as u8;
        let day = datetime.day() as u8;
        let year_mod_1900 = (datetime.year() % 100) as u8;
        
        // Convert weekday to Monday-based (0-6)
        // In chrono, weekday() returns Mon=0, Tue=1, etc., which is already what we want
        let wday_from_monday = datetime.weekday().num_days_from_monday() as u8;
        
        // Convert is_24h to value
        let is_24h_value = if self.is_24h { 2 } else { 1 };
        
        // Get date format value from enum
        let date_format_value = match self.date_format {
            DateFormat::MonthDashDayDashYear => 0,
            DateFormat::DayDashMonthDashYear => 1,
            DateFormat::YearDashMonthDashDay => 2,
            DateFormat::MonthDotDayDotYear => 4,
            DateFormat::DayDotMonthDotYear => 5,
            DateFormat::YearDotMonthDotDay => 6,
        };
        
        // Create the raw packet
        let mut raw_packet = Vec::with_capacity(16);
        raw_packet.push(CPACKET_TIME);  // Time command code
        raw_packet.push(self.zone);     // Time zone number (1 or 2) 
        raw_packet.push(second);        // Seconds
        raw_packet.push(hour);          // Hour
        raw_packet.push(minute);        // Minute
        raw_packet.push(month);         // Month
        raw_packet.push(day);           // Day
        raw_packet.push(year_mod_1900); // Year (modulo 100)
        
        // Add name characters (3 chars)
        for &byte in self.name.as_array() {
            raw_packet.push(byte);
        }
        
        raw_packet.push(wday_from_monday); // Weekday (0 = Monday)
        raw_packet.push(is_24h_value);     // 12/24 hour format (1=12h, 2=24h)
        raw_packet.push(date_format_value); // Date format
        
        // Apply CRC wrapping
        use crate::helpers::crc_packets_wrapper::wrap_packets_with_crc;
        wrap_packets_with_crc(vec![raw_packet])
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