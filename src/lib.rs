use converter::BaseConverter;

pub mod converter;
mod fmt;

const FLICKR_BASE: &str = "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";

const COOKIE_BASE_90: &str =
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

    /// Convert uuid to short
    pub fn from_uuid(uuid_string: &str) -> ShortUuid {
        let cleaned = uuid_string.to_lowercase().replace("-", "");

        let converter = BaseConverter::new(FLICKR_BASE);

        // convert to selected base
        let result = converter.convert(&cleaned);

        ShortUuid(result)
    }

    /// Convert short to uuid
    pub fn to_uuid(self) -> uuid::Uuid {
        // Convert to hex
        let to_hex_converter = BaseConverter::new(FLICKR_BASE);

        let result = to_hex_converter.convert_to_hex(&self.0).unwrap();
        dbg!(&result);

        let formatted_uuid = format!(
            "{}-{}-{}-{}-{}",
            &result[0..8],
            &result[8..12],
            &result[12..16],
            &result[16..20],
            &result[20..32]
        );

        // Should not fail
        let uuid = uuid::Uuid::parse_str(&formatted_uuid).unwrap();

        return uuid;
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}