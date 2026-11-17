use std::path::Path;
use std::process::Command;

use super::GitCommandError;

pub fn run_git_command(
    repository_path: &Path,
    arguments: &[&str],
) -> Result<String, GitCommandError> {
    let output = Command::new("git")
        // 認証を端末で尋ねられても GUI からは答えられず、git が待ち続けてしまう。
        .env("GIT_TERMINAL_PROMPT", "0")
        .arg("-C")
        .arg(repository_path)
        .args(arguments)
        .output()
        .map_err(|error| GitCommandError {
            message: format!("could not run git: {error}"),
        })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        // git commit は、コミットするものがないときの理由を標準出力にだけ書く。
        let message = if stderr.is_empty() {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            stderr
        };
        return Err(GitCommandError { message });
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
