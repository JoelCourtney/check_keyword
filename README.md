# check_keyword

A trait for String-like types to check if a string is a keyword,
and convert it to a safe non-keyword if so. All types of keywords are supported,
and compile features can be used to check against past rust editions.
(Default is Rust 2024.)

This library assumes the strings being checked are already valid identifiers in
every way *except* that it might be a reserved keyword.

You can add this dependency with:

```toml
[dependencies]
check_keyword = "0.3.1"
```

## Example

```rust
assert!(!"not_a_keyword".is_keyword());
assert_eq!("not_a_keyword".into_safe(), "not_a_keyword");

assert!("match".is_keyword());
assert_eq!("match".into_safe(), "r#match");
```

The [CheckKeyword::into_safe] method automatically checks [CheckKeyword::is_keyword] for you.
You don't need to call [CheckKeyword::is_keyword]
if you don't care whether it was originally a keyword or not.

[CheckKeyword::is_keyword] only checks for strict and reserved keywords. For more detail, and support
for weak keywords, use [CheckKeyword::keyword_status].

## Implementors

This trait has a blanket implementation for all types that implement `AsRef<str>`. This includes
`&str` and `String`.

## Raw Identifiers

Raw identifiers are identifiers that start with `r#`, and most keywords are allowed
to be used as raw identifiers.

## Rust Editions

By default, all keywords in Rust 2024 are included.
This can be disabled with `default-features = false` and selecting an earlier edition in your Cargo.toml.

```toml
[dependencies]
check_keyword = { version = "0.3.1", default-features = false, features = [ "2021" ] }
```

This crate is up-to-date with Rust 2024. Future Rust editions may add new keywords, and this
crate will be updated to reflect that.
(Or you can create an issue on github if I forget.)

License: MIT OR Apache-2.0
