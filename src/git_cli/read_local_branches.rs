use std::path::Path;

use super::GitCommandError;
use super::run_git_command::run_git_command;
use crate::repository::{CommitId, LocalBranch};

pub fn read_local_branches(repository_path: &Path) -> Result<Vec<LocalBranch>, GitCommandError> {
    let output = run_git_command(
        repository_path,
        &[
            "for-each-ref",
            // refname:short は同名のタグがあると "heads/名前" を返し、git switch が受け付けない。
            "--format=%(HEAD)%1f%(objectname)%1f%(refname:lstrip=2)",
            "refs/heads",
        ],
    )?;
    output
        .lines()
        .map(|line| {
            let fields: Vec<&str> = line.split('\u{1f}').collect();
            let [head_marker, head_commit_id, name] = fields[..] else {
                return Err(GitCommandError {
                    message: format!("unexpected git for-each-ref output: {line:?}"),
                });
            };
            Ok(LocalBranch {
                name: name.to_string(),
                head_commit_id: CommitId(head_commit_id.to_string()),
                is_current: head_marker == "*",
            })
        })
        .collect()
}
