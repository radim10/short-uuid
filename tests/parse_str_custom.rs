#[cfg(test)]
mod tests {
    use short_uuid::{CustomTranslator, ShortUuidCustom};

    #[test]
    fn from_uuid_str_custom() {
        let custom_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let custom_alphabet2 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijk/";
        let uuid_string = "0408510d-ce4f-4761-ab67-2dfe2931c893";

        let translator = CustomTranslator::new(custom_alphabet).unwrap();
        let translator2 = CustomTranslator::new(custom_alphabet2).unwrap();

        let generated = ShortUuidCustom::from_uuid_str(&uuid_string, &translator);

        match generated {
            Ok(generated) => {
                dbg!(&generated);
                let short_string = generated.to_string();
                dbg!(&short_string);

                //
                let from_short = ShortUuidCustom::parse_str(&short_string, &translator2).unwrap();
                dbg!(&from_short);

                let from_short_uuid = from_short.to_uuid(&translator).unwrap();
                assert_eq!(from_short_uuid.to_string(), uuid_string);
            }
            Err(e) => eprintln!("{:?}", e.to_string()),
        }
    }
}
