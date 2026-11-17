use gpui::{Div, ElementId, SharedString, Stateful, div, prelude::*, px, rgb};

use crate::appearance::dark_theme_colors::{BUTTON_BACKGROUND, HOVERED_BUTTON_BACKGROUND};
use crate::appearance::repository_window_sizes::COMMIT_PANEL_BUTTON_WIDTH;

pub fn render_commit_panel_button(
    element_id: impl Into<ElementId>,
    label: impl Into<SharedString>,
) -> Stateful<Div> {
    div()
        .id(element_id)
        .flex()
        .flex_shrink_0()
        .justify_center()
        .items_center()
        .w(px(COMMIT_PANEL_BUTTON_WIDTH))
        .h_full()
        .rounded_sm()
        .bg(rgb(BUTTON_BACKGROUND))
        .hover(|button| button.bg(rgb(HOVERED_BUTTON_BACKGROUND)))
        .cursor_pointer()
        .whitespace_nowrap()
        .child(label.into())
}
