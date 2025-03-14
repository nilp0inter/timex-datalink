use timex_datalink::client::protocol4::{
    Protocol4, DateTime, TimeOfDay, DateFormat,
    Appointment, Anniversary, PhoneNumber, List, 
    SoundThemeData, WristAppData
};
use std::path::Path;

pub fn main() {
    // Create protocol elements using the new constructor methods
    let now = DateTime::now();
    let time_now = TimeOfDay::now();
    
    let data = vec![
        // Basic commands
        Protocol4::sync(),
        Protocol4::start(),
        
        // Time setting with date format
        Protocol4::Time {
            zone: 1,
            name: "Home".to_string(),
            time: now.clone(),
            is_24h: false,
            date_format: DateFormat::MonthDayYear,
        },
        
        // Alternative constructor for Time
        Protocol4::time(2, "Work", now.clone()),
        
        // Alarm with message
        Protocol4::alarm(1, time_now.clone(), "Wake up"),
        
        // EEPROM with collections
        {
            // Create base EEPROM
            let mut eeprom = Protocol4::eeprom();
            
            // Add elements to collections - would need to match for mutability in real code
            if let Protocol4::Eeprom { 
                appointments, 
                anniversaries, 
                phone_numbers, 
                lists, 
                appointment_notification_minutes 
            } = &mut eeprom {
                // Add appointments
                appointments.push(Appointment {
                    time: DateTime::new(2023, 10, 15, 9, 0, 0),
                    message: "Dentist".to_string(),
                });
                appointments.push(Appointment {
                    time: DateTime::new(2023, 10, 16, 14, 30, 0),
                    message: "Meeting".to_string(),
                });
                
                // Add anniversaries
                anniversaries.push(Anniversary {
                    time: DateTime::new(2023, 12, 25, 0, 0, 0),
                    anniversary: "Christmas".to_string(),
                });
                
                // Add phone numbers
                phone_numbers.push(PhoneNumber {
                    name: "John".to_string(),
                    number: "123-456-7890".to_string(),
                    type_: "Home".to_string(),
                });
                
                // Add list items
                lists.push(List {
                    list_entry: "Groceries".to_string(),
                    priority: 1,
                });
                
                // Set notification time
                *appointment_notification_minutes = Some(15);
            }
            
            eeprom
        },
        
        // Sound theme with default settings
        Protocol4::sound_theme("default.spc"),
        
        // Sound options
        {
            let mut options = Protocol4::sound_options();
            if let Protocol4::SoundOptions { hourly_chime, button_beep } = &mut options {
                *hourly_chime = true;
            }
            options
        },
        
        // WristApp
        Protocol4::wrist_app("calculator.zap"),
        
        // End command
        Protocol4::end()
    ];

    // Serialize to YAML and print
    let serialized = serde_yaml::to_string(&data).unwrap();
    println!("--- YAML Output ---");
    println!("{}", serialized);
    println!();
    
    // Serialize to JSON and print
    let json = serde_json::to_string_pretty(&data).unwrap();
    println!("--- JSON Output ---");
    println!("{}", json);
    
    // Example of loading files (this would be used if the files existed)
    println!("\n--- File Loading Examples (not actually executed) ---");
    println!("// Load a sound theme from an SPC file");
    println!("if let Ok(sound_theme) = Protocol4::sound_theme_from_file(\"theme.spc\", \"path/to/theme.spc\") {{");
    println!("    // Use the loaded sound theme");
    println!("}}");
    
    println!("\n// Load a WristApp from a ZAP file");
    println!("if let Ok(wrist_app) = Protocol4::wrist_app_from_file(\"app.zap\", \"path/to/app.zap\") {{");
    println!("    // Use the loaded WristApp");
    println!("}}");
}

