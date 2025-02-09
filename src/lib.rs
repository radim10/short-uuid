// THE MIT License (MIT)
//
// Copyright (c) 2024 Radim Höfer
// See the license.txt file in the project root

//! Generate and translate standard UUIDs into shorter or just different formats and back.
//!
//! A port of the JavaScript npm package [short-uuid](https://www.npmjs.com/package/short-uuid) so big thanks to the author.
//!
//! An example of short uuid string in default flickrBase58 alphabet:
//!```text
//! mhvXdrZT4jP5T8vBxuvm75
//!```
//!
//! ## Getting started
//!
//! Install the package with `cargo`:
//!
//! ```sh
//! cargo add short-uuid
//! ```
//!
//! or add it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! short-uuid = "0.1.4"
//! ```
//! ### Examples
//!
//! Generate short uuidv4 encoded in flickrBase58 format:
//
//! ```rust
//! use short_uuid::ShortUuid;
//!
//! let shortened_uuid = ShortUuid::generate();
//! ```
//!
//! Generate short uuidv4 encoded in flickrBase58 format using macro:
//! ```rust
//! use short_uuid::short;
//!
//! let shortened_uuid = short!();
//! ```
//!
//! Generate short uuidv4 using custom alphabet:
//!
//! ```rust
//! use short_uuid::{ShortUuidCustom, CustomTranslator};
//!
//! let custom_alphabet = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
//! let translator = CustomTranslator::new(custom_alphabet).unwrap();
//!
//! let custom_short = ShortUuidCustom::generate(&translator);
//! let custom_short_string = custom_short.to_string();
//! ```
//!
//! Get shortened uuid from standard uuid:
//!
//! ```rust
//! use short_uuid::ShortUuid;
//! // create normal uuid v4
//! let uuid = uuid::Uuid::new_v4();
//!
//! let short = ShortUuid::from_uuid(&uuid);
//! ```
//! Get shortened uuid from uuid using custom alphabet:
//!
//! ```rust
//! use short_uuid::{ShortUuidCustom, CustomTranslator};
//!
//! let custom_alphabet = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
//! let translator = CustomTranslator::new(custom_alphabet).unwrap();
//!
//! let uuid = uuid::Uuid::new_v4();
//! let short_custom = ShortUuidCustom::from_uuid(&uuid, &translator);
//! let short_custom_string = short_custom.to_string();
//! ```
//!
//! Get shortened uuid from uuid string:
//!
//! ```rust
//! use short_uuid::ShortUuid;
//!
//! let uuid_str = "3cfb46e7-c391-42ef-90b8-0c1d9508e752";
//! let short_uuid = ShortUuid::from_uuid_str(&uuid_str);
//! ```
//!
//! Get shortened uuid from uuid string using custom alphabet:
//!
//! ```rust
//! use short_uuid::{ShortUuidCustom, CustomTranslator};
//!
//! let custom_alphabet = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
//! let translator = CustomTranslator::new(custom_alphabet).unwrap();
//!
//! let uuid_str = "3cfb46e7-c391-42ef-90b8-0c1d9508e752";
//! let short_custom = ShortUuidCustom::from_uuid_str(&uuid_str, &translator).unwrap();
//! let short_custom_string = short_custom.to_string();
//! ```
//!
//! # References
//! * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
//! * [uuid crate](https://crates.io/crates/uuid)

use converter::BaseConverter;
use error::{CustomAlphabetError, ErrorKind, InvalidShortUuid};

/// Convert between different bases
pub mod converter;
mod error;
mod fmt;

mod macros;
use uuid;

pub const FLICKR_BASE_58: &str = "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";

pub const COOKIE_BASE_90: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!#$%&'()*+-./:<=>?@[]^_`{|}~";

type UuidError = uuid::Error;

// pub type Bytes = [u8; 16];
/// A byte array containing the ShortUuid
pub type Bytes = Vec<u8>;

/// Shortened UUID
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortUuid(Bytes);

/// Shortened UUID using custom alphabet
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortUuidCustom(Bytes);

/// Custom alphabet used for short uuid
pub type CustomAlphabet = &'static str;

/// Custom translator used use for base conversion
pub struct CustomTranslator(BaseConverter);

impl CustomTranslator {
    /// Create new custom translator
    pub fn new(custom_alphabet: CustomAlphabet) -> Result<Self, CustomAlphabetError> {
        let converter = BaseConverter::new_custom(custom_alphabet)?;
        Ok(Self(converter))
    }

    fn as_slice(&self) -> &BaseConverter {
        &self.0
    }
}

impl From<ShortUuid> for ShortUuidCustom {
    fn from(short_uuid: ShortUuid) -> Self {
        ShortUuidCustom(short_uuid.0)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ShortUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let string = String::from_utf8(self.0.clone())
            .map_err(|e| serde::ser::Error::custom(e.to_string()))?;
        serializer.serialize_str(&string)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ShortUuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        Ok(ShortUuid(string.into_bytes()))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ShortUuidCustom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let string = String::from_utf8(self.0.clone())
            .map_err(|e| serde::ser::Error::custom(e.to_string()))?;
        serializer.serialize_str(&string)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ShortUuidCustom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        Ok(ShortUuidCustom(string.into_bytes()))
    }
}

impl ShortUuid {
    /// Generate a short UUID v5 in flickrBase58
    pub fn generate() -> ShortUuid {
        generate_short(None)
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

    /// Convert short to uuid
    pub fn to_uuid(self) -> uuid::Uuid {
        // Convert to hex
        let to_hex_converter = BaseConverter::default();

        // Convert to hex string
        let result = to_hex_converter.convert_to_hex(&self.0).unwrap();

        // Format hex string as uuid
        format_uuid(result)
    }

    /// Convert short to uuid string to ShortUuid
    pub fn parse_str(short_uuid_str: &str) -> Result<Self, InvalidShortUuid> {
        let expected_len = 22;

        if short_uuid_str.len() != expected_len {
            return Err(InvalidShortUuid);
        };

        let byte_vector: Vec<u8> = short_uuid_str.as_bytes().to_vec();

        let to_hex_converter = BaseConverter::default();

        // Convert to hex string
        let result = to_hex_converter
            .convert_to_hex(&byte_vector)
            .map_err(|_| InvalidShortUuid)?;

        // validate
        uuid::Uuid::try_parse(&result).map_err(|_| InvalidShortUuid)?;

        Ok(Self(byte_vector))
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl ShortUuidCustom {
    /// Generate a short UUID v4 in custom alphabet
    pub fn generate(translator: &CustomTranslator) -> Self {
        // Generate a short UUID v4 in custom alphabet
        let generated = generate_short(Some(&translator.as_slice()));
        let short_custom: ShortUuidCustom = generated.into();

        short_custom
    }

    /// Convert uuid to short format using custom alphabet
    pub fn from_uuid(uuid: &uuid::Uuid, translator: &CustomTranslator) -> Self {
        let uuid_string = uuid.to_string();

        let cleaned = uuid_string.to_lowercase().replace("-", "");

        // convert to selected base
        let result = translator.as_slice().convert(&cleaned).unwrap();

        Self(result)
    }

    /// Convert uuid string to short format using custom alphabet
    pub fn from_uuid_str(
        uuid_string: &str,
        translator: &CustomTranslator,
    ) -> Result<Self, ErrorKind> {
        // validate
        let parsed = uuid::Uuid::parse_str(uuid_string).map_err(|e| ErrorKind::UuidError(e))?;

        let cleaned = parsed.to_string().to_lowercase().replace("-", "");

        // convert to selected base
        let result = translator.as_slice().convert(&cleaned).unwrap();

        Ok(Self(result))
    }

    /// Convert short to uuid using custom base
    pub fn to_uuid(self, translator: &CustomTranslator) -> Result<uuid::Uuid, CustomAlphabetError> {
        // Convert to hex string
        // Should not fail
        let result = translator
            .as_slice()
            .convert_to_hex(&self.as_slice())
            .unwrap();

        // Format hex string as uuid
        let uuid_value = format_uuid(result);

        Ok(uuid_value)
    }

    /// Validate that short uuid str is valid uuid using custom alphabet
    pub fn parse_str(
        short_uuid_str: &str,
        translator: &CustomTranslator,
    ) -> Result<Self, InvalidShortUuid> {
        let byte_vector: Vec<u8> = short_uuid_str.as_bytes().to_vec();

        let result_string = translator
            .as_slice()
            .convert_to_hex(&byte_vector)
            .map_err(|_| InvalidShortUuid)?;

        // validate
        uuid::Uuid::try_parse(&result_string).map_err(|_| InvalidShortUuid)?;

        Ok(Self(byte_vector))
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

fn generate_short(base_converter: Option<&BaseConverter>) -> ShortUuid {
    // Generate UUID v4
    let uuid_string = uuid::Uuid::new_v4().to_string();

    // clean string
    let cleaned = uuid_string.to_lowercase().replace("-", "");

    // convert to selected base
    let result = base_converter
        .unwrap_or(&BaseConverter::default())
        .convert(&cleaned)
        .unwrap();

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

impl From<uuid::Uuid> for ShortUuid {
    fn from(uuid: uuid::Uuid) -> ShortUuid {
        ShortUuid::from_uuid(&uuid)
    }
}

impl From<ShortUuid> for uuid::Uuid {
    fn from(short_uuid: ShortUuid) -> uuid::Uuid {
        short_uuid.to_uuid()
    }
}
