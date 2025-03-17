//! Sound Options implementation for Protocol 3
//!
//! This module handles various sound options for Timex Datalink watches.

use crate::PacketGenerator;
use crate::helpers::crc_packets_wrapper;

/// Sound Options structure for Protocol 3
///
/// This controls watch sounds like hourly chimes and button beeps.
pub struct SoundOptions {
    /// Whether the watch chimes on the hour
    pub hourly_chime: bool,
    
    /// Whether buttons make a beep sound when pressed
    pub button_beep: bool,
}

impl PacketGenerator for SoundOptions {
    fn packets(&self) -> Vec<Vec<u8>> {
        // Define constants from Ruby implementation
        const CPACKET_BEEPS: u8 = 0x71;

        // Create the raw packet
        let mut raw_packet = Vec::with_capacity(3);
        raw_packet.push(CPACKET_BEEPS);
        raw_packet.push(if self.hourly_chime { 1 } else { 0 });
        raw_packet.push(if self.button_beep { 1 } else { 0 });
        
        // Apply CRC wrapping
        crc_packets_wrapper::wrap_packets_with_crc(vec![raw_packet])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_options() {
        let sound_options = SoundOptions {
            hourly_chime: true,
            button_beep: false,
        };

        // From golden fixture: sound_options.jsonl
        let expected = vec![vec![6, 113, 1, 0, 3, 81]];

        assert_eq!(sound_options.packets(), expected);
    }
}