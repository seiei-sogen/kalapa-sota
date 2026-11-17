// IME とのやり取りは UTF-16 の位置、content の切り出しは UTF-8 のバイト位置で行うので、相互に変換する。
mod content_range_from_marked_text_range_utf16;
mod utf16_offset_from_utf8;
mod utf16_range_from_utf8;
mod utf8_offset_from_utf16;
mod utf8_range_from_utf16;

#[cfg(test)]
mod text_offset_conversion_tests;

pub use content_range_from_marked_text_range_utf16::content_range_from_marked_text_range_utf16;
pub use utf8_offset_from_utf16::utf8_offset_from_utf16;
pub use utf8_range_from_utf16::utf8_range_from_utf16;
pub use utf16_offset_from_utf8::utf16_offset_from_utf8;
pub use utf16_range_from_utf8::utf16_range_from_utf8;
