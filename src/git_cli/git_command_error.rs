/// git が失敗した理由。git の標準エラー出力をそのまま利用者に見せる。
#[derive(Clone, Debug)]
pub struct GitCommandError {
    pub message: String,
}
