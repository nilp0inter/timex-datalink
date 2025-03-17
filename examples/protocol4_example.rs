use std::time::SystemTime;
use std::env;
use std::process;
use chrono::{TimeZone, Utc};
use timex_datalink::PacketGenerator;
use timex_datalink::NotebookAdapter;
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
    // Get the serial port from command line arguments
    let args: Vec<String> = env::args().collect();
    let serial_port = match args.len() {
        1 => {
            println!("Usage: {} <serial_port_path>", args[0]);
            println!("  Example: {} /dev/ttyUSB0", args[0]);
            process::exit(1);
        },
        _ => args[1].clone(),
    };
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

    // Create the Protocol4 structure with all components
    let mut protocol = Protocol4::new();
    
    // Add mandatory components
    protocol.add(Sync { length: 100 });
    protocol.add(Start {});
    
    // Add multiple time zones
    protocol.add(Time {
        zone: 1,
        is_24h: false,
        date_format: DateFormat::MonthDashDayDashYear,
        time: time1,
        name: CharString::new("PDT", true),
    });
    
    protocol.add(Time {
        zone: 2,
        is_24h: true,
        date_format: DateFormat::MonthDashDayDashYear,
        time: time1,
        name: CharString::new("GMT", true),
    });
    
    // Add multiple alarms
    protocol.add(Alarm {
        number: 1,
        audible: true,
        time: system_time_from_time(9, 0),
        message: CharString::new("Wake up", false),
    });
    
    protocol.add(Alarm {
        number: 2,
        audible: true,
        time: system_time_from_time(9, 5),
        message: CharString::new("For real", false),
    });
    
    protocol.add(Alarm {
        number: 3,
        audible: false,
        time: system_time_from_time(9, 10),
        message: CharString::new("Get up", false),
    });
    
    protocol.add(Alarm {
        number: 4,
        audible: true,
        time: system_time_from_time(18, 0), // 6 PM
        message: CharString::new("Or not", false),
    });
    
    protocol.add(Alarm {
        number: 5,
        audible: false,
        time: system_time_from_time(14, 0), // 2 PM
        message: CharString::new("Told you", false),
    });
    
    // Add optional components
    protocol.add(SoundOptions {
        hourly_chime: true,
        button_beep: true,
    });
    
    protocol.add(SoundTheme {
        // Data from DEFHIGH.SPC
        sound_theme_data: vec![0x00, 0x01, 0x02, 0x03],
    });
    
    // Create and add EEPROM
    let eeprom = Eeprom {
        appointments,
        anniversaries,
        lists,
        phone_numbers,
        appointment_notification_minutes: Some(NotificationMinutes::FifteenMinutes),
    };
    protocol.add(eeprom);
    
    protocol.add(WristApp {
        // Data from TIMER13.ZAP
        wrist_app_data: vec![0x00, 0x01, 0x02, 0x03],
    });
    
    // Add End component (mandatory)
    protocol.add(End {});

    // Generate all packets
    let all_packets = protocol.packets();
    
    // Display results
    println!("Created Protocol4 structure with all components");
    println!("- Generated {} packet groups", all_packets.len());
    
    // Print packet summary
    for (i, packet) in all_packets.iter().enumerate() {
        // Only print the first few bytes of each packet to avoid overwhelming output
        let preview: Vec<u8> = packet.iter().take(6).cloned().collect();
        println!("Packet group {}: {} bytes, starts with {:02X?}...", i, packet.len(), preview);
    }
    
    println!("\nTransmitting data to the watch on port: {}", serial_port);
    
    // Create the notebook adapter and send the packets
    let adapter = NotebookAdapter::new(
        serial_port,
        None, // Use default byte sleep time
        None, // Use default packet sleep time
        true, // Enable verbose output
    );
    
    match adapter.write(&all_packets) {
        Ok(_) => println!("\nSuccessfully transmitted data to the watch!"),
        Err(e) => {
            eprintln!("\nError transmitting data: {}", e);
            process::exit(1);
        }
    }
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