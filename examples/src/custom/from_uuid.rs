//! Get ShortUuid from standard uuid.
//! Using cusom alphabet

use short_uuid::CustomTranslator;

fn main() {
    use short_uuid::ShortUuidCustom;

    let custom_alphabet = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
    let translator = CustomTranslator::new(custom_alphabet).unwrap();

    let uuid = uuid::Uuid::new_v4();
    let short_custom = ShortUuidCustom::from_uuid(&uuid, &translator);
    let short_custom_string = short_custom.to_string();

    println!("{}", short_custom_string);
}
