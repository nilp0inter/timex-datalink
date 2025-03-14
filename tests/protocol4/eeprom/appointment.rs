use timex_datalink::client::protocol4::{Appointment, DateTime};
use timex_datalink::client::helpers::length_packet_wrapper::length_packet_wrapper;

#[test]
fn test_appointment_packet() {
    // This mirrors the Ruby test case in protocol_4/eeprom/appointment_spec.rb
    let time = DateTime {
        year: 1997,
        month: 9,
        day: 19,
        hour: 0,
        minute: 0,
        second: 0,
    };
    
    let appointment = Appointment::new(time, "Release TIMEXDL.EXE".to_string());
    
    // The expected base packet - from Ruby test
    let expected_base_packet = vec![
        0x09, 0x13, 0x00, 0x9b, 0x53, 0x39, 0x0a, 0xe7, 0x90, 0x9d, 0x64, 0x39, 0x61, 0x53, 0xc9, 0x4e, 0xe8, 0xfc
    ];
    
    // We'll use the wrap_with_length function to generate the length-prefixed packet
    
    // First test the base packet (without length prefix)
    assert_eq!(appointment.packet(), expected_base_packet);
    
    // Manually apply the length prefix using the same algorithm as Ruby
    let length_prefixed = wrap_with_length(expected_base_packet.clone());
    assert_eq!(appointment.packet_with_length(), length_prefixed);
}

#[test]
fn test_appointment_with_different_date() {
    // Based on second test case in Ruby
    let time = DateTime {
        year: 2015,
        month: 10,
        day: 21,
        hour: 0,
        minute: 0,
        second: 0,
    };
    
    let appointment = Appointment::new(time, "Release TIMEXDL.EXE".to_string());
    
    // The expected base packet - from Ruby test
    let expected_base_packet = vec![
        0x0a, 0x15, 0x00, 0x9b, 0x53, 0x39, 0x0a, 0xe7, 0x90, 0x9d, 0x64, 0x39, 0x61, 0x53, 0xc9, 0x4e, 0xe8, 0xfc
    ];
    
    assert_eq!(appointment.packet(), expected_base_packet);
}

#[test]
fn test_appointment_with_long_message() {
    // Based on third test case in Ruby
    let time = DateTime {
        year: 1997,
        month: 9,
        day: 19,
        hour: 0,
        minute: 0,
        second: 0,
    };
    
    let appointment = Appointment::new(time, "To the Delorean with More Than 31 Characters".to_string());
    
    // The expected base packet - from Ruby test
    let expected_base_packet = vec![
        0x09, 0x13, 0x00, 0x1d, 0x46, 0x76, 0x91, 0x43, 0x36, 0x4e, 0x85, 0x6d, 0x8e, 0x72, 0x91, 0xa0, 0xd4, 0x45,
        0xa4, 0x85, 0x6d, 0x0e, 0xd9, 0x45, 0xca, 0x45, 0xfe
    ];
    
    assert_eq!(appointment.packet(), expected_base_packet);
}

#[test]
fn test_appointment_with_special_chars() {
    // Based on fourth test case in Ruby
    let time = DateTime {
        year: 1997,
        month: 9,
        day: 19,
        hour: 0,
        minute: 0,
        second: 0,
    };
    
    let appointment = Appointment::new(time, ";@_|<>[]".to_string());
    
    // The expected base packet - from Ruby test
    let expected_base_packet = vec![
        0x09, 0x13, 0x00, 0x36, 0xae, 0xef, 0x7c, 0xef, 0x93, 0x3f
    ];
    
    assert_eq!(appointment.packet(), expected_base_packet);
}

#[test]
fn test_appointment_with_invalid_chars() {
    // Based on fifth test case in Ruby
    let time = DateTime {
        year: 1997,
        month: 9,
        day: 19,
        hour: 0,
        minute: 0,
        second: 0,
    };
    
    let appointment = Appointment::new(time, "~with~invalid~characters".to_string());
    
    // The expected base packet - from Ruby test
    let expected_base_packet = vec![
        0x09, 0x13, 0x00, 0x24, 0x28, 0x75, 0x11, 0x29, 0x5d, 0x9f, 0x52, 0x49, 0x0d, 0xc9, 0x44, 0xca, 0xa6, 0x30,
        0x9d, 0xb3, 0x71, 0x3f
    ];
    
    assert_eq!(appointment.packet(), expected_base_packet);
}

// Helper function to test length packet wrapping
fn wrap_with_length(packet: Vec<u8>) -> Vec<u8> {
    length_packet_wrapper(&packet)
}

// Test that the length_packet_wrapper function works correctly
#[test]
fn test_length_packet_wrapper() {
    let packet = vec![1, 2, 3];
    let wrapped = wrap_with_length(packet);
    assert_eq!(wrapped, vec![4, 1, 2, 3]);
}