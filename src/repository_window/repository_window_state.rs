use std::path::PathBuf;

use gpui::Entity;

use crate::commit_graph::CommitGraphLayout;
use crate::git_cli::GitCommandError;
use crate::repository::RepositorySnapshot;
use crate::single_line_text_input::SingleLineTextInput;

pub struct RepositoryWindow {
    pub repository_path: PathBuf,
    pub snapshot: RepositorySnapshot,
    /// snapshot.commits から作る。snapshot と一緒にしか入れ替えない。
    pub commit_graph: CommitGraphLayout,
    pub last_git_error: Option<GitCommandError>,
    pub commit_message_input: Entity<SingleLineTextInput>,
    /// push をバックグラウンドで実行している間だけ true。この間に Push を押しても、二重には実行しない。
    pub push_in_progress: bool,
}
