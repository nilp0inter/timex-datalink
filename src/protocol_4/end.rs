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
    use crate::PacketGenerator;

    #[test]
    fn test_packets() {
        let end = End {};
        
        // Get packets from the End instance (already CRC-wrapped)
        let packets = end.packets();
        
        // Expected wrapped packet from the Ruby code (timex_datalink_client_spec.rb)
        let expected_packet = vec![0x04, 0x21, 0xd8, 0xc2];
        
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0], expected_packet);
    }
}