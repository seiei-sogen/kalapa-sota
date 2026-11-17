use std::path::PathBuf;

use crate::commit_graph::CommitGraphLayout;
use crate::git_cli::GitCommandError;
use crate::repository::RepositorySnapshot;

pub struct RepositoryWindow {
    pub repository_path: PathBuf,
    pub snapshot: RepositorySnapshot,
    /// snapshot.commits から作る。snapshot と一緒にしか入れ替えない。
    pub commit_graph: CommitGraphLayout,
    /// 直前の git 操作か、その後の読み直しの失敗。次の操作が成功するまで表示し続ける。
    pub last_git_error: Option<GitCommandError>,
}
