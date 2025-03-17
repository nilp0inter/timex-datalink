//! PhoneNumber implementation for Protocol 4 EEPROM
//!
//! This module handles phone numbers stored in the watch's EEPROM.

/// Phone number type 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhoneType {
    /// Cell phone (represented by 'c' in the Ruby implementation)
    Cell,
    /// Home phone (represented by 'h' in the Ruby implementation)
    Home,
    /// Work phone (represented by 'w' in the Ruby implementation)
    Work,
    /// Other phone type (any other character in the Ruby implementation)
    Other,
}

use crate::char_encoders::{EepromString, PhoneString};

/// PhoneNumber structure for Protocol 4 EEPROM
pub struct PhoneNumber {
    /// Contact name (EEPROM encoded, max 31 characters)
    pub name: EepromString,
    
    /// Phone number string (special phone encoding, max 10 digits)
    pub number: PhoneString,
    
    /// Type of phone number
    pub phone_type: PhoneType,
}
