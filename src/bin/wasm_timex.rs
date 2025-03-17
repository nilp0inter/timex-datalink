use std::time::SystemTime;
use wasm_bindgen::prelude::*;
use timex_datalink::{
    Protocol3, PacketGenerator,
    protocol_3::{
        Sync, Start, End, Time, Alarm, SoundOptions,
        eeprom::{Eeprom, Appointment, Anniversary, PhoneNumber, List}
    },
    char_encoders::CharString,
    protocol_3::time::DateFormat,
};
use chrono::{DateTime, Utc, TimeZone};

// Helper function to create a SystemTime from date components
fn system_time_from_date(year: i32, month: u32, day: u32, hour: u32, min: u32) -> SystemTime {
    let naive_dt = chrono::NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_opt(hour, min, 0)
        .unwrap();
    
    let dt = Utc.from_utc_datetime(&naive_dt);
    
    if dt.timestamp() < 0 {
        // For dates before 1970, just use UNIX_EPOCH as a fallback
        SystemTime::UNIX_EPOCH
    } else {
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(dt.timestamp() as u64)
    }
}

#[wasm_bindgen]
pub fn generate_protocol3_packets() -> JsValue {
    // Create a new Protocol 3 instance
    let mut protocol = Protocol3::new();
    
    // Define the appointments
    let appointments = vec![
        Appointment::new(
            system_time_from_date(2023, 10, 31, 19, 0),
            "Scare the neighbors".to_string()
        ),
        Appointment::new(
            system_time_from_date(2023, 11, 24, 17, 0),
            "Feed the neighbors".to_string()
        ),
        Appointment::new(
            system_time_from_date(2023, 12, 25, 14, 0),
            "Spoil the neighbors".to_string()
        ),
    ];
    
    // Define the anniversaries
    let anniversaries = vec![
        Anniversary::new(
            system_time_from_date(1985, 7, 3, 0, 0),
            "Release of Back to the Future".to_string()
        ),
        Anniversary::new(
            system_time_from_date(1968, 4, 6, 0, 0),
            "Release of 2001".to_string()
        ),
    ];
    
    // Define the phone numbers
    let phone_numbers = vec![
        PhoneNumber::new(
            "Marty McFly".to_string(),
            "1112223333".to_string(),
            Some("H".to_string())
        ),
        PhoneNumber::new(
            "Doc Brown".to_string(),
            "4445556666".to_string(),
            Some("C".to_string())
        ),
    ];
    
    // Define the lists
    let lists = vec![
        List::new(
            "Muffler bearings".to_string(),
            Some(2)
        ),
        List::new(
            "Headlight fluid".to_string(),
            Some(4)
        ),
    ];
    
    // Add each component to the protocol
    protocol.add(Sync::default());
    protocol.add(Start);
    
    // Add time settings - local time in zone 1
    let time1 = SystemTime::now();
    
    protocol.add(Time {
        zone: 1,
        is_24h: false,
        date_format: DateFormat::MonthDashDayDashYear,
        time: time1,
        name: CharString::new("HOME", true),
    });
    
    // Add time settings - UTC time in zone 2
    let time2 = SystemTime::now();
    protocol.add(Time {
        zone: 2,
        is_24h: true,
        date_format: DateFormat::MonthDashDayDashYear,
        time: time2,
        name: CharString::new("UTC", true),
    });
    
    // Add alarms
    protocol.add(Alarm {
        number: 1,
        audible: true,
        hour: 9,
        minute: 0,
        message: CharString::new("Wake up", true),
    });
    
    protocol.add(Alarm {
        number: 2,
        audible: true,
        hour: 9,
        minute: 5,
        message: CharString::new("For real", true),
    });
    
    // Create a new EEPROM instance with all data
    let mut eeprom = Eeprom::new();
    eeprom.appointments = appointments;
    eeprom.anniversaries = anniversaries;
    eeprom.phone_numbers = phone_numbers;
    eeprom.lists = lists;
    eeprom.appointment_notification_minutes = Some(15);
    
    // Add the EEPROM to the protocol
    protocol.add(eeprom);
    
    // Add sound options
    protocol.add(SoundOptions {
        hourly_chime: true,
        button_beep: true,
    });
    
    // Add end command
    protocol.add(End);
    
    // Generate all packets
    let packets = protocol.packets();
    
    // Convert the packets to a JavaScript array
    let js_packets = serde_wasm_bindgen::to_value(&packets).unwrap();
    
    js_packets
}