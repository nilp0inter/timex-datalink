//! Protocol 3 implementation for Timex Datalink watches.

pub mod sync;
pub mod start;
pub mod time;
pub mod alarm;
pub mod end;
pub mod sound_options;
pub mod sound_theme;
pub mod eeprom;
pub mod wrist_app;

pub use sync::Sync;
pub use start::Start;
pub use time::Time;
pub use alarm::Alarm;
pub use end::End;
pub use sound_options::SoundOptions;
pub use sound_theme::SoundTheme;
pub use eeprom::Eeprom;
pub use wrist_app::WristApp;

use crate::PacketGenerator;

/// Main Protocol 3 structure
///
/// This struct acts as a container for all Protocol 3 models that implement
/// the PacketGenerator trait. It collects and orders packets from all models
/// for transmission to the Timex Datalink watch.
pub struct Protocol3 {
    /// Collection of models that implement PacketGenerator
    models: Vec<Box<dyn PacketGenerator>>,
}

impl Protocol3 {
    /// Create a new empty Protocol3 instance
    pub fn new() -> Self {
        Protocol3 {
            models: Vec::new()
        }
    }
    
    /// Add a model to the protocol
    pub fn add<T: PacketGenerator + 'static>(&mut self, model: T) {
        self.models.push(Box::new(model));
    }
}

impl PacketGenerator for Protocol3 {
    fn packets(&self) -> Vec<Vec<u8>> {
        self.models.iter()
            .flat_map(|model| model.packets())
            .collect()
    }
}