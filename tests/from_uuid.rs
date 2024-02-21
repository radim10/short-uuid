#[cfg(test)]
mod tests {
    use short_uuid::ShortUuid;

    #[test]
    fn generate_short_uuid() {
        let generated = ShortUuid::from_uuid("0408510d-ce4f-4761-ab67-2dfe2931c898");
        let generated_string = generated.to_string();
        dbg!(&generated_string);

        assert_eq!(generated_string.len(), 22);
    }
}
