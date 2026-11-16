use std::ops::Range;

use gpui::{Context, Window, div, prelude::*, rgb, uniform_list};

use super::HistoryWindow;
use super::appearance::dark_theme_colors::{
    GIT_ERROR_BACKGROUND, GIT_ERROR_TEXT, PRIMARY_TEXT, WINDOW_BACKGROUND,
};
use super::commit_row::render_commit_row;
use super::render_local_branch_panel::render_local_branch_panel;

impl Render for HistoryWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let git_error_banner = self.last_git_error.as_ref().map(|error| {
            div()
                .px_3()
                .py_2()
                .bg(rgb(GIT_ERROR_BACKGROUND))
                .text_color(rgb(GIT_ERROR_TEXT))
                .child(error.message.clone())
        });
        let commit_history_list = uniform_list(
            "commit-history",
            self.snapshot.commits.len(),
            cx.processor(|this, visible_rows: Range<usize>, _, _| {
                visible_rows
                    .map(|row_index| {
                        render_commit_row(
                            &this.snapshot.commits[row_index],
                            &this.commit_graph.rows[row_index],
                            this.commit_graph.lane_count,
                        )
                    })
                    .collect::<Vec<_>>()
            }),
        )
        .flex_1();

        div()
            .flex()
            .size_full()
            .bg(rgb(WINDOW_BACKGROUND))
            .text_color(rgb(PRIMARY_TEXT))
            .text_sm()
            .child(render_local_branch_panel(self, cx))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .children(git_error_banner)
                    .child(commit_history_list),
            )
    }
}
