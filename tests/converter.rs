#[cfg(test)]
mod tests {
    use short_uuid::converter::BaseConverter;

    const FLICKR_BASE_58: &str = "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
    // const COOKIE_BASE_90: &str =
    //     "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!#$%&'()*+-./:<=>?@[]^_`{|}~";

    #[test]
    fn test_flickr_base_conversion() {
        let uuid_string = "0408510d-ce4f-4761-ab67-2dfe2931c898";
        let converter = BaseConverter::new(FLICKR_BASE_58).unwrap();

        let result = converter.convert(&uuid_string.to_lowercase().replace('-', ""));

        let result_string = String::from_utf8(result).unwrap();
        dbg!(&result_string);

        assert_eq!(result_string.len(), 22);
    }
}
