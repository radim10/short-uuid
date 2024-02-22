#[cfg(test)]
mod tests {
    use short_uuid::ShortUuid;

    #[test]
    fn from_uuid_str_custom() {
        let custom_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let uuid_string = "0408510d-ce4f-4761-ab67-2dfe2931c893";

        let generated = ShortUuid::from_uuid_str_custom(&uuid_string, custom_alphabet);

        match generated {
            Ok(generated) => {
                let uuid = ShortUuid::to_uuid_custom(generated, custom_alphabet).unwrap();
                dbg!(&uuid);
                assert_eq!(uuid.to_string(), uuid_string);
            }
            Err(e) => eprintln!("{:?}", e.to_string()),
        }
    }
}
