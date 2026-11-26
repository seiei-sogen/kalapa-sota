use std::path::{Path, PathBuf};

use crate::git_cli::fixture_repository::{
    create_fixture_repository, path_outside_fixture, run_git_in_fixture,
};
use crate::git_cli::run_git_command::run_git_command;
use crate::git_cli::{read_repository_snapshot, switch_local_branch};

fn create_fixture_with_hotfix_and_linked_review_worktree(test_name: &str) -> (PathBuf, PathBuf) {
    let fixture = create_fixture_repository(test_name);
    run_git_in_fixture(&fixture, &["commit", "-q", "--allow-empty", "-m", "base"]);
    run_git_in_fixture(&fixture, &["branch", "hotfix"]);
    let linked_worktree = path_outside_fixture(test_name, "linked");
    run_git_in_fixture(
        &fixture,
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "review",
            linked_worktree.to_str().unwrap(),
        ],
    );
    (fixture, linked_worktree)
}

fn current_branch_name(worktree: &Path) -> String {
    run_git_command(worktree, &["branch", "--show-current"])
        .unwrap()
        .trim()
        .to_string()
}

#[test]
fn switching_in_a_linked_worktree_moves_only_that_worktrees_branch() {
    let (fixture, linked_worktree) =
        create_fixture_with_hotfix_and_linked_review_worktree("switch-in-linked-worktree");

    switch_local_branch(&linked_worktree, "hotfix").unwrap();

    assert_eq!(current_branch_name(&linked_worktree), "hotfix");
    assert_eq!(current_branch_name(&fixture), "main");

    std::fs::remove_dir_all(&linked_worktree).unwrap();
    std::fs::remove_dir_all(&fixture).unwrap();
}

#[test]
fn switching_a_linked_worktree_to_the_main_worktrees_branch_fails_and_moves_neither_head() {
    let (fixture, linked_worktree) =
        create_fixture_with_hotfix_and_linked_review_worktree("switch-to-branch-in-use");

    assert!(switch_local_branch(&linked_worktree, "main").is_err());

    assert_eq!(current_branch_name(&linked_worktree), "review");
    assert_eq!(current_branch_name(&fixture), "main");

    std::fs::remove_dir_all(&linked_worktree).unwrap();
    std::fs::remove_dir_all(&fixture).unwrap();
}

#[test]
fn a_snapshot_read_from_a_linked_worktree_has_its_branch_as_current_and_only_its_changes() {
    let (fixture, linked_worktree) =
        create_fixture_with_hotfix_and_linked_review_worktree("snapshot-of-linked-worktree");
    std::fs::write(fixture.join("only-in-main-worktree.txt"), "main\n").unwrap();
    std::fs::write(
        linked_worktree.join("only-in-linked-worktree.txt"),
        "linked\n",
    )
    .unwrap();

    let snapshot = read_repository_snapshot(&linked_worktree).unwrap();

    let current_branch_names: Vec<&str> = snapshot
        .local_branches
        .iter()
        .filter(|branch| branch.is_current)
        .map(|branch| branch.name.as_str())
        .collect();
    assert_eq!(current_branch_names, ["review"]);
    let changed_file_paths: Vec<&str> = snapshot
        .changed_files
        .iter()
        .map(|changed_file| changed_file.path.0.as_str())
        .collect();
    assert_eq!(changed_file_paths, ["only-in-linked-worktree.txt"]);

    std::fs::remove_dir_all(&linked_worktree).unwrap();
    std::fs::remove_dir_all(&fixture).unwrap();
}
