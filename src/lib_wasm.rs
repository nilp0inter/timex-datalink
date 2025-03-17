use std::time::SystemTime;
use wasm_bindgen::prelude::*;
use serde::Deserialize;
use crate::{
    Protocol3, PacketGenerator,
    protocol_3::{
        Sync, Start, End, Time, Alarm, SoundOptions, SoundTheme, WristApp,
        eeprom::{Eeprom, Appointment, Anniversary, PhoneNumber, List}
    },
    char_encoders::CharString,
    protocol_3::time::DateFormat,
    helpers::crc_packets_wrapper::wrap_packets_with_crc,
};

// Console logging macro for WebAssembly
#[cfg(target_arch = "wasm32")]
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}

#[cfg(not(target_arch = "wasm32"))]
macro_rules! console_log {
    ($($t:tt)*) => {
        println!($($t)*);
    };
}

// Start beep has been removed

// Function to convert JavaScript Date string to SystemTime
fn parse_datetime_string(date_str: &str) -> SystemTime {
    #[cfg(target_arch = "wasm32")]
    console_log!("Parsing datetime: {}", date_str);
    
    // Try to parse ISO 8601 format (YYYY-MM-DDTHH:MM)
    if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(&format!("{}:00Z", date_str.replace("T", "T"))) {
        return SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(datetime.timestamp() as u64);
    }
    
    // Try to parse just a date (YYYY-MM-DD)
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let datetime = date.and_hms_opt(0, 0, 0).unwrap();
        let timestamp = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(datetime, chrono::Utc).timestamp();
        return SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp as u64);
    }
    
    // If parsing fails, return current time
    #[cfg(target_arch = "wasm32")]
    console_log!("Failed to parse datetime, using current time");
    
    SystemTime::now()
}

// Convert a string representation of DateFormat to the enum value
fn parse_date_format(format_str: &str) -> DateFormat {
    match format_str {
        "MonthDashDayDashYear" => DateFormat::MonthDashDayDashYear,
        "DayDashMonthDashYear" => DateFormat::DayDashMonthDashYear,
        "YearDashMonthDashDay" => DateFormat::YearDashMonthDashDay,
        "MonthDotDayDotYear" => DateFormat::MonthDotDayDotYear,
        "DayDotMonthDotYear" => DateFormat::DayDotMonthDotYear,
        "YearDotMonthDotDay" => DateFormat::YearDotMonthDotDay,
        _ => DateFormat::DayDashMonthDashYear, // Default
    }
}

// Data structure for form data
#[derive(Deserialize, Debug)]
struct FormAlarm {
    number: u8,
    audible: bool,
    hour: u8,
    minute: u8,
    message: String,
}

#[derive(Deserialize, Debug)]
struct FormAppointment {
    date: String,
    message: String,
}

#[derive(Deserialize, Debug)]
struct FormAnniversary {
    date: String,
    message: String,
}

#[derive(Deserialize, Debug)]
struct FormPhoneNumber {
    name: String,
    number: String,
    #[serde(rename = "type")]
    type_code: String,
}

#[derive(Deserialize, Debug)]
struct FormList {
    entry: String,
    priority: i32,
}

#[derive(Deserialize, Debug)]
struct FormTimeSettings {
    zone: u8,
    name: String,
    is24h: bool,
    #[serde(rename = "dateFormat")]
    date_format: String,
}

#[derive(Deserialize, Debug)]
struct FormSoundOptions {
    hourlyChime: bool,
    buttonBeep: bool,
}

#[derive(Deserialize, Debug)]
struct FormData {
    includeTime: bool,
    time1: FormTimeSettings,
    time2: FormTimeSettings,
    includeAlarms: bool,
    alarms: Vec<FormAlarm>,
    includeEeprom: bool,
    #[serde(rename = "appointmentNotification")]
    appointment_notification_minutes: Option<u8>,
    appointments: Vec<FormAppointment>,
    anniversaries: Vec<FormAnniversary>,
    #[serde(rename = "phoneNumbers")]
    phone_numbers: Vec<FormPhoneNumber>,
    lists: Vec<FormList>,
    includeSoundOptions: bool,
    soundOptions: FormSoundOptions,
    includeSoundTheme: bool,
    soundThemeData: Option<Vec<u8>>,
    includeWristApp: bool,
    wristAppData: Option<Vec<u8>>,
    syncLength: u8,
}

// Simple packet generator for demo purposes
#[wasm_bindgen(js_name = generate_demo_packets)]
pub fn generate_demo_packets() -> JsValue {
    let demo_packets = vec![
        vec![0x01, 0x02, 0x03, 0x04],
        vec![0x05, 0x06, 0x07, 0x08],
        vec![0x09, 0x0A, 0x0B, 0x0C],
    ];
    
    serde_wasm_bindgen::to_value(&demo_packets).unwrap_or_else(|_| JsValue::NULL)
}

// Main packet generator function
#[wasm_bindgen(js_name = generate_protocol3_packets)]
pub fn generate_protocol3_packets(form_data: JsValue) -> JsValue {
    #[cfg(target_arch = "wasm32")]
    console_log!("Processing form data for Protocol 3 packets (skipping Time)...");
    
    // Create a new Protocol 3 instance
    let mut protocol = Protocol3::new();
    
    // Extract sync length from form data (or use default)
    let sync_length = match js_sys::Reflect::get(&form_data, &JsValue::from_str("syncLength")) {
        Ok(value) => {
            if let Some(num) = value.as_f64() {
                num as usize
            } else {
                150 // Default
            }
        },
        Err(_) => 150 // Default
    };
    
    // Add Sync and Start
    #[cfg(target_arch = "wasm32")]
    console_log!("Adding Sync (length: {}) and Start", sync_length);
    protocol.add(Sync { length: sync_length });
    protocol.add(Start);
    
    // Handle time data
    let include_time = match js_sys::Reflect::get(&form_data, &JsValue::from_str("includeTime")) {
        Ok(value) => value.as_bool().unwrap_or(false),
        Err(_) => false
    };
    
    if include_time {
        #[cfg(target_arch = "wasm32")]
        console_log!("Processing time settings...");
        
        // Convert to SystemTime (time1 and time2 will have the same timestamp from the form)
        
        // Process Time Zone 1
        if let Ok(time1_obj) = js_sys::Reflect::get(&form_data, &JsValue::from_str("time1")) {
            if let Ok(time1) = time1_obj.dyn_into::<js_sys::Object>() {
                let name = match js_sys::Reflect::get(&time1, &JsValue::from_str("name")) {
                    Ok(val) => val.as_string().unwrap_or_else(|| "HOM".to_string()),
                    Err(_) => "HOM".to_string()
                };
                
                let is_24h = match js_sys::Reflect::get(&time1, &JsValue::from_str("is24h")) {
                    Ok(val) => val.as_bool().unwrap_or(true),
                    Err(_) => true
                };
                
                let date_format_str = match js_sys::Reflect::get(&time1, &JsValue::from_str("dateFormat")) {
                    Ok(val) => val.as_string(),
                    Err(_) => None
                };
                
                let date_format = match date_format_str.as_deref() {
                    Some("MonthDashDayDashYear") => DateFormat::MonthDashDayDashYear,
                    Some("DayDashMonthDashYear") => DateFormat::DayDashMonthDashYear,
                    Some("YearDashMonthDashDay") => DateFormat::YearDashMonthDashDay,
                    Some("MonthDotDayDotYear") => DateFormat::MonthDotDayDotYear,
                    Some("DayDotMonthDotYear") => DateFormat::DayDotMonthDotYear,
                    Some("YearDotMonthDotDay") => DateFormat::YearDotMonthDotDay,
                    _ => DateFormat::DayDashMonthDashYear, // Default
                };
                
                // Get timestamp from JS
                let timestamp = match js_sys::Reflect::get(&time1, &JsValue::from_str("timestamp")) {
                    Ok(ts) => {
                        if let Some(ts_val) = ts.as_f64() {
                            ts_val as u64
                        } else {
                            // Fallback to current time if timestamp is invalid
                            let now = js_sys::Date::new_0().get_time() / 1000.0;
                            now as u64
                        }
                    },
                    Err(_) => {
                        // Fallback to current time if timestamp is not provided
                        let now = js_sys::Date::new_0().get_time() / 1000.0;
                        now as u64
                    }
                };
                
                let system_time = std::time::UNIX_EPOCH + std::time::Duration::from_secs(timestamp);
                
                #[cfg(target_arch = "wasm32")]
                console_log!("Adding Time Zone 1: {}, 24h: {}, timestamp: {}", name, is_24h, timestamp);
                
                protocol.add(Time {
                    zone: 1,
                    is_24h,
                    date_format,
                    time: system_time,
                    name: CharString::new(&name, true),
                });
            }
        }
        
        // Process Time Zone 2
        if let Ok(time2_obj) = js_sys::Reflect::get(&form_data, &JsValue::from_str("time2")) {
            if let Ok(time2) = time2_obj.dyn_into::<js_sys::Object>() {
                let name = match js_sys::Reflect::get(&time2, &JsValue::from_str("name")) {
                    Ok(val) => val.as_string().unwrap_or_else(|| "UTC".to_string()),
                    Err(_) => "UTC".to_string()
                };
                
                let is_24h = match js_sys::Reflect::get(&time2, &JsValue::from_str("is24h")) {
                    Ok(val) => val.as_bool().unwrap_or(true),
                    Err(_) => true
                };
                
                let date_format_str = match js_sys::Reflect::get(&time2, &JsValue::from_str("dateFormat")) {
                    Ok(val) => val.as_string(),
                    Err(_) => None
                };
                
                let date_format = match date_format_str.as_deref() {
                    Some("MonthDashDayDashYear") => DateFormat::MonthDashDayDashYear,
                    Some("DayDashMonthDashYear") => DateFormat::DayDashMonthDashYear,
                    Some("YearDashMonthDashDay") => DateFormat::YearDashMonthDashDay,
                    Some("MonthDotDayDotYear") => DateFormat::MonthDotDayDotYear,
                    Some("DayDotMonthDotYear") => DateFormat::DayDotMonthDotYear,
                    Some("YearDotMonthDotDay") => DateFormat::YearDotMonthDotDay,
                    _ => DateFormat::YearDotMonthDotDay, // Default
                };
                
                // Get timestamp from JS
                let timestamp = match js_sys::Reflect::get(&time2, &JsValue::from_str("timestamp")) {
                    Ok(ts) => {
                        if let Some(ts_val) = ts.as_f64() {
                            ts_val as u64
                        } else {
                            // Fallback to current time if timestamp is invalid
                            let now = js_sys::Date::new_0().get_time() / 1000.0;
                            now as u64
                        }
                    },
                    Err(_) => {
                        // Fallback to current time if timestamp is not provided
                        let now = js_sys::Date::new_0().get_time() / 1000.0;
                        now as u64
                    }
                };
                
                let system_time = std::time::UNIX_EPOCH + std::time::Duration::from_secs(timestamp);
                
                #[cfg(target_arch = "wasm32")]
                console_log!("Adding Time Zone 2: {}, 24h: {}, timestamp: {}", name, is_24h, timestamp);
                
                protocol.add(Time {
                    zone: 2,
                    is_24h,
                    date_format,
                    time: system_time,
                    name: CharString::new(&name, true),
                });
            }
        }
    }
    
    // Check if we should include alarms
    let include_alarms = match js_sys::Reflect::get(&form_data, &JsValue::from_str("includeAlarms")) {
        Ok(value) => value.as_bool().unwrap_or(false),
        Err(_) => false
    };
    
    if include_alarms {
        #[cfg(target_arch = "wasm32")]
        console_log!("Processing alarms...");
        
        if let Ok(alarms_array) = js_sys::Reflect::get(&form_data, &JsValue::from_str("alarms")) {
            if let Ok(alarms) = js_sys::Array::from(&alarms_array).dyn_into::<js_sys::Array>() {
                let alarms_len = alarms.length();
                #[cfg(target_arch = "wasm32")]
                console_log!("Found {} alarms", alarms_len);
                
                for i in 0..alarms_len {
                    if let Ok(alarm_obj) = alarms.get(i).dyn_into::<js_sys::Object>() {
                        let number = match js_sys::Reflect::get(&alarm_obj, &JsValue::from_str("number")) {
                            Ok(num) => num.as_f64().unwrap_or(1.0) as u8,
                            Err(_) => 1
                        };
                        
                        let audible = match js_sys::Reflect::get(&alarm_obj, &JsValue::from_str("audible")) {
                            Ok(val) => val.as_bool().unwrap_or(true),
                            Err(_) => true
                        };
                        
                        let hour = match js_sys::Reflect::get(&alarm_obj, &JsValue::from_str("hour")) {
                            Ok(val) => val.as_f64().unwrap_or(9.0) as u8,
                            Err(_) => 9
                        };
                        
                        let minute = match js_sys::Reflect::get(&alarm_obj, &JsValue::from_str("minute")) {
                            Ok(val) => val.as_f64().unwrap_or(0.0) as u8,
                            Err(_) => 0
                        };
                        
                        let message = match js_sys::Reflect::get(&alarm_obj, &JsValue::from_str("message")) {
                            Ok(val) => val.as_string().unwrap_or_else(|| format!("ALARM {}", number)),
                            Err(_) => format!("ALARM {}", number)
                        };
                        
                        #[cfg(target_arch = "wasm32")]
                        console_log!("Adding alarm {}: {}:{:02} - {}", number, hour, minute, message);
                        
                        protocol.add(Alarm {
                            number,
                            audible,
                            hour,
                            minute,
                            message: CharString::new(&message, true),
                        });
                    }
                }
            }
        }
    }
    
    // Create EEPROM for appointments, anniversaries, phone numbers, and lists
    let mut eeprom = Eeprom::new();
    
    // Check if EEPROM data should be included (single global toggle)
    let include_eeprom = match js_sys::Reflect::get(&form_data, &JsValue::from_str("includeEeprom")) {
        Ok(value) => value.as_bool().unwrap_or(false),
        Err(_) => false
    };
    
    if include_eeprom {
        #[cfg(target_arch = "wasm32")]
        console_log!("Processing EEPROM data...");
        
        // Get appointment notification minutes
        if let Ok(notification_val) = js_sys::Reflect::get(&form_data, &JsValue::from_str("appointmentNotification")) {
            if let Some(minutes) = notification_val.as_f64() {
                if minutes >= 0.0 {
                    eeprom.appointment_notification_minutes = Some(minutes as u8);
                    #[cfg(target_arch = "wasm32")]
                    console_log!("Set appointment notification: {} minutes", minutes);
                }
            }
        }
        
        // Process appointments array
        if let Ok(appointments_array) = js_sys::Reflect::get(&form_data, &JsValue::from_str("appointments")) {
            if let Ok(appointments) = js_sys::Array::from(&appointments_array).dyn_into::<js_sys::Array>() {
                let appointments_len = appointments.length();
                #[cfg(target_arch = "wasm32")]
                console_log!("Found {} appointments", appointments_len);
                
                for i in 0..appointments_len {
                    if let Ok(appt_obj) = appointments.get(i).dyn_into::<js_sys::Object>() {
                        // Get message
                        let message = match js_sys::Reflect::get(&appt_obj, &JsValue::from_str("message")) {
                            Ok(val) => val.as_string().unwrap_or_else(|| "Appointment".to_string()),
                            Err(_) => "Appointment".to_string()
                        };
                        
                        // Use Unix epoch as a placeholder for time since we can't use SystemTime correctly
                        let time = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_671_000_000);
                        
                        eeprom.appointments.push(Appointment::new(time, message));
                    }
                }
            }
        }
        
        // Process anniversaries array
        if let Ok(anniversaries_array) = js_sys::Reflect::get(&form_data, &JsValue::from_str("anniversaries")) {
            if let Ok(anniversaries) = js_sys::Array::from(&anniversaries_array).dyn_into::<js_sys::Array>() {
                let anniversaries_len = anniversaries.length();
                #[cfg(target_arch = "wasm32")]
                console_log!("Found {} anniversaries", anniversaries_len);
                
                for i in 0..anniversaries_len {
                    if let Ok(anniv_obj) = anniversaries.get(i).dyn_into::<js_sys::Object>() {
                        // Get message
                        let message = match js_sys::Reflect::get(&anniv_obj, &JsValue::from_str("message")) {
                            Ok(val) => val.as_string().unwrap_or_else(|| "Anniversary".to_string()),
                            Err(_) => "Anniversary".to_string()
                        };
                        
                        // Use Unix epoch as a placeholder time
                        let time = std::time::UNIX_EPOCH;
                        
                        eeprom.anniversaries.push(Anniversary::new(time, message));
                    }
                }
            }
        }
        
        // Process phone numbers array
        if let Ok(phones_array) = js_sys::Reflect::get(&form_data, &JsValue::from_str("phoneNumbers")) {
            if let Ok(phones) = js_sys::Array::from(&phones_array).dyn_into::<js_sys::Array>() {
                let phones_len = phones.length();
                #[cfg(target_arch = "wasm32")]
                console_log!("Found {} phone numbers", phones_len);
                
                for i in 0..phones_len {
                    if let Ok(phone_obj) = phones.get(i).dyn_into::<js_sys::Object>() {
                        // Get name
                        let name = match js_sys::Reflect::get(&phone_obj, &JsValue::from_str("name")) {
                            Ok(val) => val.as_string().unwrap_or_else(|| "Contact".to_string()),
                            Err(_) => "Contact".to_string()
                        };
                        
                        // Get number
                        let number = match js_sys::Reflect::get(&phone_obj, &JsValue::from_str("number")) {
                            Ok(val) => val.as_string().unwrap_or_else(|| "0000000000".to_string()),
                            Err(_) => "0000000000".to_string()
                        };
                        
                        // Get type
                        let type_code = match js_sys::Reflect::get(&phone_obj, &JsValue::from_str("type")) {
                            Ok(val) => val.as_string(),
                            Err(_) => None
                        };
                        
                        eeprom.phone_numbers.push(PhoneNumber::new(name, number, type_code));
                    }
                }
            }
        }
        
        // Process lists array
        if let Ok(lists_array) = js_sys::Reflect::get(&form_data, &JsValue::from_str("lists")) {
            if let Ok(lists) = js_sys::Array::from(&lists_array).dyn_into::<js_sys::Array>() {
                let lists_len = lists.length();
                #[cfg(target_arch = "wasm32")]
                console_log!("Found {} list items", lists_len);
                
                for i in 0..lists_len {
                    if let Ok(list_obj) = lists.get(i).dyn_into::<js_sys::Object>() {
                        // Get entry
                        let entry = match js_sys::Reflect::get(&list_obj, &JsValue::from_str("entry")) {
                            Ok(val) => val.as_string().unwrap_or_else(|| "List item".to_string()),
                            Err(_) => "List item".to_string()
                        };
                        
                        // Get priority
                        let priority = match js_sys::Reflect::get(&list_obj, &JsValue::from_str("priority")) {
                            Ok(val) => {
                                if let Some(prio) = val.as_f64() {
                                    if prio > 0.0 && prio <= 5.0 {
                                        Some(prio as u8)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            },
                            Err(_) => None
                        };
                        
                        eeprom.lists.push(List::new(entry, priority));
                    }
                }
            }
        }
        
        // Add EEPROM data
        #[cfg(target_arch = "wasm32")]
        console_log!("Adding EEPROM data");
        protocol.add(eeprom);
    }
    
    // Process sound options
    let include_sound_options = match js_sys::Reflect::get(&form_data, &JsValue::from_str("includeSoundOptions")) {
        Ok(value) => value.as_bool().unwrap_or(true),
        Err(_) => true
    };
    
    if include_sound_options {
        #[cfg(target_arch = "wasm32")]
        console_log!("Adding sound options");
        
        let hourly_chime = match js_sys::Reflect::get(&form_data, &JsValue::from_str("soundOptions"))
            .and_then(|opts| js_sys::Reflect::get(&opts, &JsValue::from_str("hourlyChime"))) {
            Ok(val) => val.as_bool().unwrap_or(true),
            Err(_) => true
        };
        
        let button_beep = match js_sys::Reflect::get(&form_data, &JsValue::from_str("soundOptions"))
            .and_then(|opts| js_sys::Reflect::get(&opts, &JsValue::from_str("buttonBeep"))) {
            Ok(val) => val.as_bool().unwrap_or(true),
            Err(_) => true
        };
        
        protocol.add(SoundOptions {
            hourly_chime,
            button_beep,
        });
    }
    
    // Process sound theme
    let include_sound_theme = match js_sys::Reflect::get(&form_data, &JsValue::from_str("includeSoundTheme")) {
        Ok(value) => value.as_bool().unwrap_or(false),
        Err(_) => false
    };
    
    if include_sound_theme {
        #[cfg(target_arch = "wasm32")]
        console_log!("Processing sound theme...");
        
        // Get sound theme data
        if let Ok(sound_theme_array) = js_sys::Reflect::get(&form_data, &JsValue::from_str("soundThemeData")) {
            if let Ok(js_array) = js_sys::Array::from(&sound_theme_array).dyn_into::<js_sys::Array>() {
                let array_len = js_array.length() as usize;
                let mut sound_theme_bytes = Vec::with_capacity(array_len);
                
                #[cfg(target_arch = "wasm32")]
                console_log!("Sound theme data length: {}", array_len);
                
                for i in 0..array_len {
                    if let Ok(value) = js_array.get(i as u32).dyn_into::<js_sys::Number>() {
                        sound_theme_bytes.push(value.value_of() as u8);
                    }
                }
                
                // Define the header that needs to be removed
                const SOUND_DATA_HEADER: &[u8] = &[0x25, 0x04, 0x19, 0x69];
                
                // Remove the header if present
                let sound_data = if sound_theme_bytes.starts_with(SOUND_DATA_HEADER) {
                    sound_theme_bytes[SOUND_DATA_HEADER.len()..].to_vec()
                } else {
                    sound_theme_bytes
                };
                
                #[cfg(target_arch = "wasm32")]
                console_log!("Adding Sound Theme ({} bytes)", sound_data.len());
                
                protocol.add(SoundTheme::new(sound_data));
            }
        }
    }
    
    // Process wrist app
    let include_wrist_app = match js_sys::Reflect::get(&form_data, &JsValue::from_str("includeWristApp")) {
        Ok(value) => value.as_bool().unwrap_or(false),
        Err(_) => false
    };
    
    if include_wrist_app {
        #[cfg(target_arch = "wasm32")]
        console_log!("Processing wrist app...");
        
        // Get wrist app data
        if let Ok(wrist_app_array) = js_sys::Reflect::get(&form_data, &JsValue::from_str("wristAppData")) {
            if let Ok(js_array) = js_sys::Array::from(&wrist_app_array).dyn_into::<js_sys::Array>() {
                let array_len = js_array.length() as usize;
                let mut wrist_app_bytes = Vec::with_capacity(array_len);
                
                #[cfg(target_arch = "wasm32")]
                console_log!("Wrist app data length: {}", array_len);
                
                for i in 0..array_len {
                    if let Ok(value) = js_array.get(i as u32).dyn_into::<js_sys::Number>() {
                        wrist_app_bytes.push(value.value_of() as u8);
                    }
                }
                
                // Process the ZAP file if it looks like one
                let wrist_app_data = if wrist_app_bytes.iter().any(|&b| b == 0xAC) {
                    match WristApp::parse_zap_file(&wrist_app_bytes) {
                        Ok(data) => {
                            #[cfg(target_arch = "wasm32")]
                            console_log!("Successfully parsed ZAP file ({} bytes)", data.len());
                            data
                        },
                        Err(e) => {
                            #[cfg(target_arch = "wasm32")]
                            console_log!("Error parsing ZAP file: {:?}. Using raw data.", e);
                            wrist_app_bytes
                        }
                    }
                } else {
                    wrist_app_bytes
                };
                
                #[cfg(target_arch = "wasm32")]
                console_log!("Adding Wrist App ({} bytes)", wrist_app_data.len());
                
                protocol.add(WristApp { wrist_app_data });
            }
        }
    }
    
    // Add End marker
    protocol.add(End);
    
    // Generate packets
    #[cfg(target_arch = "wasm32")]
    console_log!("Generating packets...");
    
    let packets = protocol.packets();
    
    #[cfg(target_arch = "wasm32")]
    console_log!("Generated {} packets", packets.len());
    
    // Convert the packets to a JavaScript array directly
    let js_array = js_sys::Array::new();
    
    for packet in packets {
        let js_packet = js_sys::Uint8Array::new_with_length(packet.len() as u32);
        js_packet.copy_from(&packet);
        js_array.push(&js_packet.into());
    }
    
    js_array.into()
}