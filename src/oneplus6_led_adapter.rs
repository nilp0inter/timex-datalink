//! OnePlus 6 LED adapter for sending data to Timex watches
//!
//! This module provides LED control functionality to transmit
//! formatted packets to Timex Datalink watches using the OnePlus 6 notification LED.

#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::io;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;
#[cfg(not(target_arch = "wasm32"))]
use std::thread::sleep;

/// OnePlus 6 LED adapter for sending data to Timex watches
///
/// This handles the LED communication with the watch, including
/// timing constraints between bytes and packets.
pub struct OnePlus6LedAdapter {
    /// Path to the LED sysfs directory
    pub led_path: String,
    
    /// Time to sleep after sending each byte (in seconds)
    pub byte_sleep: f32,
    
    /// Time to sleep after sending a packet (in seconds)
    pub packet_sleep: f32,
    
    /// Enable verbose output
    pub verbose: bool,
}

impl OnePlus6LedAdapter {
    /// Default time to sleep after sending a byte (in seconds)
    pub const BYTE_SLEEP_DEFAULT: f32 = 0.025;
    
    /// Default time to sleep after sending a packet (in seconds)
    pub const PACKET_SLEEP_DEFAULT: f32 = 0.25;
    
    /// Default LED sysfs path
    pub const DEFAULT_LED_PATH: &'static str = "/sys/class/leds/rgb:status";
    
    /// Create a new OnePlus6LedAdapter with the given parameters
    ///
    /// # Arguments
    ///
    /// * `led_path` - Optional path to the LED sysfs directory
    /// * `byte_sleep` - Optional time to sleep after sending each byte (in seconds)
    /// * `packet_sleep` - Optional time to sleep after sending a packet (in seconds)
    /// * `verbose` - Whether to enable verbose output
    pub fn new(
        led_path: Option<String>,
        byte_sleep: Option<f32>,
        packet_sleep: Option<f32>,
        verbose: bool,
    ) -> Self {
        OnePlus6LedAdapter {
            led_path: led_path.unwrap_or_else(|| Self::DEFAULT_LED_PATH.to_string()),
            byte_sleep: byte_sleep.unwrap_or(Self::BYTE_SLEEP_DEFAULT),
            packet_sleep: packet_sleep.unwrap_or(Self::PACKET_SLEEP_DEFAULT),
            verbose,
        }
    }
    
    /// Initialize the LED by setting it to white color
    #[cfg(not(target_arch = "wasm32"))]
    fn init_led(&self) -> io::Result<()> {
        let multi_intensity_path = format!("{}/multi_intensity", self.led_path);
        fs::write(multi_intensity_path, "511 511 511\n")?;
        Ok(())
    }
    
    /// Turn the LED on (set brightness to 511)
    #[cfg(not(target_arch = "wasm32"))]
    fn led_on(&self) -> io::Result<()> {
        let brightness_path = format!("{}/brightness", self.led_path);
        fs::write(brightness_path, "511")?;
        Ok(())
    }
    
    /// Turn the LED off (set brightness to 0)
    #[cfg(not(target_arch = "wasm32"))]
    fn led_off(&self) -> io::Result<()> {
        let brightness_path = format!("{}/brightness", self.led_path);
        fs::write(brightness_path, "0")?;
        Ok(())
    }
    
    /// Send a single bit using the LED
    #[cfg(not(target_arch = "wasm32"))]
    fn send_bit(&self, bit: bool) -> io::Result<()> {
        if bit {
            self.led_on()?;
        } else {
            self.led_off()?;
        }
        Ok(())
    }
    
    /// Write packets to the LED
    ///
    /// # Arguments
    ///
    /// * `packets` - A vector of packet byte vectors to send
    ///
    /// # Errors
    ///
    /// Returns an error if the LED cannot be controlled or if writing fails
    #[cfg(not(target_arch = "wasm32"))]
    pub fn write(&self, packets: &[Vec<u8>]) -> io::Result<()> {
        // Initialize the LED to white color
        self.init_led()?;
        
        // Ensure LED is off initially
        self.led_off()?;
        sleep(Duration::from_secs_f32(0.5));
        
        for packet in packets {
            for &byte in packet {
                if self.verbose {
                    print!("{:02X} ", byte);
                }
                
                // Send each bit of the byte (MSB first)
                for i in (0..8).rev() {
                    let bit = (byte >> i) & 1 == 1;
                    self.send_bit(bit)?;
                    sleep(Duration::from_secs_f32(self.byte_sleep / 8.0));
                }
                
                // Brief pause between bytes
                self.led_off()?;
                sleep(Duration::from_secs_f32(self.byte_sleep));
            }
            
            // Longer pause between packets
            self.led_off()?;
            sleep(Duration::from_secs_f32(self.packet_sleep));
            
            if self.verbose {
                println!();
            }
        }
        
        // Ensure LED is off when done
        self.led_off()?;
        
        Ok(())
    }
    
    /// Stub implementation for wasm target
    #[cfg(target_arch = "wasm32")]
    pub fn write(&self, _packets: &[Vec<u8>]) -> Result<(), &'static str> {
        Err("LED control functionality is not available in WebAssembly")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_with_defaults() {
        let adapter = OnePlus6LedAdapter::new(None, None, None, false);
        assert_eq!(adapter.led_path, OnePlus6LedAdapter::DEFAULT_LED_PATH);
        assert_eq!(adapter.byte_sleep, OnePlus6LedAdapter::BYTE_SLEEP_DEFAULT);
        assert_eq!(adapter.packet_sleep, OnePlus6LedAdapter::PACKET_SLEEP_DEFAULT);
        assert_eq!(adapter.verbose, false);
    }
    
    #[test]
    fn test_new_with_custom_values() {
        let adapter = OnePlus6LedAdapter::new(
            Some("/custom/led/path".to_string()),
            Some(0.05),
            Some(0.5),
            true,
        );
        assert_eq!(adapter.led_path, "/custom/led/path");
        assert_eq!(adapter.byte_sleep, 0.05);
        assert_eq!(adapter.packet_sleep, 0.5);
        assert_eq!(adapter.verbose, true);
    }
}