use std::ops::Range;

use super::utf8_offset_from_utf16;

pub fn utf8_range_from_utf16(text: &str, range_utf16: &Range<usize>) -> Range<usize> {
    utf8_offset_from_utf16(text, range_utf16.start)..utf8_offset_from_utf16(text, range_utf16.end)
}
