use std::path::Path;
use std::process::Command;

use super::GitCommandError;

pub fn run_git_command(
    repository_path: &Path,
    arguments: &[&str],
) -> Result<String, GitCommandError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repository_path)
        .args(arguments)
        .output()
        .map_err(|error| GitCommandError {
            message: format!("could not run git: {error}"),
        })?;
    if !output.status.success() {
        return Err(GitCommandError {
            message: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
