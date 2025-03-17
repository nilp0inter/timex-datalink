//! SoundOptions implementation for Protocol 4
//!
//! This module handles sound options for Timex Datalink watches.

use crate::PacketGenerator;

/// SoundOptions structure for Protocol 4
pub struct SoundOptions {
    /// Toggle hourly chime sounds
    pub hourly_chime: bool,
    
    /// Toggle button beep sounds
    pub button_beep: bool,
}

impl PacketGenerator for SoundOptions {
    fn packets(&self) -> Vec<Vec<u8>> {
        // Constants from Ruby implementation
        const CPACKET_BEEPS: u8 = 0x71;
        
        // Create the raw packet
        let raw_packet = vec![
            CPACKET_BEEPS,
            if self.hourly_chime { 1 } else { 0 },
            if self.button_beep { 1 } else { 0 }
        ];
        
        // Apply CRC wrapping
        use crate::helpers::crc_packets_wrapper::wrap_packets_with_crc;
        wrap_packets_with_crc(vec![raw_packet])
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
        #[rustfmt::skip]
        let expected = vec![vec![
            6, 113, 1, 0, 3, 81
        ]];

        assert_eq!(sound_options.packets(), expected);
    }
}