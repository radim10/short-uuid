#[cfg(test)]
mod tests {
    use short_uuid::ShortUuid;

    #[test]
    fn short_custom_base() {
        let custom_base = short_uuid::COOKIE_BASE_90;
        let uuid_string = "0408510d-ce4f-4761-ab67-2dfe2931c898";

        let generated = ShortUuid::from_uuid_custom(&uuid_string, custom_base);
        dbg!(&generated.to_string());

        let uuid = ShortUuid::to_uuid_custom(generated, custom_base);
        dbg!(&uuid);
        assert_eq!(uuid.to_string(), uuid_string);
    }
}
