use crate::domain::validator;
use regex::Regex;

const EXPRESSION: &'static str = r"^[[:alpha:]]+$";

pub struct StringValidator {
    re: regex::Regex,
}

impl StringValidator {
    pub fn new() -> Box<dyn validator::StringValidate> {
        let re = Regex::new(EXPRESSION).unwrap();
        Box::new(StringValidator { re })
    }
}

impl validator::StringValidate for StringValidator {
    fn validate(&self, word: &str) -> bool {
        self.re.is_match(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_alphabets_return_true() {
        let word = "validString";
        let string_validator = StringValidator::new();
        assert!(string_validator.validate(&word));
    }

    #[test]
    fn given_non_alphabets_return_false() {
        let word = "invalid-String";
        let string_validator = StringValidator::new();
        assert!(!string_validator.validate(&word));
    }
}
