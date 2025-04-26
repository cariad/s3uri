#[test]
fn test_uri_root() {
    let uri = s3uri::from_bucket("circus");
    assert_eq!(uri, "s3://circus/");
}

#[test]
fn test_uri_with_key() {
    for k in ["circus.jpg", "/circus.jpg"] {
        let uri = s3uri::from_bucket("circus").join(k);

        assert_eq!(uri, "s3://circus/circus.jpg");
    }
}

#[test]
fn test_uri_with_key_prefix() {
    let uri = s3uri::from_bucket("circus").join("images/");

    assert_eq!(uri, "s3://circus/images/");
}

#[test]
fn test_uri_with_prefixed_key() {
    let uri = s3uri::from_bucket("circus")
        .join("images")
        .join("clowns.jpg");

    assert_eq!(uri, "s3://circus/images/clowns.jpg");
}
