//! A trait for Strings and &str's to check if a string is a reserved keyword,
//! and convert it to a safe non-keyword if so.
//!
//! Only strict and reserved keywords are checked against; weak keywords are not included.
//!
//! You can add this dependency with:
//!
//! ```toml
//! [dependencies]
//! check_keyword = "0.1.0"
//! ```
//!
//! # Examples
//!
//! ```
//! use check_keyword::CheckKeyword;
//! let keyword = "match";
//!
//! assert!(keyword.is_keyword());
//! assert_eq!(keyword.to_safe(), "r#match");
//!
//! // There's also a self-consuming version if you want
//! assert_eq!(keyword.into_safe(), "r#match");
//! ```
//!
//! # Rust Editions
//!
//! By default, the keywords added in Rust Edition 2018 are included in the list of checked keywords.
//! This can be disabled with `default-features = false` in your Cargo.toml.
//!
//! ```toml
//! [dependencies]
//! check_keyword = { version = "0.1.0", default-features = false }
//! ```
//!
//! Future Rust editions may add new keywords, and this crate will be updated to reflect that.
//! (Or you can create an issue on github if I don't.)
mod strings;

#[macro_use] mod arr_macro;

pub trait CheckKeyword {
    /// The type returned by [to_safe](Keywords::to_safe) and [into_safe](Keywords::into_safe). Currently this is [String]
    /// for both the [String] and [&str] implementations, but future implementations might not return
    /// Strings.
    type SafeOutput;

    /// Checks if `self` is a keyword.
    fn is_keyword(&self) -> bool;

    /// Creates a new instance of [SafeOutput](Self::SafeOutput). If it is not a keyword, the contents are unchanged.
    /// If is is a keyword, "r#" is prepended to it.
    fn to_safe(&self) -> Self::SafeOutput;

    /// Identical to [to_safe](Keywords::to_safe), but it consumes `self`.
    fn into_safe(self) -> Self::SafeOutput;
}

arr!(static KEYWORDS: [&'static str; _] = [

    // STRICT, 2015

    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",

    // STRICT, 2018

    #[cfg(feature = "2018")] "async",
    #[cfg(feature = "2018")] "await",
    #[cfg(feature = "2018")] "dyn",

    // RESERVED, 2015

    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",

    // RESERVED, 2018

    #[cfg(feature = "2018")] "try"

]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_length() {
        println!("{}", KEYWORDS.len());
    }
}