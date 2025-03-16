//! End implementation for Protocol 4
//!
//! This module handles the end command for Timex Datalink watches.

use crate::PacketGenerator;

/// End structure for Protocol 4
pub struct End {}

impl PacketGenerator for End {
    fn packets(&self) -> Vec<Vec<u8>> {
        todo!()
    }
}