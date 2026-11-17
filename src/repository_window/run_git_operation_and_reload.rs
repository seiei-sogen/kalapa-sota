use std::path::Path;

use super::RepositoryWindow;
use crate::commit_graph::layout_commit_graph;
use crate::git_cli::{GitCommandError, read_repository_snapshot};

impl RepositoryWindow {
    // shortcut: git を UI スレッドで同期実行する。status や checkout が遅いリポジトリで画面が固まるなら background_executor へ移す。
    pub fn run_git_operation_and_reload(
        &mut self,
        git_operation: impl FnOnce(&Path) -> Result<(), GitCommandError>,
    ) -> bool {
        let operation_result = git_operation(&self.repository_path);
        // 失敗した操作でも、フックなどがファイルを書き換えていることがあるので、必ず読み直す。
        let reload_error = match read_repository_snapshot(&self.repository_path) {
            Ok(snapshot) => {
                self.commit_graph = layout_commit_graph(&snapshot.commits);
                self.snapshot = snapshot;
                None
            }
            Err(error) => Some(error),
        };
        let operation_succeeded = operation_result.is_ok();
        self.last_git_error = operation_result.err().or(reload_error);
        operation_succeeded
    }
}
