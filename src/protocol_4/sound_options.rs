//! SoundOptions implementation for Protocol 4
//!
//! This module handles sound options for Timex Datalink watches.

use crate::PacketGenerator;

/// SoundOptions structure for Protocol 4
pub struct SoundOptions {
    /// Toggle hourly chime sounds
    pub hourly_chime: bool,
    
    /// Toggle button beep sounds
    pub button_beep: bool,
}

impl PacketGenerator for SoundOptions {
    fn packets(&self) -> Vec<Vec<u8>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_options() {
        let sound_options = SoundOptions {
            hourly_chime: true,
            button_beep: false,
        };

        // From golden fixture: sound_options.jsonl
        #[rustfmt::skip]
        let expected = vec![vec![
            6, 113, 1, 0, 3, 81
        ]];

        assert_eq!(sound_options.packets(), expected);
    }
}