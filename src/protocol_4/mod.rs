//! Protocol 4 implementation for Timex Datalink watches.

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

/// Main Protocol 4 structure
///
/// This struct holds all components of the Protocol 4 communication.
/// Only Sync, Start, and End are mandatory fields, while all others are optional.
pub struct Protocol4 {
    /// Sync component (mandatory)
    pub sync: Sync,
    
    /// Start component (mandatory)
    pub start: Start,
    
    /// Time component (optional)
    pub time: Option<Time>,
    
    /// Alarm component (optional)
    pub alarm: Option<Alarm>,
    
    /// Sound options component (optional)
    pub sound_options: Option<SoundOptions>,
    
    /// Sound theme component (optional)
    pub sound_theme: Option<SoundTheme>,
    
    /// EEPROM data component (optional)
    pub eeprom: Option<Eeprom>,
    
    /// Wrist app component (optional)
    pub wrist_app: Option<WristApp>,
    
    /// End component (mandatory)
    pub end: End,
}