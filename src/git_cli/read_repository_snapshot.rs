use std::path::Path;

use super::GitCommandError;
use super::read_commit_history::read_commit_history;
use super::read_local_branches::read_local_branches;
use crate::repository::RepositorySnapshot;

pub fn read_repository_snapshot(
    repository_path: &Path,
) -> Result<RepositorySnapshot, GitCommandError> {
    Ok(RepositorySnapshot {
        commits: read_commit_history(repository_path)?,
        local_branches: read_local_branches(repository_path)?,
    })
}
