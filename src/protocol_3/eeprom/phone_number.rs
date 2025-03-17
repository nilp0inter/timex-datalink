//! PhoneNumber implementation for Protocol 3 EEPROM
//!
//! This module handles phone number data for Timex Datalink watches.

use crate::char_encoders::{EepromString, PhoneString};
use crate::protocol_3::eeprom::EepromModel;

/// PhoneNumber structure for Protocol 3
///
/// This represents a phone number to be stored in the watch's EEPROM.
pub struct PhoneNumber {
    /// Name associated with the phone number
    pub name: String,
    
    /// The phone number
    pub number: String,
    
    /// Phone number type (e.g., "H" for Home, "W" for Work, etc.)
    pub type_code: String,
}

impl PhoneNumber {
    /// Create a new PhoneNumber
    pub fn new(name: String, number: String, type_code: Option<String>) -> Self {
        let type_code = type_code.unwrap_or_else(|| " ".to_string());
        PhoneNumber { name, number, type_code }
    }
    
    // Format the number with type for encoding
    fn number_with_type(&self) -> String {
        format!("{} {}", self.number, self.type_code)
    }
}

impl EepromModel for PhoneNumber {
    fn packet(&self) -> Vec<u8> {
        const PHONE_DIGITS: usize = 12;
        
        // Encode the phone number and type
        let number_type = self.number_with_type();
        let phone_bytes = PhoneString::new(&number_type).as_bytes().to_vec();
        
        // Encode the name
        let name_bytes = EepromString::new(&self.name).as_bytes().to_vec();
        
        // Combine the data
        let mut data = Vec::with_capacity(phone_bytes.len() + name_bytes.len());
        data.extend(phone_bytes);
        data.extend(name_bytes);
        
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
    
    #[test]
    fn test_phone_number_packet() {
        let phone_number = PhoneNumber::new(
            "Marty McFly".to_string(),
            "1112223333".to_string(),
            Some("H".to_string())
        );
        
        // The exact bytes depend on the encoding, but we can at least
        // check that the packet is non-empty and has a valid length byte
        let packet = phone_number.packet();
        
        // Check the structure
        assert!(!packet.is_empty());
        assert_eq!(packet[0], packet.len() as u8); // Length byte is correct
    }
    
    #[test]
    fn test_phone_number_default_type() {
        let phone_number = PhoneNumber::new(
            "Doc Brown".to_string(),
            "5551955".to_string(),
            None
        );
        
        // Check that the default type works
        assert_eq!(phone_number.type_code, " ");
        
        // Make sure packet can be generated
        let packet = phone_number.packet();
        assert!(!packet.is_empty());
    }
}