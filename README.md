# check_keyword

A trait for Strings and &str's to check if a string is a reserved keyword,
and convert it to a safe non-keyword if so.

Only strict and reserved keywords are checked against; weak keywords are not included.

You can add this dependency with:

```toml
[dependencies]
check_keyword = "0.1.1"
```

## Examples

```rust
use check_keyword::CheckKeyword;
let keyword = "match";

assert!(keyword.is_keyword());
assert_eq!(keyword.to_safe(), "r#match");

// There's also a self-consuming version if you want
assert_eq!(keyword.into_safe(), "r#match");
```

## Rust Editions

By default, the keywords added in Rust Edition 2018 are included in the list of checked keywords.
This can be disabled with `default-features = false` in your Cargo.toml.

```toml
[dependencies]
check_keyword = { version = "0.1.1", default-features = false }
```

Future Rust editions may add new keywords, and this crate will be updated to reflect that.
(Or you can create an issue on github if I don't.)

License: MIT OR Apache-2.0
