//! A trait for String-like types to check if a string is a reserved keyword,
//! and convert it to a safe non-keyword if so.
//!
//! Only strict and reserved keywords are checked against; weak keywords are not included.
//!
//! This library assumes the strings being checked are already valid identifiers in
//! every way *except* that it might be a reserved keyword.
//!
//! You can add this dependency with:
//!
//! ```toml
//! [dependencies]
//! check_keyword = "0.2"
//! ```
//!
//! # Example
//!
//! ```
//! # use check_keyword::CheckKeyword;
//! assert!(!"not_a_keyword".is_keyword());
//! assert_eq!("not_a_keyword".into_safe(), "not_a_keyword");
//!
//! assert!("match".is_keyword());
//! assert_eq!("match".into_safe(), "r#match");
//! ```
//! 
//! The [CheckKeyword::into_safe] method automatically checks [CheckKeyword::is_keyword] for you.
//! You don't need to call [CheckKeyword::is_keyword]
//! if you don't care whether it was originally a keyword or not.
//!
//! # Raw Identifiers
//!
//! Raw identifiers are identifiers that start with `r#`, and most keywords are allowed
//! to be used as raw identifiers.
//!
//! # Implementations
//! 
//! There is a special implementation of `CheckKeyword<String>` for [&str], and a
//! blanket implementation of `CheckKeyword<T>` where `T: AsRef<str> + From<String>`.
//! 
//! The blanket implementation covers [String], and is only tested for that, but should
//! cover any other String-like type as well. I can try to broaden the definition to fit
//! other types if needed (open an issue).
//! 
//! # Rust Editions
//!
//! By default, the keywords added in Rust Edition 2018 are included in the list of checked keywords.
//! This can be disabled with `default-features = false` in your Cargo.toml.
//!
//! ```toml
//! [dependencies]
//! check_keyword = { version = "0.2", default-features = false }
//! ```
//!
//! Future Rust editions may add new keywords, and this crate will be updated to reflect that.
//! (Or you can create an issue on github if I don't.)

use phf::phf_map;

/// The main trait.
/// 
/// The generic argument `T` is the output type of `into_safe`, and in the blanket implementation
/// is equal to `Self`. I would have used an associated type,
/// but I ran into the good-old "upstream crates may add new impl of trait" error when implementing [str].
pub trait CheckKeyword {
    fn is_keyword(&self) -> bool;

    fn keyword_status(&self) -> KeywordStatus;

    /// If it is a keyword, add "r#" to the beginning if possible,
    /// or "_" to the end if not.
    fn into_safe(self) -> String;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum KeywordStatus {
    NotKeyword,
    Strict {
        can_be_raw: bool
    },
    Reserved,
    Weak {
        restriction: WeakRestriction
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum WeakRestriction {
    None,
    Lifetime,
    Dyn
}

use KeywordStatus::*;
use WeakRestriction::*;

static KEYWORDS: phf::Map<&'static str, KeywordStatus> = phf_map! {

    // STRICT, 2015

    "as" => Strict { can_be_raw: true },
    "break" => Strict { can_be_raw: true },
    "const" => Strict { can_be_raw: true },
    "continue" => Strict { can_be_raw: true },
    "crate" => Strict { can_be_raw: false },
    "else" => Strict { can_be_raw: true },
    "enum" => Strict { can_be_raw: true },
    "extern" => Strict { can_be_raw: true },
    "false" => Strict { can_be_raw: true },
    "fn" => Strict { can_be_raw: true },
    "for" => Strict { can_be_raw: true },
    "if" => Strict { can_be_raw: true },
    "impl" => Strict { can_be_raw: true },
    "in" => Strict { can_be_raw: true },
    "let" => Strict { can_be_raw: true },
    "loop" => Strict { can_be_raw: true },
    "match" => Strict { can_be_raw: true },
    "mod" => Strict { can_be_raw: true },
    "move" => Strict { can_be_raw: true },
    "mut" => Strict { can_be_raw: true },
    "pub" => Strict { can_be_raw: true },
    "ref" => Strict { can_be_raw: true },
    "return" => Strict { can_be_raw: true },
    "self" => Strict { can_be_raw: false },
    "Self" => Strict { can_be_raw: false },
    "static" => Strict { can_be_raw: true },
    "struct" => Strict { can_be_raw: true },
    "super" => Strict { can_be_raw: false },
    "trait" => Strict { can_be_raw: true },
    "true" => Strict { can_be_raw: true },
    "type" => Strict { can_be_raw: true },
    "unsafe" => Strict { can_be_raw: true },
    "use" => Strict { can_be_raw: true },
    "where" => Strict { can_be_raw: true },
    "while" => Strict { can_be_raw: true },

    // STRICT, 2018

    "async" => if cfg!(feature = "2018") { Strict { can_be_raw: true } } else { NotKeyword },
    "await" => if cfg!(feature = "2018") { Strict { can_be_raw: true } } else { NotKeyword },

    // DYN

    "dyn" => if cfg!(feature = "2018") {
        Strict { can_be_raw: true }
    } else {
        Weak { restriction: Dyn }
    },

    // RESERVED, 2015

    "abstract" => Reserved,
    "become" => Reserved,
    "box" => Reserved,
    "do" => Reserved,
    "final" => Reserved,
    "macro" => Reserved,
    "override" => Reserved,
    "priv" => Reserved,
    "typeof" => Reserved,
    "unsized" => Reserved,
    "virtual" => Reserved,
    "yield" => Reserved,

    // RESERVED, 2018

    "try" => if cfg!(feature = "2018") { Reserved } else { NotKeyword },

    // WEAK

    "macro_rules" => Weak { restriction: None },
    "union" => Weak { restriction: None },
    "'static" => Weak { restriction: Lifetime }
};

impl<T: AsRef<str>> CheckKeyword for T {
    fn is_keyword(&self) -> bool {
        match self.keyword_status() {
            Strict { .. } | Reserved => true,
            _ => false
        }
    }

    fn keyword_status(&self) -> KeywordStatus {
        *KEYWORDS.get(self.as_ref()).unwrap_or(&NotKeyword)
    }

    fn into_safe(self) -> String {
        let self_ref = self.as_ref();
        match self.keyword_status() {
            Strict { can_be_raw: false } | Weak { restriction: Lifetime } => format!("{self_ref}_"),
            Strict { .. } | Reserved | Weak { restriction: Dyn } => format!("r#{self_ref}"),
            _ => self_ref.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_keyword() {
        assert!(String::from("match").is_keyword());
        assert!(!"hello".is_keyword());

        assert!("crate".is_keyword());

        assert_eq!(String::from("async").is_keyword(), cfg!(feature = "2018"));
    }

    #[test]
    fn keyword_status() {
        assert_eq!("asdf".keyword_status(), NotKeyword);

        assert_eq!(
            "dyn".keyword_status(),
            if cfg!(feature = "2018") {
                Strict { can_be_raw: true }
            } else {
                Weak { restriction: Dyn }
            }
        );

        assert_eq!(
            "'static".keyword_status(),
            Weak { restriction: Lifetime }
        );
    }

    #[test]
    fn into_safe() {
        assert_eq!(String::from("match").into_safe(), "r#match");
        assert_eq!("asdf".into_safe(), "asdf");

        assert_eq!("await".into_safe(),
                   if cfg!(feature = "2018") {
                       "r#await"
                   } else {
                       "await"
                   }
        );

        assert_eq!("self".into_safe(), "self_");
    }
}
