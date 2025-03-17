//! EEPROM implementation for Protocol 3
//!
//! This module handles EEPROM data storage for Timex Datalink watches.

use crate::PacketGenerator;
use crate::helpers::crc_packets_wrapper;
use crate::helpers::cpacket_paginator;
use chrono::{DateTime, Utc, Datelike};

pub mod anniversary;
pub mod appointment;
pub mod list;
pub mod phone_number;

pub use anniversary::Anniversary;
pub use appointment::Appointment;
pub use list::List;
pub use phone_number::PhoneNumber;

// Common trait for all EEPROM models in Protocol 3
pub trait EepromModel {
    // Generate packet data for this model
    fn packet(&self) -> Vec<u8>;
}

/// EEPROM structure for Protocol 3
///
/// This structure handles storing various data in the watch's EEPROM.
pub struct Eeprom {
    /// List of appointments
    pub appointments: Vec<Appointment>,
    
    /// List of anniversaries
    pub anniversaries: Vec<Anniversary>,
    
    /// List of phone numbers
    pub phone_numbers: Vec<PhoneNumber>,
    
    /// List of list items
    pub lists: Vec<List>,
    
    /// Appointment notification minutes (in 5-minute increments from 0 to 30, or None)
    pub appointment_notification_minutes: Option<u8>,
}

impl Eeprom {
    /// Create a new empty EEPROM instance
    pub fn new() -> Self {
        Eeprom {
            appointments: Vec::new(),
            anniversaries: Vec::new(),
            phone_numbers: Vec::new(),
            lists: Vec::new(),
            appointment_notification_minutes: None,
        }
    }
    
    // Helper to calculate items_addresses as in Ruby
    fn items_addresses(&self) -> Vec<u8> {
        // This function directly follows the Ruby code:
        // def items_addresses
        //   address = START_ADDRESS
        //
        //   all_items.each_with_object([]) do |items, addresses|
        //     addresses.concat(address.divmod(256))
        //
        //     address += items.sum { |item| item.packet.length }
        //   end
        // end
        
        const START_ADDRESS: u16 = 0x0236;
        let mut address = START_ADDRESS;
        let mut addresses = Vec::new();
        
        // Process appointments
        let (lsb, msb) = (address as u8, (address >> 8) as u8);
        addresses.push(msb);
        addresses.push(lsb);
        address += self.appointments.iter().map(|item| item.packet().len() as u16).sum::<u16>();
        
        // Process lists
        let (lsb, msb) = (address as u8, (address >> 8) as u8);
        addresses.push(msb);
        addresses.push(lsb);
        address += self.lists.iter().map(|item| item.packet().len() as u16).sum::<u16>();
        
        // Process phone numbers
        let (lsb, msb) = (address as u8, (address >> 8) as u8);
        addresses.push(msb);
        addresses.push(lsb);
        address += self.phone_numbers.iter().map(|item| item.packet().len() as u16).sum::<u16>();
        
        // Process anniversaries
        let (lsb, msb) = (address as u8, (address >> 8) as u8);
        addresses.push(msb);
        addresses.push(lsb);
        
        addresses
    }
    
    // Helper to get item counts
    fn items_lengths(&self) -> Vec<u8> {
        vec![
            self.appointments.len() as u8,
            self.lists.len() as u8,
            self.phone_numbers.len() as u8,
            self.anniversaries.len() as u8,
        ]
    }
    
    // Helper to find the earliest appointment year
    fn earliest_appointment_year(&self) -> u8 {
        if self.appointments.is_empty() {
            return 0;
        }
        
        let mut earliest_year = 99; // Initialize to a high value
        
        for appt in &self.appointments {
            let duration = appt.time
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards");
                
            let dt = DateTime::<Utc>::from_timestamp(
                duration.as_secs() as i64,
                0
            ).expect("Invalid timestamp");
            
            let year = (dt.year() % 100) as u8;
            if year < earliest_year {
                earliest_year = year;
            }
        }
        
        earliest_year
    }
    
    // Helper to convert appointment notification minutes to value
    fn appointment_notification_minutes_value(&self) -> u8 {
        const APPOINTMENT_NO_NOTIFICATION: u8 = 0xff;
        
        match self.appointment_notification_minutes {
            Some(minutes) => minutes / 5,
            None => APPOINTMENT_NO_NOTIFICATION,
        }
    }
}

impl PacketGenerator for Eeprom {
    fn packets(&self) -> Vec<Vec<u8>> {
        // Define constants from Ruby implementation
        const CPACKET_CLEAR: [u8; 2] = [0x93, 0x01];
        const CPACKET_SECT: [u8; 2] = [0x90, 0x01];
        const CPACKET_DATA: [u8; 2] = [0x91, 0x01];
        const CPACKET_END: [u8; 2] = [0x92, 0x01];
        const CPACKET_DATA_LENGTH: usize = 32;
        
        // Create the clear packet
        let clear_packet = CPACKET_CLEAR.to_vec();
        
        // Create header packet
        let mut header = Vec::with_capacity(16);
        header.extend_from_slice(&CPACKET_SECT);
        
        // All models combined into a single byte array
        let mut all_packets = Vec::new();
        for item in &self.appointments {
            all_packets.extend(item.packet());
        }
        for item in &self.lists {
            all_packets.extend(item.packet());
        }
        for item in &self.phone_numbers {
            all_packets.extend(item.packet());
        }
        for item in &self.anniversaries {
            all_packets.extend(item.packet());
        }
        
        // Calculate payloads
        let payloads = cpacket_paginator::paginate_cpackets(
            &CPACKET_DATA,
            CPACKET_DATA_LENGTH,
            &all_packets
        );
        
        // Continue building header
        header.push(payloads.len() as u8);
        header.extend(self.items_addresses());
        header.extend(self.items_lengths());
        header.push(self.earliest_appointment_year());
        header.push(self.appointment_notification_minutes_value());
        
        // Create the end packet
        let end_packet = CPACKET_END.to_vec();
        
        // Combine all packets
        let mut all_packets = Vec::with_capacity(2 + payloads.len() + 1);
        all_packets.push(clear_packet);
        all_packets.push(header);
        all_packets.extend(payloads);
        all_packets.push(end_packet);
        
        // Wrap with CRC
        crc_packets_wrapper::wrap_packets_with_crc(all_packets)
    }
}

impl Default for Eeprom {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use std::time::SystemTime;
    
    // Helper function to create a SystemTime from date components
    fn system_time_from_date(year: i32, month: u32, day: u32, hour: u32, min: u32) -> SystemTime {
        let naive_dt = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, min, 0)
            .unwrap();
        
        let dt = Utc.from_utc_datetime(&naive_dt);
        
        if dt.timestamp() < 0 {
            // For dates before 1970
            SystemTime::UNIX_EPOCH
        } else {
            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(dt.timestamp() as u64)
        }
    }
    
    #[test]
    fn test_empty_eeprom() {
        let eeprom = Eeprom::new();
        
        let packets = eeprom.packets();
        assert!(!packets.is_empty());
    }
    
    #[test]
    fn test_eeprom_with_appointment() {
        let mut eeprom = Eeprom::new();
        
        // Add an appointment
        eeprom.appointments.push(Appointment::new(
            system_time_from_date(2022, 10, 31, 19, 0),
            "Scare the neighbors".to_string()
        ));
        
        // Generate the packets
        let packets = eeprom.packets();
        
        // Verify the packets match the golden fixture
        #[rustfmt::skip]
        let expected = vec![
            vec![5, 147, 1, 49, 189],
            vec![20, 144, 1, 1, 2, 54, 2, 73, 2, 73, 2, 73, 1, 0, 0, 0, 22, 255, 146, 12],
            vec![25, 145, 1, 1, 19, 10, 31, 76, 28, 163, 108, 14, 217, 69, 14, 121, 57, 18, 20, 45, 216, 198, 253, 161, 244],
            vec![5, 146, 1, 161, 188]
        ];
        
        assert_eq!(packets, expected, "EEPROM appointment packets don't match golden fixture");
    }
    
    #[test]
    fn test_eeprom_with_anniversary() {
        let mut eeprom = Eeprom::new();
        
        // Add an anniversary
        eeprom.anniversaries.push(Anniversary::new(
            system_time_from_date(1985, 7, 3, 0, 0),
            "Release of Back to the Future".to_string()
        ));
        
        // Generate the packets
        let packets = eeprom.packets();
        
        // Verify the packets match the golden fixture
        #[rustfmt::skip]
        let expected = vec![
            vec![5, 147, 1, 49, 189],
            vec![20, 144, 1, 1, 2, 54, 2, 54, 2, 54, 2, 54, 0, 0, 0, 1, 0, 255, 195, 98],
            vec![32, 145, 1, 1, 26, 7, 3, 155, 83, 57, 10, 231, 144, 216, 67, 46, 10, 67, 145, 29, 70, 118, 145, 67, 62, 94, 231, 109, 206, 15, 200, 224],
            vec![5, 146, 1, 161, 188]
        ];
        
        assert_eq!(packets, expected, "EEPROM anniversary packets don't match golden fixture");
    }
    
    #[test]
    fn test_eeprom_with_phone_number() {
        let mut eeprom = Eeprom::new();
        
        // Add a phone number
        eeprom.phone_numbers.push(PhoneNumber::new(
            "Marty McFly".to_string(),
            "1112223333".to_string(),
            Some("H".to_string())
        ));
        
        // Generate the packets
        let packets = eeprom.packets();
        
        // Verify the packets match the golden fixture
        #[rustfmt::skip]
        let expected = vec![
            vec![5, 147, 1, 49, 189],
            vec![20, 144, 1, 1, 2, 54, 2, 54, 2, 54, 2, 70, 0, 0, 1, 0, 0, 255, 56, 67],
            vec![22, 145, 1, 1, 16, 17, 33, 34, 51, 51, 207, 150, 178, 117, 34, 105, 49, 79, 37, 254, 203, 73],
            vec![5, 146, 1, 161, 188]
        ];
        
        assert_eq!(packets, expected, "EEPROM phone number packets don't match golden fixture");
    }
    
    #[test]
    fn test_eeprom_with_list() {
        let mut eeprom = Eeprom::new();
        
        // Add a list item
        eeprom.lists.push(List::new(
            "Muffler bearings".to_string(),
            Some(2)
        ));
        
        // Generate the packets
        let packets = eeprom.packets();
        
        // Verify the packets match the golden fixture
        #[rustfmt::skip]
        let expected = vec![
            vec![5, 147, 1, 49, 189],
            vec![20, 144, 1, 1, 2, 54, 2, 54, 2, 69, 2, 69, 0, 1, 0, 0, 0, 255, 54, 61],
            vec![21, 145, 1, 1, 15, 2, 150, 247, 60, 149, 179, 145, 139, 163, 108, 210, 5, 113, 63, 68, 23],
            vec![5, 146, 1, 161, 188]
        ];
        
        assert_eq!(packets, expected, "EEPROM list packets don't match golden fixture");
    }
    
    #[test]
    fn test_eeprom_with_notification() {
        let mut eeprom = Eeprom::new();
        
        // Set notification minutes (must be multiple of 5)
        eeprom.appointment_notification_minutes = Some(15);
        
        // Add an appointment
        eeprom.appointments.push(Appointment::new(
            system_time_from_date(2022, 10, 31, 19, 0),
            "Scare the neighbors".to_string()
        ));
        
        // Generate the packets
        let packets = eeprom.packets();
        
        // Verify there are multiple packets
        assert!(packets.len() > 1);
    }
}