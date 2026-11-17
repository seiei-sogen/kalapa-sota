use std::path::PathBuf;

use gpui::{AppContext, Context, Window};

use super::RepositoryWindow;
use crate::commit_graph::layout_commit_graph;
use crate::repository::RepositorySnapshot;
use crate::single_line_text_input::SingleLineTextInput;

impl RepositoryWindow {
    pub fn new(
        repository_path: PathBuf,
        snapshot: RepositorySnapshot,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe_window_activation(window, |repository_window, window, cx| {
            if window.is_window_active() {
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
            commit_message_input: cx.new(|cx| SingleLineTextInput::new("Commit message", cx)),
            push_in_progress: false,
        }
    }
}
