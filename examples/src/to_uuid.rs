//! Convert ShortUuid to standard uuid.
//! Using default converter (flickrBase58)

fn main() {
    use short_uuid::ShortUuid;

    let uuid_str = "3cfb46e7-c391-42ef-90b8-0c1d9508e752";
    let short_uuid = ShortUuid::from_uuid_str(&uuid_str).unwrap();

    let uuid = short_uuid.to_uuid();
    println!("{:?}", uuid);
}
