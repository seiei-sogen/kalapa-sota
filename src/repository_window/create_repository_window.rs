use std::path::PathBuf;

use gpui::{Context, Window};

use super::RepositoryWindow;
use crate::commit_graph::layout_commit_graph;
use crate::repository::RepositorySnapshot;

impl RepositoryWindow {
    pub fn new(
        repository_path: PathBuf,
        snapshot: RepositorySnapshot,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        // エディタで保存したファイルは git から知らせが来ないので、ウィンドウに戻ってきたときに読み直す。
        cx.observe_window_activation(window, |repository_window, window, cx| {
            if window.is_window_active() {
                repository_window.run_git_operation_and_reload(|_| Ok(()));
                cx.notify();
            }
        })
        .detach();
        Self {
            commit_graph: layout_commit_graph(&snapshot.commits),
            repository_path,
            snapshot,
            last_git_error: None,
        }
    }
}
