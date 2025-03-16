//! List implementation for Protocol 4 EEPROM
//!
//! This module handles to-do lists stored in the watch's EEPROM.

use crate::PacketGenerator;

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

impl PacketGenerator for List {
    fn packets(&self) -> Vec<Vec<u8>> {
        todo!()
    }
}