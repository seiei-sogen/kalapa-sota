/// git が失敗した理由。git の標準エラー出力 (空なら標準出力) をそのまま利用者に見せる。
#[derive(Clone, Debug)]
pub struct GitCommandError {
    pub message: String,
}
