//! Start implementation for Protocol 4
//!
//! This module handles the start command for Timex Datalink watches.

use crate::PacketGenerator;

/// CPACKET_START value for the start command
const CPACKET_START: [u8; 4] = [0x20, 0x00, 0x00, 0x04];

/// Start structure for Protocol 4
pub struct Start {}

impl PacketGenerator for Start {
    fn packets(&self) -> Vec<Vec<u8>> {
        // Generate raw packets
        let raw_packets = vec![CPACKET_START.to_vec()];
        
        // Apply CRC wrapping (like the Ruby prepend CrcPacketsWrapper)
        use crate::helpers::crc_packets_wrapper::wrap_packets_with_crc;
        wrap_packets_with_crc(raw_packets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start() {
        let start = Start {};
        
        // From golden fixture: start.jsonl
        #[rustfmt::skip]
        let expected = vec![vec![
            7, 32, 0, 0, 4, 195, 191
        ]];

        assert_eq!(start.packets(), expected);
    }
}
