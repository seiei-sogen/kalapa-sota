use std::path::PathBuf;

use super::fixture_repository::{create_fixture_repository, run_git_in_fixture};
use super::{read_repository_snapshot, switch_local_branch};

fn create_fixture_with_merged_feature_branch() -> PathBuf {
    let fixture = create_fixture_repository("git-cli-round-trip");
    run_git_in_fixture(&fixture, &["commit", "-q", "--allow-empty", "-m", "base"]);
    run_git_in_fixture(&fixture, &["switch", "-q", "-c", "feature"]);
    run_git_in_fixture(
        &fixture,
        &["commit", "-q", "--allow-empty", "-m", "feature work"],
    );
    run_git_in_fixture(&fixture, &["switch", "-q", "main"]);
    run_git_in_fixture(
        &fixture,
        &["commit", "-q", "--allow-empty", "-m", "main work"],
    );
    run_git_in_fixture(
        &fixture,
        &["merge", "-q", "--no-ff", "feature", "-m", "merge feature"],
    );
    // ブランチと同名のタグがあっても、ブランチ名をそのまま読めて切り替えられることを確かめる。
    run_git_in_fixture(&fixture, &["tag", "feature"]);
    fixture
}

#[test]
fn reads_history_and_branches_then_switches_branch() {
    let fixture = create_fixture_with_merged_feature_branch();

    let snapshot = read_repository_snapshot(&fixture).unwrap();
    let summaries: Vec<&str> = snapshot
        .commits
        .iter()
        .map(|commit| commit.summary.as_str())
        .collect();
    assert_eq!(summaries.len(), 4);
    assert_eq!(summaries[0], "merge feature");
    assert_eq!(summaries[3], "base");
    let merge = &snapshot.commits[0];
    assert_eq!(merge.parent_ids.len(), 2);
    assert_eq!(merge.author_name, "Fixture Author");
    let branch_states: Vec<(&str, bool)> = snapshot
        .local_branches
        .iter()
        .map(|branch| (branch.name.as_str(), branch.is_current))
        .collect();
    assert_eq!(branch_states, [("feature", false), ("main", true)]);
    assert_eq!(snapshot.local_branches[1].head_commit_id, merge.id);

    switch_local_branch(&fixture, "feature").unwrap();
    let switched = read_repository_snapshot(&fixture).unwrap();
    let current: Vec<&str> = switched
        .local_branches
        .iter()
        .filter(|branch| branch.is_current)
        .map(|branch| branch.name.as_str())
        .collect();
    assert_eq!(current, ["feature"]);

    let error = switch_local_branch(&fixture, "no-such-branch").unwrap_err();
    assert!(
        error.message.contains("no-such-branch"),
        "git stderr reaches the caller: {}",
        error.message
    );

    std::fs::remove_dir_all(&fixture).unwrap();
}
