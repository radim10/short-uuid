#[cfg(test)]
mod tests {
    use short_uuid::ShortUuid;

    #[test]
    fn short_custom_base() {
        let custom_base = "sb";
        let uuid_string = "0408510d-ce4f-4761-ab67-2dfe2931c893";

        let generated = ShortUuid::from_uuid_custom(&uuid_string, custom_base);

        match generated {
            Ok(generated) => {
                dbg!(&generated.to_string());

                let uuid = ShortUuid::to_uuid_custom(generated, custom_base).unwrap();
                dbg!(&uuid);
                assert_eq!(uuid.to_string(), uuid_string);
            }
            Err(e) => eprintln!("{:?}", e.to_string()),
        }
    }
}
