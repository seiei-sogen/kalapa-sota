use std::path::{Path, PathBuf};

use super::run_git_command::run_git_command;
use super::{read_repository_snapshot, switch_local_branch};

fn run_git_in_fixture(fixture: &Path, arguments: &[&str]) {
    let identity = [
        "-c",
        "user.name=Fixture Author",
        "-c",
        "user.email=fixture@example.com",
        "-c",
        "commit.gpgsign=false",
    ];
    run_git_command(fixture, &[&identity[..], arguments].concat()).expect("fixture git command");
}

fn create_fixture_with_merged_feature_branch() -> PathBuf {
    let fixture = std::env::temp_dir().join(format!("kalapa-git-cli-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&fixture);
    std::fs::create_dir_all(&fixture).unwrap();
    run_git_in_fixture(&fixture, &["init", "-q", "-b", "main"]);
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
    fixture
}

#[test]
fn reads_history_and_branches_then_switches_branch() {
    let fixture = create_fixture_with_merged_feature_branch();

    let snapshot = read_repository_snapshot(&fixture).unwrap();
    let summaries: Vec<&str> = snapshot
        .commits
        .iter()
        .map(|c| c.summary.as_str())
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
        .map(|b| (b.name.as_str(), b.is_current))
        .collect();
    assert_eq!(branch_states, [("feature", false), ("main", true)]);
    assert_eq!(snapshot.local_branches[1].head_commit_id, merge.id);

    switch_local_branch(&fixture, "feature").unwrap();
    let switched = read_repository_snapshot(&fixture).unwrap();
    let current: Vec<&str> = switched
        .local_branches
        .iter()
        .filter(|b| b.is_current)
        .map(|b| b.name.as_str())
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
