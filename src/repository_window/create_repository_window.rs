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
        cx.observe_window_activation(window, |repository_window, window, cx| {
            if window.is_window_active() {
                // 読み直しが成功しても前の失敗の表示は消さない。よそのウィンドウにいる間に失敗した push に気づけなくなるため。
                if let Err(error) = repository_window.reread_repository_snapshot() {
                    repository_window.last_git_error = Some(error);
                }
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
