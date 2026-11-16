use super::CommitId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalBranch {
    pub name: String,
    pub head_commit_id: CommitId,
    pub is_current: bool,
}
