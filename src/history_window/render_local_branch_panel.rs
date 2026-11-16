use gpui::{Context, Div, FontWeight, SharedString, Stateful, div, prelude::*, px, rgb};

use super::HistoryWindow;
use super::appearance::branch_lane_color;
use super::appearance::dark_theme_colors::{
    BRANCH_PANEL_BACKGROUND, CURRENT_BRANCH_BACKGROUND, HOVERED_ROW_BACKGROUND, PANEL_BORDER,
    SECONDARY_TEXT,
};
use super::appearance::history_window_sizes::{BRANCH_PANEL_WIDTH, ROW_HEIGHT};

pub fn render_local_branch_panel(
    history_window: &HistoryWindow,
    cx: &mut Context<HistoryWindow>,
) -> Stateful<Div> {
    let branch_rows = history_window.snapshot.local_branches.iter().map(|branch| {
        // ラベルの色は、ブランチの先頭コミットが載っているレーンの色にそろえる。
        let label_color = history_window
            .snapshot
            .commits
            .iter()
            .position(|commit| commit.id == branch.head_commit_id)
            .map(|row_index| {
                branch_lane_color(history_window.commit_graph.rows[row_index].commit_lane)
            })
            .unwrap_or(rgb(SECONDARY_TEXT));
        let branch_name = branch.name.clone();
        div()
            .id(SharedString::from(branch.name.clone()))
            .flex()
            .items_center()
            .gap_2()
            .flex_shrink_0()
            .h(px(ROW_HEIGHT))
            .px_3()
            .cursor_pointer()
            .hover(|row| row.bg(rgb(HOVERED_ROW_BACKGROUND)))
            .when(branch.is_current, |row| {
                row.bg(rgb(CURRENT_BRANCH_BACKGROUND))
                    .font_weight(FontWeight::BOLD)
            })
            .on_click(cx.listener(move |this, _, _, cx| {
                this.switch_branch_and_reload(&branch_name);
                cx.notify();
            }))
            .child(
                div()
                    .w(px(12.))
                    .text_color(label_color)
                    .child(if branch.is_current { "●" } else { "" }),
            )
            .child(
                div()
                    .truncate()
                    .text_color(label_color)
                    .child(branch.name.clone()),
            )
    });

    div()
        .id("local-branch-panel")
        .overflow_y_scroll()
        .flex()
        .flex_col()
        .flex_shrink_0()
        .w(px(BRANCH_PANEL_WIDTH))
        .h_full()
        .bg(rgb(BRANCH_PANEL_BACKGROUND))
        .border_r_1()
        .border_color(rgb(PANEL_BORDER))
        .child(
            div()
                .flex()
                .items_center()
                .flex_shrink_0()
                .h(px(ROW_HEIGHT))
                .px_3()
                .text_color(rgb(SECONDARY_TEXT))
                .child("BRANCHES"),
        )
        .children(branch_rows)
}
