use gpui::{
    App, Context, CursorStyle, FocusHandle, Focusable, MouseButton, Window, div, prelude::*, px,
    rgb,
};

use super::SingleLineTextInput;
use super::single_line_text_element::SingleLineTextElement;
use super::single_line_text_input_colors::{INPUT_BACKGROUND, INPUT_BORDER, INPUT_TEXT};
use super::single_line_text_input_key_bindings::SINGLE_LINE_TEXT_INPUT_KEY_CONTEXT;

const LINE_HEIGHT: f32 = 20.;
const PADDING: f32 = 4.;

impl Render for SingleLineTextInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .key_context(SINGLE_LINE_TEXT_INPUT_KEY_CONTEXT)
            .track_focus(&self.focus_handle(cx))
            .cursor(CursorStyle::IBeam)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::home))
            .on_action(cx.listener(Self::end))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::copy))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .bg(rgb(INPUT_BACKGROUND))
            .border_1()
            .border_color(rgb(INPUT_BORDER))
            .text_color(rgb(INPUT_TEXT))
            .line_height(px(LINE_HEIGHT))
            .text_size(px(14.))
            // shortcut: 横スクロールしない。入力欄より長い文は右端で切れて見えない。長い件名を打つ使い方が多いなら、カーソル位置に合わせて描画位置をずらす。
            .overflow_hidden()
            .child(
                div()
                    .h(px(LINE_HEIGHT + PADDING * 2.))
                    .w_full()
                    .p(px(PADDING))
                    .child(SingleLineTextElement { input: cx.entity() }),
            )
    }
}

impl Focusable for SingleLineTextInput {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
