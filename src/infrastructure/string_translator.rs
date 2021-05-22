use crate::domain::translator::StringTranslator;

struct LowerTranslator {
}

impl StringTranslator for LowerTranslator {
    fn translate(word: &str) -> String {
        word.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_capital_return_lower_characters() {
        let word = "aBc";
        assert_eq!("abc", LowerTranslator::translate(&word));
        let word = "DEF";
        assert_eq!("def", LowerTranslator::translate(&word));
    }
}
