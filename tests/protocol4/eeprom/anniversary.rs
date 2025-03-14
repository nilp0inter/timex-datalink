use timex_datalink::client::protocol4::{Anniversary, DateTime};
use timex_datalink::client::helpers::length_packet_wrapper::length_packet_wrapper;

#[test]
fn test_anniversary_packet() {
    // Recreate the equivalent of Time.new(1997, 9, 19) from Ruby test
    let time = DateTime {
        year: 1997,
        month: 9,
        day: 19,
        hour: 0,
        minute: 0,
        second: 0,
    };
    
    let anniversary = Anniversary::new(time, "TIMEXDL.EXE modified date".to_string());
    
    // The packet should match the Ruby test expectation
    let expected = vec![
        0x09, 0x13, 0x9d, 0x64, 0x39, 0x61, 0x53, 0xc9, 0x4e, 0xe8, 0x90, 0x16, 0xd6, 0x48, 0x8f, 0xe4, 0x34, 0x64, 0xa3,
        0x74, 0xce, 0x0f
    ];
    
    assert_eq!(anniversary.packet(), expected);
    
    // Test the length-prefixed version
    let expected_with_length = length_packet_wrapper(&expected);
    assert_eq!(anniversary.packet_with_length(), expected_with_length);
}

#[test]
fn test_anniversary_packet_with_different_date() {
    // Recreate the equivalent of Time.new(2015, 10, 21) from Ruby test
    let time = DateTime {
        year: 2015,
        month: 10,
        day: 21,
        hour: 0,
        minute: 0,
        second: 0,
    };
    
    let anniversary = Anniversary::new(time, "TIMEXDL.EXE modified date".to_string());
    
    // The packet should match the Ruby test expectation
    let expected = vec![
        0x0a, 0x15, 0x9d, 0x64, 0x39, 0x61, 0x53, 0xc9, 0x4e, 0xe8, 0x90, 0x16, 0xd6, 0x48, 0x8f, 0xe4, 0x34, 0x64,
        0xa3, 0x74, 0xce, 0x0f
    ];
    
    assert_eq!(anniversary.packet(), expected);
}

#[test]
fn test_anniversary_packet_with_long_message() {
    let time = DateTime {
        year: 1997,
        month: 9,
        day: 19,
        hour: 0,
        minute: 0,
        second: 0,
    };
    
    let anniversary = Anniversary::new(time, "To the Delorean with More Than 31 Characters".to_string());
    
    // The packet should match the Ruby test expectation
    let expected = vec![
        0x09, 0x13, 0x1d, 0x46, 0x76, 0x91, 0x43, 0x36, 0x4e, 0x85, 0x6d, 0x8e, 0x72, 0x91, 0xa0, 0xd4, 0x45, 0xa4,
        0x85, 0x6d, 0x0e, 0xd9, 0x45, 0xca, 0x45, 0xfe
    ];
    
    assert_eq!(anniversary.packet(), expected);
}

#[test]
fn test_anniversary_packet_with_special_chars() {
    let time = DateTime {
        year: 1997,
        month: 9,
        day: 19,
        hour: 0,
        minute: 0,
        second: 0,
    };
    
    let anniversary = Anniversary::new(time, ";@_|<>[]".to_string());
    
    // The packet should match the Ruby test expectation
    let expected = vec![
        0x09, 0x13, 0x36, 0xae, 0xef, 0x7c, 0xef, 0x93, 0x3f
    ];
    
    assert_eq!(anniversary.packet(), expected);
}

#[test]
fn test_anniversary_packet_with_invalid_chars() {
    let time = DateTime {
        year: 1997,
        month: 9,
        day: 19,
        hour: 0,
        minute: 0,
        second: 0,
    };
    
    let anniversary = Anniversary::new(time, "~with~invalid~characters".to_string());
    
    // The packet should match the Ruby test expectation
    let expected = vec![
        0x09, 0x13, 0x24, 0x28, 0x75, 0x11, 0x29, 0x5d, 0x9f, 0x52, 0x49, 0x0d, 0xc9, 0x44, 0xca, 0xa6, 0x30, 0x9d,
        0xb3, 0x71, 0x3f
    ];
    
    assert_eq!(anniversary.packet(), expected);
}