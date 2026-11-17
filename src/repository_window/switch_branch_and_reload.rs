use super::RepositoryWindow;
use crate::git_cli::switch_local_branch;

impl RepositoryWindow {
    // shortcut: git を UI スレッドで同期実行する。checkout が遅いリポジトリで画面が固まるなら background_executor へ移す。
    pub fn switch_branch_and_reload(&mut self, branch_name: &str) {
        let switched_and_reloaded = switch_local_branch(&self.repository_path, branch_name)
            .and_then(|()| RepositoryWindow::load(self.repository_path.clone()));
        match switched_and_reloaded {
            Ok(reloaded) => *self = reloaded,
            Err(error) => self.last_git_error = Some(error),
        }
    }
}
