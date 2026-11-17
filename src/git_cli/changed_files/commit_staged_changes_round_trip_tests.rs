use super::{commit_staged_changes, read_changed_files};
use crate::git_cli::fixture_repository::{create_fixture_repository, run_git_in_fixture};
use crate::git_cli::run_git_command::run_git_command;
use crate::repository::{ChangedFileStaging, FileChangeKind};

#[test]
fn commits_only_the_staged_file_and_lets_git_refuse_an_empty_index_or_message() {
    let fixture = create_fixture_repository("commit-staged-changes");
    for file_name in ["staged.txt", "unstaged.txt"] {
        std::fs::write(fixture.join(file_name), "base\n").unwrap();
    }
    run_git_in_fixture(&fixture, &["add", "--all"]);
    run_git_in_fixture(&fixture, &["commit", "-q", "-m", "base"]);
    for file_name in ["staged.txt", "unstaged.txt"] {
        std::fs::write(fixture.join(file_name), "changed\n").unwrap();
    }
    run_git_in_fixture(&fixture, &["add", "staged.txt"]);
    let read_head = |format: &str| {
        run_git_command(
            &fixture,
            &["show", "--no-patch", &format!("--format={format}")],
        )
        .unwrap()
        .trim()
        .to_string()
    };

    commit_staged_changes(&fixture, "docs: staged.txt を更新").unwrap();
    assert_eq!(read_head("%s"), "docs: staged.txt を更新");
    let committed_paths = run_git_command(&fixture, &["show", "--name-only", "--format="]).unwrap();
    assert_eq!(committed_paths.trim(), "staged.txt");
    let left_over: Vec<(String, ChangedFileStaging)> = read_changed_files(&fixture)
        .unwrap()
        .into_iter()
        .map(|changed_file| (changed_file.path.0, changed_file.staging))
        .collect();
    assert_eq!(
        left_over,
        [(
            "unstaged.txt".to_string(),
            ChangedFileStaging::Unstaged(FileChangeKind::Modified)
        )]
    );

    let head_before_refusals = read_head("%H");
    let nothing_staged = commit_staged_changes(&fixture, "nothing staged").unwrap_err();
    assert!(
        !nothing_staged.message.is_empty(),
        "git explains on stdout only, and that reaches the caller"
    );
    run_git_in_fixture(&fixture, &["add", "unstaged.txt"]);
    let empty_message = commit_staged_changes(&fixture, "").unwrap_err();
    assert!(!empty_message.message.is_empty(), "git explains why");
    assert_eq!(read_head("%H"), head_before_refusals);

    std::fs::remove_dir_all(&fixture).unwrap();
}
