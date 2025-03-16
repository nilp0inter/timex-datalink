//! Sync implementation for Protocol 4
//!
//! This module handles the synchronization protocol for Timex Datalink watches.

use crate::PacketGenerator;

/// Sync structure for Protocol 4
pub struct Sync {
    /// Number of SYNC_1_BYTE to use
    pub length: usize,
}

impl PacketGenerator for Sync {
    fn packets(&self) -> Vec<Vec<u8>> {
        todo!()
    }
}