use converter::BaseConverter;
use error::ErrorKind;

pub mod converter;
mod error;
mod fmt;

pub const FLICKR_BASE: &str = "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";

pub const COOKIE_BASE_90: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!#$%&'()*+-./:<=>?@[]^_`{|}~";

type UuidError = uuid::Error;

// pub type Bytes = [u8; 16];
pub type Bytes = Vec<u8>;

#[derive(Debug)]
pub struct ShortUuid(Bytes);

impl ShortUuid {
    /// Generate a short UUID v4 in flickrBase58
    pub fn generate() -> ShortUuid {
        // Generate UUID v4
        let uuid_string = uuid::Uuid::new_v4().to_string();

        // clean string
        let cleaned = uuid_string.to_lowercase().replace("-", "");

        let converter = BaseConverter::new(FLICKR_BASE).unwrap();

        // convert to selected base
        let result = converter.convert(&cleaned);

        ShortUuid(result)
    }

    /// Convert uuid to short format using flickrBase58
    pub fn from_uuid_str(uuid_string: &str) -> Result<ShortUuid, UuidError> {
        // validate
        uuid::Uuid::parse_str(uuid_string)?;

        let cleaned = uuid_string.to_lowercase().replace("-", "");

        let converter = BaseConverter::new(FLICKR_BASE).unwrap();

        // convert to selected base
        let result = converter.convert(&cleaned);

        Ok(ShortUuid(result))
    }

    /// Convert uuid to short format using flickrBase58
    pub fn from_uuid_custom(
        uuid_string: &str,
        custom_base: &'static str,
    ) -> Result<ShortUuid, ErrorKind> {
        // validate
        let is_valid_uuid = uuid::Uuid::parse_str(uuid_string);

        if let Err(e) = is_valid_uuid {
            return Err(ErrorKind::UuidError(e));
        }

        let converter = BaseConverter::new(custom_base).map_err(|_| ErrorKind::EmptyAlphabet)?;

        let cleaned = uuid_string.to_lowercase().replace("-", "");

        // convert to selected base
        let result = converter.convert(&cleaned);

        Ok(ShortUuid(result))
    }

    /// Convert short to uuid
    pub fn to_uuid(self) -> uuid::Uuid {
        // Convert to hex
        let to_hex_converter = BaseConverter::new(FLICKR_BASE).unwrap();

        // Convert to hex string
        let result = to_hex_converter.convert_to_hex(&self.0).unwrap();

        // Format hex string as uuid
        format_uuid(result)
    }

    /// Convert short to uuid using custom base
    pub fn to_uuid_custom(self, custom_base: &'static str) -> Result<uuid::Uuid, ErrorKind> {
        let to_hex_converter =
            BaseConverter::new(custom_base).map_err(|_| ErrorKind::EmptyAlphabet)?;

        // Convert to hex string
        let result = to_hex_converter.convert_to_hex(&self.0).unwrap();

        // Format hex string as uuid
        let uuid_value = format_uuid(result);

        Ok(uuid_value)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
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
