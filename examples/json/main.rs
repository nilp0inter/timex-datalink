use timex_datalink::client::protocol4::{Protocol4, Time, Alarm, Eeprom, SoundTheme, SoundOptions, WristApp, DateTime, TimeOfDay, Appointment, Anniversary, PhoneNumber, List};

pub fn main() -> () {
    let data = vec![
        Protocol4::Sync,
        Protocol4::Start,
        Protocol4::Time(Time {
            zone: 1,
            time: DateTime,
            is_24h: false,
            date_format: "%_m-%d-%y".to_string(),
        }),
        Protocol4::Alarm(Alarm {
            number: 1,
            audible: true,
            time: TimeOfDay,
            message: "Wake up".to_string(),
        }),
        Protocol4::Eeprom(Eeprom {
            appointments: vec![
                Appointment {
                    time: DateTime,
                    message: "Dentist".to_string(),
                },
                Appointment {
                    time: DateTime,
                    message: "Meeting".to_string(),
                },
            ],
            anniversaries: vec![
                Anniversary {
                    time: DateTime,
                    anniversary: "Birthday".to_string(),
                },
            ],
            phone_numbers: vec![
                PhoneNumber {
                    name: "John".to_string(),
                    number: "123-456-7890".to_string(),
                    type_: "Home".to_string(),
                },
            ],
            lists: vec![
                List {
                    list_entry: "Groceries".to_string(),
                    priority: 1,
                },
            ],
            appointment_notification_minutes: None,
        }),
        Protocol4::SoundTheme(SoundTheme {
            spc_file: "spe_file".to_string(),
        }),
        Protocol4::SoundOptions(SoundOptions {
            hourly_chime: true,
            button_beep: true,
        }),
        Protocol4::WristApp(WristApp { zap_file: "zap_file".to_string() }),
        Protocol4::End
    ];

    let serialized = serde_yaml::to_string(&data).unwrap();
    println!("{}", serialized);

}

