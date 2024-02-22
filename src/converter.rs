pub struct BaseConverter {
    /// Alphabet used for encoding or decoding
    alphabet: &'static str,
    // dst_alphabet: &'static str,
    // src_alphabet: &'static str,
}

impl BaseConverter {
    // pub const BIN: &'static str = "01";
    // pub const OCT: &'static str = "01234567";
    // pub const DEC: &'static str = "0123456789";
    pub const HEX: &'static str = "0123456789abcdef";
    //
    pub fn new(alphabet: &'static str) -> Self {
        if alphabet.is_empty() {
            panic!("Bad alphabet");
        }

        Self { alphabet }
    }

    pub fn is_valid(&self, number: &str) -> bool {
        number.chars().all(|c| self.alphabet.contains(c))
    }

    pub fn convert(&self, uuid_string: &str) -> Vec<u8> {
        // decode hex uuid into bytes
        let decoded_bytes = decode_hex(&uuid_string).unwrap();

        let alphabet_length = get_short_id_length(self.alphabet.len() as f64);

        // encode bytes into custom alphabet bytes
        let result_bytes = bytes_to_custom_bytes(
            &decoded_bytes,
            self.alphabet.as_bytes(),
            alphabet_length,
            self.alphabet.chars().next().unwrap(),
        );

        result_bytes

        // let result_string = bytes_to_custom_string(&decoded_bytes, self.dst_alphabet.as_bytes());
        //
        // let alphabet_length = get_short_id_length(self.dst_alphabet.len() as f64);
        //
        // let padded = pad_start(
        //     &result_string,
        //     alphabet_length as usize,
        //     self.dst_alphabet.chars().next().unwrap(),
        // );
        //
        // result_string

        // return String::from_utf8(result_bytes).unwrap();
    }

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

fn decode_hex(hex_string: &str) -> Result<Vec<u8>, &'static str> {
    let hex_chars: Vec<char> = hex_string.chars().collect();
    let mut result = Vec::new();

    if hex_chars.len() % 2 != 0 {
        return Err("Invalid hexadecimal string length");
    }

    for i in (0..hex_chars.len()).step_by(2) {
        let first_digit = hex_chars[i].to_digit(16);
        let second_digit = hex_chars[i + 1].to_digit(16);

        match (first_digit, second_digit) {
            (Some(first), Some(second)) => {
                result.push((first << 4 | second) as u8);
            }
            _ => return Err("Invalid hexadecimal character"),
        }
    }

    Ok(result)
}

// fn bytes_to_custom_bytes(bytes: &[u8], alphabet: &[u8]) -> Vec<u8> {
//     let base = alphabet.len() as u128;
//
//     let mut result = Vec::new();
//     let mut value = 0u128;
//
//     for &byte in bytes {
//         value = value * 256 + byte as u128;
//     }
//
//     while value > 0 {
//         let index = (value % base) as usize;
//         result.push(alphabet[index]);
//         value /= base;
//     }
//
//     result.reverse(); // Reverse the result since we're building it from the least significant digit
//
//     result
// }
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

fn bytes_to_custom_string(bytes: &[u8], alphabet: &[u8]) -> String {
    let base = alphabet.len() as u128;

    let mut result = String::new();
    let mut value = 0u128;

    for &byte in bytes {
        value = value * 256 + byte as u128;
    }

    while value > 0 {
        let index = (value % base) as usize;
        result.insert(0, alphabet[index] as char);
        value /= base;
    }

    result
}

fn custom_bytes_to_bytes(encoded_bytes: &[u8], alphabet: &[u8]) -> Result<Vec<u8>, &'static str> {
    let base = alphabet.len() as u128;

    let mut result = 0u128;

    for &byte in encoded_bytes {
        let index = alphabet.iter().position(|&c| c == byte);

        match index {
            Some(i) => result = result * base + i as u128,
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
