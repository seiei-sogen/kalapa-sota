use std::path::Path;

use crate::git_cli::GitCommandError;
use crate::git_cli::run_git_command::run_git_command;

/// メッセージが空のときと、何もステージしていないときは、git が理由を返して断る。
pub fn commit_staged_changes(
    repository_path: &Path,
    commit_message: &str,
) -> Result<(), GitCommandError> {
    run_git_command(
        repository_path,
        &["commit", "--quiet", "-m", commit_message],
    )
    .map(|_| ())
}
