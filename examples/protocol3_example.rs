use std::time::SystemTime;
use std::env;
use std::process;
use timex_datalink::{
    Protocol3, PacketGenerator, NotebookAdapter,
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
        // (The watch doesn't care about the full date in most cases)
        SystemTime::UNIX_EPOCH
    } else {
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(dt.timestamp() as u64)
    }
}

// Removed the unused function system_time_from_time

fn main() {
    // Get the serial port from command line arguments
    let args: Vec<String> = env::args().collect();
    let serial_port = match args.len() {
        1 => {
            println!("Serial port not specified. Running in preview mode only.");
            println!("Usage: {} <serial_port_path>", args[0]);
            println!("  Example: {} /dev/ttyUSB0", args[0]);
            String::new() // Empty string for preview mode
        },
        _ => args[1].clone(),
    };

    // Create a new Protocol 3 instance
    let mut protocol = Protocol3::new();
    
    // Define the appointments
    let appointments = vec![
        Appointment::new(
            system_time_from_date(2022, 10, 31, 19, 0),
            "Scare the neighbors".to_string()
        ),
        Appointment::new(
            system_time_from_date(2022, 11, 24, 17, 0),
            "Feed the neighbors".to_string()
        ),
        Appointment::new(
            system_time_from_date(2022, 12, 25, 14, 0),
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
    // Convert to DateTime for display
    let duration = time1.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let dt1 = DateTime::<Utc>::from_timestamp(duration.as_secs() as i64, 0).unwrap();
    println!("Setting local time to: {}", dt1.format("%Y-%m-%d %H:%M:%S"));
    
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
    
    protocol.add(Alarm {
        number: 3,
        audible: false,
        hour: 9,
        minute: 10,
        message: CharString::new("Get up", true),
    });
    
    protocol.add(Alarm {
        number: 4,
        audible: true,
        hour: 9,
        minute: 15,
        message: CharString::new("Or not", true),
    });
    
    protocol.add(Alarm {
        number: 5,
        audible: false,
        hour: 11,
        minute: 30,
        message: CharString::new("Told you", true),
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
    
    // Use absolute paths for the fixture files (commented out since not used)
    // let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    
    // // Load a sound theme from the example SPC file
    // println!("Loading sound theme from EXAMPLE.SPC...");
    // let spc_path = base_path.join("fixtures").join("EXAMPLE.SPC");
    // let sound_theme = match SoundTheme::from_spc_file(&spc_path) {
    //     Ok(theme) => theme,
    //     Err(e) => {
    //         println!("Warning: Could not load SPC file: {}. Using simulated data.", e);
    //         // Create a placeholder with minimal data for the example
    //         SoundTheme::new(vec![0; 32])
    //     }
    // };
    // protocol.add(sound_theme);
    
    // Add sound options
    protocol.add(SoundOptions {
        hourly_chime: true,
        button_beep: true,
    });
    
    // // Load a wrist app from the example ZAP file
    // println!("Loading wrist app from EXAMPLE.ZAP...");
    // let zap_path = base_path.join("fixtures").join("EXAMPLE.ZAP");
    // let wrist_app = match WristApp::from_zap_file(&zap_path) {
    //     Ok(app) => app,
    //     Err(e) => {
    //         println!("Warning: Could not load ZAP file: {}. Using simulated data.", e);
    //         // Create a simulated wrist app with demo data
    //         let demo_data = vec![
    //             49, 53, 48, 32, 100, 97, 116, 97, 58, 32, 76, 111, 114, 101, 109, 32,
    //             105, 112, 115, 117, 109, 32, 100, 111, 108, 111, 114, 32, 115, 105, 116, 32,
    //             97, 109, 101, 116, 44, 32, 99, 111, 110, 115, 101, 99, 116, 101, 116, 117,
    //             114, 32, 97, 100, 105, 112, 105, 115, 99, 105, 110, 103, 32, 101, 108, 105,
    //             116, 44, 32, 115, 101, 100, 32, 100, 111, 32, 101, 105, 117, 115, 109, 111,
    //             100, 32, 116, 101, 109, 112, 111, 114, 32, 105, 110, 99, 105, 100, 105, 100,
    //             117, 110, 116, 32, 117, 116, 32, 108, 97, 98, 111, 114, 101, 32, 101, 116,
    //             32, 100, 111, 108, 111, 114, 101, 32, 109, 97, 103, 110, 97, 32, 97, 108,
    //             105, 113, 117, 97, 46
    //         ];
    //         WristApp::new(demo_data)
    //     }
    // };
    // protocol.add(wrist_app);
    
    // Add end command
    protocol.add(End);
    
    // Generate all packets
    let packets = protocol.packets();
    
    // Print number of packets
    println!("Generated {} packets for Protocol 3", packets.len());
    
    // Print packet summary
    println!("\nPacket summary:");
    for (i, packet) in packets.iter().enumerate().take(5) {
        // Only print first few bytes to avoid overwhelming output
        let preview_len = std::cmp::min(packet.len(), 16);
        let preview: Vec<u8> = packet.iter().take(preview_len).cloned().collect();
        println!("Packet {}: {} bytes, starts with {:02X?}{}",
                i + 1, packet.len(), preview,
                if packet.len() > preview_len { "..." } else { "" });
    }
    if packets.len() > 5 {
        println!("... and {} more packets", packets.len() - 5);
    }
    
    // If a serial port was provided, transmit the data
    if !serial_port.is_empty() {
        println!("\nTransmitting data to the watch on port: {}", serial_port);
        
        // Create the notebook adapter and send the packets
        let adapter = NotebookAdapter::new(
            serial_port,
            Some(0.014), // Use faster sleep times for the example
            Some(0.08), // Use faster sleep times for the example
            true, // Enable verbose output
        );
        
        match adapter.write(&packets) {
            Ok(_) => println!("\nSuccessfully transmitted data to the watch!"),
            Err(e) => {
                eprintln!("\nError transmitting data: {}", e);
                process::exit(1);
            }
        }
    } else {
        println!("\nNo serial port specified. Run with a serial port to transmit data to the watch:");
        println!("Example: cargo run --example protocol3_example /dev/ttyUSB0");
    }
}
