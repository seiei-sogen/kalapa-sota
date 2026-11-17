use std::path::Path;

use crate::git_cli::GitCommandError;
use crate::git_cli::run_git_command::run_git_command;
use crate::repository::{ChangedFile, CommitInclusion, decide_commit_inclusion};

pub fn toggle_changed_file_in_commit(
    repository_path: &Path,
    changed_file: &ChangedFile,
) -> Result<(), GitCommandError> {
    // top はサブディレクトリで開いてもルートからのパスとして読ませ、literal は * や [ をグロブとして扱わせない。
    let pathspec = format!(":(top,literal){}", changed_file.path.0);
    let arguments: &[&str] = match decide_commit_inclusion(&changed_file.staging) {
        // git restore --staged は最初のコミットの前だと失敗するが、git reset は失敗しない。
        CommitInclusion::Included => &["reset", "-q", "--", &pathspec],
        CommitInclusion::PartlyIncluded | CommitInclusion::Excluded => &["add", "--", &pathspec],
    };
    run_git_command(repository_path, arguments).map(|_| ())
}
