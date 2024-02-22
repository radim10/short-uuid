use std::fmt;

use crate::{error::CustomAlphabetError, FLICKR_BASE};

pub struct BaseConverter {
    /// Alphabet used for encoding or decoding
    alphabet: &'static str,
}

/// Create a new BaseConverter with default flickrBase58 alphabet
impl Default for BaseConverter {
    fn default() -> Self {
        Self {
            alphabet: FLICKR_BASE,
        }
    }
}

impl BaseConverter {
    pub const HEX: &'static str = "0123456789abcdef";

    /// Create a new BaseConverter with custom alphabet
    pub fn new_custom(alphabet: &'static str) -> Result<Self, CustomAlphabetError> {
        let converter = Self { alphabet };
        converter.validate()?;

        Ok(converter)
    }

    /// Validate custom alphabet
    pub fn validate(&self) -> Result<(), CustomAlphabetError> {
        let trimmed = self.alphabet.trim();

        if trimmed.is_empty() {
            return Err(CustomAlphabetError::EmptyAlphabet);
        }

        if trimmed.len() == 1 {
            return Err(CustomAlphabetError::Length);
        }

        let has_duplicates = trimmed
            .chars()
            .any(|c| trimmed.chars().filter(|&x| x == c).count() > 1);

        if has_duplicates {
            return Err(CustomAlphabetError::DuplicateAlphabetCharacter);
        }

        Ok(())
    }

    /// Convert uuid to custom bytes
    pub fn convert(&self, uuid_string: &str) -> Result<Vec<u8>, DecodeHexError> {
        // decode hex uuid into bytes
        let decoded_bytes = decode_hex(&uuid_string)?;

        let alphabet_length = get_short_id_length(self.alphabet.len() as f64);

        // encode bytes into custom alphabet bytes
        let result_bytes = bytes_to_custom_bytes(
            &decoded_bytes,
            self.alphabet.as_bytes(),
            alphabet_length,
            self.alphabet.chars().next().unwrap(),
        );

        Ok(result_bytes)
    }

    /// Convert custom bytes to hex
    pub fn convert_to_hex(&self, target_bytes: &[u8]) -> Result<String, &'static str> {
        // Convert custom-encoded bytes to regular bytes using custom_bytes_to_hex
        let regular_bytes = custom_bytes_to_bytes(target_bytes, self.alphabet.as_bytes())?;

        // Convert regular bytes to hex string using encode_hex
        let hex_string = encode_hex(&regular_bytes);

        // Pad the hex string to ensure it has the correct length
        let padded = pad_start(&hex_string, 32, '0');

        Ok(padded)
    }
}

#[derive(Debug)]
pub enum DecodeHexError {
    InvalidLength,

    /// Undex:Index of invalid character (without dashes)
    /// Invalid character value
    InvalidCharacter {
        index: usize,
        character: char,
    },
}

impl fmt::Display for DecodeHexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeHexError::InvalidLength => write!(f, "Invalid hexadecimal string length"),
            DecodeHexError::InvalidCharacter {
                character: char,
                index,
            } => {
                write!(f, "Invalid character '{}' at index {}", char, index)
            }
        }
    }
}

fn is_valid_hex_char(c: char) -> bool {
    c.is_digit(16)
}

fn decode_hex(hex_string: &str) -> Result<Vec<u8>, DecodeHexError> {
    let hex_chars: Vec<char> = hex_string.chars().collect();
    let mut result = Vec::new();

    if hex_chars.len() % 2 != 0 {
        return Err(DecodeHexError::InvalidLength);
    }

    let mut i = 0;
    while i < hex_chars.len() {
        let current_char = hex_chars[i];
        if !is_valid_hex_char(current_char) {
            return Err(DecodeHexError::InvalidCharacter {
                index: i,
                character: current_char,
            });
        }

        let first_digit = current_char.to_digit(16);
        let second_digit = hex_chars[i + 1].to_digit(16);

        match (first_digit, second_digit) {
            (Some(first), Some(second)) => {
                result.push((first << 4 | second) as u8);
            }
            _ => {
                return Err(DecodeHexError::InvalidCharacter {
                    index: i,
                    character: hex_chars[i + 1],
                });
            }
        }

        i += 2;
    }
    // for i in (0..hex_chars.len()).step_by(1) {
    //     let first_digit = hex_chars[i].to_digit(16);
    //     let second_digit = hex_chars[i + 1].to_digit(16);
    //
    //     match (first_digit, second_digit) {
    //         (Some(first), Some(second)) => {
    //             result.push((first << 4 | second) as u8);
    //         }
    //         _ => {
    //             return Err(DecodeHexError::InvalidCharacter(hex_chars[i]));
    //         }
    //     }
    // }

    Ok(result)
}

fn bytes_to_custom_bytes(
    bytes: &[u8],
    alphabet: &[u8],
    target_length: usize,
    padding_char: char,
) -> Vec<u8> {
    let base = alphabet.len() as u128;

    let mut result = Vec::new();
    let mut value = 0u128;

    for &byte in bytes {
        value = value * 256 + byte as u128;
    }

    while value > 0 {
        let index = (value % base) as usize;
        result.push(alphabet[index]);
        value /= base;
    }

    result.reverse(); // Reverse the result since we're building it from the least significant digit

    // Pad the result to the target length with the specified padding character
    while result.len() < target_length {
        // result.push(padding_char as u8);
        result.insert(0, padding_char as u8);
    }

    result
}

fn custom_bytes_to_bytes(encoded_bytes: &[u8], alphabet: &[u8]) -> Result<Vec<u8>, &'static str> {
    let base = alphabet.len() as u128;

    let mut result = 0u128;

    for &byte in encoded_bytes {
        let index = alphabet.iter().position(|&c| c == byte);

        match index {
            Some(i) => {
                // Check for overflow before multiplying
                result = result.checked_mul(base).ok_or("Multiplication overflow")? + i as u128;
            }
            None => return Err("Invalid character in custom base"),
        }
    }

    let mut decoded_bytes = Vec::new();

    // Reconstruct the bytes from the numerical value
    while result > 0 {
        decoded_bytes.push((result % 256) as u8);
        result /= 256;
    }

    decoded_bytes.reverse(); // Reverse the result since we're building it from the least significant digit
    Ok(decoded_bytes)
}

fn encode_hex(bytes: &[u8]) -> String {
    let hex_chars: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    hex_chars.join("")
}

fn pad_start(input: &str, target_length: usize, padding: char) -> String {
    if input.len() >= target_length {
        return input.to_string();
    }

    let padding_length = target_length - input.len();
    let padded_string: String = std::iter::repeat(padding).take(padding_length).collect();
    format!("{}{}", padded_string, input)
}

fn get_short_id_length(alphabet_length: f64) -> usize {
    ((2.0_f64.powi(128)).log(alphabet_length).ceil()) as usize
}
//
