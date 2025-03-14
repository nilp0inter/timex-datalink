use serde::{Serialize, Deserialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Date format options supported by Protocol 4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DateFormat {
    #[serde(rename = "M-D-Y")]
    MonthDayYear,
    #[serde(rename = "D-M-Y")]
    DayMonthYear,
    #[serde(rename = "Y-M-D")]
    YearMonthDay,
}

impl Default for DateFormat {
    fn default() -> Self {
        DateFormat::MonthDayYear
    }
}

impl DateFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            DateFormat::MonthDayYear => "M-D-Y",
            DateFormat::DayMonthYear => "D-M-Y",
            DateFormat::YearMonthDay => "Y-M-D",
        }
    }
}

/// Complete date and time with timezone support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Default for DateTime {
    fn default() -> Self {
        Self {
            year: 2000,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
        }
    }
}

impl DateTime {
    /// Create a new DateTime with the given components
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
    
    /// Create a new DateTime from the current system time
    pub fn now() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        
        // This is a very simplified conversion. In a real implementation,
        // you would use a proper datetime library like chrono.
        let timestamp = now.as_secs();
        let second = (timestamp % 60) as u8;
        let minute = ((timestamp / 60) % 60) as u8;
        let hour = ((timestamp / 3600) % 24) as u8;
        
        // This is a very rough approximation for demo purposes
        // A real implementation would use proper calendar calculations
        let days_since_epoch = timestamp / 86400;
        let year = 1970 + (days_since_epoch / 365) as u16;
        let day = ((days_since_epoch % 365) % 31 + 1) as u8;
        let month = (((days_since_epoch % 365) / 31) + 1) as u8;
        
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
    
    /// Convert to a string in the format specified by DateFormat
    pub fn format(&self, format: &DateFormat) -> String {
        match format {
            DateFormat::MonthDayYear => {
                format!("{:02}-{:02}-{:04}", self.month, self.day, self.year)
            }
            DateFormat::DayMonthYear => {
                format!("{:02}-{:02}-{:04}", self.day, self.month, self.year)
            }
            DateFormat::YearMonthDay => {
                format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
            }
        }
    }
}

/// Time of day without date components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeOfDay {
    pub hour: u8,
    pub minute: u8,
}

impl Default for TimeOfDay {
    fn default() -> Self {
        Self {
            hour: 0,
            minute: 0,
        }
    }
}

impl TimeOfDay {
    /// Create a new TimeOfDay with the given hour and minute
    pub fn new(hour: u8, minute: u8) -> Self {
        Self { hour, minute }
    }
    
    /// Create a TimeOfDay from the current system time
    pub fn now() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        
        let timestamp = now.as_secs();
        let minute = ((timestamp / 60) % 60) as u8;
        let hour = ((timestamp / 3600) % 24) as u8;
        
        Self { hour, minute }
    }
    
    /// Format as a string in 24-hour format
    pub fn format(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }
    
    /// Format as a string in 12-hour format with AM/PM
    pub fn format_12h(&self) -> String {
        let period = if self.hour < 12 { "AM" } else { "PM" };
        let display_hour = if self.hour == 0 {
            12
        } else if self.hour > 12 {
            self.hour - 12
        } else {
            self.hour
        };
        
        format!("{:02}:{:02} {}", display_hour, self.minute, period)
    }
}

/// Sound theme data from SPC file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SoundThemeData {
    pub data: Vec<u8>,
}

impl SoundThemeData {
    /// Load sound theme data from an SPC file
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let data = fs::read(path)?;
        Ok(Self { data })
    }
}

/// WristApp data from ZAP file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WristAppData {
    pub data: Vec<u8>,
}

impl WristAppData {
    /// Load WristApp data from a ZAP file
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let data = fs::read(path)?;
        Ok(Self { data })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Appointment {
    pub time: DateTime,
    pub message: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Anniversary {
    pub time: DateTime,
    pub anniversary: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub name: String,
    pub number: String,
    pub type_: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct List {
    pub list_entry: String,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Protocol4 {
    Sync,
    Start,
    Time {
        zone: u8,
        name: String,
        time: DateTime,
        is_24h: bool,
        date_format: DateFormat,
    },
    Alarm {
        number: u8,
        audible: bool,
        time: TimeOfDay,
        message: String,
    },
    Eeprom{
        appointments: Vec<Appointment>,
        anniversaries: Vec<Anniversary>,
        phone_numbers: Vec<PhoneNumber>,
        lists: Vec<List>,
        appointment_notification_minutes: Option<u8>,
    },
    SoundTheme {
        spc_file: String,
        sound_theme_data: Option<SoundThemeData>,
    },
    SoundOptions {
        hourly_chime: bool,
        button_beep: bool,
    },
    WristApp {
        zap_file: String,
        wrist_app_data: Option<WristAppData>,
    },
    End,
}

impl Protocol4 {
    /// Create a new Start command
    pub fn start() -> Self {
        Self::Start
    }
    
    /// Create a new Sync command
    pub fn sync() -> Self {
        Self::Sync
    }
    
    /// Create a new End command
    pub fn end() -> Self {
        Self::End
    }
    
    /// Create a new Time setting with default values
    pub fn time(zone: u8, name: &str, time: DateTime) -> Self {
        Self::Time {
            zone,
            name: name.to_string(),
            time,
            is_24h: false,
            date_format: DateFormat::default(),
        }
    }
    
    /// Create a new Alarm with default values
    pub fn alarm(number: u8, time: TimeOfDay, message: &str) -> Self {
        Self::Alarm {
            number,
            audible: true,
            time,
            message: message.to_string(),
        }
    }
    
    /// Create a new Eeprom setting with empty collections
    pub fn eeprom() -> Self {
        Self::Eeprom {
            appointments: Vec::new(),
            anniversaries: Vec::new(),
            phone_numbers: Vec::new(),
            lists: Vec::new(),
            appointment_notification_minutes: None,
        }
    }
    
    /// Create a new SoundTheme from an SPC file path
    pub fn sound_theme(spc_file: &str) -> Self {
        Self::SoundTheme {
            spc_file: spc_file.to_string(),
            sound_theme_data: None,
        }
    }
    
    /// Create a new SoundTheme with loaded data
    pub fn sound_theme_with_data(spc_file: &str, data: SoundThemeData) -> Self {
        Self::SoundTheme {
            spc_file: spc_file.to_string(),
            sound_theme_data: Some(data),
        }
    }
    
    /// Load a SoundTheme from a file path
    pub fn sound_theme_from_file<P: AsRef<Path>>(spc_file: &str, path: P) -> io::Result<Self> {
        let data = SoundThemeData::from_file(path)?;
        Ok(Self::sound_theme_with_data(spc_file, data))
    }
    
    /// Create SoundOptions with default values
    pub fn sound_options() -> Self {
        Self::SoundOptions {
            hourly_chime: false,
            button_beep: true,
        }
    }
    
    /// Create a new WristApp from a ZAP file path
    pub fn wrist_app(zap_file: &str) -> Self {
        Self::WristApp {
            zap_file: zap_file.to_string(),
            wrist_app_data: None,
        }
    }
    
    /// Create a new WristApp with loaded data
    pub fn wrist_app_with_data(zap_file: &str, data: WristAppData) -> Self {
        Self::WristApp {
            zap_file: zap_file.to_string(),
            wrist_app_data: Some(data),
        }
    }
    
    /// Load a WristApp from a file path
    pub fn wrist_app_from_file<P: AsRef<Path>>(zap_file: &str, path: P) -> io::Result<Self> {
        let data = WristAppData::from_file(path)?;
        Ok(Self::wrist_app_with_data(zap_file, data))
    }
    
    /// Check if this Protocol4 variant is Time
    pub fn is_time(&self) -> bool {
        matches!(self, Self::Time { .. })
    }
    
    /// Check if this Protocol4 variant is Alarm
    pub fn is_alarm(&self) -> bool {
        matches!(self, Self::Alarm { .. })
    }
    
    /// Check if this Protocol4 variant is Eeprom
    pub fn is_eeprom(&self) -> bool {
        matches!(self, Self::Eeprom { .. })
    }
    
    /// Check if this Protocol4 variant is SoundTheme
    pub fn is_sound_theme(&self) -> bool {
        matches!(self, Self::SoundTheme { .. })
    }
    
    /// Check if this Protocol4 variant is SoundOptions
    pub fn is_sound_options(&self) -> bool {
        matches!(self, Self::SoundOptions { .. })
    }
    
    /// Check if this Protocol4 variant is WristApp
    pub fn is_wrist_app(&self) -> bool {
        matches!(self, Self::WristApp { .. })
    }
}
