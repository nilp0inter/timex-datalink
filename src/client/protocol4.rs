use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
    pub zone: u8,
    pub time: DateTime,  // Represents the datetime type
    pub is_24h: bool,
    pub date_format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alarm {
    pub number: u8,
    pub audible: bool,
    pub time: TimeOfDay,  // Represents the time type (without date)
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Appointment {
    pub time: DateTime,  // Represents the datetime type
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Anniversary {
    pub time: DateTime,  // Represents the datetime type
    pub anniversary: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub name: String,
    pub number: String,
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub list_entry: String,
    pub priority: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Eeprom {
    pub appointments: Vec<Appointment>,
    pub anniversaries: Vec<Anniversary>,
    pub phone_numbers: Vec<PhoneNumber>,
    pub lists: Vec<List>,
    pub appointment_notification_minutes: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoundTheme {
    pub spc_file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoundOptions {
    pub hourly_chime: bool,
    pub button_beep: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WristApp {
    pub zap_file: String,
}

// Enum that ties them all together
#[derive(Debug, Serialize, Deserialize)]
pub enum Protocol4 {
    Sync,
    Start,
    Time(Time),
    Alarm(Alarm),
    Eeprom(Eeprom),
    SoundTheme(SoundTheme),
    SoundOptions(SoundOptions),
    WristApp(WristApp),
    End,
}


// Placeholder types for datetime and time
#[derive(Debug, Serialize, Deserialize)]
pub struct DateTime;  // Represents the datetime type

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeOfDay;  // Represents the time of day type (without date)
