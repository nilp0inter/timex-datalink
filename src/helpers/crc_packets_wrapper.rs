//! CRC Packets Wrapper module
//! 
//! This module provides functionality to wrap packets with CRC checksums
//! Ported from Ruby's CrcPacketsWrapper

use crc16::*;

/// Wrapper function that adds CRC headers and footers to packets
/// 
/// # Arguments
/// 
/// * `packets` - A vector of packet vectors to be wrapped with CRC
/// 
/// # Returns
/// 
/// A vector of vectors with CRC headers and footers added
pub fn wrap_packets_with_crc(packets: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    packets.into_iter().map(|packet| {
        let header = crc_header(&packet);
        let footer = crc_footer(&packet);
        
        let mut wrapped_packet = Vec::with_capacity(1 + packet.len() + 2);
        wrapped_packet.extend(header);
        wrapped_packet.extend(packet);
        wrapped_packet.extend(footer);
        
        wrapped_packet
    }).collect()
}

/// Calculates the CRC header for a packet
/// 
/// # Arguments
/// 
/// * `packet` - The packet to calculate a header for
/// 
/// # Returns
/// 
/// A vector containing the header byte(s)
fn crc_header(packet: &[u8]) -> Vec<u8> {
    vec![(packet.len() + 3) as u8]
}

/// Calculates the CRC footer for a packet
/// 
/// # Arguments
/// 
/// * `packet` - The packet to calculate a footer for
/// 
/// # Returns
/// 
/// A vector containing the footer bytes
fn crc_footer(packet: &[u8]) -> Vec<u8> {
    // Create the CRC check bytes (header + packet)
    let mut crc_check = Vec::with_capacity(1 + packet.len());
    crc_check.push((packet.len() + 3) as u8); // Header
    crc_check.extend_from_slice(packet);
    
    // Calculate CRC16-ARC
    let mut state = State::<ARC>::new();
    state.update(&crc_check);
    let crc = state.get();
    
    // Split into two bytes (equivalent to Ruby's divmod(256))
    vec![(crc >> 8) as u8, (crc & 0xFF) as u8]
}

