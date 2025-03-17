use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process;
use std::time::SystemTime;

use chrono::{Datelike, TimeZone, Timelike, Utc, Local};
use clap::{Arg, ArgAction, Command};
use timex_datalink::{
    helpers::crc_packets_wrapper::wrap_packets_with_crc,
    char_encoders::CharString,
    devices::timex_datalink_150::TimexData,
    protocol_3::{
        eeprom::{Anniversary, Appointment, Eeprom, List, PhoneNumber},
        time::DateFormat,
        Alarm, End, SoundOptions, SoundTheme, Start, Sync, Time, WristApp,
    },
    NotebookAdapter, PacketGenerator, Protocol3,
};

// Custom beep model equivalent to Ruby's Beep class
struct Beep;

impl PacketGenerator for Beep {
    fn packets(&self) -> Vec<Vec<u8>> {
        let raw_packets = vec![vec![0x23, 0x04, 0x3e, 0xa6, 0x01, 0xc7, 0x00, 0x28, 0x81]];
        wrap_packets_with_crc(raw_packets)
    }
}

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

// Parse a time string to SystemTime
fn parse_time_string(time_str: &str) -> SystemTime {
    // Try to parse as RFC3339 first
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(time_str) {
        return system_time_from_date(
            dt.year(),
            dt.month(),
            dt.day(),
            dt.hour(),
            dt.minute(),
        );
    }
    
    // Try parsing ISO8601 format
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M:%S") {
        let dt_utc = Utc.from_utc_datetime(&dt);
        return system_time_from_date(
            dt_utc.year(),
            dt_utc.month(),
            dt_utc.day(),
            dt_utc.hour(),
            dt_utc.minute(),
        );
    }
    
    // Try parsing standard datetime format
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S") {
        let dt_utc = Utc.from_utc_datetime(&dt);
        return system_time_from_date(
            dt_utc.year(),
            dt_utc.month(),
            dt_utc.day(),
            dt_utc.hour(),
            dt_utc.minute(),
        );
    }
    
    // Try parsing just a date
    if let Ok(date) = chrono::NaiveDate::parse_from_str(time_str, "%Y-%m-%d") {
        let dt = date.and_hms_opt(0, 0, 0).unwrap();
        let dt_utc = Utc.from_utc_datetime(&dt);
        return system_time_from_date(
            dt_utc.year(),
            dt_utc.month(),
            dt_utc.day(),
            0,
            0,
        );
    }
    
    // If all parsing attempts fail, log and return current time
    eprintln!("Failed to parse time: {}", time_str);
    SystemTime::now()
}

fn parse_json_data(file_path: &str) -> Result<TimexData, String> {
    let mut file = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    serde_json::from_str(&contents).map_err(|e| format!("Failed to parse JSON: {}", e))
}

fn main() {
    let matches = Command::new("td150")
        .about("Timex Datalink 150 protocol 3 data transfer tool")
        .arg(Arg::new("json_file").help("JSON file with watch data"))
        .arg(
            Arg::new("sound-theme")
                .long("sound-theme")
                .help("Specify a sound theme SPC file")
                .value_name("SPC_FILE"),
        )
        .arg(
            Arg::new("wrist-app")
                .long("wrist-app")
                .help("Specify a wrist app ZAP file")
                .value_name("ZAP_FILE"),
        )
        .arg(
            Arg::new("serial-device")
                .long("serial-device")
                .help("Specify a serial device")
                .value_name("DEVICE")
                .default_value("/dev/ttyACM0"),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .help("Enable verbose mode")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-appointments")
                .long("no-appointments")
                .help("Skip creating appointments")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-anniversaries")
                .long("no-anniversaries")
                .help("Skip creating anniversaries")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-phone-numbers")
                .long("no-phone-numbers")
                .help("Skip creating phone numbers")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-lists")
                .long("no-lists")
                .help("Skip creating lists")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-alarms")
                .long("no-alarms")
                .help("Skip creating alarms")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-time")
                .long("no-time")
                .help("Skip creating time models")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("sync-length")
                .long("sync-length")
                .help("Specify the sync length")
                .value_name("LENGTH")
                .value_parser(clap::value_parser!(u8))
                .default_value("150"),
        )
        .arg(
            Arg::new("start-beep")
                .long("start-beep")
                .help("Send a start beep")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Get options from command line
    let json_file = matches.get_one::<String>("json_file");
    let sound_theme_file = matches.get_one::<String>("sound-theme");
    let wrist_app_file = matches.get_one::<String>("wrist-app");
    let serial_device = matches.get_one::<String>("serial-device").unwrap();
    let verbose = matches.get_flag("verbose");
    let no_appointments = matches.get_flag("no-appointments");
    let no_anniversaries = matches.get_flag("no-anniversaries");
    let no_phone_numbers = matches.get_flag("no-phone-numbers");
    let no_lists = matches.get_flag("no-lists");
    let no_alarms = matches.get_flag("no-alarms");
    let no_time = matches.get_flag("no-time");
    let sync_length = *matches.get_one::<u8>("sync-length").unwrap();
    let start_beep = matches.get_flag("start-beep");

    // Parse JSON data if a file was provided
    let data = match json_file {
        Some(path) => match parse_json_data(path) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error parsing JSON file: {}", e);
                process::exit(1);
            }
        },
        None => TimexData::new(),
    };

    // Create a new Protocol 3 instance
    let mut protocol = Protocol3::new();

    // Create appointments
    let mut appointments = Vec::new();
    if !no_appointments && !data.appointments.is_empty() {
        for appointment in &data.appointments {
            appointments.push(Appointment::new(
                parse_time_string(&appointment.time),
                appointment.message.clone(),
            ));
        }
    }

    // Create anniversaries
    let mut anniversaries = Vec::new();
    if !no_anniversaries && !data.anniversaries.is_empty() {
        let current_year = Utc::now().year();
        for anniversary in &data.anniversaries {
            let time = parse_time_string(&anniversary.time);
            
            // Extract the month and day, then set to current year - 1
            let dt = Utc.timestamp_opt(
                time.duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
                0,
            )
            .unwrap();
            
            let modified_time = system_time_from_date(
                current_year - 1,
                dt.month(),
                dt.day(),
                0,
                0,
            );

            anniversaries.push(Anniversary::new(
                modified_time,
                anniversary.anniversary.clone(),
            ));
        }

        // Sort anniversaries by date
        anniversaries.sort_by(|a, b| a.time.cmp(&b.time));
    }

    // Create phone numbers
    let mut phone_numbers = Vec::new();
    if !no_phone_numbers && !data.phone_numbers.is_empty() {
        for phone_number in &data.phone_numbers {
            phone_numbers.push(PhoneNumber::new(
                phone_number.name.clone(),
                phone_number.number.clone(),
                phone_number.r#type.clone(),
            ));
        }
    }

    // Create lists
    let mut lists = Vec::new();
    if !no_lists && !data.lists.is_empty() {
        for list in &data.lists {
            // Convert i32 to u8 for priority
            let priority_u8 = list.priority.map(|p| p.clamp(1, 5) as u8);
            lists.push(List::new(
                list.list_entry.clone(),
                priority_u8,
            ));
        }
    }

    // Create time models
    let mut time_models = Vec::new();
    if !no_time {
        // Get the current local time
        let local_now = Local::now();
        
        // Calculate the timezone offset in seconds
        let offset_seconds = local_now.offset().local_minus_utc() as i64;
        
        // Get the current UTC time
        let utc_now = Utc::now();
        
        // For time1, we need to create a SystemTime that, when interpreted as UTC by the watch,
        // will display as the correct local time
        let time1 = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(
            (utc_now.timestamp() + offset_seconds) as u64
        );
        
        // UTC time stays the same
        let time2 = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(utc_now.timestamp() as u64);

        // Try to get a more meaningful timezone name
        // First try to get the timezone from the TZ environment variable
        let iana_tz = match std::env::var("TZ") {
            Ok(tz) => tz,
            // If TZ is not set, use a default format based on the current offset
            Err(_) => format!("UTC{}", local_now.format("%:z")),
        };
        
        // Extract a meaningful name from the IANA timezone
        let tz_name = if iana_tz.contains('/') {
            // For IANA names like "Europe/Madrid", extract the city part
            let city = iana_tz.split('/').last().unwrap_or("HOME");
            
            // Clean up the city name (remove underscores, etc.)
            city.replace('_', " ").to_string()
        } else if iana_tz.starts_with("UTC") {
            // For UTC offsets, use a more friendly name
            "HOME".to_string()
        } else {
            // Use whatever we have
            iana_tz.clone()
        };
        
        if verbose {
            println!("Detected timezone: {}", iana_tz);
            println!("Using timezone name: {}", tz_name);
            println!("Setting {} time to: {} (offset: {} seconds)", 
                     tz_name,
                     local_now.format("%Y-%m-%d %H:%M:%S"),
                     offset_seconds);
            println!("Setting UTC time to: {}", utc_now.format("%Y-%m-%d %H:%M:%S"));
            
            // Convert the times back to DateTime objects for verification
            let time1_utc = Utc.timestamp_opt(
                time1.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64, 0
            ).unwrap();
            let time2_utc = Utc.timestamp_opt(
                time2.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64, 0
            ).unwrap();
            
            println!("Time values sent to watch:");
            println!("  Zone 1 ({}): {}", tz_name, time1_utc.format("%Y-%m-%d %H:%M:%S UTC"));
            println!("  Zone 2 (UTC): {}", time2_utc.format("%Y-%m-%d %H:%M:%S UTC"));
        }

        time_models.push(Time {
            zone: 1,
            is_24h: true,
            date_format: DateFormat::DayDashMonthDashYear,
            time: time1,
            name: CharString::new(&tz_name, true),
        });

        time_models.push(Time {
            zone: 2,
            is_24h: true,
            date_format: DateFormat::YearDotMonthDotDay,
            time: time2,
            name: CharString::new("UTC", true),
        });
    }

    // Create alarms
    let mut alarms = Vec::new();
    if !no_alarms && !data.alarms.is_empty() {
        for alarm in &data.alarms {
            alarms.push(Alarm {
                number: alarm.number,
                audible: alarm.audible,
                hour: alarm.hour,
                minute: alarm.minute,
                message: CharString::new(&alarm.message, true),
            });
        }
    }

    // Sound options
    let sound_options = data.sound_options.as_ref().map(|opts| SoundOptions {
        hourly_chime: opts.hourly_chime,
        button_beep: opts.button_beep,
    });

    // Start building the protocol model sequence
    protocol.add(Sync { length: sync_length as usize });
    protocol.add(Start);

    // Add start beep if requested
    if start_beep {
        protocol.add(Beep);
    }

    // Add time models
    for time_model in time_models {
        protocol.add(time_model);
    }

    // Add alarms
    for alarm in alarms {
        protocol.add(alarm);
    }

    // Create and add EEPROM data if we have any
    if !appointments.is_empty() || !anniversaries.is_empty() || !lists.is_empty() || !phone_numbers.is_empty() {
        let mut eeprom = Eeprom::new();
        eeprom.appointments = appointments;
        eeprom.anniversaries = anniversaries;
        eeprom.lists = lists;
        eeprom.phone_numbers = phone_numbers;
        eeprom.appointment_notification_minutes = data.appointment_notification_minutes;
        
        protocol.add(eeprom);
    }

    // Add sound theme if provided
    if let Some(sound_theme_path) = sound_theme_file {
        let path = PathBuf::from(sound_theme_path);
        match SoundTheme::from_spc_file(&path) {
            Ok(theme) => protocol.add(theme),
            Err(e) => {
                eprintln!("Error loading sound theme: {}", e);
                process::exit(1);
            }
        }
    }

    // Add wrist app if provided
    if let Some(wrist_app_path) = wrist_app_file {
        let path = PathBuf::from(wrist_app_path);
        match WristApp::from_zap_file(path) {
            Ok(app) => protocol.add(app),
            Err(e) => {
                eprintln!("Error loading wrist app: {}", e);
                process::exit(1);
            }
        }
    }

    // Add sound options if provided
    if let Some(opts) = sound_options {
        protocol.add(opts);
    }

    // Add end marker
    protocol.add(End);

    // Generate all packets
    let packets = protocol.packets();

    if verbose {
        println!("Generated {} packets for Protocol 3", packets.len());
    }

    // Create the notebook adapter and send the packets
    if !serial_device.is_empty() {
        if verbose {
            println!("Transmitting data to the watch on port: {}", serial_device);
        }
        
        let adapter = NotebookAdapter::new(
            serial_device.to_string(),
            None,     // Use default sleep time
            None,     // Use default sleep time
            verbose,  // Verbosity flag from command line
        );
        
        match adapter.write(&packets) {
            Ok(_) => {
                if verbose {
                    println!("Successfully transmitted data to the watch!");
                }
            },
            Err(e) => {
                eprintln!("Error transmitting data: {}", e);
                process::exit(1);
            }
        }
    } else {
        println!("No serial port specified. Running in preview mode only.");
    }
}
