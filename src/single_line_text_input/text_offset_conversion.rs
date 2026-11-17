use std::ops::Range;

use unicode_segmentation::UnicodeSegmentation;

// IME とのやり取りは UTF-16 の位置、content の切り出しは UTF-8 のバイト位置で行うので、相互に変換する。

pub(super) fn utf8_offset_from_utf16(text: &str, utf16_offset: usize) -> usize {
    let mut utf8_offset = 0;
    let mut utf16_count = 0;

    for ch in text.chars() {
        if utf16_count >= utf16_offset {
            break;
        }
        utf16_count += ch.len_utf16();
        utf8_offset += ch.len_utf8();
    }

    utf8_offset
}

pub(super) fn utf16_offset_from_utf8(text: &str, utf8_offset: usize) -> usize {
    let mut utf16_offset = 0;
    let mut utf8_count = 0;

    for ch in text.chars() {
        if utf8_count >= utf8_offset {
            break;
        }
        utf8_count += ch.len_utf8();
        utf16_offset += ch.len_utf16();
    }

    utf16_offset
}

pub(super) fn utf8_range_from_utf16(text: &str, range_utf16: &Range<usize>) -> Range<usize> {
    utf8_offset_from_utf16(text, range_utf16.start)..utf8_offset_from_utf16(text, range_utf16.end)
}

pub(super) fn utf16_range_from_utf8(text: &str, range: &Range<usize>) -> Range<usize> {
    utf16_offset_from_utf8(text, range.start)..utf16_offset_from_utf8(text, range.end)
}

/// IME は変換中の文字列の中での UTF-16 位置を渡すので、content 全体でのバイト位置に直す。
pub(super) fn content_range_from_marked_text_range_utf16(
    content: &str,
    marked_text_start: usize,
    range_in_marked_text_utf16: &Range<usize>,
) -> Range<usize> {
    let marked_text_start_utf16 = utf16_offset_from_utf8(content, marked_text_start);
    utf8_range_from_utf16(
        content,
        &(marked_text_start_utf16 + range_in_marked_text_utf16.start
            ..marked_text_start_utf16 + range_in_marked_text_utf16.end),
    )
}

pub(super) fn previous_grapheme_boundary(text: &str, offset: usize) -> usize {
    text.grapheme_indices(true)
        .rev()
        .find_map(|(index, _)| (index < offset).then_some(index))
        .unwrap_or(0)
}

pub(super) fn next_grapheme_boundary(text: &str, offset: usize) -> usize {
    text.grapheme_indices(true)
        .find_map(|(index, _)| (index > offset).then_some(index))
        .unwrap_or(text.len())
}
