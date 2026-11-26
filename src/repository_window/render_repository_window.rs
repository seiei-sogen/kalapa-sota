use std::ops::Range;

use gpui::{Context, Window, div, prelude::*, px, rgb, uniform_list};

use super::RepositoryWindow;
use super::commit_panel::render_commit_panel;
use super::commit_row::render_commit_row;
use super::sidebar::render_sidebar;
use crate::appearance::dark_theme_colors::{
    GIT_ERROR_BACKGROUND, GIT_ERROR_TEXT, PRIMARY_TEXT, WINDOW_BACKGROUND,
};
use crate::appearance::repository_window_sizes::GIT_ERROR_BANNER_MAX_HEIGHT;

impl Render for RepositoryWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let git_error_banner = self.last_git_error.as_ref().map(|error| {
            // フックや push の失敗は何十行にもなることがあるので、高さを抑えてコミット欄を画面外へ押し出さない。
            div()
                .id("git-error-banner")
                .max_h(px(GIT_ERROR_BANNER_MAX_HEIGHT))
                .overflow_y_scroll()
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
            .child(render_sidebar(self, cx))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .children(git_error_banner)
                    .child(commit_history_list)
                    .child(render_commit_panel(self, cx)),
            )
    }
}
