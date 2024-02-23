//! Generating random shortened UUIDv4.
//! Using default converter (flickrBase58)

fn main() {
    use short_uuid::short;

    let generated = short!();
    let generated_string = generated.to_string();

    println!("{}", generated_string);
}
