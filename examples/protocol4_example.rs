use std::time::SystemTime;
use chrono::{TimeZone, Utc};
use timex_datalink::protocol_4::{
    Protocol4,
    alarm::Alarm,
    eeprom::{
        Anniversary,
        Appointment,
        List,
        PhoneNumber,
        Eeprom,
        phone_number::PhoneType,
        list::Priority,
        NotificationMinutes,
    },
    time::{Time, DateFormat},
    sound_options::SoundOptions,
    sound_theme::SoundTheme,
    wrist_app::WristApp,
    start::Start,
    sync::Sync,
    end::End,
};
use timex_datalink::char_encoders::{CharString, EepromString, PhoneString};

fn main() {
    // Create appointments
    let appointments = vec![
        Appointment {
            time: system_time_from_date(2022, 10, 31, 19, 0, 0),
            message: EepromString::new("Scare the neighbors"),
        },
        Appointment {
            time: system_time_from_date(2022, 11, 24, 17, 0, 0),
            message: EepromString::new("Feed the neighbors"),
        },
        Appointment {
            time: system_time_from_date(2022, 12, 25, 14, 0, 0),
            message: EepromString::new("Spoil the neighbors"),
        },
    ];

    // Create anniversaries
    let anniversaries = vec![
        Anniversary {
            time: system_time_from_date(1985, 7, 3, 0, 0, 0),
            anniversary: EepromString::new("Release of Back to the Future"),
        },
        Anniversary {
            time: system_time_from_date(1968, 4, 6, 0, 0, 0),
            anniversary: EepromString::new("Release of 2001"),
        },
    ];

    // Create phone numbers
    let phone_numbers = vec![
        PhoneNumber {
            name: EepromString::new("Marty McFly"),
            number: PhoneString::new("1112223333"),
            phone_type: PhoneType::Home,
        },
        PhoneNumber {
            name: EepromString::new("Doc Brown"),
            number: PhoneString::new("4445556666"),
            phone_type: PhoneType::Cell,
        },
    ];

    // Create lists
    let lists = vec![
        List {
            list_entry: EepromString::new("Muffler bearings"),
            priority: Some(Priority::Two),
        },
        List {
            list_entry: EepromString::new("Headlight fluid"),
            priority: Some(Priority::Four),
        },
    ];

    // Current time
    let time1 = SystemTime::now();
    // In a real app, we would use a time zone library to handle UTC conversion

    // Create the Protocol4 structure with all components
    let protocol = Protocol4 {
        sync: Sync {
            length: 100,  // Default length for sync pattern
        },
        start: Start {},
        
        times: vec![
            Time {
                zone: 1,
                is_24h: false,
                date_format: DateFormat::MonthDashDayDashYear,
                time: time1,
                name: CharString::new("PDT", true),
            },
            Time {
                zone: 2,
                is_24h: true,
                date_format: DateFormat::MonthDashDayDashYear,
                time: time1, // In a real app, this would be UTC time
                name: CharString::new("GMT", true),
            },
        ],
        
        alarms: vec![
            Alarm {
                number: 1,
                audible: true,
                time: system_time_from_time(9, 0),
                message: CharString::new("Wake up", false),
            },
            Alarm {
                number: 2,
                audible: true,
                time: system_time_from_time(9, 5),
                message: CharString::new("For real", false),
            },
            Alarm {
                number: 3,
                audible: false,
                time: system_time_from_time(9, 10),
                message: CharString::new("Get up", false),
            },
            Alarm {
                number: 4,
                audible: true,
                time: system_time_from_time(18, 0), // 6 PM
                message: CharString::new("Or not", false),
            },
            Alarm {
                number: 5,
                audible: false,
                time: system_time_from_time(14, 0), // 2 PM
                message: CharString::new("Told you", false),
            },
        ],
        
        sound_options: Some(SoundOptions {
            hourly_chime: true,
            button_beep: true,
        }),
        
        sound_theme: Some(SoundTheme {
            // In a real app, we would load the data from "DEFHIGH.SPC"
            // For this example, we use placeholder data
            sound_theme_data: vec![0x00, 0x01, 0x02, 0x03], // Data from DEFHIGH.SPC would go here
        }),
        
        eeprom: Some(Eeprom {
            appointments,
            anniversaries,
            lists,
            phone_numbers,
            appointment_notification_minutes: Some(NotificationMinutes::FifteenMinutes),
        }),
        
        wrist_app: Some(WristApp {
            // In a real app, we would load the data from "TIMER13.ZAP" 
            // For this example, we use placeholder data
            wrist_app_data: vec![0x00, 0x01, 0x02, 0x03], // Data from TIMER13.ZAP would go here
        }),
        
        end: End {},
    };

    // In a real application, we would serialize the protocol data
    // and transmit it to the watch.
    
    println!("Created Protocol4 structure with all components");
    println!("- Time zones: {}", protocol.times.len());
    println!("- Alarms: {}", protocol.alarms.len());
    println!("- Appointments: {}", protocol.eeprom.as_ref().unwrap().appointments.len());
    println!("- Anniversaries: {}", protocol.eeprom.as_ref().unwrap().anniversaries.len());
    println!("- Phone numbers: {}", protocol.eeprom.as_ref().unwrap().phone_numbers.len());
    println!("- Lists: {}", protocol.eeprom.as_ref().unwrap().lists.len());
    
    // This is just a placeholder since we're not actually sending data to a device
    println!("\nIn a real application, we would send this data to the watch.");
}

// Helper function to create a SystemTime from date components
fn system_time_from_date(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> SystemTime {
    // Create a DateTime with chrono
    let naive_dt = chrono::NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_opt(hour, min, sec)
        .unwrap();
    
    let dt = Utc.from_utc_datetime(&naive_dt);
    
    // Convert to system time - handle negative timestamps properly
    if dt.timestamp() >= 0 {
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(dt.timestamp() as u64)
    } else {
        SystemTime::UNIX_EPOCH - std::time::Duration::from_secs((-dt.timestamp()) as u64)
    }
}

// Helper function to create a SystemTime with just hour and minute
fn system_time_from_time(hour: u32, min: u32) -> SystemTime {
    // For alarms, we only care about hour and minute, so use a fixed date
    // Using 2000-01-01 as the base date
    let naive_dt = chrono::NaiveDate::from_ymd_opt(2000, 1, 1)
        .unwrap()
        .and_hms_opt(hour, min, 0)
        .unwrap();
    
    let dt = Utc.from_utc_datetime(&naive_dt);
    
    // Convert to system time
    SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(dt.timestamp() as u64)
}