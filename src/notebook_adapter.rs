//! Notebook adapter for sending data to Timex watches
//!
//! This module provides the serial communication functionality to transmit
//! formatted packets to Timex Datalink watches.

use std::io;
use std::time::Duration;
use std::thread::sleep;

/// Notebook adapter for sending data to Timex watches
///
/// This handles the serial communication with the watch, including
/// timing constraints between bytes and packets.
pub struct NotebookAdapter {
    /// Path to the serial device
    pub serial_device: String,
    
    /// Time to sleep after sending each byte (in seconds)
    pub byte_sleep: f32,
    
    /// Time to sleep after sending a packet (in seconds)
    pub packet_sleep: f32,
    
    /// Enable verbose output
    pub verbose: bool,
}

impl NotebookAdapter {
    /// Default time to sleep after sending a byte (in seconds)
    pub const BYTE_SLEEP_DEFAULT: f32 = 0.025;
    
    /// Default time to sleep after sending a packet (in seconds)
    pub const PACKET_SLEEP_DEFAULT: f32 = 0.25;
    
    /// Create a new NotebookAdapter with the given parameters
    ///
    /// # Arguments
    ///
    /// * `serial_device` - Path to the serial device
    /// * `byte_sleep` - Optional time to sleep after sending each byte (in seconds)
    /// * `packet_sleep` - Optional time to sleep after sending a packet (in seconds)
    /// * `verbose` - Whether to enable verbose output
    pub fn new(
        serial_device: String,
        byte_sleep: Option<f32>,
        packet_sleep: Option<f32>,
        verbose: bool,
    ) -> Self {
        NotebookAdapter {
            serial_device,
            byte_sleep: byte_sleep.unwrap_or(Self::BYTE_SLEEP_DEFAULT),
            packet_sleep: packet_sleep.unwrap_or(Self::PACKET_SLEEP_DEFAULT),
            verbose,
        }
    }
    
    /// Write packets to the serial device
    ///
    /// # Arguments
    ///
    /// * `packets` - A vector of packet byte vectors to send
    ///
    /// # Errors
    ///
    /// Returns an error if the serial device cannot be opened or if writing fails
    pub fn write(&self, packets: &[Vec<u8>]) -> io::Result<()> {
        let port = serial2::SerialPort::open(&self.serial_device, 9600)?;
        
        for packet in packets {
            for &byte in packet {
                if self.verbose {
                    print!("{:02X} ", byte);
                }
                
                port.write(&[byte])?;
                
                sleep(Duration::from_secs_f32(self.byte_sleep));
            }
            
            sleep(Duration::from_secs_f32(self.packet_sleep));
            
            if self.verbose {
                println!();
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Add tests here if needed
}