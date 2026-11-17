use super::{
    content_range_from_marked_text_range_utf16, utf8_offset_from_utf16, utf8_range_from_utf16,
    utf16_offset_from_utf8, utf16_range_from_utf8,
};

// "🎉" は UTF-8 で 4 バイト、UTF-16 で 2 単位。"コ" は 3 バイト、1 単位。
const ASCII_JAPANESE_AND_EMOJI: &str = "fix: コミット🎉";

#[test]
fn utf16_offsets_convert_to_utf8_offsets_and_back() {
    let text = ASCII_JAPANESE_AND_EMOJI;
    let expected_utf16_and_utf8_offsets = [(0, 0), (5, 5), (6, 8), (9, 17), (11, 21)];
    for (utf16_offset, utf8_offset) in expected_utf16_and_utf8_offsets {
        assert_eq!(utf8_offset_from_utf16(text, utf16_offset), utf8_offset);
        assert_eq!(utf16_offset_from_utf8(text, utf8_offset), utf16_offset);
    }
    assert_eq!(utf8_range_from_utf16(text, &(5..9)), 5..17);
    assert_eq!(utf16_range_from_utf8(text, &(5..17)), 5..9);
}

#[test]
fn offsets_past_the_end_clamp_to_the_end() {
    let text = ASCII_JAPANESE_AND_EMOJI;
    assert_eq!(utf8_offset_from_utf16(text, 100), text.len());
    assert_eq!(utf16_offset_from_utf8(text, 100), 11);
}

#[test]
fn ime_cursor_inside_marked_text_maps_to_the_whole_content() {
    let content = "fix: こんに";
    assert_eq!(
        content_range_from_marked_text_range_utf16(content, 5, &(1..2)),
        8..11
    );
    assert_eq!(
        content_range_from_marked_text_range_utf16(content, 5, &(3..3)),
        14..14
    );
}
