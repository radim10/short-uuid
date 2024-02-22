use converter::BaseConverter;
use error::{CustomAlphabetError, ErrorKind, InvalidShortUuid, ParseStrCustomErrorKind};

pub mod converter;
mod error;
mod fmt;

mod macros;
use uuid;

pub const FLICKR_BASE: &str = "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";

pub const COOKIE_BASE_90: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!#$%&'()*+-./:<=>?@[]^_`{|}~";

type UuidError = uuid::Error;

// pub type Bytes = [u8; 16];
pub type Bytes = Vec<u8>;

#[derive(Debug)]
pub struct ShortUuid(Bytes);

// TODO: separate ShortUuid and ShortUuidCustom
impl ShortUuid {
    /// Generate a short UUID v5 in flickrBase58
    pub fn generate() -> ShortUuid {
        let default_converter = BaseConverter::default();
        generate_short(default_converter)
    }

    /// Generate a short UUID v4 in custom alphabet
    pub fn generate_custom(
        custom_alphabet: &'static str,
    ) -> Result<ShortUuid, CustomAlphabetError> {
        let converter = BaseConverter::new_custom(custom_alphabet)?;

        // Generate a short UUID v4 in custom alphabet
        let generated = generate_short(converter);

        Ok(generated)
    }

    /// Convert uuid to short format using flickrBase58
    pub fn from_uuid_str(uuid_string: &str) -> Result<ShortUuid, UuidError> {
        // validate
        let parsed = uuid::Uuid::parse_str(uuid_string)?;

        let cleaned = parsed.to_string().to_lowercase().replace("-", "");

        let converter = BaseConverter::default();

        // convert to selected base
        let result = converter.convert(&cleaned).unwrap();

        Ok(ShortUuid(result))
    }

    /// Convert uuid to short format using flickrBase58
    pub fn from_uuid(uuid: &uuid::Uuid) -> ShortUuid {
        let uuid_string = uuid.to_string();

        let cleaned = uuid_string.to_lowercase().replace("-", "");

        let converter = BaseConverter::default();

        // convert to selected base
        let result = converter.convert(&cleaned).unwrap();

        ShortUuid(result)
    }

    /// Convert uuid string to short format using custom alphabet
    pub fn from_uuid_str_custom(
        uuid_string: &str,
        custom_alphabet: &'static str,
    ) -> Result<ShortUuid, ErrorKind> {
        // validate
        let parsed = uuid::Uuid::parse_str(uuid_string).map_err(|e| ErrorKind::UuidError(e))?;

        // Validate custom alphabet
        let converter =
            BaseConverter::new_custom(custom_alphabet).map_err(|e| ErrorKind::CustomAlphabet(e))?;

        let cleaned = parsed.to_string().to_lowercase().replace("-", "");

        // convert to selected base
        let result = converter.convert(&cleaned).unwrap();

        Ok(ShortUuid(result))
    }

    /// Convert short to uuid
    pub fn to_uuid(self) -> uuid::Uuid {
        // Convert to hex
        let to_hex_converter = BaseConverter::default();

        // Convert to hex string
        let result = to_hex_converter.convert_to_hex(&self.0).unwrap();

        // Format hex string as uuid
        format_uuid(result)
    }

    /// Convert short to uuid using custom base
    pub fn to_uuid_custom(
        self,
        custom_alphabet: &'static str,
    ) -> Result<uuid::Uuid, CustomAlphabetError> {
        let to_hex_converter = BaseConverter::new_custom(custom_alphabet)?;

        // Convert to hex string
        // Should not fail
        let result = to_hex_converter.convert_to_hex(&self.0).unwrap();

        // Format hex string as uuid
        let uuid_value = format_uuid(result);

        Ok(uuid_value)
    }

    /// Validate that short uuid str is valid uuid
    pub fn parse_str(short_uuid_str: &str) -> Result<Self, InvalidShortUuid> {
        if short_uuid_str.len() != 22 {
            return Err(InvalidShortUuid);
        };

        let byte_vector: Vec<u8> = short_uuid_str.as_bytes().to_vec();

        let to_hex_converter = BaseConverter::default();

        // Convert to hex string
        let result = to_hex_converter.convert_to_hex(&byte_vector).unwrap();

        // validate
        uuid::Uuid::try_parse(&result).map_err(|_| InvalidShortUuid)?;

        Ok(Self(byte_vector))
    }

    /// Validate that short uuid str is valid uuid using custom alphabet
    pub fn parse_str_custom(
        short_uuid_str: &str,
        custom_alphabet: &'static str,
    ) -> Result<Self, ParseStrCustomErrorKind> {
        let byte_vector: Vec<u8> = short_uuid_str.as_bytes().to_vec();
        let converter = BaseConverter::new_custom(custom_alphabet)
            .map_err(|e| ParseStrCustomErrorKind::CustomAlphabet(e))?;

        //
        let result_string = converter.convert_to_hex(&byte_vector).unwrap();

        // validate
        uuid::Uuid::try_parse(&result_string)
            .map_err(|_| ParseStrCustomErrorKind::InvalidShortUuid)?;

        Ok(Self(byte_vector))
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

fn generate_short(base_converter: BaseConverter) -> ShortUuid {
    // Generate UUID v4
    let uuid_string = uuid::Uuid::new_v4().to_string();

    // clean string
    let cleaned = uuid_string.to_lowercase().replace("-", "");

    // convert to selected base
    let result = base_converter.convert(&cleaned).unwrap();

    ShortUuid(result)
}

fn format_uuid(value: String) -> uuid::Uuid {
    let formatted_uuid = format!(
        "{}-{}-{}-{}-{}",
        &value[0..8],
        &value[8..12],
        &value[12..16],
        &value[16..20],
        &value[20..32]
    );

    // Should not fail
    let uuid = uuid::Uuid::parse_str(&formatted_uuid).unwrap();

    return uuid;
}
