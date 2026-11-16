#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CommitId(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commit {
    pub id: CommitId,
    pub parent_ids: Vec<CommitId>,
    pub summary: String,
    pub author_name: String,
    /// 表示専用なので、git が整形した文字列のまま持つ。
    pub committed_at: String,
}
