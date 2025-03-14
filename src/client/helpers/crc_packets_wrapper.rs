use crc16::{State, ARC};

/// Wraps packets with CRC header and footer
/// 
/// This matches the Ruby implementation in the original gem
/// where the header is based on packet length + 3,
/// and the CRC is calculated including the header.
pub fn crc_packets_wrapper(packets: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    packets.into_iter()
        .map(|packet| {
            let header = vec![packet.len() as u8 + 3];
            let crc_check = [&header[..], &packet[..]].concat();
            
            let mut state = State::<ARC>::new();
            state.update(&crc_check);
            let crc = state.get();
            
            // CRC divmod by 256 to get MSB and LSB
            let msb = (crc >> 8) as u8;
            let lsb = (crc & 0xFF) as u8;
            
            // Combine header, packet, and footer (lsb, msb)
            let mut result = header;
            result.extend_from_slice(&packet);
            result.push(lsb);
            result.push(msb);
            
            result
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc_packets_wrapper() {
        // Example based on Protocol1::Time test
        let packets = vec![
            vec![0x30, 0x01, 0x13, 0x1c, 0x0a, 0x15, 0x0f, 0x02, 0x20, 0x01]
        ];
        
        let wrapped = crc_packets_wrapper(packets);
        
        // The wrapped packet should start with the length + 3 and end with CRC bytes
        assert_eq!(wrapped[0][0], 13); // 10 (original length) + 3
        assert_eq!(wrapped[0].len(), 13); // Original 10 bytes + header (1) + footer (2)
    }
    
    #[test]
    fn test_crc_value_calculation() {
        // Test case with known CRC values to verify algorithm
        let packet = vec![0x30, 0x01, 0x13, 0x1c, 0x0a, 0x15, 0x0f, 0x02, 0x20, 0x01];
        let header = vec![packet.len() as u8 + 3];
        let crc_check = [&header[..], &packet[..]].concat();
        
        let mut state = State::<ARC>::new();
        state.update(&crc_check);
        let crc = state.get();
        
        let lsb = (crc & 0xFF) as u8;
        let msb = (crc >> 8) as u8;
        
        // Known values from Ruby implementation
        let wrapped = crc_packets_wrapper(vec![packet]);
        
        // Verify that LSB is at second to last position, MSB at last position
        assert_eq!(wrapped[0][wrapped[0].len() - 2], lsb);
        assert_eq!(wrapped[0][wrapped[0].len() - 1], msb);
    }
}