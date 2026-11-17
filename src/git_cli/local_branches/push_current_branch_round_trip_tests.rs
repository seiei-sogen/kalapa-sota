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

    std::fs::remove_dir_all(&fixture).unwrap();
    std::fs::remove_dir_all(&bare_remote).unwrap();
}
