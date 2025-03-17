//! EEPROM implementation for Protocol 4
//!
//! This module handles EEPROM data storage for Timex Datalink watches.

use crate::PacketGenerator;

pub mod anniversary;
pub mod appointment;
pub mod phone_number;
pub mod list;

pub use anniversary::Anniversary;
pub use appointment::Appointment;
pub use phone_number::PhoneNumber;
pub use list::List;

/// Valid appointment notification minutes (0, 5, 10, 15, 20, 25, 30)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationMinutes {
    /// No notification
    None,
    /// 5 minutes before
    FiveMinutes,
    /// 10 minutes before
    TenMinutes,
    /// 15 minutes before
    FifteenMinutes,
    /// 20 minutes before
    TwentyMinutes,
    /// 25 minutes before
    TwentyFiveMinutes,
    /// 30 minutes before
    ThirtyMinutes,
}

/// EEPROM structure for Protocol 4
pub struct Eeprom {
    /// Appointments to be added to EEPROM data
    pub appointments: Vec<Appointment>,
    
    /// Anniversaries to be added to EEPROM data
    pub anniversaries: Vec<Anniversary>,
    
    /// Phone numbers to be added to EEPROM data
    pub phone_numbers: Vec<PhoneNumber>,
    
    /// Lists to be added to EEPROM data
    pub lists: Vec<List>,
    
    /// Appointment notification in minutes
    pub appointment_notification_minutes: Option<NotificationMinutes>,
}

impl PacketGenerator for Eeprom {
    fn packets(&self) -> Vec<Vec<u8>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use crate::char_encoders::{EepromString, PhoneString};
    use phone_number::PhoneType;
    use list::Priority;
    use std::time::SystemTime;

    // Helper function to create a SystemTime from date components
    fn system_time_from_date(year: i32, month: u32, day: u32, hour: u32, min: u32) -> SystemTime {
        // Create a DateTime with chrono
        let naive_dt = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, min, 0)
            .unwrap();
        
        let dt = Utc.from_utc_datetime(&naive_dt);
        
        // Convert to system time
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(dt.timestamp() as u64)
    }

    // Helper function for anniversaries (no time component)
    fn anniversary_time_from_date(year: i32, month: u32, day: u32) -> SystemTime {
        system_time_from_date(year, month, day, 0, 0)
    }

    #[test]
    fn test_eeprom_anniversary() {
        let eeprom = Eeprom {
            anniversaries: vec![
                Anniversary {
                    time: anniversary_time_from_date(1985, 7, 3),
                    anniversary: EepromString::new("Release of Back to the Future"),
                },
            ],
            appointments: vec![],
            phone_numbers: vec![],
            lists: vec![],
            appointment_notification_minutes: None,
        };

        // From golden fixture: anniversary.jsonl
        #[rustfmt::skip]
        let expected = vec![
            vec![5, 147, 1, 49, 189],
            vec![20, 144, 1, 1, 2, 54, 2, 54, 2, 54, 2, 54, 0, 0, 0, 1, 0, 255, 195, 98],
            vec![32, 145, 1, 1, 26, 7, 3, 155, 83, 57, 10, 231, 144, 216, 67, 46, 10, 67, 145, 29, 70, 118, 145, 67, 62, 94, 231, 109, 206, 15, 200, 224],
            vec![5, 146, 1, 161, 188]
        ];

        assert_eq!(eeprom.packets(), expected);
    }

    #[test]
    fn test_eeprom_appointment() {
        let eeprom = Eeprom {
            anniversaries: vec![],
            appointments: vec![
                Appointment {
                    time: system_time_from_date(2022, 10, 31, 19, 0),
                    message: EepromString::new("Scare the neighbors"),
                },
            ],
            phone_numbers: vec![],
            lists: vec![],
            appointment_notification_minutes: None,
        };

        // From golden fixture: appointment.jsonl
        #[rustfmt::skip]
        let expected = vec![
            vec![5, 147, 1, 49, 189],
            vec![20, 144, 1, 1, 2, 54, 2, 73, 2, 73, 2, 73, 1, 0, 0, 0, 22, 255, 146, 12],
            vec![25, 145, 1, 1, 19, 10, 31, 76, 28, 163, 108, 14, 217, 69, 14, 121, 57, 18, 20, 45, 216, 198, 253, 161, 244],
            vec![5, 146, 1, 161, 188]
        ];

        assert_eq!(eeprom.packets(), expected);
    }

    #[test]
    fn test_eeprom_list_item() {
        let eeprom = Eeprom {
            anniversaries: vec![],
            appointments: vec![],
            phone_numbers: vec![],
            lists: vec![
                List {
                    list_entry: EepromString::new("Muffler bearings"),
                    priority: Some(Priority::Two),
                },
            ],
            appointment_notification_minutes: None,
        };

        // From golden fixture: list_item.jsonl
        #[rustfmt::skip]
        let expected = vec![
            vec![5, 147, 1, 49, 189],
            vec![20, 144, 1, 1, 2, 54, 2, 54, 2, 69, 2, 69, 0, 1, 0, 0, 0, 255, 54, 61],
            vec![21, 145, 1, 1, 15, 2, 150, 247, 60, 149, 179, 145, 139, 163, 108, 210, 5, 113, 63, 68, 23],
            vec![5, 146, 1, 161, 188]
        ];

        assert_eq!(eeprom.packets(), expected);
    }

    #[test]
    fn test_eeprom_phone_number() {
        let eeprom = Eeprom {
            anniversaries: vec![],
            appointments: vec![],
            phone_numbers: vec![
                PhoneNumber {
                    name: EepromString::new("Marty McFly"),
                    number: PhoneString::new("1112223333"),
                    phone_type: PhoneType::Home,
                },
            ],
            lists: vec![],
            appointment_notification_minutes: None,
        };

        // From golden fixture: phone_number.jsonl
        #[rustfmt::skip]
        let expected = vec![
            vec![5, 147, 1, 49, 189],
            vec![20, 144, 1, 1, 2, 54, 2, 54, 2, 54, 2, 70, 0, 0, 1, 0, 0, 255, 56, 67],
            vec![22, 145, 1, 1, 16, 17, 33, 34, 51, 51, 207, 150, 178, 117, 34, 105, 49, 79, 37, 254, 203, 73],
            vec![5, 146, 1, 161, 188]
        ];

        assert_eq!(eeprom.packets(), expected);
    }
}