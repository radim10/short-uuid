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
        let uuid_str = "0408510d-ce4f-4761-ab67-2dfe2931c898";
        let short = ShortUuid::from_uuid_str(uuid_str).unwrap();

        let custom_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123465";

        let translator =
            CustomTranslator::new(custom_alphabet).expect("Failed to create translator");

        let custom_short = ShortUuidCustom::from_uuid_str(uuid_str, &translator).unwrap();

        let test_struct = TestStruct {
            id: short.clone(),
            custom_id: custom_short.clone(),
        };

        // Test serialization
        let serialized = serde_json::to_string(&test_struct).unwrap();

        // Verify the serialized output is a string
        let json_value: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        assert!(json_value["id"].is_string());
        assert!(json_value["custom_id"].is_string());

        // Test deserialization
        let deserialized: TestStruct = serde_json::from_str(&serialized).unwrap();

        // Verify round-trip equality
        assert_eq!(test_struct, deserialized);
        assert_eq!(short, deserialized.id);
        assert_eq!(custom_short, deserialized.custom_id);

        // Test direct string serialization
        let short_serialized = serde_json::to_string(&short).unwrap();
        assert_eq!(short_serialized, format!("\"{}\"", short.to_string()));

        let custom_short_serialized = serde_json::to_string(&custom_short).unwrap();
        panic!("{}", custom_short_serialized);
        assert_eq!(
            custom_short_serialized,
            format!("\"{}\"", custom_short.to_string())
        );
    }
}
