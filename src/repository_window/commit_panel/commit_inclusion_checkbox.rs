use crate::appearance::dark_theme_colors::{CHECKED_CHECKBOX, SECONDARY_TEXT};
use crate::repository::CommitInclusion;

pub fn commit_inclusion_checkbox(commit_inclusion: CommitInclusion) -> (&'static str, u32) {
    match commit_inclusion {
        CommitInclusion::Included => ("☑", CHECKED_CHECKBOX),
        CommitInclusion::PartlyIncluded => ("⊟", CHECKED_CHECKBOX),
        CommitInclusion::Excluded => ("☐", SECONDARY_TEXT),
    }
}
