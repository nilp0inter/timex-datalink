//! Sync implementation for Protocol 4
//!
//! This module handles the synchronization protocol for Timex Datalink watches.

/// Sync structure for Protocol 4
pub struct Sync {
    /// Number of SYNC_1_BYTE to use
    pub length: usize,
}