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
                dbg!(&generated);
                let short_string = generated.to_string();
                dbg!(&short_string);

                //
                let from_short =
                    ShortUuid::parse_str_custom(&short_string, custom_alphabet).unwrap();
                dbg!(&from_short);

                let from_short_uuid = from_short.to_uuid_custom(custom_alphabet).unwrap();
                assert_eq!(from_short_uuid.to_string(), uuid_string);
            }
            Err(e) => eprintln!("{:?}", e.to_string()),
        }
    }
}
