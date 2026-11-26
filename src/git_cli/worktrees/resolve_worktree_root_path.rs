use std::path::{Path, PathBuf};

use crate::git_cli::GitCommandError;
use crate::git_cli::run_git_command::run_git_command;
use crate::repository::WorktreeRootPath;

pub fn resolve_worktree_root_path(path: &Path) -> Result<WorktreeRootPath, GitCommandError> {
    // bare リポジトリでは --show-toplevel が失敗する。--absolute-git-dir なら git worktree list の bare の行と同じパスになる。
    let root_query =
        if run_git_command(path, &["rev-parse", "--is-inside-work-tree"])?.trim() == "true" {
            "--show-toplevel"
        } else {
            "--absolute-git-dir"
        };
    let output = run_git_command(path, &["rev-parse", root_query])?;
    Ok(WorktreeRootPath(PathBuf::from(
        output.strip_suffix('\n').unwrap_or(&output),
    )))
}
