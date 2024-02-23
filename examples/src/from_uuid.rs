//! Get ShortUuid from standard uuid.
//! Using default converter (flickrBase58)

fn main() {
    use short_uuid::ShortUuid;
    let uuid = uuid::Uuid::new_v4();

    let generated = ShortUuid::from_uuid(&uuid);
    let generated_string = generated.to_string();

    println!("{}", generated_string);
}
