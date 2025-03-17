//! Timex-datalink client library in rust
//! This is a port of the ruby `timex_datalink_client` gem.

/// A trait for packet generation that can be used across different protocols
///
/// This trait defines the core functionality for generating packet bytes
/// that will be transmitted to Timex Datalink devices.
pub trait PacketGenerator {
    /// Generate packets as a vector of vectors of bytes
    ///
    /// # Returns
    ///
    /// A vector of vectors of bytes representing the packets to be transmitted
    fn packets(&self) -> Vec<Vec<u8>>;
}

pub mod protocol_3;
pub mod protocol_4;
pub mod helpers;
pub mod char_encoders;
pub mod notebook_adapter;
pub mod devices;

pub use protocol_3::Protocol3;
pub use protocol_4::Protocol4;
pub use notebook_adapter::NotebookAdapter;