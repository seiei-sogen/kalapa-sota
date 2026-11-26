use super::RepositoryWindow;
use crate::commit_graph::layout_commit_graph;
use crate::git_cli::{GitCommandError, read_repository_snapshot};

impl RepositoryWindow {
    pub fn reread_repository_snapshot(&mut self) -> Result<(), GitCommandError> {
        let snapshot = read_repository_snapshot(&self.opened_worktree_root_path.0)?;
        self.commit_graph = layout_commit_graph(&snapshot.commits);
        self.snapshot = snapshot;
        Ok(())
    }
}
