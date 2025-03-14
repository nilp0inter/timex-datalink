mod start;
mod time;
mod alarm;
mod sync;

// Import common test helpers
use timex_datalink::client::helpers::crc_packets_wrapper::crc_packets_wrapper;

// Helper function to wrap packets with CRC for tests
fn wrap_with_crc(packets: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    crc_packets_wrapper(packets)
}