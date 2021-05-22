pub trait StringValidator {
    /// 文字列を受け取って有効であるかどうかを返すメソッド
    /// @param word 確認対象の文字列
    /// @return 有効であるかどうか
    fn validate(&self, word: &str) -> bool;
}
