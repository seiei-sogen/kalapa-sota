use super::{WorktreeCheckout, WorktreeRootPath};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Worktree {
    pub root_path: WorktreeRootPath,
    pub checkout: WorktreeCheckout,
}
