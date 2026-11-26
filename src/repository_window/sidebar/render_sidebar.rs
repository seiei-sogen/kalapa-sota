use gpui::{Context, Div, Stateful, div, prelude::*, px, rgb};

use super::render_local_branch_section::render_local_branch_section;
use super::render_worktree_section::render_worktree_section;
use crate::appearance::dark_theme_colors::{PANEL_BACKGROUND, PANEL_BORDER};
use crate::appearance::repository_window_sizes::SIDEBAR_WIDTH;
use crate::repository_window::RepositoryWindow;

pub fn render_sidebar(
    repository_window: &RepositoryWindow,
    cx: &mut Context<RepositoryWindow>,
) -> Stateful<Div> {
    div()
        .id("sidebar")
        .overflow_y_scroll()
        .flex()
        .flex_col()
        .flex_shrink_0()
        .w(px(SIDEBAR_WIDTH))
        .h_full()
        .bg(rgb(PANEL_BACKGROUND))
        .border_r_1()
        .border_color(rgb(PANEL_BORDER))
        .child(render_worktree_section(repository_window, cx))
        .child(render_local_branch_section(repository_window, cx))
}
