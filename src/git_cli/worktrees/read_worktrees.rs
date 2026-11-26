use std::path::Path;

use super::parse_worktree_list_porcelain::parse_worktree_list_porcelain;
use crate::git_cli::GitCommandError;
use crate::git_cli::run_git_command::run_git_command;
use crate::repository::Worktree;

pub fn read_worktrees(repository_path: &Path) -> Result<Vec<Worktree>, GitCommandError> {
    let output = run_git_command(repository_path, &["worktree", "list", "--porcelain", "-z"])?;
    parse_worktree_list_porcelain(&output)
}
