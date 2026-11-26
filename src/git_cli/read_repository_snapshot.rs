use std::path::Path;

use super::GitCommandError;
use super::changed_files::read_changed_files;
use super::local_branches::read_local_branches;
use super::read_commit_history::read_commit_history;
use super::worktrees::read_worktrees;
use crate::repository::RepositorySnapshot;

pub fn read_repository_snapshot(
    repository_path: &Path,
) -> Result<RepositorySnapshot, GitCommandError> {
    Ok(RepositorySnapshot {
        commits: read_commit_history(repository_path)?,
        local_branches: read_local_branches(repository_path)?,
        changed_files: read_changed_files(repository_path)?,
        worktrees: read_worktrees(repository_path)?,
    })
}
