use super::{next_grapheme_boundary, previous_grapheme_boundary};

#[test]
fn grapheme_boundaries_skip_whole_characters() {
    // "が" を "か" と結合濁点 U+3099 に分けた形。2 文字で 1 書記素になる。
    let text = "aか\u{3099}🎉";
    assert_eq!(next_grapheme_boundary(text, 0), 1);
    assert_eq!(next_grapheme_boundary(text, 1), 7);
    assert_eq!(next_grapheme_boundary(text, 7), 11);
    assert_eq!(next_grapheme_boundary(text, 11), 11);
    assert_eq!(previous_grapheme_boundary(text, 11), 7);
    assert_eq!(previous_grapheme_boundary(text, 7), 1);
    assert_eq!(previous_grapheme_boundary(text, 0), 0);
}
