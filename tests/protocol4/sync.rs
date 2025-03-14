use timex_datalink::client::protocol4::{PacketGenerator, Protocol4};

#[test]
fn test_sync_default_length() {
    // Create a Sync instance with default length
    let sync = Protocol4::sync();
    
    // Assuming the Sync implements PacketGenerator
    let packets = sync.packets();
    
    // Sync packets are not CRC wrapped, so we just check the content directly
    // The expected format is:
    // - Start with PING_BYTE (0x78)
    // - Followed by SYNC_1_BYTE (0x55) repeated `length` times (default 300)
    // - Followed by SYNC_2_BYTE (0xaa) repeated SYNC_2_LENGTH times (default 40)
    
    // Check there's exactly one packet
    assert_eq!(packets.len(), 1);
    
    // Check the first byte is PING_BYTE
    assert_eq!(packets[0][0], 0x78);
    
    // Check that bytes 1-300 are all SYNC_1_BYTE (0x55)
    for i in 1..301 {
        assert_eq!(packets[0][i], 0x55);
    }
    
    // Check that bytes 301-340 are all SYNC_2_BYTE (0xaa)
    for i in 301..341 {
        assert_eq!(packets[0][i], 0xaa);
    }
    
    // Check total length is 1 + 300 + 40 = 341
    assert_eq!(packets[0].len(), 341);
}

#[test]
fn test_sync_custom_length() {
    // Create a Sync instance with custom length of 100
    let sync = Protocol4::sync_with_length(100);
    
    // Assuming the Sync implements PacketGenerator
    let packets = sync.packets();
    
    // Check there's exactly one packet
    assert_eq!(packets.len(), 1);
    
    // Check the first byte is PING_BYTE
    assert_eq!(packets[0][0], 0x78);
    
    // Check that bytes 1-100 are all SYNC_1_BYTE (0x55)
    for i in 1..101 {
        assert_eq!(packets[0][i], 0x55);
    }
    
    // Check that bytes 101-140 are all SYNC_2_BYTE (0xaa)
    for i in 101..141 {
        assert_eq!(packets[0][i], 0xaa);
    }
    
    // Check total length is 1 + 100 + 40 = 141
    assert_eq!(packets[0].len(), 141);
}