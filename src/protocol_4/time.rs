//! Time implementation for Protocol 4
//!
//! This module handles time functionality for Timex Datalink watches.

use std::time::SystemTime;

/// Date format options for Protocol 4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateFormat {
    /// Format: MM-DD-YY
    MonthDashDayDashYear,
    
    /// Format: DD-MM-YY
    DayDashMonthDashYear,
    
    /// Format: YY-MM-DD
    YearDashMonthDashDay,
    
    /// Format: MM.DD.YY
    MonthDotDayDotYear,
    
    /// Format: DD.MM.YY
    DayDotMonthDotYear,
    
    /// Format: YY.MM.DD
    YearDotMonthDotDay,
}

/// Time structure for Protocol 4
pub struct Time {
    /// Time zone number (1 or 2)
    pub zone: u8,
    
    /// Whether to use 24 hour time format
    pub is_24h: bool,
    
    /// Date format to use
    pub date_format: DateFormat,
    
    /// System time to use
    pub time: SystemTime,
    
    /// Name of time zone (optional, 3 chars max)
    pub name: Option<String>,
}