use gpui::{Context, Window};

use super::{RepositoryWindow, repository_window_title};
use crate::commit_graph::layout_commit_graph;
use crate::git_cli::read_repository_snapshot;
use crate::repository::WorktreeRootPath;

impl RepositoryWindow {
    pub fn open_worktree_in_window(
        &mut self,
        worktree_root_path: WorktreeRootPath,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // 読めてから入れ替えるので、ディレクトリが消えたワークツリーを押しても、元のワークツリーを開いたままになる。
        match read_repository_snapshot(&worktree_root_path.0) {
            Ok(snapshot) => {
                self.commit_graph = layout_commit_graph(&snapshot.commits);
                self.snapshot = snapshot;
                self.opened_worktree_root_path = worktree_root_path;
                self.last_git_error = None;
                window.set_window_title(&repository_window_title(&self.opened_worktree_root_path));
            }
            Err(error) => self.last_git_error = Some(error),
        }
        cx.notify();
    }
}
