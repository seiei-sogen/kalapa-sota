use std::path::Path;

use crate::git_cli::GitCommandError;
use crate::git_cli::run_git_command::run_git_command;

pub fn pull_current_branch_fast_forward_only(
    repository_path: &Path,
) -> Result<(), GitCommandError> {
    run_git_command(
        repository_path,
        &[
            // 枝分かれしたときに git が足すヒントの 9 行を、赤い帯に出さない。
            "-c",
            "advice.diverging=false",
            "pull",
            // 古い git は、pull.rebase=true を --ff-only より優先して rebase を始めてしまう。
            "--no-rebase",
            "--ff-only",
            "--quiet",
        ],
    )
    .map(|_| ())
}
