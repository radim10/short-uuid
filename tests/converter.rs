#[cfg(test)]
mod tests {
    use short_uuid::converter::BaseConverter;

    #[test]
    fn test_flickr_base_conversion() {
        let uuid_string = "0408510d-ce4f-4761-ab67-2dfe2931c898";
        let converter = BaseConverter::default();

        let result = converter
            .convert(&uuid_string.to_lowercase().replace('-', ""))
            .unwrap();

        let result_string = String::from_utf8(result).unwrap();
        dbg!(&result_string);

        assert_eq!(result_string.len(), 22);
    }
}
