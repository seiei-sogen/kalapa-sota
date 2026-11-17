use std::path::Path;

use crate::git_cli::GitCommandError;
use crate::git_cli::run_git_command::run_git_command;

/// 上流ブランチがなければ、git が同じ名前でリモートに公開し、追跡を設定する。
pub fn push_current_branch(repository_path: &Path) -> Result<(), GitCommandError> {
    run_git_command(
        repository_path,
        &["-c", "push.autoSetupRemote=true", "push", "--quiet"],
    )
    .map(|_| ())
}
