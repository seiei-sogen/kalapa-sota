use gpui::{Div, div, prelude::*, px, rgb};

use super::paint_commit_graph_cell::paint_commit_graph_cell;
use crate::commit_graph::CommitGraphRow;
use crate::history_window::appearance::dark_theme_colors::SECONDARY_TEXT;
use crate::history_window::appearance::history_window_sizes::{
    AUTHOR_COLUMN_WIDTH, COMMITTED_AT_COLUMN_WIDTH, GRAPH_LANE_WIDTH, ROW_HEIGHT,
};
use crate::repository::Commit;

pub fn render_commit_row(
    commit: &Commit,
    graph_row: &CommitGraphRow,
    graph_lane_count: usize,
) -> Div {
    div()
        .flex()
        .items_center()
        .gap_3()
        .w_full()
        .h(px(ROW_HEIGHT))
        .px_2()
        .whitespace_nowrap()
        .child(
            div()
                .flex_shrink_0()
                .w(px(GRAPH_LANE_WIDTH * graph_lane_count as f32))
                .h_full()
                .child(paint_commit_graph_cell(graph_row.clone()).size_full()),
        )
        .child(
            div()
                .flex_1()
                .min_w_0()
                .truncate()
                .child(commit.summary.clone()),
        )
        .child(
            div()
                .flex_shrink_0()
                .w(px(AUTHOR_COLUMN_WIDTH))
                .truncate()
                .text_color(rgb(SECONDARY_TEXT))
                .child(commit.author_name.clone()),
        )
        .child(
            div()
                .flex_shrink_0()
                .w(px(COMMITTED_AT_COLUMN_WIDTH))
                .text_color(rgb(SECONDARY_TEXT))
                .child(commit.committed_at.clone()),
        )
}
