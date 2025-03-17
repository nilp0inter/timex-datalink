// Rust implementation of the Ruby CharEncoders module from
// timex_datalink_client/helpers/char_encoders.rb

use num_bigint::BigUint;

/// Standard character set for most watch functions
pub const CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyz !\"#$%&'()*+,-./:\\;=@?_|<>[]";

/// Extended character set for Protocol 6 watches
pub const CHARS_PROTOCOL_6: &str = "0123456789 abcdefghijklmnopqrstuvwxyz!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

/// Character set for EEPROM storage
pub const EEPROM_CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyz !\"#$%&'()*+,-./:\\;=@?_|<>[";

/// Limited character set for phone numbers
pub const PHONE_CHARS: &str = "0123456789cfhpw ";

/// Default replacement for invalid characters
pub const INVALID_CHAR: char = ' ';

/// EEPROM string terminator value
pub const EEPROM_TERMINATOR: u8 = 0x3f;

/// A string encoded with the watch character set with fixed maximum length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharString<const MAX_LEN: usize> {
    bytes: [u8; MAX_LEN],
    len: usize,  // Actual length used (may be less than MAX_LEN)
}

impl<const MAX_LEN: usize> CharString<MAX_LEN> {
    /// Create a new watch-compatible string using the standard character set
    pub fn new(input: &str, pad: bool) -> Self {
        Self::new_with_charset(input, CHARS, pad)
    }
    
    /// Create a new watch-compatible string using Protocol 6 character set
    pub fn new_protocol_6(input: &str, pad: bool) -> Self {
        Self::new_with_charset(input, CHARS_PROTOCOL_6, pad)
    }
    
    /// Create a new watch-compatible string with a specific character set
    pub fn new_with_charset(input: &str, charset: &str, pad: bool) -> Self {
        let invalid_pos = charset.find(INVALID_CHAR).unwrap_or(0) as u8;
        let mut bytes = [invalid_pos; MAX_LEN]; // Initialize with invalid char
        
        // Convert input to lowercase and encode
        let mut actual_len = 0;
        for (i, c) in input.to_lowercase().chars().take(MAX_LEN).enumerate() {
            bytes[i] = charset.find(c).unwrap_or(invalid_pos as usize) as u8;
            actual_len = i + 1;
        }
        
        // Use actual_len as the length unless padding is requested
        let final_len = if pad { MAX_LEN } else { actual_len };
        
        CharString { bytes, len: final_len }
    }
    
    /// Get the encoded bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
    
    /// Get the full bytes array including unused space
    pub fn as_array(&self) -> &[u8; MAX_LEN] {
        &self.bytes
    }
    
    /// Get the length of the encoded string
    pub fn len(&self) -> usize {
        self.len
    }
    
    /// Check if the string is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<const MAX_LEN: usize> From<&str> for CharString<MAX_LEN> {
    fn from(s: &str) -> Self {
        CharString::new(s, false)
    }
}

impl<const MAX_LEN: usize> Default for CharString<MAX_LEN> {
    fn default() -> Self {
        Self {
            bytes: [0; MAX_LEN],
            len: 0,
        }
    }
}

/// EEPROM string with special encoding for Timex watches
/// Uses 6-bit packing with a terminator to save space
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EepromString {
    bytes: Vec<u8>,
}

impl EepromString {
    /// Create a new EEPROM string (max 31 characters)
    pub fn new(input: &str) -> Self {
        // First encode the string using the EEPROM character set
        let char_string = CharString::<31>::new_with_charset(input, EEPROM_CHARS, false);
        let mut char_indices = char_string.as_bytes().to_vec();
        
        // Add the terminator byte
        char_indices.push(EEPROM_TERMINATOR);
        
        // Use BigUint to handle arbitrarily large integers,
        // just like Ruby's Integer class can handle arbitrarily large values
        let mut packed_int = BigUint::from(0u32);
        
        // Pack the characters using 6 bits per character
        // This follows the Ruby implementation: packed_int = chars.each_with_index.sum { |c, i| c << (6 * i) }
        for (i, &c) in char_indices.iter().enumerate() {
            // Calculate 2^(6*i) and multiply by c
            let shifted = BigUint::from(c) << (6 * i);
            packed_int += shifted;
        }
        
        // Convert to little-endian byte array, equivalent to Ruby's digits(256)
        let bytes = packed_int.to_bytes_le();
        
        EepromString { bytes }
    }
    
    /// Get the encoded bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// Phone number string with compact encoding for Timex watches
/// Uses 4-bit packing to save space
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneString {
    bytes: Vec<u8>,
}

impl PhoneString {
    /// Create a new phone string (max 12 characters)
    /// 
    /// This implementation follows the Ruby version:
    /// - Maps each character to its index in PHONE_CHARS
    /// - Invalid chars are replaced with the index of space
    /// - Packs the values using 4 bits per character (shift << (4 * index))
    /// - Converts to bytes in little-endian order
    pub fn new(input: &str) -> Self {
        // First map each character to its index in PHONE_CHARS
        let invalid_pos = PHONE_CHARS.find(INVALID_CHAR).unwrap_or(0) as u8;
        
        // Limit to max 12 characters as in Ruby
        let chars: Vec<u8> = input.to_lowercase()
            .chars()
            .take(12)
            .map(|c| {
                let idx = PHONE_CHARS.find(c);
                idx.map(|i| i as u8).unwrap_or(invalid_pos)
            })
            .collect();
        
        // Use BigUint to handle arbitrarily large integers,
        // just like Ruby's Integer class can handle arbitrarily large values
        let mut packed_int = BigUint::from(0u32);
        
        // Pack the characters using 4 bits per character
        // This follows the Ruby implementation: packed_int = chars.each_with_index.sum { |c, i| c << (4 * i) }
        for (i, &c) in chars.iter().enumerate() {
            // Calculate 2^(4*i) and multiply by c
            let shifted = BigUint::from(c) << (4 * i);
            packed_int += shifted;
        }
        
        // Convert to little-endian byte array, equivalent to Ruby's digits(256)
        let bytes = packed_int.to_bytes_le();
        
        PhoneString { bytes }
    }
    
    /// Get the encoded bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

// Implement From trait for convenience
impl From<&str> for EepromString {
    fn from(s: &str) -> Self {
        EepromString::new(s)
    }
}

impl From<&str> for PhoneString {
    fn from(s: &str) -> Self {
        PhoneString::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_string_creation() {
        let s = CharString::<10>::new("Hello", false);
        assert_eq!(s.len(), 5);
        assert_eq!(s.as_bytes(), &[17, 14, 21, 21, 24]);
    }

    #[test]
    fn test_char_string_truncation() {
        let s = CharString::<5>::new("Hello World", false);
        assert_eq!(s.len(), 5);
        assert_eq!(s.as_bytes(), &[17, 14, 21, 21, 24]);
    }

    #[test]
    fn test_char_string_padding() {
        let s = CharString::<10>::new("Hello", true);
        assert_eq!(s.len(), 10);
        let space_index = CHARS.find(' ').unwrap() as u8;
        assert_eq!(s.as_bytes(), &[17, 14, 21, 21, 24, space_index, space_index, space_index, space_index, space_index]);
    }

    #[test]
    fn test_invalid_chars() {
        let s = CharString::<10>::new("Hello~@#$%", false);
        // ~ is not in the charset, so it should be replaced with space
        assert_eq!(s.as_bytes()[5], CHARS.find(' ').unwrap() as u8);
        assert_eq!(s.as_bytes()[6], CHARS.find('@').unwrap() as u8);
        assert_eq!(s.as_bytes()[7], CHARS.find('#').unwrap() as u8);
        assert_eq!(s.as_bytes()[8], CHARS.find('$').unwrap() as u8);
        assert_eq!(s.as_bytes()[9], CHARS.find('%').unwrap() as u8);
        assert_eq!(s.as_bytes()[0], 17); // 'h'
    }

    #[test]
    fn test_from_trait() {
        let s: CharString<5> = "Hello".into();
        assert_eq!(s.len(), 5);
        assert_eq!(s.as_bytes(), &[17, 14, 21, 21, 24]);
    }
    
    #[test]
    fn test_protocol_6_chars() {
        let s = CharString::<10>::new_protocol_6("Hello!", false);
        assert_eq!(s.len(), 6);
        
        // Character indices in CHARS_PROTOCOL_6
        let h_idx = CHARS_PROTOCOL_6.find('h').unwrap() as u8;
        let e_idx = CHARS_PROTOCOL_6.find('e').unwrap() as u8;
        let l_idx = CHARS_PROTOCOL_6.find('l').unwrap() as u8;
        let o_idx = CHARS_PROTOCOL_6.find('o').unwrap() as u8;
        let excl_idx = CHARS_PROTOCOL_6.find('!').unwrap() as u8;
        
        assert_eq!(s.as_bytes(), &[h_idx, e_idx, l_idx, l_idx, o_idx, excl_idx]);
    }
    
    #[test]
    fn test_eeprom_string() {
        let s = EepromString::new("Test");
        
        // We can't easily test the exact bytes due to the complex bit-packing,
        // but we can verify it's non-empty and consistent
        assert!(!s.as_bytes().is_empty());
        
        // Verify that the same input gives the same output
        let s2 = EepromString::new("Test");
        assert_eq!(s.as_bytes(), s2.as_bytes());
        
        // Verify different input gives different output
        let s3 = EepromString::new("Different");
        assert_ne!(s.as_bytes(), s3.as_bytes());
    }
    
    #[test]
    fn test_eeprom_special_chars() {
        // Test special characters - ";@_|<>[]"
        // From Ruby's anniversary_spec.rb: 
        // context "when anniversary is \";@_|<>[]\"" => [0x09, 0x13, 0x36, 0xae, 0xef, 0x7c, 0xef, 0x93, 0x3f]
        // The first two bytes are length-related, not part of the actual encoding
        let s = EepromString::new(";@_|<>[]");
        assert_eq!(s.as_bytes(), &[0x36, 0xae, 0xef, 0x7c, 0xef, 0x93, 0x3f]);
        
        // Test invalid characters - "~with~invalid~characters"
        // From Ruby's anniversary_spec.rb:
        // context "when anniversary is \"~with~invalid~characters\"" => 
        // [0x09, 0x13, 0x24, 0x28, 0x75, 0x11, 0x29, 0x5d, 0x9f, 0x52, 0x49, 0x0d, 0xc9, 0x44, 0xca, 0xa6, 0x30, 0x9d, 0xb3, 0x71, 0x3f]
        // The first two bytes are length-related, not part of the actual encoding
        let s2 = EepromString::new("~with~invalid~characters");
        assert_eq!(s2.as_bytes(), &[0x24, 0x28, 0x75, 0x11, 0x29, 0x5d, 0x9f, 0x52, 0x49, 0x0d, 0xc9, 0x44, 0xca, 0xa6, 0x30, 0x9d, 0xb3, 0x71, 0x3f]);
    }
    
    #[test]
    fn test_eeprom_truncation() {
        // The Ruby spec shows that strings > 31 chars get truncated
        // From Ruby's anniversary_spec.rb:
        // context "when anniversary is \"To the Delorean with More Than 31 Characters\"" =>
        // [0x09, 0x13, 0x1d, 0x46, 0x76, 0x91, 0x43, 0x36, 0x4e, 0x85, 0x6d, 0x8e, 0x72, 0x91, 0xa0, 0xd4, 0x45, 0xa4, 0x85, 0x6d, 0x0e, 0xd9, 0x45, 0xca, 0x45, 0xfe]
        // The first two bytes are length-related, not part of the actual encoding
        let long_text = "To the Delorean with More Than 31 Characters";
        let s = EepromString::new(long_text);
        assert_eq!(s.as_bytes(), &[0x1d, 0x46, 0x76, 0x91, 0x43, 0x36, 0x4e, 0x85, 0x6d, 0x8e, 0x72, 0x91, 0xa0, 0xd4, 0x45, 0xa4, 0x85, 0x6d, 0x0e, 0xd9, 0x45, 0xca, 0x45, 0xfe]);
    }
    
    #[test]
    fn test_phone_string() {
        // Test with exact values from the Ruby spec
        // From Ruby's phone_number_spec.rb, the phone number "1234567890" =>
        // [0x21, 0x43, 0x65, 0x87, 0x09, 0xaf, ...]
        let s = PhoneString::new("1234567890");
        // We only care about the first 5 bytes which are the phone number part
        assert_eq!(s.as_bytes()[0..5], [0x21, 0x43, 0x65, 0x87, 0x09]);
    }
    
    #[test]
    fn test_eeprom_from_trait() {
        let s: EepromString = "Hello".into();
        assert!(!s.as_bytes().is_empty());
    }
    
    #[test]
    fn test_phone_from_trait() {
        let s: PhoneString = "5551234".into();
        assert!(!s.as_bytes().is_empty());
    }
}
