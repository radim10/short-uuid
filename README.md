# short-uuid

Generate and translate standard UUIDs into shorter or just different formats and back.

A port of the JavaScript npm package [short-uuid](https://www.npmjs.com/package/short-uuid) so big thanks to the author.

An example of short uuid string in default flickrBase58 alphabet:

```
mhvXdrZT4jP5T8vBxuvm75
```

## Getting started

Install the package with `cargo`:

```sh
cargo add short-uuid
```

or add it to your `Cargo.toml`:

```toml
[dependencies]
short-uuid = "0.1.0"
```

### Examples

Generate short uuidv4 encoded in flickrBase58 format:

```rust
use short_uuid::ShortUuid;

let shortened_uuid = ShortUuid::generate();
```

Generate short uuidv4 encoded in flickrBase58 format using macro:

```rust
use short_uuid::short;

let shortened_uuid = short!();
```

Generate short uuidv4 using custom alphabet:

```rust
use short_uuid::{ShortUuidCustom, CustomTranslator};

let custom_alphabet = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
let translator = CustomTranslator::new(custom_alphabet).unwrap();

let custom_short = ShortUuidCustom::generate(&translator);
let custom_short_string = custom_short.to_string();
```

Get shortened uuid from standard uuid:

```rust
use short_uuid::ShortUuid;
// create normal uuid v4
let uuid = uuid::Uuid::new_v4();

let short = ShortUuid::from_uuid(&uuid);
```

Get shortened uuid from standard uuid using custom alphabet:

```rust
use short_uuid::{ShortUuidCustom, CustomTranslator};

let custom_alphabet = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
let translator = CustomTranslator::new(custom_alphabet).unwrap();

let uuid = uuid::Uuid::new_v4();
let short_custom = ShortUuidCustom::from_uuid(&uuid, &translator);
let short_custom_string = short_custom.to_string();
```

Get shortened uuid from uuid string:

```rust
use short_uuid::ShortUuid;

let uuid_str = "3cfb46e7-c391-42ef-90b8-0c1d9508e752";
let short_uuid = ShortUuid::from_uuid_str(&uuid_str);
```

Get shortened uuid from uuid string using custom alphabet:

```rust
use short_uuid::{ShortUuidCustom, CustomTranslator};

let custom_alphabet = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
let translator = CustomTranslator::new(custom_alphabet).unwrap();

let uuid_str = "3cfb46e7-c391-42ef-90b8-0c1d9508e752";
let short_custom = ShortUuidCustom::from_uuid_str(&uuid_str, &translator).unwrap();
let short_custom_string = short_custom.to_string();
```

# References

- [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
- [uuid crate](https://crates.io/crates/uuid)
