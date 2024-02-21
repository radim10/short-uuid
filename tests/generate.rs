#[cfg(test)]
mod tests {
    use short_uuid::ShortUuid;

    #[test]
    fn generate_short_uuid() {
        let generated = ShortUuid::generate();
        let generated_string = generated.to_string();
        dbg!(&generated_string);

        assert_eq!(generated_string.len(), 22);
    }
}
