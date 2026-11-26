use std::path::{Path, PathBuf};

use super::pull_current_branch_fast_forward_only;
use crate::git_cli::fixture_repository::{
    create_fixture_repository, path_outside_fixture, run_git_in_fixture,
};
use crate::git_cli::run_git_command::run_git_command;

fn create_fixture_tracking_main_on_bare_remote(test_name: &str) -> (PathBuf, PathBuf) {
    let fixture = create_fixture_repository(test_name);
    let bare_remote = path_outside_fixture(test_name, "remote.git");
    std::fs::create_dir_all(&bare_remote).unwrap();
    run_git_command(&bare_remote, &["init", "-q", "--bare"]).unwrap();
    run_git_in_fixture(&fixture, &["commit", "-q", "--allow-empty", "-m", "base"]);
    run_git_in_fixture(
        &fixture,
        &["remote", "add", "origin", bare_remote.to_str().unwrap()],
    );
    run_git_in_fixture(&fixture, &["push", "-q", "-u", "origin", "main"]);
    (fixture, bare_remote)
}

fn push_empty_commit_from_second_clone(test_name: &str, bare_remote: &Path, branch: &str) {
    let second_clone = path_outside_fixture(test_name, "second-clone");
    run_git_command(
        bare_remote,
        &[
            "clone",
            "-q",
            "-b",
            branch,
            bare_remote.to_str().unwrap(),
            second_clone.to_str().unwrap(),
        ],
    )
    .unwrap();
    run_git_command(
        &second_clone,
        &[
            "-c",
            "user.name=Bob",
            "-c",
            "user.email=bob@example.com",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "remote work",
        ],
    )
    .unwrap();
    run_git_command(&second_clone, &["push", "-q"]).unwrap();
    std::fs::remove_dir_all(&second_clone).unwrap();
}

fn rev_parse(repository: &Path, revision: &str) -> String {
    run_git_command(repository, &["rev-parse", revision]).unwrap()
}

#[test]
fn pulling_a_new_remote_commit_moves_head_to_the_remote_head() {
    let test_name = "pull-new-remote-commit";
    let (fixture, bare_remote) = create_fixture_tracking_main_on_bare_remote(test_name);
    push_empty_commit_from_second_clone(test_name, &bare_remote, "main");

    pull_current_branch_fast_forward_only(&fixture).unwrap();

    assert_eq!(rev_parse(&fixture, "HEAD"), rev_parse(&bare_remote, "main"));

    std::fs::remove_dir_all(&fixture).unwrap();
    std::fs::remove_dir_all(&bare_remote).unwrap();
}

#[test]
fn pulling_a_diverged_branch_with_pull_rebase_set_fails_and_leaves_head_without_rebase_state() {
    let test_name = "pull-diverged-branch";
    let (fixture, bare_remote) = create_fixture_tracking_main_on_bare_remote(test_name);
    push_empty_commit_from_second_clone(test_name, &bare_remote, "main");
    run_git_in_fixture(&fixture, &["commit", "-q", "--allow-empty", "-m", "local"]);
    run_git_in_fixture(&fixture, &["config", "pull.rebase", "true"]);
    let head_before_pull = rev_parse(&fixture, "HEAD");

    let error = pull_current_branch_fast_forward_only(&fixture).unwrap_err();

    assert!(
        error.message.contains("Not possible to fast-forward"),
        "the banner gets git's reason: {}",
        error.message
    );
    assert_eq!(rev_parse(&fixture, "HEAD"), head_before_pull);
    assert!(!fixture.join(".git/rebase-merge").exists());

    std::fs::remove_dir_all(&fixture).unwrap();
    std::fs::remove_dir_all(&bare_remote).unwrap();
}

#[test]
fn pulling_a_branch_without_upstream_returns_git_reason() {
    let test_name = "pull-without-upstream";
    let (fixture, bare_remote) = create_fixture_tracking_main_on_bare_remote(test_name);
    run_git_in_fixture(&fixture, &["switch", "-q", "-c", "no-upstream"]);

    let error = pull_current_branch_fast_forward_only(&fixture).unwrap_err();

    assert!(
        error.message.contains("no tracking information"),
        "the banner gets git's reason: {}",
        error.message
    );

    std::fs::remove_dir_all(&fixture).unwrap();
    std::fs::remove_dir_all(&bare_remote).unwrap();
}

#[test]
fn pulling_in_a_linked_worktree_moves_only_that_worktrees_branch() {
    let test_name = "pull-in-linked-worktree";
    let (fixture, bare_remote) = create_fixture_tracking_main_on_bare_remote(test_name);
    let linked_worktree = path_outside_fixture(test_name, "linked");
    let linked_worktree_path = linked_worktree.to_str().unwrap();
    run_git_in_fixture(
        &fixture,
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "review",
            linked_worktree_path,
        ],
    );
    run_git_command(&linked_worktree, &["push", "-q", "-u", "origin", "review"]).unwrap();
    push_empty_commit_from_second_clone(test_name, &bare_remote, "review");
    let main_worktree_head_before_pull = rev_parse(&fixture, "HEAD");

    pull_current_branch_fast_forward_only(&linked_worktree).unwrap();

    assert_eq!(
        rev_parse(&linked_worktree, "HEAD"),
        rev_parse(&bare_remote, "review")
    );
    assert_eq!(rev_parse(&fixture, "HEAD"), main_worktree_head_before_pull);

    std::fs::remove_dir_all(&linked_worktree).unwrap();
    std::fs::remove_dir_all(&fixture).unwrap();
    std::fs::remove_dir_all(&bare_remote).unwrap();
}
