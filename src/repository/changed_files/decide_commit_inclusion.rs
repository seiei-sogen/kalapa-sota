use super::{ChangedFileStaging, CommitInclusion};

pub fn decide_commit_inclusion(staging: &ChangedFileStaging) -> CommitInclusion {
    match staging {
        ChangedFileStaging::Staged(_) => CommitInclusion::Included,
        ChangedFileStaging::PartiallyStaged { .. } => CommitInclusion::PartlyIncluded,
        ChangedFileStaging::Untracked
        | ChangedFileStaging::Unstaged(_)
        | ChangedFileStaging::Conflicted => CommitInclusion::Excluded,
    }
}
