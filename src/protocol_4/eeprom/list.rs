//! List implementation for Protocol 4 EEPROM
//!
//! This module handles to-do lists stored in the watch's EEPROM.

/// Priority level for list items (1-5, with 5 being highest priority)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    /// Priority level 1 (lowest)
    One,
    /// Priority level 2
    Two,
    /// Priority level 3
    Three,
    /// Priority level 4
    Four,
    /// Priority level 5 (highest)
    Five,
}

use crate::char_encoders::EepromString;

/// List structure for Protocol 4 EEPROM
pub struct List {
    /// List entry text (EEPROM encoded, max 31 characters)
    pub list_entry: EepromString,
    
    /// Priority level (optional)
    pub priority: Option<Priority>,
}

impl List {
    /// Convert Priority enum to its numeric value
    fn priority_value(&self) -> u8 {
        match self.priority {
            Some(Priority::One) => 1,
            Some(Priority::Two) => 2,
            Some(Priority::Three) => 3,
            Some(Priority::Four) => 4,
            Some(Priority::Five) => 5,
            None => 0,
        }
    }
    
    /// Create the packet for a list item, similar to Ruby's packet method
    /// 
    /// This returns the raw packet bytes without the length prefix
    fn packet_content(&self) -> Vec<u8> {
        let mut packet = Vec::new();
        
        // Add priority value
        packet.push(self.priority_value());
        
        // Add list entry text
        packet.extend_from_slice(self.list_entry.as_bytes());
        
        packet
    }
    
    /// Create the full packet including length prefix (LengthPacketWrapper in Ruby)
    pub fn packet(&self) -> Vec<u8> {
        let content = self.packet_content();
        let mut result = Vec::with_capacity(content.len() + 1);
        
        // Add length byte (content length + 1 for the length byte itself)
        result.push((content.len() + 1) as u8);
        result.extend(content);
        
        result
    }
}
