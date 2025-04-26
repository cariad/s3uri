# s3uri: AWS S3 URIs in Rust

`s3uri` is a Rust library with structs and functions for building and parsing Amazon Web Services S3 URIs.

## Installation

```bash
cargo add s3uri
```

## Examples

### Parsing a URI

`s3uri::from_uri` parses a string URI and returns a `s3uri::S3Uri` struct with the bucket name and key set.

```rust
let uri = s3uri::from_uri("s3://circus/images/clowns.jpg").unwrap();

assert_eq!(uri.bucket, "circus");
assert_eq!(uri.key, "images/clowns.jpg");
```

### Creating a new URI

To create a new URI from scratch, pass the bucket name to `s3uri::from_bucket` to get a `s3uri::S3Uri` struct.

```rust
let uri = s3uri::from_bucket("circus").unwrap();

assert_eq!(uri, "s3://circus/");
assert_eq!(uri.bucket, "circus");
assert_eq!(uri.key, "");
```

### Joining keys and key prefixes

Call `s3uri::S3Uri::join()` to build up the key. Separators will be normalised and added between each segment as-needed.

```rust
let images_prefix = s3uri::from_bucket("circus")
    .unwrap()
    .join("images/");

assert_eq!(images_prefix, "s3://circus/images/");

let clowns_uri = images_prefix
    .join("staff")
    .join("clowns.jpg");

assert_eq!(clowns_uri, "s3://circus/images/staff/clowns.jpg");
```

### Check if a key is a prefix

Call `s3uri::S3Key::is_prefix()` to check if a key is a prefix (i.e. ends with a `/`) or not.

```rust
let images_uri = s3uri::from_bucket("circus")
    .unwrap()
    .join("images/");

assert!(images_uri.key.is_prefix());

let clowns_uri = images_uri.join("clowns.jpg");
assert!(!clowns_uri.key.is_prefix());
```

### Key leaf

Call `s3uri::S3Key::leaf()` to get the segment of the key after the final separator.

```rust
let images_uri = s3uri::from_bucket("circus")
    .unwrap()
    .join("images/");

assert_eq!(images_uri.key.leaf(), Some("images/"));

let clowns_uri = images_uri
    .join("staff")
    .join("clowns.jpg");

assert_eq!(clowns_uri.key.leaf(), Some("clowns.jpg"));
```

## Author

Hello! 👋 I'm Cariad Eccleston. You can find me at [cariad.earth](https://www.cariad.earth), [github.com/cariad](https://github.com/cariad), [linkedin.com/in/cariad](https://linkedin.com/in/cariad) and [@cariad.earth](https://bsky.app/profile/cariad.earth) on Bluesky.
