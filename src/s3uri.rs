use std::fmt;

use crate::S3Uri;

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
        write!(f, "s3://{}/{}", self.bucket, self.key)
    }
}

impl S3Uri {
    /// Joins a suffix to the key.
    ///
    /// Separators at the start and end of the suffix will be normalised
    /// as-needed.
    ///
    /// For example, to join a key onto a bucket:
    ///
    /// ```rust
    /// let uri = s3uri::from_bucket("circus")
    ///     .unwrap()
    ///     .join("clowns.jpg");
    ///
    /// assert_eq!(uri, "s3://circus/clowns.jpg");
    /// ```
    ///
    /// `.join()` calls can be chained:
    ///
    /// ```rust
    /// let uri = s3uri::from_bucket("circus")
    ///     .unwrap()
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
    ///     .unwrap()
    ///     .join("images");
    ///
    /// let clowns = images.join("clowns.jpg");
    /// let tents = images.join("tents.jpg");
    ///
    /// assert_eq!(clowns, "s3://circus/images/clowns.jpg");
    /// assert_eq!(tents, "s3://circus/images/tents.jpg");
    /// ```
    pub fn join(&self, part: &str) -> S3Uri {
        S3Uri {
            bucket: self.bucket.clone(),
            key: self.key.join(part),
        }
    }
}
