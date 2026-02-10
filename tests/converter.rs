#[cfg(test)]
mod tests {
    use short_uuid::converter::BaseConverter;
    use short_uuid::ShortUuid;

    #[test]
    fn test_flickr_base_conversion() {
        let uuid_string = "0408510d-ce4f-4761-ab67-2dfe2931c898";
        let converter = BaseConverter::default();

        let result = converter
            .convert(&uuid_string.to_lowercase().replace('-', ""))
            .unwrap();

        let result_string = String::from_utf8(result).unwrap();
        assert_eq!(result_string.len(), 22);
        assert_eq!(result_string, "1uT6L1R6xPSdPC4Nr1kvnJ");

        let uuid = ShortUuid::parse_str(&result_string).unwrap().to_uuid();
        assert_eq!(uuid.to_string() , uuid_string);
    }
}
