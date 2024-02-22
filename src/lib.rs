use converter::BaseConverter;

pub mod converter;
mod fmt;

pub const FLICKR_BASE: &str = "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";

pub const COOKIE_BASE_90: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!#$%&'()*+-./:<=>?@[]^_`{|}~";

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

        let converter = BaseConverter::new(FLICKR_BASE);

        // convert to selected base
        let result = converter.convert(&cleaned);

        ShortUuid(result)
    }

    /// Convert uuid to short format using flickrBase58
    pub fn from_uuid(uuid_string: &str) -> ShortUuid {
        let cleaned = uuid_string.to_lowercase().replace("-", "");

        let converter = BaseConverter::new(FLICKR_BASE);

        // convert to selected base
        let result = converter.convert(&cleaned);

        ShortUuid(result)
    }

    /// Convert uuid to short format using flickrBase58
    pub fn from_uuid_custom(uuid_string: &str, custom_base: &'static str) -> ShortUuid {
        let cleaned = uuid_string.to_lowercase().replace("-", "");

        let converter = BaseConverter::new(custom_base);

        // convert to selected base
        let result = converter.convert(&cleaned);

        ShortUuid(result)
    }

    /// Convert short to uuid
    pub fn to_uuid(self) -> uuid::Uuid {
        // Convert to hex
        let to_hex_converter = BaseConverter::new(FLICKR_BASE);

        // Convert to hex string
        let result = to_hex_converter.convert_to_hex(&self.0).unwrap();

        // Format hex string as uuid
        format_uuid(result)
    }

    /// Convert short to uuid using custom base
    pub fn to_uuid_custom(self, custom_base: &'static str) -> uuid::Uuid {
        let to_hex_converter = BaseConverter::new(custom_base);

        // Convert to hex string
        let result = to_hex_converter.convert_to_hex(&self.0).unwrap();

        // Format hex string as uuid
        format_uuid(result)
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
