#[cfg(test)]
mod tests {
    use short_uuid::{converter::BaseConverter, ShortUuid};

    #[test]
    fn parse_str() {
        let uuid_string = "0408510d-ce4f-4761-ab67-2dfe2931c898";
        let converter = BaseConverter::default();

        let result = converter.convert(&uuid_string.to_lowercase().replace('-', ""));

        let short_str = String::from_utf8(result).unwrap();

        let invalid = format!("{short_str}");

        let short_uuid = ShortUuid::parse_str(&invalid).unwrap();
        dbg!(&short_uuid);

        let uuid = short_uuid.to_uuid();
        dbg!(&uuid);

        assert_eq!(uuid.to_string(), uuid_string);
    }
}
