use std::path::Path;

use crate::git_cli::GitCommandError;
use crate::git_cli::run_git_command::run_git_command;

pub fn switch_local_branch(
    repository_path: &Path,
    branch_name: &str,
) -> Result<(), GitCommandError> {
    run_git_command(repository_path, &["switch", branch_name]).map(|_| ())
}
