//! Sync implementation for Protocol 4
//!
//! This module handles the synchronization protocol for Timex Datalink watches.

use crate::PacketGenerator;

/// Sync structure for Protocol 4
pub struct Sync {
    /// Number of SYNC_1_BYTE to use
    pub length: usize,
}

impl Default for Sync {
    fn default() -> Self {
        Self { length: 300 }
    }
}

impl PacketGenerator for Sync {
    fn packets(&self) -> Vec<Vec<u8>> {
        // Define constants matching Ruby implementation
        const PING_BYTE: u8 = 0x78;
        const SYNC_1_BYTE: u8 = 0x55;
        const SYNC_2_BYTE: u8 = 0xaa;
        const SYNC_2_LENGTH: usize = 40;

        // Create a vector to hold our bytes
        let mut packet = Vec::with_capacity(1 + self.length + SYNC_2_LENGTH);
        
        // Add ping byte
        packet.push(PING_BYTE);
        
        // Add SYNC_1 bytes
        packet.extend(vec![SYNC_1_BYTE; self.length]);
        
        // Add SYNC_2 bytes
        packet.extend(vec![SYNC_2_BYTE; SYNC_2_LENGTH]);
        
        vec![packet]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync() {
        let sync = Sync::default();
        
        // From golden fixture: sync.jsonl
        #[rustfmt::skip]
        let expected = vec![vec![120,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,85,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170,170]];

        assert_eq!(sync.packets(), expected);
    }
}
