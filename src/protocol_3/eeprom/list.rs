//! List implementation for Protocol 3 EEPROM
//!
//! This module handles list item data for Timex Datalink watches.

use crate::char_encoders::EepromString;
use crate::protocol_3::eeprom::EepromModel;

/// List structure for Protocol 3
///
/// This represents a list item to be stored in the watch's EEPROM.
pub struct List {
    /// The list entry text
    pub list_entry: String,
    
    /// Priority of the list item (1-5, or None)
    pub priority: Option<u8>,
}

impl List {
    /// Create a new List with the given text and priority
    ///
    /// Priority must be between 1 and 5, or None
    pub fn new(list_entry: String, priority: Option<u8>) -> Self {
        // Validate priority (should be 1-5 or None)
        if let Some(p) = priority {
            assert!(p >= 1 && p <= 5, "Priority must be between 1 and 5");
        }
        
        List { list_entry, priority }
    }
    
    // Convert priority to a value (0 if None)
    fn priority_value(&self) -> u8 {
        self.priority.unwrap_or(0)
    }
}

impl EepromModel for List {
    fn packet(&self) -> Vec<u8> {
        // Get priority value
        let priority = self.priority_value();
        
        // Encode the list entry
        let list_entry_bytes = EepromString::new(&self.list_entry).as_bytes().to_vec();
        
        // Combine the data
        let mut data = Vec::with_capacity(1 + list_entry_bytes.len());
        data.push(priority);
        data.extend(list_entry_bytes);
        
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
    fn test_list_with_priority() {
        let list = List::new(
            "Muffler bearings".to_string(),
            Some(2)
        );
        
        // The exact bytes depend on the encoding, but we can at least
        // check the first few bytes which we know the exact values of
        let packet = list.packet();
        
        // Check the structure
        assert!(!packet.is_empty());
        assert_eq!(packet[0], packet.len() as u8); // Length byte is correct
        assert_eq!(packet[1], 2); // Priority value
    }
    
    #[test]
    fn test_list_without_priority() {
        let list = List::new(
            "Grocery list".to_string(),
            None
        );
        
        // Check the packet
        let packet = list.packet();
        assert!(!packet.is_empty());
        assert_eq!(packet[1], 0); // No priority should be 0
    }
    
    #[test]
    #[should_panic(expected = "Priority must be between 1 and 5")]
    fn test_list_invalid_priority() {
        // This should panic because 6 is not a valid priority
        List::new("Invalid priority".to_string(), Some(6));
    }
}