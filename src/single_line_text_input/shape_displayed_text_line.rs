use gpui::{Hsla, ShapedLine, TextRun, UnderlineStyle, Window, px, rgb};

use super::SingleLineTextInput;
use super::single_line_text_input_colors::PLACEHOLDER_TEXT;

pub(super) fn shape_displayed_text_line(
    input: &SingleLineTextInput,
    window: &mut Window,
) -> ShapedLine {
    let style = window.text_style();

    let (display_text, text_color) = if input.content.is_empty() {
        (input.placeholder.clone(), Hsla::from(rgb(PLACEHOLDER_TEXT)))
    } else {
        (input.content.clone(), style.color)
    };

    let run = TextRun {
        len: display_text.len(),
        font: style.font(),
        color: text_color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    let runs = if let Some(marked_range) = input.marked_range.as_ref() {
        vec![
            TextRun {
                len: marked_range.start,
                ..run.clone()
            },
            TextRun {
                len: marked_range.end - marked_range.start,
                underline: Some(UnderlineStyle {
                    color: Some(run.color),
                    thickness: px(1.0),
                    wavy: false,
                }),
                ..run.clone()
            },
            TextRun {
                len: display_text.len() - marked_range.end,
                ..run
            },
        ]
        .into_iter()
        .filter(|run| run.len > 0)
        .collect()
    } else {
        vec![run]
    };

    let font_size = style.font_size.to_pixels(window.rem_size());
    window
        .text_system()
        .shape_line(display_text, font_size, &runs, None)
}
