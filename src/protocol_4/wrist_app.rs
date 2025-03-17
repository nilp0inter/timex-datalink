//! WristApp implementation for Protocol 4
//!
//! This module handles wrist applications for Timex Datalink watches.

use crate::PacketGenerator;

/// WristApp structure for Protocol 4
pub struct WristApp {
    /// Wrist app data bytes
    pub wrist_app_data: Vec<u8>,
}

impl PacketGenerator for WristApp {
    fn packets(&self) -> Vec<Vec<u8>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Include the actual ZAP file at compile time 
    // The path is relative to the Cargo.toml file
    const EXAMPLE_ZAP: &[u8] = include_bytes!("../../fixtures/EXAMPLE.ZAP");

    #[test]
    fn test_wrist_app() {
        let wrist_app = WristApp {
            wrist_app_data: EXAMPLE_ZAP.to_vec(),
        };

        // From golden fixture: wrist_app.jsonl
        #[rustfmt::skip]
        let expected = vec![
            vec![5, 147, 2, 48, 253],
            vec![7, 144, 2, 5, 1, 144, 251],
            vec![38, 145, 2, 1, 49, 53, 48, 115, 32, 100, 97, 116, 97, 58, 32, 76, 111, 114, 101, 109, 32, 105, 112, 115, 117, 109, 32, 100, 111, 108, 111, 114, 32, 115, 105, 116, 28, 52],
            vec![38, 145, 2, 2, 32, 97, 109, 101, 116, 44, 32, 99, 111, 110, 115, 101, 99, 116, 101, 116, 117, 114, 32, 97, 100, 105, 112, 105, 115, 99, 105, 110, 103, 32, 101, 108, 240, 169],
            vec![38, 145, 2, 3, 105, 116, 44, 32, 115, 101, 100, 32, 100, 111, 32, 101, 105, 117, 115, 109, 111, 100, 32, 116, 101, 109, 112, 111, 114, 32, 105, 110, 99, 105, 100, 105, 19, 82],
            vec![38, 145, 2, 4, 100, 117, 110, 116, 32, 117, 116, 32, 108, 97, 98, 111, 114, 101, 32, 101, 116, 32, 100, 111, 108, 111, 114, 101, 32, 109, 97, 103, 110, 97, 32, 97, 208, 63],
            vec![12, 145, 2, 5, 108, 105, 113, 117, 97, 46, 127, 67],
            vec![5, 146, 2, 160, 252]
        ];

        assert_eq!(wrist_app.packets(), expected);
    }
}