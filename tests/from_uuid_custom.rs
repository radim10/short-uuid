#[cfg(test)]
mod tests {
    use short_uuid::{CustomTranslator, ShortUuidCustom};

    #[test]
    fn from_uuid_str_custom() {
        let custom_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let uuid_string = "0408510d-ce4f-4761-ab67-2dfe2931c893";

        let translator = CustomTranslator::new(custom_alphabet).unwrap();

        let generated = ShortUuidCustom::from_uuid_str(&uuid_string, &translator);

        match generated {
            Ok(generated) => {
                let uuid = generated.to_uuid(&translator).unwrap();

                dbg!(&uuid);
                assert_eq!(uuid.to_string(), uuid_string);
            }
            Err(e) => eprintln!("{:?}", e.to_string()),
        }
    }
}
