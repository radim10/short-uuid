#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use short_uuid::{CustomTranslator, ShortUuid, ShortUuidCustom};

    #[cfg(feature = "serde")]
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestStruct {
        id: ShortUuid,
        custom_id: ShortUuidCustom,
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let orig_uuid_str = "0408510d-ce4f-4761-ab67-2dfe2931c898";
        let uuid = ShortUuid::from_uuid_str(&orig_uuid_str).unwrap();

        let custom_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123465";

        let translator =
            CustomTranslator::new(custom_alphabet).expect("Failed to create translator");

        let custom_uuid = ShortUuidCustom::from_uuid_str(&orig_uuid_str, &translator).unwrap();

        let test_struct = TestStruct {
            id: uuid.clone(),
            custom_id: custom_uuid.clone(),
        };

        // Test serialization
        let serialized = serde_json::to_string(&test_struct).unwrap();

        // Test deserialization
        let deserialized: TestStruct = serde_json::from_str(&serialized).unwrap();

        assert_eq!(test_struct, deserialized);
        assert_eq!(uuid, deserialized.id);
        assert_eq!(custom_uuid, deserialized.custom_id);
    }
}
