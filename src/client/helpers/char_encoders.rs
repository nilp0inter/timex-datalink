/// Character encoders for Timex Datalink watches
/// 
/// This module provides functions to encode ASCII characters into the watch's
/// proprietary character codes.

/// Character mapping for standard watch character set
pub const CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyz !\"#$%&'()*+,-./:\\;=@?_|<>[]";

/// Character mapping for Protocol 6 watch character set
pub const CHARS_PROTOCOL_6: &str = "0123456789 abcdefghijklmnopqrstuvwxyz!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

/// Character mapping for EEPROM data
pub const EEPROM_CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyz !\"#$%&'()*+,-./:\\;=@?_|<>[";

/// Character mapping for phone number data
pub const PHONE_CHARS: &str = "0123456789cfhpw ";

/// Character to use when an invalid character is encountered
pub const INVALID_CHAR: char = ' ';

/// EEPROM terminator value
pub const EEPROM_TERMINATOR: u8 = 0x3f;

/// Convert a string to watch character codes
///
/// # Arguments
/// * `string_chars` - The string to convert
/// * `length` - Optional maximum length
/// * `pad` - Whether to pad the result to length
/// * `char_map` - The character map to use (defaults to CHARS)
///
/// # Returns
/// A vector of character codes
pub fn chars_for(string_chars: &str, length: usize, pad: bool) -> Vec<u8> {
    chars_for_with_map(string_chars, length, pad, CHARS)
}

/// Convert a string to watch character codes with a custom character map
///
/// # Arguments
/// * `string_chars` - The string to convert
/// * `length` - Optional maximum length
/// * `pad` - Whether to pad the result to length
/// * `char_map` - The character map to use
///
/// # Returns
/// A vector of character codes
pub fn chars_for_with_map(string_chars: &str, length: usize, pad: bool, char_map: &str) -> Vec<u8> {
    // Convert to lowercase and truncate if needed
    let mut formatted_chars = string_chars.to_lowercase();
    if formatted_chars.len() > length {
        formatted_chars = formatted_chars.chars().take(length).collect();
    }
    
    // Pad if requested
    let formatted_chars = if pad && formatted_chars.len() < length {
        format!("{:<width$}", formatted_chars, width = length)
    } else {
        formatted_chars
    };
    
    // Convert each character to its code
    formatted_chars.chars()
        .map(|c| {
            char_map.chars()
                .position(|map_char| map_char == c)
                .map(|pos| pos as u8)
                .unwrap_or_else(|| {
                    // If character not found, use code for INVALID_CHAR
                    char_map.chars()
                        .position(|map_char| map_char == INVALID_CHAR)
                        .map(|pos| pos as u8)
                        .unwrap_or(0)
                })
        })
        .collect()
}

/// Convert a string to Protocol 6 watch character codes
///
/// # Arguments
/// * `string_chars` - The string to convert
/// * `length` - Optional maximum length
/// * `pad` - Whether to pad the result to length
///
/// # Returns
/// A vector of character codes
pub fn protocol_6_chars_for(string_chars: &str, length: usize, pad: bool) -> Vec<u8> {
    chars_for_with_map(string_chars, length, pad, CHARS_PROTOCOL_6)
}

/// Convert a string to EEPROM character codes
///
/// # Arguments
/// * `string_chars` - The string to convert
/// * `length` - Maximum length (defaults to 31)
///
/// # Returns
/// A vector of packed bytes
pub fn eeprom_chars_for(string_chars: &str, length: usize) -> Vec<u8> {
    // Get character codes
    let mut chars = chars_for_with_map(string_chars, length, false, EEPROM_CHARS);
    
    // Append terminator
    chars.push(EEPROM_TERMINATOR);
    
    // Pack into bytes using 6-bit encoding
    // This implementation follows the Ruby code's approach:
    // packed_int = chars.each_with_index.sum do |char, index|
    //   char << (6 * index)
    // end
    // packed_int.digits(256)
    
    let mut packed_int: u128 = 0;
    for (index, &char) in chars.iter().enumerate() {
        packed_int |= (char as u128) << (6 * index);
    }
    
    // Convert to bytes (base 256)
    let mut result = Vec::new();
    let mut remaining = packed_int;
    while remaining > 0 {
        result.push((remaining % 256) as u8);
        remaining /= 256;
    }
    
    result
}

/// Convert a string to phone number character codes
///
/// # Arguments
/// * `string_chars` - The string to convert
///
/// # Returns
/// A vector of packed bytes
pub fn phone_chars_for(string_chars: &str) -> Vec<u8> {
    // Get character codes (12 chars max)
    let chars = chars_for_with_map(string_chars, 12, false, PHONE_CHARS);
    
    // Pack into bytes using 4-bit encoding
    // This implementation follows the Ruby code's approach:
    // packed_int = chars.each_with_index.sum do |char, index|
    //   char << (4 * index)
    // end
    // packed_int.digits(256)
    
    let mut packed_int: u64 = 0;
    for (index, &char) in chars.iter().enumerate() {
        packed_int |= (char as u64) << (4 * index);
    }
    
    // Convert to bytes (base 256)
    let mut result = Vec::new();
    let mut remaining = packed_int;
    while remaining > 0 {
        result.push((remaining % 256) as u8);
        remaining /= 256;
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chars_for_basic() {
        let result = chars_for("abc", 3, false);
        assert_eq!(result, vec![10, 11, 12]);
    }
    
    #[test]
    fn test_chars_for_with_padding() {
        let result = chars_for("a", 3, true);
        assert_eq!(result, vec![10, 36, 36]); // 'a' followed by 2 spaces
    }
    
    #[test]
    fn test_chars_for_truncation() {
        let result = chars_for("abcdef", 3, false);
        assert_eq!(result, vec![10, 11, 12]); // truncated to 'abc'
    }
    
    #[test]
    fn test_chars_for_special_chars() {
        let result = chars_for("a@b", 3, false);
        assert_eq!(result[1], CHARS.find('@').unwrap() as u8);
    }
    
    #[test]
    fn test_chars_for_invalid_chars() {
        // Using a character not in the map should result in space
        let result = chars_for("a`b", 3, false);
        let space_index = CHARS.find(' ').unwrap() as u8;
        assert_eq!(result[1], space_index);
    }
    
    #[test]
    fn test_eeprom_chars_for() {
        let result = eeprom_chars_for("test", 10);
        // This would need to be updated with the expected packed bytes
        assert!(!result.is_empty());
    }
}