# check_keyword

A trait for String-like types to check if a string is a reserved keyword,
and convert it to a safe non-keyword if so.

Only strict and reserved keywords are checked against; weak keywords are not included.

You can add this dependency with:

```toml
[dependencies]
check_keyword = "0.2"
```

## Example

```rust
use check_keyword::CheckKeyword;
let keyword = "match";

assert!(keyword.is_keyword());
assert_eq!(keyword.into_safe(), "r#match");
```

The [CheckKeyword::into_safe] method automatically checks [CheckKeyword::is_keyword] for you.
You don't need to call [CheckKeyword::is_keyword]
if you don't care whether it was originally a keyword or not.

## Implementations

There is a special implementation of `CheckKeyword<String>` for [&str], and a
blanket implementation of `CheckKeyword<T>` where `T: AsRef<str> + From<String>`.

The blanket implementation covers [String], and is only tested for that, but should
cover any other String-like type as well. I can try to broaden the definition to fit
other types if needed (open an issue).

## Rust Editions

By default, the keywords added in Rust Edition 2018 are included in the list of checked keywords.
This can be disabled with `default-features = false` in your Cargo.toml.

```toml
[dependencies]
check_keyword = { version = "0.2", default-features = false }
```

Future Rust editions may add new keywords, and this crate will be updated to reflect that.
(Or you can create an issue on github if I don't.)

License: MIT OR Apache-2.0
