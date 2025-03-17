use serde::{Deserialize, Serialize};

/// Data structures for Timex Datalink 150 watch communication
/// These match the JSON format used by the Ruby implementation

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentData {
    pub time: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnniversaryData {
    pub time: String,
    pub anniversary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumberData {
    pub name: String,
    pub number: String,
    #[serde(default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListData {
    pub list_entry: String,
    #[serde(default)]
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmData {
    pub number: u8,
    pub audible: bool,
    pub hour: u8,
    pub minute: u8,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundOptionsData {
    #[serde(default)]
    pub hourly_chime: bool,
    #[serde(default)]
    pub button_beep: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimexData {
    #[serde(default)]
    pub appointments: Vec<AppointmentData>,
    #[serde(default)]
    pub anniversaries: Vec<AnniversaryData>,
    #[serde(default)]
    pub phone_numbers: Vec<PhoneNumberData>,
    #[serde(default)]
    pub lists: Vec<ListData>,
    #[serde(default)]
    pub alarms: Vec<AlarmData>,
    #[serde(default)]
    pub sound_options: Option<SoundOptionsData>,
    #[serde(default)]
    pub appointment_notification_minutes: Option<u8>,
}

impl TimexData {
    /// Create a new empty TimexData instance
    pub fn new() -> Self {
        Default::default()
    }

    /// Parse a JSON string into TimexData
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    /// Serialize TimexData to a JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}