//! Alarm implementation for Protocol 3
//!
//! This module handles alarm functionality for Timex Datalink watches.

use crate::PacketGenerator;
use crate::helpers::crc_packets_wrapper;
use crate::char_encoders::CharString;

/// Alarm structure for Protocol 3
///
/// This structure handles alarms with a message and configurable sound.
pub struct Alarm {
    /// Alarm number (1-5)
    pub number: u8,
    
    /// Whether the alarm makes sound when triggered
    pub audible: bool,
    
    /// Hour of the alarm (0-23)
    pub hour: u8,
    
    /// Minute of the alarm (0-59)
    pub minute: u8,
    
    /// Message to display (8 chars max)
    pub message: CharString<8>,
}

impl PacketGenerator for Alarm {
    fn packets(&self) -> Vec<Vec<u8>> {
        // Define constants from Ruby implementation
        const CPACKET_ALARM: u8 = 0x50;

        // Create the raw packet
        let mut raw_packet = Vec::with_capacity(16);
        raw_packet.push(CPACKET_ALARM);   // Alarm command
        raw_packet.push(self.number);      // Alarm number (1-5)
        raw_packet.push(self.hour);        // Hour
        raw_packet.push(self.minute);      // Minute
        raw_packet.push(0);                // Unused bytes in protocol 3
        raw_packet.push(0);                // Unused bytes in protocol 3
        
        // Add message characters (8 chars)
        for &byte in self.message.as_bytes() {
            raw_packet.push(byte);
        }
        
        // Audible flag
        raw_packet.push(if self.audible { 1 } else { 0 });
        
        // Apply CRC wrapping
        crc_packets_wrapper::wrap_packets_with_crc(vec![raw_packet])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alarm_basic() {
        let alarm = Alarm {
            number: 1,
            audible: true,
            hour: 9,
            minute: 0,
            message: CharString::new("Wake up", true),
        };

        // From golden fixture: alarm_basic.jsonl
        #[rustfmt::skip]
        let expected = vec![vec![
            18, 80, 1, 9, 0, 0, 0, 32, 10, 20, 14, 36, 30, 25, 36, 1, 32, 240
        ]];

        assert_eq!(alarm.packets(), expected);
    }

    #[test]
    fn test_alarm_silent() {
        let alarm = Alarm {
            number: 3,
            audible: false,
            hour: 9,
            minute: 10,
            message: CharString::new("Get up", true),
        };

        // From golden fixture: alarm_silent.jsonl
        #[rustfmt::skip]
        let expected = vec![vec![
            18, 80, 3, 9, 10, 0, 0, 16, 14, 29, 36, 30, 25, 36, 36, 0, 191, 169
        ]];

        assert_eq!(alarm.packets(), expected);
    }
}