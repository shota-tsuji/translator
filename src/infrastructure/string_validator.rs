use crate::domain::validator;
use regex::Regex;

const EXPRESSION: &'static str = r"^[[:alpha:]]+$";

pub struct AlphabetValidator {
    re: regex::Regex,
}

impl AlphabetValidator {
    pub fn new() -> Box<dyn validator::StringValidator> {
        let re = Regex::new(EXPRESSION).unwrap();
        Box::new(AlphabetValidator { re })
    }
}

impl validator::StringValidator for AlphabetValidator {
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
        let string_validator = AlphabetValidator::new();
        assert!(string_validator.validate(&word));
    }

    #[test]
    fn given_non_alphabets_return_false() {
        let word = "invalid-String";
        let string_validator = AlphabetValidator::new();
        assert!(!string_validator.validate(&word));
    }
}
