use std::ops::Range;

use gpui::{Bounds, Context, FocusHandle, Pixels, ShapedLine, SharedString};

// shortcut: 1 行だけ扱う。複数行のコミットメッセージ本文が要るなら、行ごとの ShapedLine と上下移動を足す。
pub struct SingleLineTextInput {
    pub(super) focus_handle: FocusHandle,
    pub(super) content: SharedString,
    pub(super) placeholder: SharedString,
    /// content の UTF-8 バイト位置。
    pub(super) selected_range: Range<usize>,
    pub(super) selection_reversed: bool,
    /// IME で変換中の範囲。
    pub(super) marked_range: Option<Range<usize>>,
    /// 直前の描画結果。マウス位置と IME 候補窓の位置を文字位置へ変えるのに使う。
    pub(super) last_layout: Option<ShapedLine>,
    pub(super) last_bounds: Option<Bounds<Pixels>>,
    pub(super) is_selecting: bool,
}

impl SingleLineTextInput {
    pub fn new(placeholder: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            content: "".into(),
            placeholder: placeholder.into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
        }
    }

    pub fn text(&self) -> &str {
        &self.content
    }

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.content = "".into();
        self.selected_range = 0..0;
        self.selection_reversed = false;
        self.marked_range = None;
        self.last_layout = None;
        self.last_bounds = None;
        self.is_selecting = false;
        cx.notify();
    }
}
