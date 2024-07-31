// Declare the main module
pub mod client {
    // Declare the submodules within timex_datalink_client
    pub mod helpers {
        pub mod char_encoders;
        pub mod cpacket_paginator;
        pub mod crc_packets_wrapper;
        pub mod four_byte_formatter;
        pub mod length_packet_wrapper;
        pub mod lsb_msb_formatter;
    }

    pub mod protocol4;
}

