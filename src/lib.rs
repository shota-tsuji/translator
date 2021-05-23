mod domain;
pub mod infrastructure;

pub use crate::domain::validator::StringValidator;

#[cfg(feature = "mock")]
pub use crate::domain::validator::MockStringValidator;

#[cfg(test)]
mod tests {
    use super::*;

    // 関数が実行されると検証と変換の処理がされている
    #[test]
    fn when_run_then_validated_and_translated() {
        let mut validator = MockStringValidator::new();
        validator.expect_validate()
            .times(1)
            .returning(|_| true);
        validator.validate("validWord");

        //let mut translator = domain::translator::MockStringTranslator::new();
        //translator.expect_translate()
        //    .times(1)
        //    .returning(|_| "abc");
        //translator.translate("ABC");
    }
}
