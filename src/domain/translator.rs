pub trait StringTranslate {
    /// 文字列を受け取って小文字にして返すメソッド
    /// @param word 操作対象の文字列
    /// @return 小文字化された文字列
    fn to_lower(word: &str) -> String;
}
