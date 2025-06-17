#[doc = include_str!("../README.md")]
use regex::Regex;

pub mod s3key;
pub mod s3uri;

/// An S3 key.
#[derive(Clone, Debug)]
pub struct S3Key(String);

/// An S3 URI.
#[derive(Clone, Debug)]
pub struct S3Uri {
    /// Bucket name.
    pub bucket: String,

    /// Key.
    pub key: Option<S3Key>,
}

/// Creates an `S3Uri` from a bucket name.
///
/// ```rust
/// let uri = s3uri::from_bucket("circus");
///
/// assert_eq!(uri, "s3://circus/");
/// ```
pub fn from_bucket(bucket: &str) -> S3Uri {
    S3Uri {
        bucket: bucket.to_string(),
        key: None,
    }
}

/// Creates an `S3Uri` from a string URI.
///
/// ```rust
/// let uri = s3uri::from_uri("s3://circus/clowns.jpg").unwrap();
///
/// assert_eq!(uri.bucket, "circus");
/// assert!(uri.key.is_some_and(|k| k == "clowns.jpg"));
/// ```
pub fn from_uri(uri: &str) -> Result<S3Uri, String> {
    let re = Regex::new(r"^s3://([^/]+)/?(.*)$").unwrap();

    match re.captures(uri) {
        Some(cap) => match cap.get(1) {
            Some(bucket) => {
                let mut uri = crate::from_bucket(bucket.as_str());

                if let Some(key) = cap.get(2) {
                    if !key.is_empty() {
                        uri = uri.join(key.as_str())
                    }
                }

                Ok(uri)
            }
            None => Err(format!("failed to parse bucket in S3 URI \"{}\"", uri)),
        },
        None => Err(format!("failed to parse \"{}\" as an S3 URI", uri)),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_from_bucket() {
        let uri = crate::from_bucket("circus");
        assert_eq!(uri.bucket, "circus");
    }

    #[test]
    fn test_from_uri() {
        let cases = [
            ["s3://circus/clowns.jpg", "circus", "clowns.jpg"],
            [
                "s3://circus/images/clowns.jpg",
                "circus",
                "images/clowns.jpg",
            ],
        ];

        for case in cases {
            let uri = crate::from_uri(case[0]).unwrap();

            assert_eq!(uri.bucket, case[1]);
            assert!(uri.key.is_some_and(|k| k == case[2]));
        }
    }

    #[test]
    fn test_from_uri_with_no_key() {
        let cases = [["s3://circus", "circus"], ["s3://circus/", "circus"]];

        for case in cases {
            let uri = crate::from_uri(case[0]).unwrap();

            assert_eq!(uri.bucket, case[1]);
            assert!(uri.key.is_none());
        }
    }

    #[test]
    fn test_from_uri_with_error() {
        let cases = [["s3://", "failed to parse \"s3://\" as an S3 URI"]];

        for case in cases {
            let uri = crate::from_uri(case[0]);
            assert!(uri.is_err_and(|e| e == case[1]));
        }
    }
}
