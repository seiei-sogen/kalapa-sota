mod next_grapheme_boundary;
mod previous_grapheme_boundary;

#[cfg(test)]
mod grapheme_boundary_tests;

pub use next_grapheme_boundary::next_grapheme_boundary;
pub use previous_grapheme_boundary::previous_grapheme_boundary;
