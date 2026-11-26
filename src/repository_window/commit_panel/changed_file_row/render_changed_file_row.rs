use gpui::{Context, Div, SharedString, Stateful, div, prelude::*, px, rgb};

use super::changed_file_kind_marker::changed_file_kind_marker;
use super::commit_inclusion_checkbox::commit_inclusion_checkbox;
use crate::appearance::dark_theme_colors::HOVERED_ROW_BACKGROUND;
use crate::appearance::repository_window_sizes::ROW_HEIGHT;
use crate::git_cli::toggle_changed_file_in_commit;
use crate::repository::{ChangedFile, decide_commit_inclusion};
use crate::repository_window::RepositoryWindow;

pub fn render_changed_file_row(
    changed_file: &ChangedFile,
    cx: &mut Context<RepositoryWindow>,
) -> Stateful<Div> {
    let (checkbox_glyph, checkbox_color) =
        commit_inclusion_checkbox(decide_commit_inclusion(&changed_file.staging));
    let (kind_letter, kind_color) = changed_file_kind_marker(&changed_file.staging);
    let clicked_file = changed_file.clone();
    div()
        .id(SharedString::from(changed_file.path.0.clone()))
        .flex()
        .items_center()
        .gap_2()
        .w_full()
        .h(px(ROW_HEIGHT))
        .px_3()
        .whitespace_nowrap()
        .cursor_pointer()
        .hover(|row| row.bg(rgb(HOVERED_ROW_BACKGROUND)))
        .on_click(cx.listener(move |repository_window, _, _, cx| {
            repository_window.run_git_operation_and_reload(|worktree_root_path| {
                toggle_changed_file_in_commit(worktree_root_path, &clicked_file)
            });
            cx.notify();
        }))
        .child(
            div()
                .flex_shrink_0()
                .w(px(14.))
                .text_color(rgb(checkbox_color))
                .child(checkbox_glyph),
        )
        .child(
            div()
                .flex_shrink_0()
                .w(px(12.))
                .text_color(rgb(kind_color))
                .child(kind_letter),
        )
        .child(
            div()
                .flex_1()
                .min_w_0()
                .truncate()
                .child(changed_file.path.0.clone()),
        )
}
