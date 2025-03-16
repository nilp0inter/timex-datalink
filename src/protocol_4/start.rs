//! Start implementation for Protocol 4
//!
//! This module handles the start command for Timex Datalink watches.

use crate::PacketGenerator;

/// Start structure for Protocol 4
pub struct Start {}

impl PacketGenerator for Start {
    fn packets(&self) -> Vec<Vec<u8>> {
        todo!()
    }
}