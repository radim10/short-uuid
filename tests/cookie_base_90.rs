#[cfg(test)]
mod tests {
    use short_uuid::{CustomTranslator, ShortUuidCustom};

    #[test]
    fn cookie_base_90() {
        let translator = CustomTranslator::new(short_uuid::COOKIE_BASE_90).unwrap();
        let orig_uuid_str = "420e04de-4f06-4e3b-8718-9f6ede92ea9e";

        let short = ShortUuidCustom::from_uuid_str(orig_uuid_str, &translator).unwrap();
        let short_uuid_str = short.to_string();

        let short2 = ShortUuidCustom::parse_str(&short_uuid_str, &translator).unwrap();
        let string2 = short2.to_uuid(&translator).unwrap();

        assert_eq!(string2.to_string(), orig_uuid_str);
    }
}
