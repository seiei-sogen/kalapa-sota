use super::FileChangeKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangedFileStaging {
    Untracked,
    Unstaged(FileChangeKind),
    Staged(FileChangeKind),
    PartiallyStaged {
        staged: FileChangeKind,
        unstaged: FileChangeKind,
    },
    Conflicted,
}
