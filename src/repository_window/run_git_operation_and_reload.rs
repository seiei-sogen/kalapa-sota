use std::path::Path;

use super::RepositoryWindow;
use crate::git_cli::GitCommandError;

impl RepositoryWindow {
    // shortcut: git を UI スレッドで同期実行する。status や checkout が遅いリポジトリで画面が固まるなら background_executor へ移す。
    pub fn run_git_operation_and_reload(
        &mut self,
        git_operation: impl FnOnce(&Path) -> Result<(), GitCommandError>,
    ) {
        let operation_result = git_operation(&self.repository_path);
        // 失敗した操作でも、フックなどがファイルを書き換えていることがあるので、必ず読み直す。
        let reread_result = self.reread_repository_snapshot();
        self.last_git_error = operation_result.err().or(reread_result.err());
    }
}
