use gpui::Entity;

use crate::commit_graph::CommitGraphLayout;
use crate::git_cli::GitCommandError;
use crate::repository::{RepositorySnapshot, WorktreeRootPath};
use crate::single_line_text_input::SingleLineTextInput;

pub struct RepositoryWindow {
    pub opened_worktree_root_path: WorktreeRootPath,
    pub snapshot: RepositorySnapshot,
    /// snapshot.commits から作る。snapshot と一緒にしか入れ替えない。
    pub commit_graph: CommitGraphLayout,
    pub last_git_error: Option<GitCommandError>,
    pub commit_message_input: Entity<SingleLineTextInput>,
    pub push_in_progress: bool,
}
