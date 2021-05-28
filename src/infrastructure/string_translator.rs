pub struct LowerTranslator {
}

#[cfg_attr(feature = "mock", mockall::automock)]
impl LowerTranslator {
    pub fn translate(word: &str) -> String {
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
