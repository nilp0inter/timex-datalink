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

/// PhoneNumber structure for Protocol 4 EEPROM
pub struct PhoneNumber {
    /// Contact name (will be truncated to fit watch requirements)
    pub name: String,
    
    /// Phone number string (digits and allowed symbols)
    pub number: String,
    
    /// Type of phone number
    pub phone_type: PhoneType,
}