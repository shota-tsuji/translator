use crate::domain::translator::StringTranslate;

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
