use crate::repository::CommitId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorktreeCheckout {
    Branch {
        branch_name: String,
    },
    DetachedHead {
        head_commit_id: CommitId,
    },
    Bare,
    /// HEAD がブランチもコミットも指していない。git worktree list が (error) と出すワークツリー。
    UnreadableHead,
}
