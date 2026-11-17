use crate::appearance::dark_theme_colors::{
    ADDED_FILE_MARKER, CONFLICTED_FILE_MARKER, DELETED_FILE_MARKER, MODIFIED_FILE_MARKER,
    TYPE_CHANGED_FILE_MARKER, UNTRACKED_FILE_MARKER,
};
use crate::repository::{ChangedFileStaging, FileChangeKind};

pub fn changed_file_kind_marker(staging: &ChangedFileStaging) -> (&'static str, u32) {
    let shown_change_kind = match *staging {
        ChangedFileStaging::Untracked => return ("?", UNTRACKED_FILE_MARKER),
        ChangedFileStaging::Conflicted => return ("!", CONFLICTED_FILE_MARKER),
        ChangedFileStaging::Unstaged(change_kind) | ChangedFileStaging::Staged(change_kind) => {
            change_kind
        }
        // 新しく追加してから書き足したファイルを A のまま見せるため、インデックス側の種類を出す。
        ChangedFileStaging::PartiallyStaged { staged, .. } => staged,
    };
    match shown_change_kind {
        FileChangeKind::Added => ("A", ADDED_FILE_MARKER),
        FileChangeKind::Modified => ("M", MODIFIED_FILE_MARKER),
        FileChangeKind::Deleted => ("D", DELETED_FILE_MARKER),
        FileChangeKind::TypeChanged => ("T", TYPE_CHANGED_FILE_MARKER),
    }
}
