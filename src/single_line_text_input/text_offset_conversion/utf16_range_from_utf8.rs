use std::ops::Range;

use super::utf16_offset_from_utf8;

pub fn utf16_range_from_utf8(text: &str, range: &Range<usize>) -> Range<usize> {
    utf16_offset_from_utf8(text, range.start)..utf16_offset_from_utf8(text, range.end)
}
