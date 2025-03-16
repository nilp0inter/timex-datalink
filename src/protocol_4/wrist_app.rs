//! WristApp implementation for Protocol 4
//!
//! This module handles wrist applications for Timex Datalink watches.

/// WristApp structure for Protocol 4
pub struct WristApp {
    /// Wrist app data bytes
    pub wrist_app_data: Vec<u8>,
}