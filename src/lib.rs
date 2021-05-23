mod domain;
pub mod infrastructure;

pub use crate::domain::validator::StringValidator;
pub use crate::domain::translator::StringTranslator;

#[cfg(feature = "mock")]
pub use crate::domain::validator::MockStringValidator;
pub use crate::domain::translator::MockStringTranslator;

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;

    // 関数が実行されると検証と変換の処理がされている
    #[test]
    fn when_run_then_validated_and_translated() {
        let mut validator = MockStringValidator::new();
        validator.expect_validate()
            .times(1)
            .returning(|_| true);
        validator.validate("validWord");

        let ctx_translator = MockStringTranslator::translate_context();
        ctx_translator.expect()
            .with(eq("ABC"))
            .times(1)
            .returning(|_| "abc".to_string());
        assert_eq!("abc", MockStringTranslator::translate("ABC"));
    }
}
