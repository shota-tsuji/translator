mod domain;
pub mod infrastructure;

pub use crate::domain::validator::StringValidator;
pub use crate::infrastructure::string_validator::AlphabetValidator;

#[cfg_attr(feature = "mock", mockall_double::double)]
use crate::infrastructure::string_translator::LowerTranslator;

pub use crate::domain::translator::MockStringTranslator;
#[cfg(feature = "mock")]
pub use crate::domain::validator::MockStringValidator;
pub use crate::infrastructure::string_translator::MockLowerTranslator;

pub struct LowerProcesser {
    validator: Box<dyn StringValidator>,
}

impl LowerProcesser {
    pub fn run(&self, word: &str) -> String {
        if (*self.validator).validate(word) {
            return LowerTranslator::translate(word);
        } else {
            return "abc".to_string();
        }
    }

    pub fn new() -> Self {
        let validator = AlphabetValidator::new();
        LowerProcesser {
            validator: validator,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;

    // 関数が実行されると検証と変換の処理がされている
    #[test]
    fn when_run_then_validated_and_translated() {
        let mut validator = MockStringValidator::new();
        validator.expect_validate().times(1).return_const(true);

        let ctx_translator = MockLowerTranslator::translate_context();
        ctx_translator
            .expect()
            .with(eq("ABC"))
            .times(1)
            .return_const("abc".to_string());

        let word_translator = LowerProcesser {
            validator: Box::new(validator),
        };
        assert_eq!("abc", word_translator.run("ABC"));
    }

    // 異常系のテスト
}
