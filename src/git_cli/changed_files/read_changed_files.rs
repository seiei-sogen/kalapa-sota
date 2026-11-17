use std::path::Path;

use super::parse_porcelain_status::parse_porcelain_status;
use crate::git_cli::GitCommandError;
use crate::git_cli::run_git_command::run_git_command;
use crate::repository::ChangedFile;

pub fn read_changed_files(repository_path: &Path) -> Result<Vec<ChangedFile>, GitCommandError> {
    // ベアリポジトリや .git の中では git status が失敗するが、履歴とブランチは見せたいので、変更なしとして読む。
    if run_git_command(repository_path, &["rev-parse", "--is-inside-work-tree"])?.trim() != "true" {
        return Ok(Vec::new());
    }
    // shortcut: UTF-8 でないファイル名は置換文字に化け、そのファイルはステージできない。必要になったらパスを OsString で持つ。
    let output = run_git_command(
        repository_path,
        &[
            // 読むだけなので index.lock を取らない。利用者が端末で同時に打った git を失敗させないため。
            "--no-optional-locks",
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=all",
            "--no-renames",
        ],
    )?;
    parse_porcelain_status(&output)
}
