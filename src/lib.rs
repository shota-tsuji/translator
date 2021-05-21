use regex::Regex;

pub trait StringTranslate {
    /// 文字列を受け取って小文字にして返すメソッド
    /// @param word 操作対象の文字列
    /// @return 小文字化された文字列
    fn to_lower(word: &str) -> String;
}

pub trait StringValidate {
    /// 文字列を受け取ってアルファベットのみであるかどうかを返すメソッド
    /// @param word 確認対象の文字列
    /// @return アルファベットのみの文字列であるかどうか
    fn validate(&self, word: &str) -> bool;
}

struct StringTranslator {
}

impl StringTranslate for StringTranslator {
    fn to_lower(word: &str) -> String {
        word.to_lowercase()
    }
}

pub struct StringValidator {
    re: regex::Regex,
}

impl StringValidator {
    pub fn new() -> Box<dyn StringValidate> {
        let re = Regex::new(r"[[:alpha:]]+").unwrap();
        Box::new(StringValidator { re })
    }
}

impl StringValidate for StringValidator {
    fn validate(&self, word: &str) -> bool {
        self.re.is_match(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_capital_return_lower_characters() {
        let word = "aBc";
        assert_eq!("abc", StringTranslator::to_lower(&word));
        let word = "DEF";
        assert_eq!("def", StringTranslator::to_lower(&word));
    }

    #[test]
    fn given_alphabets_return_true() {
        let word = "validString";
        let string_validator = StringValidator::new();
        assert!(string_validator.validate(&word));
    }
}
