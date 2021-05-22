pub trait StringTranslator {
    /// 文字列を受け取って変換して返すメソッド
    /// @param word 操作対象の文字列
    /// @return 変換された文字列
    fn translate(word: &str) -> String;
}
