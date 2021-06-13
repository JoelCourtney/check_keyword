use super::*;

impl Keywords for String {
    type SafeOutput = Self;

    fn is_keyword(&self) -> bool {
        KEYWORDS.contains(&self.as_str())
    }

    fn to_safe(&self) -> Self {
        if self.is_keyword() {
            format!("r#{}", self)
        } else {
            self.clone()
        }
    }

    fn into_safe(self) -> Self {
        if self.is_keyword() {
            format!("r#{}", self)
        } else {
            self
        }
    }
}

impl Keywords for &str {
    type SafeOutput = String;

    fn is_keyword(&self) -> bool {
        KEYWORDS.contains(self)
    }

    fn to_safe(&self) -> String {
        if self.is_keyword() {
            format!("r#{}", self)
        } else {
            self.to_string()
        }
    }

    fn into_safe(self) -> String {
        self.to_safe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_keyword() {
        assert!(String::from("match").is_keyword());
        assert!(!"hello".is_keyword());

        assert_eq!(String::from("async").is_keyword(), cfg!(feature = "2018"));
    }

    #[test]
    fn to_safe() {
        assert_eq!(String::from("match").to_safe(), "r#match");
        assert_eq!("asdf".into_safe(), "asdf");

        assert_eq!("await".to_safe(),
            if cfg!(feature = "2018") {
                "r#await"
            } else {
                "await"
            }
        )
    }
}