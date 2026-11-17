use super::parse_porcelain_status::parse_porcelain_status;
use crate::repository::{
    ChangedFile, ChangedFileStaging, FileChangeKind, RepositoryRootRelativePath,
};
use ChangedFileStaging::{Conflicted, PartiallyStaged, Staged, Unstaged, Untracked};
use FileChangeKind::{Added, Deleted, Modified, TypeChanged};

fn parsed_staging(porcelain_records: &[&str]) -> Vec<ChangedFileStaging> {
    let output: String = porcelain_records
        .iter()
        .map(|record| format!("{record}\0"))
        .collect();
    parse_porcelain_status(&output)
        .unwrap()
        .into_iter()
        .map(|changed_file| changed_file.staging)
        .collect()
}

#[test]
fn reads_each_status_class_into_staging() {
    assert_eq!(
        parsed_staging(&[
            "?? untracked.txt",
            " M unstaged.txt",
            " D unstaged-deleted.txt",
            " T unstaged-type.txt",
            " A intent-to-add.txt",
            "M  staged.txt",
            "A  staged-added.txt",
            "D  staged-deleted.txt",
            "T  staged-type.txt",
            "MM partly.txt",
            "AM partly-added.txt",
            "MD partly-deleted.txt",
        ]),
        [
            Untracked,
            Unstaged(Modified),
            Unstaged(Deleted),
            Unstaged(TypeChanged),
            Unstaged(Added),
            Staged(Modified),
            Staged(Added),
            Staged(Deleted),
            Staged(TypeChanged),
            PartiallyStaged {
                staged: Modified,
                unstaged: Modified
            },
            PartiallyStaged {
                staged: Added,
                unstaged: Modified
            },
            PartiallyStaged {
                staged: Modified,
                unstaged: Deleted
            },
        ]
    );
}

#[test]
fn reads_every_unmerged_combination_as_conflicted() {
    assert_eq!(
        parsed_staging(&["DD a", "AU b", "UD c", "UA d", "DU e", "AA f", "UU g"]),
        [Conflicted; 7]
    );
}

#[test]
fn keeps_paths_with_spaces_and_japanese_as_is() {
    let changed_files =
        parse_porcelain_status("?? 新しい フォルダ/メモ 1.txt\0 M a b.txt\0").unwrap();
    assert_eq!(
        changed_files,
        [
            ChangedFile {
                path: RepositoryRootRelativePath("新しい フォルダ/メモ 1.txt".to_string()),
                staging: Untracked,
            },
            ChangedFile {
                path: RepositoryRootRelativePath("a b.txt".to_string()),
                staging: Unstaged(Modified),
            },
        ]
    );
}

#[test]
fn reads_a_clean_working_tree_as_no_changes() {
    assert_eq!(parse_porcelain_status("").unwrap(), []);
}

#[test]
fn rejects_status_it_does_not_know() {
    for unknown_record in [
        "R  renamed.txt",
        "!! ignored.txt",
        "XY x.txt",
        "   blank.txt",
        "M",
        "MMx.txt",
    ] {
        let error = parse_porcelain_status(&format!("{unknown_record}\0")).unwrap_err();
        assert!(
            error.message.contains(unknown_record),
            "the error names the record: {}",
            error.message
        );
    }
}
