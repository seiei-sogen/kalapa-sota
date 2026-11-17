use super::FileChangeKind;

/// 変更がインデックスと作業ツリーのどちらにあるか。git の index がそのままコミットに含めるものの選択になる。
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
