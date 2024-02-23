//! Generating random shortened UUIDv4.
//! Using default converter (flickrBase58)

fn main() {
    use short_uuid::ShortUuid;

    let short = ShortUuid::generate();
    let short_string = short.to_string();

    println!("{}", short_string);
}
