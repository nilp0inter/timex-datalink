/// Length packet wrapper for Timex Datalink watches
/// 
/// This module provides functions to wrap packets with a length byte prefix
/// The length byte is the length of the packet + 1

/// Wrap a packet with a length byte prefix
/// 
/// # Arguments
/// * `packet` - The packet to wrap
/// 
/// # Returns
/// A new packet with a length byte prefix (packet length + 1)
pub fn length_packet_wrapper(packet: &[u8]) -> Vec<u8> {
    // Following the Ruby implementation: [packet.length + 1] + packet
    let mut result = Vec::with_capacity(packet.len() + 1);
    result.push((packet.len() + 1) as u8);
    result.extend_from_slice(packet);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_length_packet_wrapper() {
        let packet = vec![0x01, 0x02, 0x03];
        let wrapped = length_packet_wrapper(&packet);
        assert_eq!(wrapped, vec![0x04, 0x01, 0x02, 0x03]);
    }
    
    #[test]
    fn test_length_packet_wrapper_empty() {
        let packet = Vec::<u8>::new();
        let wrapped = length_packet_wrapper(&packet);
        assert_eq!(wrapped, vec![0x01]);
    }
    
    #[test]
    fn test_length_packet_wrapper_large() {
        let packet = vec![0; 254];
        let wrapped = length_packet_wrapper(&packet);
        assert_eq!(wrapped[0], 255);
        assert_eq!(wrapped.len(), 255);
    }
}