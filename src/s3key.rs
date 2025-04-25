use std::{fmt, ops::Deref};

use crate::S3Key;

impl Deref for S3Key {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<str> for S3Key {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }

    fn ne(&self, other: &str) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<&str> for S3Key {
    fn eq(&self, other: &&str) -> bool {
        self.0.eq(other)
    }

    fn ne(&self, other: &&str) -> bool {
        !self.eq(other)
    }
}

impl fmt::Display for S3Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self)
    }
}

impl S3Key {
    /// Creates an empty S3 key.
    ///
    /// ```rust
    /// let key = s3uri::S3Key::empty();
    /// assert!(key.is_empty());
    /// ```
    pub fn empty() -> S3Key {
        S3Key(String::from(""))
    }

    /// Joins a suffix to the key.
    ///
    /// Slashes at the start and end of the suffix will be normalised as-needed.
    ///
    /// For example, to join the prefix "images" and "clowns.jpg":
    ///
    /// ```rust
    /// let key = s3uri::S3Key::new("images")
    ///     .join("clowns.jpg");
    ///
    /// assert_eq!(key, "images/clowns.jpg");
    /// ```
    ///
    /// The same result is reached if slashes are included:
    ///
    /// ```rust
    /// let key = s3uri::S3Key::new("images/")
    ///     .join("/clowns.jpg");
    ///
    /// assert_eq!(key, "images/clowns.jpg");
    /// ```
    ///
    /// `.join()` calls can be chained:
    ///
    /// ```rust
    /// let key = s3uri::S3Key::new("images")
    ///     .join("staff")
    ///     .join("clowns.jpg");
    ///
    /// assert_eq!(key, "images/staff/clowns.jpg");
    /// ```
    ///
    /// This example creates an "images" key prefix and uses it to create two
    /// further keys.
    ///
    /// ```rust
    /// let images = s3uri::S3Key::new("images");
    /// let clowns = images.join("clowns.jpg");
    /// let tents = images.join("tents.jpg");
    ///
    /// assert_eq!(clowns, "images/clowns.jpg");
    /// assert_eq!(tents, "images/tents.jpg");
    /// ```
    pub fn join(&self, part: &str) -> S3Key {
        let suffix = if part.ends_with("/") { "/" } else { "" };
        let normalized = part.trim_matches('/').to_string();
        let needs_separator = !self.is_empty() && !self.ends_with("/");
        let separator = if needs_separator { "/" } else { "" };
        S3Key(format!("{}{}{}{}", self, separator, normalized, suffix))
    }

    /// Creates a new S3 key.
    ///
    /// `key` can be a full key, or a key prefix to add to later with `S3Key::join`.
    ///
    /// ```rust
    /// let key = s3uri::S3Key::new("clowns.jpg");
    /// assert_eq!(key, "clowns.jpg");
    /// ```
    pub fn new(key: &str) -> S3Key {
        S3Key::empty().join(key)
    }
}

#[cfg(test)]
mod tests {
    use crate::S3Key;

    #[test]
    fn test_empty() {
        let key = S3Key::empty();
        assert_eq!(key, "");
    }

    #[test]
    fn test_new() {
        let cases = [
            ["clowns.jpg", "clowns.jpg"],
            ["/clowns.jpg", "clowns.jpg"],
            ["//clowns.jpg", "clowns.jpg"],
            ["images/", "images/"],
            ["images//", "images/"],
        ];

        for case in cases {
            let key = S3Key::new(case[0]);
            assert_eq!(key, case[1]);
        }
    }

    #[test]
    fn test_join_chained() {
        let cases = [
            ["images", "clowns.jpg", "images/clowns.jpg"],
            ["images/", "clowns.jpg", "images/clowns.jpg"],
            ["images", "/clowns.jpg", "images/clowns.jpg"],
            ["images/", "/clowns.jpg", "images/clowns.jpg"],
        ];

        for case in cases {
            let key = S3Key::new(case[0]).join(case[1]);
            assert_eq!(key, case[2]);
        }
    }
}
