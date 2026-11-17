use super::{push_current_branch, read_local_branches};
use crate::git_cli::fixture_repository::{create_fixture_repository, run_git_in_fixture};
use crate::git_cli::run_git_command::run_git_command;
use crate::repository::UpstreamTracking::{self, NoUpstream, Tracking};

#[test]
fn publishes_a_branch_without_upstream_then_pushes_a_later_commit() {
    let fixture = create_fixture_repository("push-current-branch");
    let bare_remote = std::env::temp_dir().join(format!(
        "kalapa-push-current-branch-remote-{}.git",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&bare_remote);
    std::fs::create_dir_all(&bare_remote).unwrap();
    run_git_command(&bare_remote, &["init", "-q", "--bare"]).unwrap();
    run_git_in_fixture(&fixture, &["commit", "-q", "--allow-empty", "-m", "first"]);
    run_git_in_fixture(
        &fixture,
        &["remote", "add", "origin", bare_remote.to_str().unwrap()],
    );
    run_git_in_fixture(&fixture, &["branch", "other"]);
    run_git_in_fixture(&fixture, &["push", "-q", "origin", "other"]);
    let other_on_remote_before = run_git_command(&bare_remote, &["rev-parse", "other"]).unwrap();
    run_git_in_fixture(&fixture, &["switch", "-q", "other"]);
    run_git_in_fixture(&fixture, &["commit", "-q", "--allow-empty", "-m", "other work"]);
    run_git_in_fixture(&fixture, &["switch", "-q", "main"]);
    run_git_in_fixture(&fixture, &["config", "push.default", "matching"]);
    let read_current_branch_tracking = || -> UpstreamTracking {
        read_local_branches(&fixture)
            .unwrap()
            .into_iter()
            .find(|branch| branch.is_current)
            .unwrap()
            .upstream_tracking
    };
    let in_sync = Tracking {
        commits_ahead: 0,
        commits_behind: 0,
    };
    assert_eq!(read_current_branch_tracking(), NoUpstream);

    push_current_branch(&fixture).unwrap();
    assert_eq!(
        read_current_branch_tracking(),
        in_sync,
        "published with upstream set"
    );

    run_git_in_fixture(&fixture, &["commit", "-q", "--allow-empty", "-m", "second"]);
    assert_eq!(
        read_current_branch_tracking(),
        Tracking {
            commits_ahead: 1,
            commits_behind: 0
        }
    );

    push_current_branch(&fixture).unwrap();
    assert_eq!(read_current_branch_tracking(), in_sync);
    assert_eq!(
        run_git_command(&bare_remote, &["rev-parse", "main"]).unwrap(),
        run_git_command(&fixture, &["rev-parse", "HEAD"]).unwrap(),
        "the bare remote has the second commit"
    );
    assert_eq!(
        run_git_command(&bare_remote, &["rev-parse", "other"]).unwrap(),
        other_on_remote_before,
        "push.default=matching in the user's config does not push other branches"
    );

    std::fs::remove_dir_all(&fixture).unwrap();
    std::fs::remove_dir_all(&bare_remote).unwrap();
}

#[test]
fn a_push_without_any_remote_returns_git_reason() {
    let fixture = create_fixture_repository("push-without-remote");
    run_git_in_fixture(&fixture, &["commit", "-q", "--allow-empty", "-m", "first"]);

    let error = push_current_branch(&fixture).unwrap_err();
    assert!(!error.message.is_empty(), "the banner gets git's reason");

    std::fs::remove_dir_all(&fixture).unwrap();
}
