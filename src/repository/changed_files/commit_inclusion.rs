/// ファイルの変更が、次のコミットにどれだけ入るか。チェックボックスの 3 つの状態に当たる。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitInclusion {
    Included,
    PartlyIncluded,
    Excluded,
}
