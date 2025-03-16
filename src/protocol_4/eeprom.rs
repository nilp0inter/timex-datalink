//! EEPROM implementation for Protocol 4
//!
//! This module handles EEPROM data storage for Timex Datalink watches.

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