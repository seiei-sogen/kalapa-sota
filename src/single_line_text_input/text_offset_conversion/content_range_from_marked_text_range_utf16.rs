use std::ops::Range;

use super::{utf8_range_from_utf16, utf16_offset_from_utf8};

/// IME は変換中の文字列の中での UTF-16 位置を渡すので、content 全体でのバイト位置に直す。
pub fn content_range_from_marked_text_range_utf16(
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
