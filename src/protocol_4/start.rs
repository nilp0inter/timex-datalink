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
    use crate::PacketGenerator;

    #[test]
    fn test_packets() {
        let start = Start {};
        
        // Get packets from the Start instance (already CRC-wrapped)
        let packets = start.packets();
        
        // Expected wrapped packet from the Ruby code (timex_datalink_client_spec.rb)
        let expected_packet = vec![0x07, 0x20, 0x00, 0x00, 0x04, 0xc3, 0xbf];
        
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0], expected_packet);
    }
}
