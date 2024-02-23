//! Get ShortUuid from standard uuid.
//! Using cusom alphabet

use short_uuid::CustomTranslator;

fn main() {
    use short_uuid::ShortUuidCustom;

    let custom_alphabet = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
    let translator = CustomTranslator::new(custom_alphabet).unwrap();

    let uuid_str = "3cfb46e7-c391-42ef-90b8-0c1d9508e752";

    let short_custom = ShortUuidCustom::from_uuid_str(&uuid_str, &translator).unwrap();
    let uuid = short_custom.to_uuid(&translator).unwrap();

    let uuid_str = uuid.to_string();

    println!("{}", uuid_str);
}
