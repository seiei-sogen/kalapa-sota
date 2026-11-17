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
    /// 一度だけ作って使い続ける。作り直すと、IME で変換中の文字とフォーカスが消える。
    pub commit_message_input: Entity<SingleLineTextInput>,
}
