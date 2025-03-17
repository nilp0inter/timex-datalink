//! SoundTheme implementation for Protocol 4
//!
//! This module handles sound themes for Timex Datalink watches.

use crate::PacketGenerator;

/// SoundTheme structure for Protocol 4
pub struct SoundTheme {
    /// Sound theme data bytes
    pub sound_theme_data: Vec<u8>,
}

impl PacketGenerator for SoundTheme {
    fn packets(&self) -> Vec<Vec<u8>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Include the actual SPC file at compile time
    // The path is relative to the Cargo.toml file
    const EXAMPLE_SPC: &[u8] = include_bytes!("../../fixtures/EXAMPLE.SPC");

    #[test]
    fn test_sound_theme() {
        let sound_theme = SoundTheme {
            sound_theme_data: EXAMPLE_SPC.to_vec(),
        };

        // From golden fixture: sound_theme.jsonl
        #[rustfmt::skip]
        let expected = vec![
            vec![7, 144, 3, 2, 215, 254, 41],
            vec![38, 145, 3, 1, 98, 105, 110, 97, 114, 121, 32, 115, 111, 117, 110, 100, 32, 100, 97, 116, 97, 32, 116, 104, 97, 116, 32, 103, 101, 116, 115, 32, 115, 101, 110, 116, 28, 235],
            vec![15, 145, 3, 2, 32, 118, 101, 114, 98, 97, 116, 105, 109, 75, 236],
            vec![5, 146, 3, 96, 61]
        ];

        assert_eq!(sound_theme.packets(), expected);
    }
}