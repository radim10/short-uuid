//! Generating random shortened UUIDv4.
//! Using cusom alphabet

use short_uuid::CustomTranslator;

fn main() {
    use short_uuid::ShortUuidCustom;

    let custom_alphabet = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
    let translator = CustomTranslator::new(custom_alphabet).unwrap();

    let custom_short = ShortUuidCustom::generate(&translator);
    let custom_short_string = custom_short.to_string();

    println!("{}", custom_short_string);
}
