use gpui::{Context, Div, div, prelude::*};

use super::render_commit_panel_button::render_commit_panel_button;
use crate::repository_window::RepositoryWindow;

pub fn render_commit_and_push_row(
    repository_window: &RepositoryWindow,
    cx: &mut Context<RepositoryWindow>,
) -> Div {
    div()
        .flex()
        .gap_2()
        .flex_shrink_0()
        .px_3()
        .py_2()
        .child(
            div()
                .flex_1()
                .min_w_0()
                .child(repository_window.commit_message_input.clone()),
        )
        .child(
            render_commit_panel_button("commit-button", "Commit").on_click(cx.listener(
                |repository_window, _, _, cx| {
                    repository_window.commit_staged_changes_with_typed_message(cx)
                },
            )),
        )
}
