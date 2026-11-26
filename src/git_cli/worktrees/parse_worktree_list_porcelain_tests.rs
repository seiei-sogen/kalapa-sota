use std::path::PathBuf;

use super::parse_worktree_list_porcelain::parse_worktree_list_porcelain;
use crate::repository::{CommitId, Worktree, WorktreeCheckout, WorktreeRootPath};

const HEAD_COMMIT_ID: &str = "0123456789abcdef0123456789abcdef01234567";

// git worktree list --porcelain -z は、属性ごとに NUL で終え、レコードの終わりにもう 1 つ NUL を置く。
fn worktree_list_porcelain_output(records: &[&[&str]]) -> String {
    records
        .iter()
        .map(|attributes| {
            let record: String = attributes
                .iter()
                .map(|attribute| format!("{attribute}\0"))
                .collect();
            format!("{record}\0")
        })
        .collect()
}

fn worktree(root_path: &str, checkout: WorktreeCheckout) -> Worktree {
    Worktree {
        root_path: WorktreeRootPath(PathBuf::from(root_path)),
        checkout,
    }
}

fn branch(branch_name: &str) -> WorktreeCheckout {
    WorktreeCheckout::Branch {
        branch_name: branch_name.to_string(),
    }
}

fn detached_head() -> WorktreeCheckout {
    WorktreeCheckout::DetachedHead {
        head_commit_id: CommitId(HEAD_COMMIT_ID.to_string()),
    }
}

#[test]
fn reads_branch_detached_and_bare_checkouts() {
    let head = format!("HEAD {HEAD_COMMIT_ID}");
    let output = worktree_list_porcelain_output(&[
        &["worktree /repos/project.git", "bare"],
        &[
            "worktree /repos/main",
            &head,
            "branch refs/heads/feature/login",
        ],
        &["worktree /repos/review", &head, "detached"],
    ]);

    assert_eq!(
        parse_worktree_list_porcelain(&output).unwrap(),
        [
            worktree("/repos/project.git", WorktreeCheckout::Bare),
            worktree("/repos/main", branch("feature/login")),
            worktree("/repos/review", detached_head()),
        ]
    );
}

#[test]
fn skips_locked_prunable_and_unknown_attributes() {
    let head = format!("HEAD {HEAD_COMMIT_ID}");
    let output = worktree_list_porcelain_output(&[
        &[
            "worktree /repos/locked",
            &head,
            "branch refs/heads/main",
            "locked",
        ],
        &[
            "worktree /repos/locked-with-reason",
            &head,
            "detached",
            "locked on a removable drive",
        ],
        &[
            "worktree /repos/deleted",
            &head,
            "branch refs/heads/deleted",
            "prunable gitdir file points to non-existent location",
        ],
        &[
            "worktree /repos/future",
            &head,
            "detached",
            "attribute-from-a-newer-git value",
        ],
    ]);

    assert_eq!(
        parse_worktree_list_porcelain(&output).unwrap(),
        [
            worktree("/repos/locked", branch("main")),
            worktree("/repos/locked-with-reason", detached_head()),
            worktree("/repos/deleted", branch("deleted")),
            worktree("/repos/future", detached_head()),
        ]
    );
}

#[test]
fn keeps_paths_with_spaces_and_newlines_as_is() {
    let head = format!("HEAD {HEAD_COMMIT_ID}");
    let output = worktree_list_porcelain_output(&[&[
        "worktree /repos/新しい フォルダ/line\nbreak",
        &head,
        "branch refs/heads/main",
    ]]);

    assert_eq!(
        parse_worktree_list_porcelain(&output).unwrap(),
        [worktree(
            "/repos/新しい フォルダ/line\nbreak",
            branch("main")
        )]
    );
}

// git は、HEAD を解決できないワークツリーを worktree と HEAD だけのレコードで返す。
#[test]
fn reads_a_head_that_is_not_a_branch_or_a_commit_as_unreadable_instead_of_failing_the_list() {
    let unresolved_head = format!("HEAD {}", "0".repeat(40));
    let output = worktree_list_porcelain_output(&[
        &["worktree /repos/broken-head", &unresolved_head],
        &[
            "worktree /repos/head-on-a-tag",
            &unresolved_head,
            "branch refs/tags/v1",
        ],
    ]);

    assert_eq!(
        parse_worktree_list_porcelain(&output).unwrap(),
        [
            worktree("/repos/broken-head", WorktreeCheckout::UnreadableHead),
            worktree("/repos/head-on-a-tag", WorktreeCheckout::UnreadableHead),
        ]
    );
}

#[test]
fn rejects_records_it_cannot_read() {
    let head = format!("HEAD {HEAD_COMMIT_ID}");
    for (unreadable_record, attributes) in [
        (
            "a record that does not start with worktree",
            vec![head.as_str(), "branch refs/heads/main"],
        ),
        (
            "a detached record without HEAD",
            vec!["worktree /repos/main", "detached"],
        ),
    ] {
        let output = worktree_list_porcelain_output(&[&attributes]);
        assert!(
            parse_worktree_list_porcelain(&output).is_err(),
            "{unreadable_record} is rejected"
        );
    }
}
