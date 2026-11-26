use gpui::{Div, FontWeight, div, prelude::*, px, rgb};

use super::worktree_checkout_label::worktree_checkout_label;
use crate::appearance::dark_theme_colors::{CURRENT_SIDEBAR_ROW_BACKGROUND, SECONDARY_TEXT};
use crate::appearance::repository_window_sizes::ROW_HEIGHT;
use crate::repository_window::RepositoryWindow;

pub fn render_worktree_section(repository_window: &RepositoryWindow) -> Div {
    let worktree_rows = repository_window.snapshot.worktrees.iter().map(|worktree| {
        let is_opened_worktree = worktree.root_path == repository_window.opened_worktree_root_path;
        let directory_name = worktree.root_path.0.file_name().map_or_else(
            || worktree.root_path.0.display().to_string(),
            |file_name| file_name.to_string_lossy().into_owned(),
        );
        div()
            .flex()
            .items_center()
            .gap_2()
            .flex_shrink_0()
            .h(px(ROW_HEIGHT))
            .px_3()
            .when(is_opened_worktree, |row| {
                row.bg(rgb(CURRENT_SIDEBAR_ROW_BACKGROUND))
                    .font_weight(FontWeight::BOLD)
            })
            .child(
                div()
                    .w(px(12.))
                    .flex_shrink_0()
                    .child(if is_opened_worktree { "●" } else { "" }),
            )
            .child(div().truncate().child(directory_name))
            .child(
                div()
                    .truncate()
                    .text_color(rgb(SECONDARY_TEXT))
                    .child(worktree_checkout_label(&worktree.checkout)),
            )
    });

    div()
        .flex()
        .flex_col()
        .flex_shrink_0()
        .child(
            div()
                .flex()
                .items_center()
                .flex_shrink_0()
                .h(px(ROW_HEIGHT))
                .px_3()
                .text_color(rgb(SECONDARY_TEXT))
                .child("WORKTREES"),
        )
        .children(worktree_rows)
}
