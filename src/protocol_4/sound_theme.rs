//! SoundTheme implementation for Protocol 4
//!
//! This module handles sound themes for Timex Datalink watches.

use crate::PacketGenerator;
use crate::helpers::cpacket_paginator::paginate_cpackets;

/// SoundTheme structure for Protocol 4
pub struct SoundTheme {
    /// Sound theme data bytes
    pub sound_theme_data: Vec<u8>,
}

impl PacketGenerator for SoundTheme {
    fn packets(&self) -> Vec<Vec<u8>> {
        // Constants from Ruby implementation
        const CPACKET_SECT: [u8; 2] = [0x90, 0x03];
        const CPACKET_DATA: [u8; 2] = [0x91, 0x03];
        const CPACKET_END: [u8; 2] = [0x92, 0x03];
        const CPACKET_DATA_LENGTH: usize = 32;
        
        // Check if data has the SPC file header and remove it if present
        const SOUND_DATA_HEADER: &[u8] = &[0x25, 0x04, 0x19, 0x69];
        
        let sound_data = if self.sound_theme_data.starts_with(SOUND_DATA_HEADER) {
            &self.sound_theme_data[SOUND_DATA_HEADER.len()..]
        } else {
            &self.sound_theme_data
        };
        
        // Calculate offset similar to Ruby implementation
        let offset = 0x100 - sound_data.len();
        
        // Create load_sect packet
        let payloads = paginate_cpackets(&CPACKET_DATA, CPACKET_DATA_LENGTH, sound_data);
        let mut load_sect = Vec::new();
        load_sect.extend_from_slice(&CPACKET_SECT);
        load_sect.push(payloads.len() as u8);
        load_sect.push(offset as u8);
        
        // Create end packet
        let end_packet = CPACKET_END.to_vec();
        
        // Combine all packets
        let mut all_packets = Vec::with_capacity(payloads.len() + 2);
        all_packets.push(load_sect);
        all_packets.extend(payloads);
        all_packets.push(end_packet);
        
        // Apply CRC wrapping
        use crate::helpers::crc_packets_wrapper::wrap_packets_with_crc;
        wrap_packets_with_crc(all_packets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Include the actual SPC file at compile time
    // The path is relative to the Cargo.toml file
    const EXAMPLE_SPC: &[u8] = include_bytes!("../../fixtures/EXAMPLE.SPC");

    #[test]
    fn test_sound_theme() {
        let sound_theme = SoundTheme {
            sound_theme_data: EXAMPLE_SPC.to_vec(),
        };

        // From golden fixture: sound_theme.jsonl
        #[rustfmt::skip]
        let expected = vec![
            vec![7, 144, 3, 2, 215, 254, 41],
            vec![38, 145, 3, 1, 98, 105, 110, 97, 114, 121, 32, 115, 111, 117, 110, 100, 32, 100, 97, 116, 97, 32, 116, 104, 97, 116, 32, 103, 101, 116, 115, 32, 115, 101, 110, 116, 28, 235],
            vec![15, 145, 3, 2, 32, 118, 101, 114, 98, 97, 116, 105, 109, 75, 236],
            vec![5, 146, 3, 96, 61]
        ];

        assert_eq!(sound_theme.packets(), expected);
    }
}