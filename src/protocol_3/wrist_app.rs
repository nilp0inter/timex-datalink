//! Wrist App implementation for Protocol 3
//!
//! This module handles wrist app functionality for Timex Datalink watches.

use std::path::PathBuf;
use crate::PacketGenerator;
use crate::helpers::crc_packets_wrapper;
use crate::helpers::cpacket_paginator;

/// Wrist App structure for Protocol 3
///
/// This allows loading wrist apps from ZAP files or raw data.
pub struct WristApp {
    /// The wrist app data bytes
    pub wrist_app_data: Vec<u8>,
}

impl WristApp {
    /// Create a new WristApp from raw data
    pub fn new(wrist_app_data: Vec<u8>) -> Self {
        Self {
            wrist_app_data,
        }
    }
    
    /// Create a new WristApp from a ZAP file path
    /// 
    /// This follows the Ruby implementation's approach of parsing ZAP files
    // pub fn from_zap_file(file_path: &str) -> std::io::Result<Self> {
    pub fn from_zap_file(file_path: PathBuf) -> std::io::Result<Self> {
        // Read the ZAP file
        let file_data = std::fs::read(file_path)?;
        
        // Parse the ZAP file
        let wrist_app_data = Self::parse_zap_file(&file_data)?;
        
        Ok(Self {
            wrist_app_data,
        })
    }
    
    /// Process a ZAP file to extract binary data
    /// 
    /// This implementation follows the Ruby version which:
    /// 1. Finds sections delimited by "\xac.*\r\n"
    /// 2. Extracts section at WRIST_APP_CODE_INDEX (8)
    /// 3. Decodes the hex string to binary
    pub fn parse_zap_file(zap_data: &[u8]) -> std::io::Result<Vec<u8>> {
        // Constants from Ruby implementation
        const WRIST_APP_CODE_INDEX: usize = 8; // Index 8 as in Ruby implementation
        
        // Split the ZAP file by the delimiter '\xAC'
        let mut sections = Vec::new();
        let mut current_section = Vec::new();
        let mut i = 0;
        
        while i < zap_data.len() {
            if zap_data[i] == 0xAC {
                // End of a section
                if !current_section.is_empty() {
                    sections.push(current_section);
                    current_section = Vec::new();
                }
                
                // Skip the delimiter and look for \r\n
                i += 1;
                while i < zap_data.len() && !(zap_data[i] == b'\r' && i + 1 < zap_data.len() && zap_data[i + 1] == b'\n') {
                    i += 1;
                }
                
                // Skip the \r\n if found
                if i < zap_data.len() && zap_data[i] == b'\r' && i + 1 < zap_data.len() && zap_data[i + 1] == b'\n' {
                    i += 2; // Skip \r\n
                }
            } else {
                // Add to current section
                current_section.push(zap_data[i]);
                i += 1;
            }
        }
        
        // Add the last section if not empty
        if !current_section.is_empty() {
            sections.push(current_section);
        }
        
        // Check if we have enough sections
        if sections.len() <= WRIST_APP_CODE_INDEX {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("ZAP file does not contain enough sections, expected at least {}", WRIST_APP_CODE_INDEX + 1)
            ));
        }
        
        // Get the target section - using the fixed index from Ruby implementation
        let target_section = &sections[WRIST_APP_CODE_INDEX];
        
        // Convert the hex string to binary
        // In Ruby: [zap_file_data_ascii].pack("H*")
        let mut result = Vec::new();
        
        // Each pair of hex chars becomes one byte
        let mut i = 0;
        while i + 1 < target_section.len() {
            let high = Self::hex_digit_to_value(target_section[i]);
            let low = Self::hex_digit_to_value(target_section[i + 1]);
            
            if let (Some(high), Some(low)) = (high, low) {
                result.push((high << 4) | low);
            } else {
                // If high isn't valid, skip to the next char
                if high.is_none() {
                    i += 1;
                }
                // If low isn't valid, we've already used high, so proceed as normal
            }
            
            i += 2;
        }
        
        Ok(result)
    }
    
    /// Convert a hex digit (0-9, A-F) to its numeric value
    fn hex_digit_to_value(digit: u8) -> Option<u8> {
        match digit {
            b'0'..=b'9' => Some(digit - b'0'),
            b'A'..=b'F' => Some(digit - b'A' + 10),
            b'a'..=b'f' => Some(digit - b'a' + 10),
            _ => None,
        }
    }
}

impl PacketGenerator for WristApp {
    fn packets(&self) -> Vec<Vec<u8>> {
        // Define constants from Ruby implementation
        const CPACKET_CLEAR: [u8; 2] = [0x93, 0x02];
        const CPACKET_SECT: [u8; 2] = [0x90, 0x02];
        const CPACKET_DATA: [u8; 2] = [0x91, 0x02];
        const CPACKET_END: [u8; 2] = [0x92, 0x02];
        const CPACKET_DATA_LENGTH: usize = 32;
        
        // Create payloads using the cpacket_paginator (as in Ruby)
        let payloads = cpacket_paginator::paginate_cpackets(
            &CPACKET_DATA,
            CPACKET_DATA_LENGTH,
            &self.wrist_app_data
        );
        
        // Create sect_packet like in Ruby
        let mut sect_packet = Vec::new();
        sect_packet.extend_from_slice(&CPACKET_SECT);
        sect_packet.push(payloads.len() as u8);
        sect_packet.push(1); // Constant value in protocol 3 (always 1)
        
        // Combine all packets in the right order as in Ruby
        // Ruby: [CPACKET_CLEAR, cpacket_sect] + payloads + [CPACKET_END]
        let mut all_packets = Vec::with_capacity(payloads.len() + 3);
        all_packets.push(CPACKET_CLEAR.to_vec());
        all_packets.push(sect_packet);
        all_packets.extend(payloads);
        all_packets.push(CPACKET_END.to_vec());
        
        // Wrap with CRC
        crc_packets_wrapper::wrap_packets_with_crc(all_packets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_wrist_app() {
        // Use the example ZAP file from fixtures
        let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures")
            .join("EXAMPLE.ZAP");
            
        let wrist_app = WristApp::from_zap_file(fixture_path).unwrap();
        
        // From golden fixture: wrist_app.jsonl
        #[rustfmt::skip]
        let expected = vec![
          vec![ 5, 147, 2, 48, 253 ],
          vec![ 7, 144, 2, 5, 1, 144, 251 ],
          vec![ 38, 145, 2, 1, 49, 53, 48, 32, 100, 97, 116, 97, 58, 32, 76, 111, 114, 101, 109, 32, 105, 112, 115, 117, 109, 32, 100, 111, 108, 111, 114, 32, 115, 105, 116, 32, 211, 127 ],
          vec![ 38, 145, 2, 2, 97, 109, 101, 116, 44, 32, 99, 111, 110, 115, 101, 99, 116, 101, 116, 117, 114, 32, 97, 100, 105, 112, 105, 115, 99, 105, 110, 103, 32, 101, 108, 105, 63, 42 ],
          vec![ 38, 145, 2, 3, 116, 44, 32, 115, 101, 100, 32, 100, 111, 32, 101, 105, 117, 115, 109, 111, 100, 32, 116, 101, 109, 112, 111, 114, 32, 105, 110, 99, 105, 100, 105, 100, 140, 40 ],
          vec![ 38, 145, 2, 4, 117, 110, 116, 32, 117, 116, 32, 108, 97, 98, 111, 114, 101, 32, 101, 116, 32, 100, 111, 108, 111, 114, 101, 32, 109, 97, 103, 110, 97, 32, 97, 108, 167, 146 ],
          vec![ 11, 145, 2, 5, 105, 113, 117, 97, 46, 102, 103 ],
          vec![ 5, 146, 2, 160, 252 ]
        ];

        assert_eq!(wrist_app.packets(), expected);
    }
}
