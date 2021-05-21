mod domain;
pub mod infrastructure;

pub trait StringTranslate {
    /// 文字列を受け取って小文字にして返すメソッド
    /// @param word 操作対象の文字列
    /// @return 小文字化された文字列
    fn to_lower(word: &str) -> String;
}

struct StringTranslator {
}

impl StringTranslate for StringTranslator {
    fn to_lower(word: &str) -> String {
        word.to_lowercase()
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
}
