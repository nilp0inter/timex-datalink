use timex_datalink::client::protocol4::{PacketGenerator, Protocol4};
use super::wrap_with_crc;

#[test]
fn test_start_packets() {
    // Create an instance of Start
    let start = Protocol4::Start;
    
    // Assuming the Start implements PacketGenerator
    let packets = start.packets();
    
    // Expected packet without CRC wrapping
    let expected_packet = vec![0x20, 0x00, 0x00, 0x04];
    
    // The packets method should return the expected packet
    assert_eq!(packets, vec![expected_packet]);
    
    // Test with CRC wrapping
    let wrapped = wrap_with_crc(packets);
    
    // Check wrapped packet has correct length
    // 4 (original) + 1 (header) + 2 (CRC) = 7
    assert_eq!(wrapped[0].len(), 7);
    // Check header byte is correct (packet length + 3)
    assert_eq!(wrapped[0][0], 7);
}