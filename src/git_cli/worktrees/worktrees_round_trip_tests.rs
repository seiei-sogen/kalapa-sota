use std::path::Path;

use super::{read_worktrees, resolve_worktree_root_path};
use crate::git_cli::fixture_repository::{
    create_fixture_repository, path_outside_fixture, run_git_in_fixture,
};
use crate::git_cli::read_repository_snapshot;
use crate::git_cli::run_git_command::run_git_command;
use crate::repository::{CommitId, Worktree, WorktreeCheckout, WorktreeRootPath};

// macOS の一時ディレクトリは /var の下にあり、git は /private/var で始まるパスを返すので、正規化して比べる。
fn canonical_worktree_root_path(path: &Path) -> WorktreeRootPath {
    WorktreeRootPath(path.canonicalize().unwrap())
}

#[test]
fn a_subdirectory_resolves_to_its_worktree_root() {
    let fixture = create_fixture_repository("resolve-subdirectory");
    let subdirectory = fixture.join("sub");
    std::fs::create_dir(&subdirectory).unwrap();

    assert_eq!(
        resolve_worktree_root_path(&subdirectory).unwrap(),
        canonical_worktree_root_path(&fixture)
    );

    std::fs::remove_dir_all(&fixture).unwrap();
}

#[test]
fn a_bare_repository_resolves_to_itself() {
    let bare_repository = path_outside_fixture("resolve", "bare");
    run_git_in_fixture(
        &std::env::temp_dir(),
        &["init", "-q", "--bare", bare_repository.to_str().unwrap()],
    );

    assert_eq!(
        resolve_worktree_root_path(&bare_repository).unwrap(),
        canonical_worktree_root_path(&bare_repository)
    );

    std::fs::remove_dir_all(&bare_repository).unwrap();
}

fn checked_out_branch(branch_name: &str) -> WorktreeCheckout {
    WorktreeCheckout::Branch {
        branch_name: branch_name.to_string(),
    }
}

#[test]
fn lists_the_main_and_a_linked_worktree_with_their_checkouts() {
    let fixture = create_fixture_repository("read-worktrees");
    run_git_in_fixture(&fixture, &["commit", "-q", "--allow-empty", "-m", "base"]);
    let linked_worktree = path_outside_fixture("read-worktrees", "linked");
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
    let linked_subdirectory = linked_worktree.join("sub");
    std::fs::create_dir(&linked_subdirectory).unwrap();

    let worktrees = read_worktrees(&fixture).unwrap();
    assert_eq!(
        worktrees,
        [
            Worktree {
                root_path: canonical_worktree_root_path(&fixture),
                checkout: checked_out_branch("main"),
            },
            Worktree {
                root_path: canonical_worktree_root_path(&linked_worktree),
                checkout: checked_out_branch("review"),
            },
        ]
    );
    assert_eq!(
        resolve_worktree_root_path(&linked_subdirectory).unwrap(),
        worktrees[1].root_path
    );

    std::fs::remove_dir_all(&linked_worktree).unwrap();
    std::fs::remove_dir_all(&fixture).unwrap();
}

#[test]
fn a_linked_worktree_whose_directory_was_deleted_stays_in_the_list() {
    let fixture = create_fixture_repository("read-deleted-worktree");
    run_git_in_fixture(&fixture, &["commit", "-q", "--allow-empty", "-m", "base"]);
    let head_commit_id = run_git_command(&fixture, &["rev-parse", "HEAD"]).unwrap();
    let deleted_worktree = path_outside_fixture("read-deleted-worktree", "deleted");
    run_git_in_fixture(
        &fixture,
        &[
            "worktree",
            "add",
            "-q",
            "--detach",
            deleted_worktree.to_str().unwrap(),
        ],
    );
    let deleted_worktree_root_path = canonical_worktree_root_path(&deleted_worktree);
    std::fs::remove_dir_all(&deleted_worktree).unwrap();

    assert_eq!(
        read_worktrees(&fixture).unwrap()[1..],
        [Worktree {
            root_path: deleted_worktree_root_path,
            checkout: WorktreeCheckout::DetachedHead {
                head_commit_id: CommitId(head_commit_id.trim().to_string()),
            },
        }]
    );

    std::fs::remove_dir_all(&fixture).unwrap();
}

#[test]
fn a_bare_repository_with_a_linked_worktree_reads_as_bare_first_without_changes() {
    let fixture = create_fixture_repository("read-bare-worktrees");
    run_git_in_fixture(&fixture, &["commit", "-q", "--allow-empty", "-m", "base"]);
    let bare_repository = path_outside_fixture("read-bare-worktrees", "bare");
    let linked_worktree = path_outside_fixture("read-bare-worktrees", "linked");
    run_git_in_fixture(
        &std::env::temp_dir(),
        &[
            "clone",
            "-q",
            "--bare",
            fixture.to_str().unwrap(),
            bare_repository.to_str().unwrap(),
        ],
    );
    run_git_in_fixture(
        &bare_repository,
        &[
            "worktree",
            "add",
            "-q",
            linked_worktree.to_str().unwrap(),
            "main",
        ],
    );

    let snapshot = read_repository_snapshot(&bare_repository).unwrap();
    assert_eq!(
        snapshot.worktrees,
        [
            Worktree {
                root_path: resolve_worktree_root_path(&bare_repository).unwrap(),
                checkout: WorktreeCheckout::Bare,
            },
            Worktree {
                root_path: canonical_worktree_root_path(&linked_worktree),
                checkout: checked_out_branch("main"),
            },
        ]
    );
    assert_eq!(snapshot.changed_files, []);

    for directory in [&linked_worktree, &bare_repository, &fixture] {
        std::fs::remove_dir_all(directory).unwrap();
    }
}
