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
        // git commit は、コミットするものがないとき status 全体を標準出力に書き、理由はその最後の行に書く。
        let message = if stderr.is_empty() {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .rfind(|line| !line.trim().is_empty())
                .unwrap_or_default()
                .trim()
                .to_string()
        } else {
            stderr
        };
        return Err(GitCommandError { message });
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
