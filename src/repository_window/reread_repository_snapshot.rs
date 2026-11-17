use super::RepositoryWindow;
use crate::commit_graph::layout_commit_graph;
use crate::git_cli::{GitCommandError, read_repository_snapshot};

impl RepositoryWindow {
    /// リポジトリを読み直して snapshot と commit_graph を入れ替える。last_git_error には触らない。
    pub fn reread_repository_snapshot(&mut self) -> Result<(), GitCommandError> {
        let snapshot = read_repository_snapshot(&self.repository_path)?;
        self.commit_graph = layout_commit_graph(&snapshot.commits);
        self.snapshot = snapshot;
        Ok(())
    }
}
