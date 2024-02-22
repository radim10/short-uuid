#[cfg(test)]
mod tests {
    use short_uuid::ShortUuid;

    #[test]
    fn convert_from_uuid_and_back() {
        let uuid = uuid::Uuid::new_v4();
        let uuid_string = uuid.to_string();

        let generated = ShortUuid::from_uuid(&uuid_string).unwrap();
        let _ = generated.to_string();

        dbg!("{:#?}", &generated);
        dbg!(&generated.to_string());

        let uuid = ShortUuid::to_uuid(generated);

        assert_eq!(uuid.to_string(), uuid_string);
        dbg!(&uuid);
    }
}
