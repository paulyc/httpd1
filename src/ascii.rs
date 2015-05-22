//! Operations on ASCII slices, in the vein of `std::ascii::AsciiExt`.
//!
//! Really, we are not working with ASCII, but with an unspecified 8-bit
//! character encoding that happens to be a superset of ASCII.  HTTP left the
//! character encoding used by headers and the like unspecified, but for our
//! purposes we'll pretend it's ISO-8859-1, aka the first 256 codepoints in
//! Unicode.

use std::ascii::AsciiExt;

/// Trait for objects that can have a prefix of ASCII (or really 8-bit, e.g.
/// ISO8859-1) characters.
pub trait AsciiPrefix {
  /// Checks whether this object starts with a certain sequence of 8-bit
  /// characters.
  fn starts_with_ignore_ascii_case(&self, prefix: &[u8]) -> bool;
}

impl AsciiPrefix for Vec<u8> {
  fn starts_with_ignore_ascii_case(&self, prefix: &[u8]) -> bool {
    // Reuse the implementation for &[u8].
    (&self[..]).starts_with_ignore_ascii_case(prefix)
  }
}

impl<'a> AsciiPrefix for &'a [u8] {
  fn starts_with_ignore_ascii_case(&self, prefix: &[u8]) -> bool {
    if self.len() < prefix.len() {
      false
    } else {
      (&self[..prefix.len()]).eq_ignore_ascii_case(prefix)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::AsciiPrefix;

  #[test]
  fn test_starts_with_ignore_ascii_case() {
    assert_eq!(true, b"".as_ref().starts_with_ignore_ascii_case(b""));
    assert_eq!(true, b"foobar".as_ref().starts_with_ignore_ascii_case(b""));
    assert_eq!(true, b"foobar".as_ref().starts_with_ignore_ascii_case(b"foo"));
    assert_eq!(true, b"FOOBAR".as_ref().starts_with_ignore_ascii_case(b"foo"));

    assert_eq!(false, b"foo".as_ref().starts_with_ignore_ascii_case(b"foobar"));
    assert_eq!(false, b"".as_ref().starts_with_ignore_ascii_case(b"foobar"));
  }
}
