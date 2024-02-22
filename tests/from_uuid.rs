#[cfg(test)]
mod tests {
    use short_uuid::ShortUuid;

    #[test]
    fn from_uuid_str() {
        let generated = ShortUuid::from_uuid_str("4308510d-ce4f-4761-ab67-2dfe2931c834");

        match generated {
            Ok(generated) => {
                let generated_string = generated.to_string();
                dbg!(&generated_string);

                assert_eq!(generated_string.len(), 22);
            }
            Err(err) => {
                eprintln!("{:?}", err);
            }
        }
    }
}
