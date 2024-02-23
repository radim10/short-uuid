#[cfg(test)]
mod tests {
    use short_uuid::ShortUuid;

    #[test]
    fn from_uuid_str_success() {
        let generated = ShortUuid::from_uuid_str("4308510d-ce4f-4761-ab67-2dfe2931c834")
            .expect("Failed to generate short uuid");

        let generated_string = generated.to_string();
        assert_eq!(generated_string.len(), 22);
    }

    #[test]
    fn from_uuid_str_errors() {
        let generated = ShortUuid::from_uuid_str("").expect_err("Test should have failed");

        dbg!(&generated);
    }
}
