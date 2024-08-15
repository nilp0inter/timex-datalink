use serde::{Serialize, Deserialize};


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
        time: DateTime,
        is_24h: bool,
        date_format: String,
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
    },
    SoundOptions {
        hourly_chime: bool,
        button_beep: bool,
    },
    WristApp {
        zap_file: String,
    },
    End,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateTime;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeOfDay;
