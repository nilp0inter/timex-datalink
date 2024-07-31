use crc16::{State, ARC};

pub fn crc_packets_wrapper(packets: Vec<u8>) -> Vec<u8> {
    let mut state = State::<ARC>::new();
    // Header is the length of packets + 3
    let header = packets.len() as u8 + 3;

    for packet in packets.iter() {
        state.update(&[*packet]);
    }
    // Footer is the arc checksum of packets divmod by 256
    let footer = state.get() as u8;

    // Return a new vector extended with the header, packets, and footer
    let mut new_packets = vec![header];
    new_packets.extend(packets);
    new_packets.push(footer);

    new_packets
}
