use std::path::Path;

use super::GitCommandError;
use super::run_git_command::run_git_command;
use crate::repository::{Commit, CommitId};

pub fn read_commit_history(repository_path: &Path) -> Result<Vec<Commit>, GitCommandError> {
    // shortcut: 新しい順に 1000 件までしか読まない。古い履歴まで見たくなったら、スクロールに合わせて続きを読む。
    let output = run_git_command(
        repository_path,
        &[
            "log",
            "--exclude=refs/stash",
            "--all",
            "--topo-order",
            "--max-count=1000",
            "-z",
            "--date=format-local:%Y-%m-%d %H:%M",
            "--format=%H%x1f%P%x1f%an%x1f%cd%x1f%s",
        ],
    )?;
    output
        .split('\0')
        .filter(|record| !record.is_empty())
        .map(|record| {
            let fields: Vec<&str> = record.splitn(5, '\u{1f}').collect();
            let [id, parent_ids, author_name, committed_at, summary] = fields[..] else {
                return Err(GitCommandError {
                    message: format!("unexpected git log output: {record:?}"),
                });
            };
            Ok(Commit {
                id: CommitId(id.to_string()),
                parent_ids: parent_ids
                    .split_whitespace()
                    .map(|parent_id| CommitId(parent_id.to_string()))
                    .collect(),
                summary: summary.to_string(),
                author_name: author_name.to_string(),
                committed_at: committed_at.to_string(),
            })
        })
        .collect()
}
