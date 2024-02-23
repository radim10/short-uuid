#[cfg(test)]
mod tests {
    use short_uuid::ShortUuid;

    #[test]
    fn parse_str_success() {
        let short_string = "mhvXdrZT4jP5T8vBxuvm75";
        let short_uuid = ShortUuid::parse_str(short_string).expect("Failed to generate short uuid");
        dbg!(&short_uuid);
    }

    #[test]
    // Test with invalid short uuid
    fn parse_str_success_error() {
        let short_string = "mhvXdrZ";
        let short_uuid = ShortUuid::parse_str(short_string).expect_err("Test should have failed");
        dbg!(&short_uuid);
    }
}
