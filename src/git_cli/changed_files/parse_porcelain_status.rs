use crate::git_cli::GitCommandError;
use crate::repository::{
    ChangedFile, ChangedFileStaging, FileChangeKind, RepositoryRootRelativePath,
};

pub fn parse_porcelain_status(output: &str) -> Result<Vec<ChangedFile>, GitCommandError> {
    let change_kind = |letter: u8| match letter {
        b'A' => Some(FileChangeKind::Added),
        b'M' => Some(FileChangeKind::Modified),
        b'D' => Some(FileChangeKind::Deleted),
        b'T' => Some(FileChangeKind::TypeChanged),
        _ => None,
    };
    output
        .split('\0')
        .filter(|record| !record.is_empty())
        .map(|record| {
            let unexpected_record = || GitCommandError {
                message: format!("unexpected git status output: {record:?}"),
            };
            let (Some(status), Some(" "), Some(path)) =
                (record.get(..2), record.get(2..3), record.get(3..))
            else {
                return Err(unexpected_record());
            };
            let staging = match *status.as_bytes() {
                [b'?', b'?'] => Some(ChangedFileStaging::Untracked),
                // git status の説明で unmerged とされる組み合わせ。AA と DD もここに入る。
                [b'U', _] | [_, b'U'] | [b'A', b'A'] | [b'D', b'D'] => {
                    Some(ChangedFileStaging::Conflicted)
                }
                [b' ', unstaged] => change_kind(unstaged).map(ChangedFileStaging::Unstaged),
                [staged, b' '] => change_kind(staged).map(ChangedFileStaging::Staged),
                [staged, unstaged] => {
                    change_kind(staged)
                        .zip(change_kind(unstaged))
                        .map(|(staged, unstaged)| ChangedFileStaging::PartiallyStaged {
                            staged,
                            unstaged,
                        })
                }
                _ => None,
            }
            .ok_or_else(unexpected_record)?;
            Ok(ChangedFile {
                path: RepositoryRootRelativePath(path.to_string()),
                staging,
            })
        })
        .collect()
}
