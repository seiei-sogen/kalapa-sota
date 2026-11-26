use super::{ChangedFile, Commit, LocalBranch, Worktree};

/// commits は子が親より先に来る順に並ぶ。
pub struct RepositorySnapshot {
    pub commits: Vec<Commit>,
    pub local_branches: Vec<LocalBranch>,
    pub changed_files: Vec<ChangedFile>,
    pub worktrees: Vec<Worktree>,
}
