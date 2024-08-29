#[cfg(test)]
mod tests {
    use short_uuid::{CustomTranslator, ShortUuidCustom};

    #[test]
    fn from_uuid_str_custom_success() {
        let custom_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let uuid_string = "0408510d-ce4f-4761-ab67-2dfe2931c893";

        let translator =
            CustomTranslator::new(custom_alphabet).expect("Failed to create translator");

        let custom_short_from_uuid = ShortUuidCustom::from_uuid_str(&uuid_string, &translator)
            .expect("Failed to generate short uuid from uuid string using custom alphabet");

        let expected = "AECFENzk9HYatnLf4pMciT";

        assert_eq!(custom_short_from_uuid.to_string(), expected);
    }

    #[test]
    /// Test with bad translator
    fn from_uuid_str_custom_bad_translator() {
        let custom_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let custom_alphabet2 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijk/";
        let uuid_string = "0408510d-ce4f-4761-ab67-2dfe2931c893";

        let translator =
            CustomTranslator::new(custom_alphabet).expect("Failed to create translator");

        let custom_short_from_uuid = ShortUuidCustom::from_uuid_str(&uuid_string, &translator)
            .expect("Failed to generate short uuid from uuid string using custom alphabet");

        let translator2 = CustomTranslator::new(custom_alphabet2).unwrap();

        let from_short =
            ShortUuidCustom::parse_str(&custom_short_from_uuid.to_string(), &translator2);

        assert!(from_short.is_err());

        dbg!(&from_short);
    }

    #[test]
    // Test with invalid short uuid
    fn parse_str_success_error() {
        let custom_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let short_string = "mhvXdrZ";
        let translator = CustomTranslator::new(custom_alphabet).unwrap();
        let short_uuid = ShortUuidCustom::parse_str(short_string, &translator)
            .expect_err("Test should have failed");
        dbg!(&short_uuid);
    }
}
