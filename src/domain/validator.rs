pub trait StringValidate {
    /// 文字列を受け取ってアルファベットのみであるかどうかを返すメソッド
    /// @param word 確認対象の文字列
    /// @return アルファベットのみの文字列であるかどうか
    fn validate(&self, word: &str) -> bool;
}
