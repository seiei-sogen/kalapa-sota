use std::ops::Range;

use gpui::{Context, Div, div, prelude::*, px, rgb, uniform_list};

use super::render_changed_file_row::render_changed_file_row;
use crate::appearance::dark_theme_colors::{PANEL_BACKGROUND, PANEL_BORDER, SECONDARY_TEXT};
use crate::appearance::repository_window_sizes::{COMMIT_PANEL_HEIGHT, ROW_HEIGHT};
use crate::repository_window::RepositoryWindow;

pub fn render_commit_panel(
    repository_window: &RepositoryWindow,
    cx: &mut Context<RepositoryWindow>,
) -> Div {
    let changed_file_count = repository_window.snapshot.changed_files.len();
    let changed_file_list = uniform_list(
        "changed-files",
        changed_file_count,
        cx.processor(|repository_window, visible_rows: Range<usize>, _, cx| {
            visible_rows
                .map(|row_index| {
                    render_changed_file_row(
                        &repository_window.snapshot.changed_files[row_index],
                        cx,
                    )
                })
                .collect::<Vec<_>>()
        }),
    )
    .flex_1();

    div()
        .flex()
        .flex_col()
        .flex_shrink_0()
        .h(px(COMMIT_PANEL_HEIGHT))
        .bg(rgb(PANEL_BACKGROUND))
        .border_t_1()
        .border_color(rgb(PANEL_BORDER))
        .child(
            div()
                .flex()
                .items_center()
                .flex_shrink_0()
                .h(px(ROW_HEIGHT))
                .px_3()
                .text_color(rgb(SECONDARY_TEXT))
                .child(format!("CHANGES ({changed_file_count})")),
        )
        .child(changed_file_list)
}
