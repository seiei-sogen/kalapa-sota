use gpui::{Context, Div, div, prelude::*};

use super::push_button_label::push_button_label;
use super::render_commit_panel_button::render_commit_panel_button;
use crate::repository_window::RepositoryWindow;

pub fn render_commit_and_push_row(
    repository_window: &RepositoryWindow,
    cx: &mut Context<RepositoryWindow>,
) -> Div {
    let current_branch_tracking = repository_window
        .snapshot
        .local_branches
        .iter()
        .find(|branch| branch.is_current)
        .map(|branch| branch.upstream_tracking);
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
        .child(
            render_commit_panel_button(
                "push-button",
                push_button_label(repository_window.push_in_progress, current_branch_tracking),
            )
            .when(repository_window.push_in_progress, |push_button| {
                push_button.opacity(0.6).cursor_default()
            })
            .on_click(cx.listener(|repository_window, _, _, cx| {
                repository_window.push_current_branch_in_background(cx)
            })),
        )
}
