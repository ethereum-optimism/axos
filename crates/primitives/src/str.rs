//! Strings
//!
//! Type aliases for rust string types.
//!
//! This module contains two type aliases, which are conditionally
//! compiled based on the presence of the `alloc` and `std` features.
//!
//! - If `std` is enabled, [`GenericString`] is an alias for [`alloc::string::String`].
//! - If `alloc` is enabled and `std` is not enabled, [`GenericString`] is an alias for [`alloc::string::String`].
//! - If neither `std` nor `alloc` are enabled, [`GenericString`] is an alias for `&'static str`.

/// An allocated string type alias for [`std::string::String`].
#[cfg(feature = "std")]
pub type GenericString = alloc::string::String;

/// An allocated string type alias for [`alloc::string::String`].
#[cfg(all(feature = "alloc", not(feature = "std")))]
pub type GenericString = alloc::string::String;

/// A string type that can be used with no heap allocation.
#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
pub type GenericString = &'static str;

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
impl From<&'static str> for GenericString {
    fn from(s: &'static str) -> Self {
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_alloc_generic_string() {
        let s = GenericString::from("hello world");
        assert_eq!(s, "hello world");
    }

    #[test]
    #[cfg(not(feature = "alloc"))]
    fn test_static_generic_string() {
        let s = GenericString::from("hello world");
        assert_eq!(s, "hello world");
    }
}
