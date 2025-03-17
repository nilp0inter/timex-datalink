//! Start implementation for Protocol 3
//!
//! This module handles the start command for Timex Datalink watches.

use crate::PacketGenerator;
use crate::helpers::crc_packets_wrapper;

/// Start structure for Protocol 3
///
/// This initiates communication with the watch.
pub struct Start;

impl PacketGenerator for Start {
    fn packets(&self) -> Vec<Vec<u8>> {
        // Define constants matching Ruby implementation
        const CPACKET_START: [u8; 4] = [0x20, 0x00, 0x00, 0x03];
        
        // Wrap packets with CRC
        crc_packets_wrapper::wrap_packets_with_crc(vec![CPACKET_START.to_vec()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start() {
        let start = Start;
        
        // From golden fixture: start.jsonl
        let expected = vec![vec![7, 32, 0, 0, 3, 1, 254]];

        assert_eq!(start.packets(), expected);
    }
}