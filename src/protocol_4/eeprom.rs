//! EEPROM implementation for Protocol 4
//!
//! This module handles EEPROM data storage for Timex Datalink watches.

use crate::PacketGenerator;
use chrono::Datelike;

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

/// Convert NotificationMinutes to its numeric value
fn notification_minutes_value(minutes: Option<NotificationMinutes>) -> u8 {
    match minutes {
        Some(NotificationMinutes::None) => 0,
        Some(NotificationMinutes::FiveMinutes) => 1,
        Some(NotificationMinutes::TenMinutes) => 2,
        Some(NotificationMinutes::FifteenMinutes) => 3,
        Some(NotificationMinutes::TwentyMinutes) => 4,
        Some(NotificationMinutes::TwentyFiveMinutes) => 5,
        Some(NotificationMinutes::ThirtyMinutes) => 6,
        None => 0xFF, // APPOINTMENT_NO_NOTIFICATION
    }
}

impl PacketGenerator for Eeprom {
    fn packets(&self) -> Vec<Vec<u8>> {
        // Constants from Ruby implementation
        const CPACKET_CLEAR: [u8; 2] = [0x93, 0x01];
        const CPACKET_SECT: [u8; 2] = [0x90, 0x01];
        const CPACKET_DATA: [u8; 2] = [0x91, 0x01];
        const CPACKET_END: [u8; 2] = [0x92, 0x01];
        const CPACKET_DATA_LENGTH: usize = 32;
        const START_ADDRESS: u16 = 0x0236;
        
        // Get packet data for each type
        let appointment_packets: Vec<Vec<u8>> = self.appointments.iter()
            .map(|app| app.packet())
            .collect();
            
        let list_packets: Vec<Vec<u8>> = self.lists.iter()
            .map(|list| list.packet())
            .collect();
            
        let phone_packets: Vec<Vec<u8>> = self.phone_numbers.iter()
            .map(|phone| phone.packet())
            .collect();
            
        let anniversary_packets: Vec<Vec<u8>> = self.anniversaries.iter()
            .map(|anniv| anniv.packet())
            .collect();
        
        // Combine all packets
        let mut all_items = Vec::new();
        all_items.extend(appointment_packets.clone());
        all_items.extend(list_packets.clone());
        all_items.extend(phone_packets.clone());
        all_items.extend(anniversary_packets.clone());
        
        // Calculate starting addresses for each section
        let mut address = START_ADDRESS;
        let mut addresses = Vec::new();
        
        // Calculate address for appointment section
        addresses.push((address >> 8) as u8);
        addresses.push((address & 0xFF) as u8);
        for packet in &appointment_packets {
            address += packet.len() as u16;
        }
        
        // Calculate address for list section
        addresses.push((address >> 8) as u8);
        addresses.push((address & 0xFF) as u8);
        for packet in &list_packets {
            address += packet.len() as u16;
        }
        
        // Calculate address for phone section
        addresses.push((address >> 8) as u8);
        addresses.push((address & 0xFF) as u8);
        for packet in &phone_packets {
            address += packet.len() as u16;
        }
        
        // Calculate address for anniversary section
        addresses.push((address >> 8) as u8);
        addresses.push((address & 0xFF) as u8);
        for packet in &anniversary_packets {
            address += packet.len() as u16;
        }
        
        // Get all items lengths
        let items_lengths = vec![
            self.appointments.len() as u8,
            self.lists.len() as u8,
            self.phone_numbers.len() as u8,
            self.anniversaries.len() as u8,
        ];
        
        // Get earliest appointment year
        let earliest_appointment_year = self.appointments
            .iter()
            .min_by_key(|app| {
                let duration_since_epoch = app.time
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Time went backwards");
                
                let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(
                    duration_since_epoch.as_secs() as i64,
                    0
                ).expect("Invalid timestamp");
                
                datetime.year()
            })
            .map(|app| {
                let duration_since_epoch = app.time
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Time went backwards");
                
                let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(
                    duration_since_epoch.as_secs() as i64,
                    0
                ).expect("Invalid timestamp");
                
                (datetime.year() % 100) as u8
            })
            .unwrap_or(0);
        
        // Get appointment notification minutes value
        let appointment_notification_minutes_value = 
            notification_minutes_value(self.appointment_notification_minutes);
        
        // Create the header packet
        let mut header = Vec::new();
        header.extend_from_slice(&CPACKET_SECT);
        
        // Calculate number of payload packets
        let num_payload_packets = (all_items.len() / CPACKET_DATA_LENGTH) + 1;
        header.push(num_payload_packets as u8); // Number of payload packets
        header.extend_from_slice(&addresses);
        header.extend_from_slice(&items_lengths);
        header.push(earliest_appointment_year);
        header.push(appointment_notification_minutes_value);
        
        // Create payload packets using the paginator
        use crate::helpers::cpacket_paginator::paginate_cpackets;
        let all_data: Vec<u8> = all_items.into_iter().flatten().collect();
        let payloads = paginate_cpackets(&CPACKET_DATA, CPACKET_DATA_LENGTH, &all_data);
        
        // Create the end packet
        let end_packet = CPACKET_END.to_vec();
        
        // Combine all packets
        let mut all_packets = Vec::with_capacity(payloads.len() + 3);
        all_packets.push(CPACKET_CLEAR.to_vec());
        all_packets.push(header);
        all_packets.extend(payloads);
        all_packets.push(end_packet);
        
        // Apply CRC wrapping
        use crate::helpers::crc_packets_wrapper::wrap_packets_with_crc;
        wrap_packets_with_crc(all_packets)
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