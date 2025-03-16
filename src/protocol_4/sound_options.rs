//! SoundOptions implementation for Protocol 4
//!
//! This module handles sound options for Timex Datalink watches.

/// SoundOptions structure for Protocol 4
pub struct SoundOptions {
    /// Toggle hourly chime sounds
    pub hourly_chime: bool,
    
    /// Toggle button beep sounds
    pub button_beep: bool,
}