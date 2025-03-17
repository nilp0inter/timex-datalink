//! End implementation for Protocol 3
//!
//! This module handles the end command for Timex Datalink watches.

use crate::PacketGenerator;
use crate::helpers::crc_packets_wrapper;

/// End structure for Protocol 3
///
/// This finalizes communication with the watch.
pub struct End;

impl PacketGenerator for End {
    fn packets(&self) -> Vec<Vec<u8>> {
        // Define constants matching Ruby implementation
        const CPACKET_SKIP: [u8; 1] = [0x21];
        
        // Wrap packets with CRC
        crc_packets_wrapper::wrap_packets_with_crc(vec![CPACKET_SKIP.to_vec()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end() {
        let end = End;
        
        // From golden fixture: end.jsonl
        let expected = vec![vec![4, 33, 216, 194]];

        assert_eq!(end.packets(), expected);
    }
}