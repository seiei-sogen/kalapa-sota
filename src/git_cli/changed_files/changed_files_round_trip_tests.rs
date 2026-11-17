use std::path::Path;

use super::{read_changed_files, toggle_changed_file_in_commit};
use crate::git_cli::fixture_repository::{create_fixture_repository, run_git_in_fixture};
use crate::repository::{ChangedFileStaging, FileChangeKind};
use ChangedFileStaging::{PartiallyStaged, Staged, Unstaged, Untracked};
use FileChangeKind::{Added, Deleted, Modified};

fn write_fixture_file(fixture: &Path, relative_path: &str, contents: &str) {
    let path = fixture.join(relative_path);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, contents).unwrap();
}

fn read_sorted_staging(repository_path: &Path) -> Vec<(String, ChangedFileStaging)> {
    let mut staging_by_path: Vec<(String, ChangedFileStaging)> =
        read_changed_files(repository_path)
            .unwrap()
            .into_iter()
            .map(|changed_file| (changed_file.path.0, changed_file.staging))
            .collect();
    staging_by_path.sort_by(|(left_path, _), (right_path, _)| left_path.cmp(right_path));
    staging_by_path
}

fn toggle_listed_file(repository_path: &Path, root_relative_path: &str) {
    let changed_file = read_changed_files(repository_path)
        .unwrap()
        .into_iter()
        .find(|changed_file| changed_file.path.0 == root_relative_path)
        .unwrap_or_else(|| panic!("{root_relative_path} is listed"));
    toggle_changed_file_in_commit(repository_path, &changed_file).unwrap();
}

fn expected_staging_by_path(
    staging_by_path: &[(&str, ChangedFileStaging)],
) -> Vec<(String, ChangedFileStaging)> {
    staging_by_path
        .iter()
        .map(|(path, staging)| (path.to_string(), *staging))
        .collect()
}

#[test]
fn lists_each_kind_of_change_and_toggles_it_in_and_out_of_the_commit() {
    let fixture = create_fixture_repository("changed-file-toggle");
    for committed in [
        "a*b[1].txt",
        "aXb1.txt",
        "deleted.txt",
        "modified.txt",
        "partly.txt",
        "staged.txt",
        "日本語/メモ 1.txt",
    ] {
        write_fixture_file(&fixture, committed, "base\n");
    }
    run_git_in_fixture(&fixture, &["add", "--all"]);
    run_git_in_fixture(&fixture, &["commit", "-q", "-m", "base"]);
    for modified in [
        "a*b[1].txt",
        "aXb1.txt",
        "modified.txt",
        "partly.txt",
        "staged.txt",
        "日本語/メモ 1.txt",
    ] {
        write_fixture_file(&fixture, modified, "changed\n");
    }
    run_git_in_fixture(&fixture, &["add", "partly.txt", "staged.txt"]);
    write_fixture_file(&fixture, "partly.txt", "changed again\n");
    std::fs::remove_file(fixture.join("deleted.txt")).unwrap();
    write_fixture_file(&fixture, "new dir/untracked.txt", "new\n");

    let as_read = expected_staging_by_path(&[
        ("a*b[1].txt", Unstaged(Modified)),
        ("aXb1.txt", Unstaged(Modified)),
        ("deleted.txt", Unstaged(Deleted)),
        ("modified.txt", Unstaged(Modified)),
        ("new dir/untracked.txt", Untracked),
        (
            "partly.txt",
            PartiallyStaged {
                staged: Modified,
                unstaged: Modified,
            },
        ),
        ("staged.txt", Staged(Modified)),
        ("日本語/メモ 1.txt", Unstaged(Modified)),
    ]);
    assert_eq!(read_sorted_staging(&fixture), as_read);

    for (path, _) in &as_read {
        if path != "aXb1.txt" {
            toggle_listed_file(&fixture, path);
        }
    }
    assert_eq!(
        read_sorted_staging(&fixture),
        expected_staging_by_path(&[
            ("a*b[1].txt", Staged(Modified)),
            ("aXb1.txt", Unstaged(Modified)),
            ("deleted.txt", Staged(Deleted)),
            ("modified.txt", Staged(Modified)),
            ("new dir/untracked.txt", Staged(Added)),
            ("partly.txt", Staged(Modified)),
            ("staged.txt", Unstaged(Modified)),
            ("日本語/メモ 1.txt", Staged(Modified)),
        ]),
        "a*b[1].txt is staged on its own, without matching aXb1.txt as a glob"
    );

    for path in ["deleted.txt", "new dir/untracked.txt", "partly.txt"] {
        toggle_listed_file(&fixture, path);
    }
    let after_unstaging = read_sorted_staging(&fixture);
    assert!(after_unstaging.contains(&("deleted.txt".to_string(), Unstaged(Deleted))));
    assert!(after_unstaging.contains(&("new dir/untracked.txt".to_string(), Untracked)));
    assert!(after_unstaging.contains(&("partly.txt".to_string(), Unstaged(Modified))));

    std::fs::remove_dir_all(&fixture).unwrap();
}

#[test]
fn toggles_a_file_outside_the_subdirectory_the_repository_was_opened_at() {
    let fixture = create_fixture_repository("changed-file-subdirectory");
    write_fixture_file(&fixture, "top.txt", "base\n");
    write_fixture_file(&fixture, "sub/inner.txt", "base\n");
    run_git_in_fixture(&fixture, &["add", "--all"]);
    run_git_in_fixture(&fixture, &["commit", "-q", "-m", "base"]);
    write_fixture_file(&fixture, "top.txt", "changed\n");
    write_fixture_file(&fixture, "sub/inner.txt", "changed\n");
    let opened_at_subdirectory = fixture.join("sub");

    toggle_listed_file(&opened_at_subdirectory, "top.txt");
    toggle_listed_file(&opened_at_subdirectory, "sub/inner.txt");

    assert_eq!(
        read_sorted_staging(&opened_at_subdirectory),
        expected_staging_by_path(&[
            ("sub/inner.txt", Staged(Modified)),
            ("top.txt", Staged(Modified))
        ])
    );
    std::fs::remove_dir_all(&fixture).unwrap();
}

#[test]
fn unstages_a_file_before_the_first_commit() {
    let fixture = create_fixture_repository("changed-file-before-first-commit");
    write_fixture_file(&fixture, "first.txt", "first\n");

    toggle_listed_file(&fixture, "first.txt");
    assert_eq!(
        read_sorted_staging(&fixture),
        expected_staging_by_path(&[("first.txt", Staged(Added))])
    );
    toggle_listed_file(&fixture, "first.txt");
    assert_eq!(
        read_sorted_staging(&fixture),
        expected_staging_by_path(&[("first.txt", Untracked)])
    );

    std::fs::remove_dir_all(&fixture).unwrap();
}

#[test]
fn reads_no_changes_where_there_is_no_work_tree() {
    let fixture = create_fixture_repository("changed-file-no-work-tree");
    write_fixture_file(&fixture, "untracked.txt", "new\n");

    assert_eq!(read_changed_files(&fixture.join(".git")).unwrap(), []);

    std::fs::remove_dir_all(&fixture).unwrap();
}
