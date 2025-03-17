//! Sound Theme implementation for Protocol 3
//!
//! This module handles sound theme functionality for Timex Datalink watches.

use std::fs;
use std::io;
use std::path::Path;
use crate::PacketGenerator;
use crate::helpers::crc_packets_wrapper;
use crate::helpers::cpacket_paginator;

/// Sound Theme structure for Protocol 3
///
/// This allows loading sound themes from SPC files or raw data.
pub struct SoundTheme {
    /// The sound theme data bytes
    pub sound_theme_data: Vec<u8>,
}

impl SoundTheme {
    /// Create a new SoundTheme instance from raw data
    pub fn new(sound_theme_data: Vec<u8>) -> Self {
        SoundTheme { sound_theme_data }
    }
    
    /// Create a new SoundTheme instance from an SPC file
    pub fn from_spc_file<P: AsRef<Path>>(spc_file: P) -> io::Result<Self> {
        // Define the header that needs to be removed
        const SOUND_DATA_HEADER: &[u8] = &[0x25, 0x04, 0x19, 0x69];
        
        // Read the file
        let data = fs::read(spc_file)?;
        
        // Remove the header if present
        let sound_data = if data.starts_with(SOUND_DATA_HEADER) {
            data[SOUND_DATA_HEADER.len()..].to_vec()
        } else {
            data
        };
        
        Ok(SoundTheme::new(sound_data))
    }
}

impl PacketGenerator for SoundTheme {
    fn packets(&self) -> Vec<Vec<u8>> {
        // Define constants from Ruby implementation
        const CPACKET_SECT: [u8; 2] = [0x90, 0x03];
        const CPACKET_DATA: [u8; 2] = [0x91, 0x03];
        const CPACKET_END: [u8; 2] = [0x92, 0x03];
        const CPACKET_DATA_LENGTH: usize = 32;
        
        // Paginate the data into chunks
        let payloads = cpacket_paginator::paginate_cpackets(
            &CPACKET_DATA, 
            CPACKET_DATA_LENGTH, 
            &self.sound_theme_data
        );
        
        // Calculate offset as in Ruby implementation
        let offset = 0x100 - self.sound_theme_data.len();
        
        // Create the section header packet
        let mut load_sect = Vec::with_capacity(4);
        load_sect.extend_from_slice(&CPACKET_SECT);
        load_sect.push(payloads.len() as u8);
        load_sect.push(offset as u8);
        
        // Create the end packet
        let end_packet = CPACKET_END.to_vec();
        
        // Combine all packets and wrap with CRC
        let mut all_packets = Vec::with_capacity(payloads.len() + 2);
        all_packets.push(load_sect);
        all_packets.extend(payloads);
        all_packets.push(end_packet);
        
        crc_packets_wrapper::wrap_packets_with_crc(all_packets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_sound_theme() {
        // Use the example SPC file from fixtures
        let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures")
            .join("EXAMPLE.SPC");
            
        let sound_theme = SoundTheme::from_spc_file(fixture_path).unwrap();
        
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