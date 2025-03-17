//! CPacket Paginator module
//! 
//! This module provides functionality to paginate command packets similar to Ruby's CpacketPaginator
//! Ported from Ruby's CpacketPaginator

/// Paginates a large packet into smaller chunks with headers
/// 
/// # Arguments
/// 
/// * `header` - The header bytes to include with each packet
/// * `length` - The maximum length for each paginated packet
/// * `data` - The data bytes to paginate
/// 
/// # Returns
/// 
/// A vector of packet vectors, each containing header + index + data chunk
pub fn paginate_cpackets(header: &[u8], length: usize, data: &[u8]) -> Vec<Vec<u8>> {
    data.chunks(length)
        .enumerate()
        .map(|(i, chunk)| {
            let index = i + 1; // 1-based indexing as in Ruby
            let mut packet = Vec::with_capacity(header.len() + 1 + chunk.len());
            packet.extend_from_slice(header);
            packet.push(index as u8);
            packet.extend_from_slice(chunk);
            packet
        })
        .collect()
}