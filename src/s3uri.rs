use std::fmt;

use crate::{S3Key, S3Uri};

impl PartialEq<str> for S3Uri {
    fn eq(&self, other: &str) -> bool {
        self.to_string().eq(other)
    }
}

impl PartialEq<&str> for S3Uri {
    fn eq(&self, other: &&str) -> bool {
        self.to_string().eq(other)
    }
}

impl fmt::Display for S3Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.key {
            Some(k) => write!(f, "s3://{}/{}", self.bucket, k),
            None => write!(f, "s3://{}/", self.bucket),
        }
    }
}

impl S3Uri {
    /// Describes whether the key is a prefix (i.e. ends with `/`, returns
    /// `true`) or an object key (returns `false`).
    ///
    /// ```rust
    /// let images = s3uri::from_bucket("circus")
    ///     .join("images/");
    ///
    /// assert!(images.is_prefix());
    ///
    /// let clowns = images.join("clowns.jpg");
    /// assert!(!clowns.is_prefix());
    /// ```
    pub fn is_prefix(&self) -> bool {
        match &self.key {
            Some(k) => k.is_prefix(),
            None => true,
        }
    }

    /// Joins a suffix to the key.
    ///
    /// Separators at the start and end of the suffix will be normalised
    /// as-needed.
    ///
    /// For example, to join a key onto a bucket:
    ///
    /// ```rust
    /// let uri = s3uri::from_bucket("circus")
    ///     .join("clowns.jpg");
    ///
    /// assert_eq!(uri, "s3://circus/clowns.jpg");
    /// ```
    ///
    /// `.join()` calls can be chained:
    ///
    /// ```rust
    /// let uri = s3uri::from_bucket("circus")
    ///     .join("images")
    ///     .join("clowns.jpg");
    ///
    /// assert_eq!(uri, "s3://circus/images/clowns.jpg");
    /// ```
    ///
    /// This example creates an "images" key prefix and uses it to create two
    /// further URIs.
    ///
    /// ```rust
    /// let images = s3uri::from_bucket("circus")
    ///     .join("images");
    ///
    /// let clowns = images.join("clowns.jpg");
    /// let tents = images.join("tents.jpg");
    ///
    /// assert_eq!(clowns, "s3://circus/images/clowns.jpg");
    /// assert_eq!(tents, "s3://circus/images/tents.jpg");
    /// ```
    pub fn join(&self, part: &str) -> S3Uri {
        match &self.key {
            Some(k) => S3Uri {
                bucket: self.bucket.clone(),
                key: Some(k.join(part)),
            },
            None => S3Uri {
                bucket: self.bucket.clone(),
                key: Some(S3Key::empty().join(part)),
            },
        }
    }

    /// Gets the segment of the key after the final separator.
    ///
    /// ```rust
    /// let root = s3uri::from_bucket("circus");
    /// assert!(root.leaf().is_none());
    ///
    /// let images = root.join("images/");
    /// assert!(images.leaf().is_some_and(|k| k == "images/"));
    ///
    /// let clowns = images.join("clowns.jpg");
    /// assert!(clowns.leaf().is_some_and(|k| k == "clowns.jpg"));
    /// ```
    pub fn leaf(&self) -> Option<&str> {
        match &self.key {
            Some(k) => k.leaf(),
            None => None,
        }
    }
}

#[test]
fn test_uri_clone() {
    let uri = crate::from_bucket("circus").join("clowns.jpg");
    let clone = uri.clone();

    assert_eq!(clone, "s3://circus/clowns.jpg");
}
