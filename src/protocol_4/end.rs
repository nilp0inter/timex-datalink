//! End implementation for Protocol 4
//!
//! This module handles the end command for Timex Datalink watches.

use crate::PacketGenerator;

/// CPACKET_SKIP value for the end command
const CPACKET_SKIP: u8 = 0x21;

/// End structure for Protocol 4
pub struct End {}

impl PacketGenerator for End {
    fn packets(&self) -> Vec<Vec<u8>> {
        // Generate raw packets
        let raw_packets = vec![vec![CPACKET_SKIP]];
        
        // Apply CRC wrapping (like the Ruby prepend CrcPacketsWrapper)
        use crate::helpers::crc_packets_wrapper::wrap_packets_with_crc;
        wrap_packets_with_crc(raw_packets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end() {
        let end = End {};
        
        // From golden fixture: end.jsonl
        #[rustfmt::skip]
        let expected = vec![vec![
            4, 33, 216, 194
        ]];

        assert_eq!(end.packets(), expected);
    }
}
