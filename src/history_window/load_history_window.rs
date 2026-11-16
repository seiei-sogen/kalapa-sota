use std::path::PathBuf;

use super::HistoryWindow;
use crate::commit_graph::layout_commit_graph;
use crate::git_cli::{GitCommandError, read_repository_snapshot};

impl HistoryWindow {
    pub fn load(repository_path: PathBuf) -> Result<Self, GitCommandError> {
        let snapshot = read_repository_snapshot(&repository_path)?;
        let commit_graph = layout_commit_graph(&snapshot.commits);
        Ok(Self {
            repository_path,
            snapshot,
            commit_graph,
            last_git_error: None,
        })
    }
}
